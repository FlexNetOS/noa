#!/usr/bin/env python3
"""
Spec-Kit MCP Tool for Provider Orchestration (SK010)

Implements an MCP (Model Context Protocol) tool that enables AI providers
to orchestrate spec access and provider coordination.

Constitutional reference: §3.13, FR-037
"""

import json
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional, Union


@dataclass
class MCPToolDefinition:
    """Definition of an MCP tool."""
    name: str
    description: str
    parameters: Dict[str, Any]

    def to_dict(self) -> Dict[str, Any]:
        return {
            "name": self.name,
            "description": self.description,
            "inputSchema": {
                "type": "object",
                "properties": self.parameters,
                "required": [k for k, v in self.parameters.items() if v.get("required", False)],
            },
        }


@dataclass
class MCPToolResult:
    """Result of an MCP tool invocation."""
    success: bool
    content: Any
    error: Optional[str] = None

    def to_dict(self) -> Dict[str, Any]:
        result = {
            "success": self.success,
            "content": self.content,
        }
        if self.error:
            result["error"] = self.error
        return result


class SpecKitMCPServer:
    """
    MCP Server for Spec-Kit provider orchestration.

    Provides tools for:
    - Connecting providers to specs
    - Broadcasting specs to all providers
    - Managing provider locks
    - Checking provider health
    - Querying execution memory
    """

    def __init__(self, noa_root: Optional[Path] = None):
        """Initialize the MCP server."""
        import os
        self.noa_root = noa_root or Path(os.environ.get("NOA_ROOT", Path.cwd()))
        self._tools: Dict[str, Callable] = {}
        self._register_tools()

    def _register_tools(self) -> None:
        """Register all available tools."""
        self._tools = {
            "connect_provider": self._tool_connect_provider,
            "disconnect_provider": self._tool_disconnect_provider,
            "broadcast_spec": self._tool_broadcast_spec,
            "acquire_lock": self._tool_acquire_lock,
            "release_lock": self._tool_release_lock,
            "check_health": self._tool_check_health,
            "get_providers": self._tool_get_providers,
            "get_active_spec": self._tool_get_active_spec,
            "create_task": self._tool_create_task,
            "get_tasks": self._tool_get_tasks,
        }

    def get_tool_definitions(self) -> List[MCPToolDefinition]:
        """Get all available tool definitions."""
        return [
            MCPToolDefinition(
                name="connect_provider",
                description="Connect a provider to the shared spec distribution system",
                parameters={
                    "provider_id": {
                        "type": "string",
                        "description": "Provider identifier (e.g., 'claude-code', 'codex')",
                        "required": True,
                    },
                    "spec_path": {
                        "type": "string",
                        "description": "Path to the spec (relative to noa_root)",
                        "required": True,
                    },
                    "access_mode": {
                        "type": "string",
                        "description": "Access mode: 'read', 'write', or 'coordinate'",
                        "enum": ["read", "write", "coordinate"],
                        "default": "read",
                    },
                },
            ),
            MCPToolDefinition(
                name="disconnect_provider",
                description="Disconnect a provider from the shared spec distribution system",
                parameters={
                    "provider_id": {
                        "type": "string",
                        "description": "Provider identifier to disconnect",
                        "required": True,
                    },
                },
            ),
            MCPToolDefinition(
                name="broadcast_spec",
                description="Broadcast a spec to all connected providers in parallel",
                parameters={
                    "spec_path": {
                        "type": "string",
                        "description": "Path to the spec to broadcast",
                        "required": True,
                    },
                    "provider_ids": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Specific providers to broadcast to (optional, default: all connected)",
                    },
                },
            ),
            MCPToolDefinition(
                name="acquire_lock",
                description="Acquire a lock on a spec for write coordination",
                parameters={
                    "spec_path": {
                        "type": "string",
                        "description": "Path to the spec to lock",
                        "required": True,
                    },
                    "provider_id": {
                        "type": "string",
                        "description": "Provider requesting the lock",
                        "required": True,
                    },
                    "lock_type": {
                        "type": "string",
                        "description": "Lock type: 'read', 'write', or 'exclusive'",
                        "enum": ["read", "write", "exclusive"],
                        "default": "write",
                    },
                },
            ),
            MCPToolDefinition(
                name="release_lock",
                description="Release a lock on a spec",
                parameters={
                    "spec_path": {
                        "type": "string",
                        "description": "Path to the spec",
                        "required": True,
                    },
                    "provider_id": {
                        "type": "string",
                        "description": "Provider releasing the lock",
                        "required": True,
                    },
                },
            ),
            MCPToolDefinition(
                name="check_health",
                description="Check health of providers before spec distribution",
                parameters={
                    "provider_ids": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Providers to check (optional, default: all known)",
                    },
                },
            ),
            MCPToolDefinition(
                name="get_providers",
                description="Get list of connected providers",
                parameters={
                    "status": {
                        "type": "string",
                        "description": "Filter by status: 'connected', 'disconnected', 'all'",
                        "default": "all",
                    },
                },
            ),
            MCPToolDefinition(
                name="get_active_spec",
                description="Get information about the currently active spec",
                parameters={},
            ),
            MCPToolDefinition(
                name="create_task",
                description="Create a distributed task for provider execution",
                parameters={
                    "task_id": {
                        "type": "string",
                        "description": "Task identifier (optional, auto-generated if not provided)",
                    },
                    "priority": {
                        "type": "integer",
                        "description": "Task priority (higher = more important)",
                        "default": 0,
                    },
                    "assigned_provider": {
                        "type": "string",
                        "description": "Provider to assign task to (optional)",
                    },
                },
            ),
            MCPToolDefinition(
                name="get_tasks",
                description="Get pending tasks from the distributed task queue",
                parameters={
                    "provider_id": {
                        "type": "string",
                        "description": "Filter by assigned provider (optional)",
                    },
                    "status": {
                        "type": "string",
                        "description": "Filter by status: 'pending', 'in-progress', 'done'",
                        "default": "pending",
                    },
                },
            ),
        ]

    def invoke_tool(
        self,
        name: str,
        arguments: Dict[str, Any],
    ) -> MCPToolResult:
        """
        Invoke an MCP tool.

        Args:
            name: Tool name
            arguments: Tool arguments

        Returns:
            MCPToolResult with success status and content
        """
        if name not in self._tools:
            return MCPToolResult(
                success=False,
                content=None,
                error=f"Unknown tool: {name}",
            )

        try:
            result = self._tools[name](arguments)
            return MCPToolResult(
                success=True,
                content=result,
            )
        except Exception as e:
            return MCPToolResult(
                success=False,
                content=None,
                error=str(e),
            )

    # Tool implementations

    def _tool_connect_provider(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Connect a provider to the spec distribution system."""
        from .providers import connect_provider

        connection = connect_provider(
            provider_id=args["provider_id"],
            spec_path=args["spec_path"],
            access_mode=args.get("access_mode", "read"),
            noa_root=self.noa_root,
        )

        return connection.to_dict()

    def _tool_disconnect_provider(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Disconnect a provider from the spec distribution system."""
        from .providers import disconnect_provider

        success = disconnect_provider(
            provider_id=args["provider_id"],
            noa_root=self.noa_root,
        )

        return {"success": success, "providerId": args["provider_id"]}

    def _tool_broadcast_spec(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Broadcast a spec to providers."""
        from .broadcast import broadcast_spec

        report = broadcast_spec(
            spec_path=args["spec_path"],
            provider_ids=args.get("provider_ids"),
            noa_root=self.noa_root,
        )

        return report.to_dict()

    def _tool_acquire_lock(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Acquire a lock on a spec."""
        from .locks import acquire_spec_lock

        result = acquire_spec_lock(
            spec_path=args["spec_path"],
            provider_id=args["provider_id"],
            lock_type=args.get("lock_type", "write"),
            noa_root=self.noa_root,
        )

        return {
            "success": result.success,
            "message": result.message,
            "lockInfo": result.lock_info.to_dict() if result.lock_info else None,
        }

    def _tool_release_lock(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Release a lock on a spec."""
        from .locks import release_spec_lock

        success = release_spec_lock(
            spec_path=args["spec_path"],
            provider_id=args["provider_id"],
            noa_root=self.noa_root,
        )

        return {"success": success}

    def _tool_check_health(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Check provider health."""
        from .health import check_all_provider_health

        report = check_all_provider_health(
            provider_ids=args.get("provider_ids"),
            noa_root=self.noa_root,
        )

        return report.to_dict()

    def _tool_get_providers(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Get connected providers."""
        from .providers import get_connected_providers

        providers = get_connected_providers(self.noa_root)

        status_filter = args.get("status", "all")
        if status_filter != "all":
            providers = [p for p in providers if p.status == status_filter]

        return {
            "providers": [p.to_dict() for p in providers],
            "count": len(providers),
        }

    def _tool_get_active_spec(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Get active spec information."""
        from .providers import ProviderConnectionManager

        manager = ProviderConnectionManager(self.noa_root)
        active_spec = manager.get_active_spec()

        return active_spec or {"path": None, "hash": None, "files": []}

    def _tool_create_task(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Create a distributed task."""
        from .memory import get_execution_memory

        db = get_execution_memory(self.noa_root)
        task = db.create_task(
            task_id=args.get("task_id"),
            priority=args.get("priority", 0),
            assigned_provider=args.get("assigned_provider"),
        )

        return {
            "taskId": task.task_id,
            "status": task.status,
            "priority": task.priority,
            "assignedProvider": task.assigned_provider,
            "createdAt": task.created_at.isoformat(),
        }

    def _tool_get_tasks(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Get tasks from the queue."""
        from .memory import get_execution_memory

        db = get_execution_memory(self.noa_root)
        tasks = db.get_pending_tasks(
            provider=args.get("provider_id"),
        )

        return {
            "tasks": [
                {
                    "taskId": t.task_id,
                    "status": t.status,
                    "priority": t.priority,
                    "assignedProvider": t.assigned_provider,
                    "createdAt": t.created_at.isoformat(),
                }
                for t in tasks
            ],
            "count": len(tasks),
        }


# MCP Server instance
_server: Optional[SpecKitMCPServer] = None


def get_mcp_server(noa_root: Optional[Path] = None) -> SpecKitMCPServer:
    """Get or create the MCP server instance."""
    global _server
    if _server is None:
        _server = SpecKitMCPServer(noa_root)
    return _server


def get_tool_definitions() -> List[Dict[str, Any]]:
    """Get all tool definitions as JSON-serializable dicts."""
    server = get_mcp_server()
    return [t.to_dict() for t in server.get_tool_definitions()]


def invoke_tool(name: str, arguments: Dict[str, Any]) -> Dict[str, Any]:
    """
    Invoke an MCP tool by name.

    This is the primary entry point for SK010 - MCP Tool for Orchestration.

    Args:
        name: Tool name
        arguments: Tool arguments

    Returns:
        Result dictionary with success status and content

    Example:
        >>> from specify_cli.mcp import invoke_tool
        >>> result = invoke_tool("connect_provider", {
        ...     "provider_id": "claude-code",
        ...     "spec_path": "specs/001-noa-seed-foundation"
        ... })
        >>> print(result["success"])
        True
    """
    server = get_mcp_server()
    result = server.invoke_tool(name, arguments)
    return result.to_dict()


# JSON-RPC style interface for MCP compatibility
def handle_mcp_request(request: Dict[str, Any]) -> Dict[str, Any]:
    """
    Handle an MCP JSON-RPC request.

    Args:
        request: JSON-RPC request with method and params

    Returns:
        JSON-RPC response
    """
    method = request.get("method", "")
    params = request.get("params", {})
    request_id = request.get("id")

    if method == "tools/list":
        return {
            "jsonrpc": "2.0",
            "id": request_id,
            "result": {"tools": get_tool_definitions()},
        }

    elif method == "tools/call":
        tool_name = params.get("name", "")
        arguments = params.get("arguments", {})
        result = invoke_tool(tool_name, arguments)

        return {
            "jsonrpc": "2.0",
            "id": request_id,
            "result": {
                "content": [
                    {
                        "type": "text",
                        "text": json.dumps(result, indent=2),
                    }
                ],
                "isError": not result.get("success", True),
            },
        }

    else:
        return {
            "jsonrpc": "2.0",
            "id": request_id,
            "error": {
                "code": -32601,
                "message": f"Method not found: {method}",
            },
        }

