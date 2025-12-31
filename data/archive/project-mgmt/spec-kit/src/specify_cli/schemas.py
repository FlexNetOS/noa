#!/usr/bin/env python3
"""
Schema Validation for Spec-Kit (SK002)

Implements JSON schema validation for spec-distribution.json and related
configuration files.

Constitutional reference: §3.13, FR-037
"""

import json
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple


@dataclass
class ValidationResult:
    """Result of a schema validation."""
    valid: bool
    errors: List[str]
    warnings: List[str]

    def __bool__(self) -> bool:
        return self.valid


# JSON Schema for spec-distribution.json
SPEC_DISTRIBUTION_SCHEMA = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "required": ["version", "activeSpec", "connectedProviders", "parallelExecution"],
    "properties": {
        "$schema": {"type": "string"},
        "version": {"type": "string", "pattern": r"^\d+\.\d+\.\d+$"},
        "description": {"type": "string"},
        "lastUpdated": {"type": "string", "format": "date-time"},
        "activeSpec": {
            "type": "object",
            "properties": {
                "path": {"type": ["string", "null"]},
                "hash": {"type": ["string", "null"]},
                "files": {"type": "array", "items": {"type": "string"}},
                "lastAccessed": {"type": ["string", "null"], "format": "date-time"},
                "accessCount": {"type": "integer", "minimum": 0},
            },
        },
        "connectedProviders": {
            "type": "array",
            "items": {
                "type": "object",
                "required": ["providerId", "accessMode", "connectedAt"],
                "properties": {
                    "providerId": {"type": "string"},
                    "providerType": {
                        "type": "string",
                        "enum": ["local", "cloud", "hybrid", "ide", "unknown"],
                    },
                    "accessMode": {
                        "type": "string",
                        "enum": ["read", "write", "coordinate"],
                    },
                    "connectedAt": {"type": "string", "format": "date-time"},
                    "lastSync": {"type": ["string", "null"], "format": "date-time"},
                    "status": {
                        "type": "string",
                        "enum": ["connected", "disconnected", "error", "syncing"],
                    },
                    "capabilities": {
                        "type": "array",
                        "items": {"type": "string"},
                    },
                    "specPath": {"type": ["string", "null"]},
                    "parallelEnabled": {"type": "boolean"},
                },
            },
        },
        "parallelExecution": {
            "type": "object",
            "properties": {
                "enabled": {"type": "boolean"},
                "coordinator": {"type": "string"},
                "coordinatorEntry": {"type": "string"},
                "taskDistribution": {
                    "type": "string",
                    "enum": ["round-robin", "priority", "capability-based"],
                },
                "maxConcurrentProviders": {"type": "integer", "minimum": 1},
                "syncInterval": {"type": "integer", "minimum": 100},
            },
        },
        "lockingPolicy": {
            "type": "object",
            "properties": {
                "enabled": {"type": "boolean"},
                "lockTimeout": {"type": "integer", "minimum": 1000},
                "maxWaitTime": {"type": "integer", "minimum": 1000},
                "retryAttempts": {"type": "integer", "minimum": 0},
            },
        },
        "auditConfig": {
            "type": "object",
            "properties": {
                "enabled": {"type": "boolean"},
                "logPath": {"type": "string"},
                "retentionDays": {"type": "integer", "minimum": 1},
                "logLevel": {
                    "type": "string",
                    "enum": ["debug", "info", "warn", "error"],
                },
            },
        },
    },
}


# JSON Schema for provider connection
PROVIDER_CONNECTION_SCHEMA = {
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "required": ["providerId", "accessMode", "connectedAt"],
    "properties": {
        "providerId": {
            "type": "string",
            "minLength": 1,
            "pattern": r"^[a-z0-9-]+$",
        },
        "providerType": {
            "type": "string",
            "enum": ["local", "cloud", "hybrid", "ide", "unknown"],
        },
        "accessMode": {
            "type": "string",
            "enum": ["read", "write", "coordinate"],
        },
        "connectedAt": {"type": "string"},
        "lastSync": {"type": ["string", "null"]},
        "status": {
            "type": "string",
            "enum": ["connected", "disconnected", "error", "syncing"],
        },
        "capabilities": {
            "type": "array",
            "items": {
                "type": "string",
                "enum": [
                    "reasoning",
                    "code_generation",
                    "code_completion",
                    "analysis",
                    "tool_use",
                    "long_context",
                    "local_inference",
                ],
            },
        },
        "specPath": {"type": ["string", "null"]},
        "parallelEnabled": {"type": "boolean"},
    },
}


def validate_spec_distribution(data: Dict[str, Any]) -> ValidationResult:
    """
    Validate spec-distribution.json against schema.

    Args:
        data: Parsed JSON data to validate

    Returns:
        ValidationResult with valid flag, errors, and warnings
    """
    errors = []
    warnings = []

    # Check required fields
    required_fields = ["version", "activeSpec", "connectedProviders", "parallelExecution"]
    for field in required_fields:
        if field not in data:
            errors.append(f"Missing required field: {field}")

    if errors:
        return ValidationResult(valid=False, errors=errors, warnings=warnings)

    # Validate version format
    version = data.get("version", "")
    if not _validate_semver(version):
        errors.append(f"Invalid version format: {version}. Expected semver (e.g., '1.0.0')")

    # Validate activeSpec
    active_spec = data.get("activeSpec", {})
    if active_spec.get("path") and not isinstance(active_spec["path"], str):
        errors.append("activeSpec.path must be a string or null")

    if active_spec.get("files") and not isinstance(active_spec["files"], list):
        errors.append("activeSpec.files must be an array")

    # Validate connectedProviders
    providers = data.get("connectedProviders", [])
    if not isinstance(providers, list):
        errors.append("connectedProviders must be an array")
    else:
        for i, provider in enumerate(providers):
            provider_result = validate_provider_connection(provider)
            for error in provider_result.errors:
                errors.append(f"connectedProviders[{i}]: {error}")
            for warning in provider_result.warnings:
                warnings.append(f"connectedProviders[{i}]: {warning}")

    # Validate parallelExecution
    parallel = data.get("parallelExecution", {})
    if parallel.get("maxConcurrentProviders"):
        max_providers = parallel["maxConcurrentProviders"]
        if not isinstance(max_providers, int) or max_providers < 1:
            errors.append("parallelExecution.maxConcurrentProviders must be >= 1")

    task_dist = parallel.get("taskDistribution", "")
    valid_distributions = ["round-robin", "priority", "capability-based"]
    if task_dist and task_dist not in valid_distributions:
        errors.append(f"parallelExecution.taskDistribution must be one of: {valid_distributions}")

    # Validate lockingPolicy
    locking = data.get("lockingPolicy", {})
    if locking.get("lockTimeout"):
        if not isinstance(locking["lockTimeout"], int) or locking["lockTimeout"] < 1000:
            warnings.append("lockingPolicy.lockTimeout should be >= 1000ms")

    # Check for deprecated fields
    if "_schema" in data:
        warnings.append("_schema field is deprecated documentation; consider removing")

    return ValidationResult(
        valid=len(errors) == 0,
        errors=errors,
        warnings=warnings,
    )


def validate_provider_connection(data: Dict[str, Any]) -> ValidationResult:
    """
    Validate a provider connection entry.

    Args:
        data: Provider connection data to validate

    Returns:
        ValidationResult with valid flag, errors, and warnings
    """
    errors = []
    warnings = []

    # Check required fields
    required = ["providerId", "accessMode", "connectedAt"]
    for field in required:
        if field not in data:
            errors.append(f"Missing required field: {field}")

    if errors:
        return ValidationResult(valid=False, errors=errors, warnings=warnings)

    # Validate providerId format
    provider_id = data.get("providerId", "")
    if not _validate_provider_id(provider_id):
        errors.append(f"Invalid providerId format: {provider_id}. Use lowercase alphanumeric with hyphens.")

    # Validate accessMode
    access_mode = data.get("accessMode", "")
    valid_modes = ["read", "write", "coordinate"]
    if access_mode not in valid_modes:
        errors.append(f"accessMode must be one of: {valid_modes}")

    # Validate providerType if present
    provider_type = data.get("providerType")
    if provider_type:
        valid_types = ["local", "cloud", "hybrid", "ide", "unknown"]
        if provider_type not in valid_types:
            errors.append(f"providerType must be one of: {valid_types}")

    # Validate status if present
    status = data.get("status")
    if status:
        valid_statuses = ["connected", "disconnected", "error", "syncing"]
        if status not in valid_statuses:
            errors.append(f"status must be one of: {valid_statuses}")

    # Validate capabilities if present
    capabilities = data.get("capabilities", [])
    valid_caps = [
        "reasoning", "code_generation", "code_completion",
        "analysis", "tool_use", "long_context", "local_inference",
    ]
    for cap in capabilities:
        if cap not in valid_caps:
            warnings.append(f"Unknown capability: {cap}")

    # Validate datetime fields
    for field in ["connectedAt", "lastSync"]:
        value = data.get(field)
        if value and not _validate_iso8601(value):
            errors.append(f"{field} must be a valid ISO8601 datetime")

    return ValidationResult(
        valid=len(errors) == 0,
        errors=errors,
        warnings=warnings,
    )


def validate_file(filepath: Path) -> ValidationResult:
    """
    Validate a spec-distribution JSON file.

    Args:
        filepath: Path to the JSON file

    Returns:
        ValidationResult
    """
    errors = []
    warnings = []

    if not filepath.exists():
        return ValidationResult(
            valid=False,
            errors=[f"File not found: {filepath}"],
            warnings=[],
        )

    try:
        with open(filepath) as f:
            data = json.load(f)
    except json.JSONDecodeError as e:
        return ValidationResult(
            valid=False,
            errors=[f"Invalid JSON: {e}"],
            warnings=[],
        )

    return validate_spec_distribution(data)


def _validate_semver(version: str) -> bool:
    """Validate semantic version format."""
    import re
    return bool(re.match(r"^\d+\.\d+\.\d+(-[\w.]+)?(\+[\w.]+)?$", version))


def _validate_provider_id(provider_id: str) -> bool:
    """Validate provider ID format (lowercase alphanumeric with hyphens)."""
    import re
    return bool(re.match(r"^[a-z0-9][a-z0-9-]*[a-z0-9]$|^[a-z0-9]$", provider_id))


def _validate_iso8601(value: str) -> bool:
    """Validate ISO8601 datetime format."""
    try:
        datetime.fromisoformat(value.replace("Z", "+00:00"))
        return True
    except (ValueError, AttributeError):
        return False


# Schema registry for extensibility
SCHEMAS: Dict[str, Dict[str, Any]] = {
    "spec-distribution": SPEC_DISTRIBUTION_SCHEMA,
    "provider-connection": PROVIDER_CONNECTION_SCHEMA,
}


def get_schema(schema_name: str) -> Optional[Dict[str, Any]]:
    """Get a schema by name."""
    return SCHEMAS.get(schema_name)


def register_schema(name: str, schema: Dict[str, Any]) -> None:
    """Register a custom schema."""
    SCHEMAS[name] = schema

