# Rust Lovable - API Reference

## Overview

This document provides comprehensive API documentation for Rust Lovable, including all endpoints, request/response formats, and usage examples.

## Authentication

All API endpoints require authentication via API key passed in the `Authorization` header:

```
Authorization: Bearer <your-api-key>
```

## Base URL

```
https://api.rust-lovable.com/v1
```

## Project Management

### Create Project

**Endpoint**: `POST /api/v1/projects`

**Request Body**:
```json
{
  "name": "My Awesome App",
  "description": "A modern web application",
  "platform": "web",
  "template": "react-vite"
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "id": "proj_123456",
    "name": "My Awesome App",
    "description": "A modern web application",
    "platform": "web",
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-15T10:30:00Z",
    "pages": []
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### List Projects

**Endpoint**: `GET /api/v1/projects`

**Query Parameters**:
- `limit` (integer): Number of projects to return (default: 50)
- `offset` (integer): Pagination offset (default: 0)
- `platform` (string): Filter by platform

**Response**:
```json
{
  "success": true,
  "data": [
    {
      "id": "proj_123456",
      "name": "My Awesome App",
      "description": "A modern web application",
      "platform": "web",
      "created_at": "2024-01-15T10:30:00Z",
      "updated_at": "2024-01-15T10:30:00Z",
      "pages": []
    }
  ],
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Get Project

**Endpoint**: `GET /api/v1/projects/{project_id}`

**Response**:
```json
{
  "success": true,
  "data": {
    "id": "proj_123456",
    "name": "My Awesome App",
    "description": "A modern web application",
    "platform": "web",
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-15T10:30:00Z",
    "pages": [
      {
        "id": "page_789",
        "name": "Home",
        "path": "/",
        "components": []
      }
    ]
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Update Project

**Endpoint**: `PUT /api/v1/projects/{project_id}`

**Request Body**:
```json
{
  "name": "Updated App Name",
  "description": "Updated description"
}
```

### Delete Project

**Endpoint**: `DELETE /api/v1/projects/{project_id}`

**Response**:
```json
{
  "success": true,
  "data": null,
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

## AI Integration

### Process Message

**Endpoint**: `POST /api/v1/ai/process`

**Request Body**:
```json
{
  "project_id": "proj_123456",
  "message": "Create a modern landing page with a hero section",
  "context": {
    "current_page": "home",
    "selected_component": null
  }
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "response": "I'll create a modern landing page with a hero section featuring a gradient background and call-to-action button.",
    "ui_changes": [
      {
        "description": "Add hero section with gradient background",
        "target_component": null,
        "change_type": "create_component",
        "platform_specific": {
          "mobile": {
            "height": "50vh"
          }
        }
      }
    ],
    "confidence": 0.95
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Generate Code

**Endpoint**: `POST /api/v1/ai/generate`

**Request Body**:
```json
{
  "project_id": "proj_123456",
  "component_id": "comp_789",
  "platform": "web"
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "code": "import React from 'react';\n\nexport default function HeroSection() {\n  return (\n    <div className=\"hero\">\n      <h1>Welcome to Our App</h1>\n      <button>Get Started</button>\n    </div>\n  );\n}",
    "language": "javascript",
    "platform": "web",
    "dependencies": ["react", "tailwindcss"]
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

## Streaming Endpoints

### Generate AI Code Stream

**Endpoint**: `GET /api/v1/stream/ai/generate/{project_id}`

**Server-Sent Events**:
```
event: progress
data: {"step": 1, "total": 6, "message": "Analyzing requirements...", "percentage": 17}

event: progress
data: {"step": 2, "total": 6, "message": "Generating component structure...", "percentage": 33}

event: complete
data: {"code": "// Generated code", "components": ["Button", "Form"], "dependencies": ["react"]}
```

### Apply AI Code Stream

**Endpoint**: `GET /api/v1/stream/ai/apply/{project_id}/{component_id}`

### Monitor Vite Logs

**Endpoint**: `GET /api/v1/stream/vite/{sandbox_id}`

**Server-Sent Events**:
```
event: vite_log
data: {"level": "info", "message": "[vite] Server running at http://localhost:5173", "timestamp": "2024-01-15T10:30:00Z"}

event: vite_error
data: {"level": "error", "message": "Module not found", "file": "/src/App.tsx", "line": 3, "column": 8}
```

## Component Management

### Create Component

**Endpoint**: `POST /api/v1/projects/{project_id}/components`

**Request Body**:
```json
{
  "component_type": "Button",
  "properties": {
    "text": "Click me",
    "variant": "primary",
    "size": "medium"
  },
  "parent_id": null
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "id": "comp_789",
    "component_type": "Button",
    "properties": {
      "text": "Click me",
      "variant": "primary",
      "size": "medium"
    },
    "children": []
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### List Components

**Endpoint**: `GET /api/v1/projects/{project_id}/components`

### Update Component

**Endpoint**: `PUT /api/v1/projects/{project_id}/components/{component_id}`

### Delete Component

**Endpoint**: `DELETE /api/v1/projects/{project_id}/components/{component_id}`

## Sandbox Management

### Create Sandbox

**Endpoint**: `POST /api/v1/sandboxes`

**Request Body**:
```json
{
  "platform": "web",
  "requirements": ["react", "tailwindcss"],
  "template": "vite-react"
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "sandbox_id": "sb_123456",
    "status": "created",
    "endpoint": "http://localhost:8080/sandbox/sb_123456",
    "created_at": "2024-01-15T10:30:00Z"
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Execute Code

**Endpoint**: `POST /api/v1/sandboxes/{sandbox_id}/execute`

**Request Body**:
```json
{
  "code": "console.log('Hello, World!');",
  "language": "javascript",
  "timeout": 30,
  "input": null
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "execution_id": "exec_789",
    "result": {
      "success": true,
      "output": "Hello, World!\n",
      "error": null,
      "exit_code": 0,
      "execution_time": 150
    },
    "logs": ["Execution started", "Execution completed"],
    "duration": 150
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Get Sandbox Status

**Endpoint**: `GET /api/v1/sandboxes/{sandbox_id}`

### Get Sandbox Logs

**Endpoint**: `GET /api/v1/sandboxes/{sandbox_id}/logs`

### Kill Sandbox

**Endpoint**: `DELETE /api/v1/sandboxes/{sandbox_id}`

## File Operations

### Get Sandbox Files

**Endpoint**: `GET /api/v1/sandboxes/{sandbox_id}/files`

**Response**:
```json
{
  "success": true,
  "data": {
    "sandbox_id": "sb_123456",
    "current_path": "/",
    "files": [
      {
        "name": "src",
        "path": "/src",
        "is_directory": true,
        "size": null,
        "modified_at": "2024-01-15T10:30:00Z",
        "permissions": "755"
      },
      {
        "name": "package.json",
        "path": "/package.json",
        "is_directory": false,
        "size": 2048,
        "modified_at": "2024-01-15T10:30:00Z",
        "permissions": "644"
      }
    ],
    "total_size": 3584,
    "file_count": 3
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Read File

**Endpoint**: `POST /api/v1/sandboxes/{sandbox_id}/files/read`

**Request Body**:
```json
{
  "file_path": "/package.json"
}
```

### Write File

**Endpoint**: `POST /api/v1/sandboxes/{sandbox_id}/files/write`

**Request Body**:
```json
{
  "file_path": "/src/App.js",
  "content": "import React from 'react';\n\nexport default function App() {\n  return <h1>Hello World</h1>;\n}",
  "create_if_not_exists": true
}
```

### Delete File

**Endpoint**: `DELETE /api/v1/sandboxes/{sandbox_id}/files`

### Create Directory

**Endpoint**: `POST /api/v1/sandboxes/{sandbox_id}/directories`

### Search Files

**Endpoint**: `POST /api/v1/sandboxes/{sandbox_id}/search`

**Request Body**:
```json
{
  "query": "className",
  "file_pattern": "*.js",
  "case_sensitive": false
}
```

## Package Management

### Detect and Install Packages

**Endpoint**: `POST /api/v1/sandboxes/{sandbox_id}/packages/detect`

**Request Body**:
```json
{
  "code": "import React from 'react';\nimport { useState } from 'react';\nimport * as lodash from 'lodash';",
  "file_path": "/src/App.js"
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "detected_packages": [
      {
        "name": "react",
        "package_type": "dependency",
        "usage_count": 2,
        "import_examples": ["import React from 'react'", "import { useState } from 'react'"],
        "confidence": 0.9
      },
      {
        "name": "lodash",
        "package_type": "dependency",
        "usage_count": 1,
        "import_examples": ["import * as lodash from 'lodash'"],
        "confidence": 0.9
      }
    ],
    "package_manager": "npm",
    "confidence": 0.85
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Install Packages

**Endpoint**: `POST /api/v1/sandboxes/{sandbox_id}/packages/install`

**Request Body**:
```json
{
  "packages": ["react", "tailwindcss"],
  "package_manager": "npm",
  "install_dev": false
}
```

## Vite Integration

### Check Vite Errors

**Endpoint**: `GET /api/v1/sandboxes/{sandbox_id}/vite/errors`

**Response**:
```json
{
  "success": true,
  "data": {
    "has_errors": true,
    "error_count": 2,
    "errors": [
      {
        "id": "err_001",
        "message": "Module not found: Can't resolve 'react'",
        "file": "/src/App.tsx",
        "line": 3,
        "column": 8,
        "severity": "error",
        "stack_trace": "Error: Cannot find module 'react'\n    at ...",
        "timestamp": "2024-01-15T10:30:00Z",
        "resolved": false
      }
    ],
    "last_check": "2024-01-15T10:30:00Z"
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Report Vite Error

**Endpoint**: `POST /api/v1/sandboxes/{sandbox_id}/vite/errors`

### Clear Vite Errors Cache

**Endpoint**: `DELETE /api/v1/sandboxes/{sandbox_id}/vite/errors`

### Restart Vite

**Endpoint**: `POST /api/v1/sandboxes/{sandbox_id}/vite/restart`

## Export and Deployment

### Create ZIP Export

**Endpoint**: `POST /api/v1/export/zip`

**Request Body**:
```json
{
  "project_id": "proj_123456",
  "include_node_modules": false,
  "include_build": true,
  "format": "zip"
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "download_id": "dl_789",
    "download_url": "/api/v1/download/dl_789",
    "file_size": 5242880,
    "file_count": 25,
    "expires_at": "2024-01-16T10:30:00Z"
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Download ZIP

**Endpoint**: `GET /api/v1/download/{download_id}`

### Export to GitHub

**Endpoint**: `POST /api/v1/export/github`

**Request Body**:
```json
{
  "project_id": "proj_123456",
  "repo_name": "my-awesome-app",
  "github_token": "ghp_xxxxxxxxxxxx",
  "is_private": false,
  "description": "A modern web application built with Rust Lovable"
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "success": true,
    "repo_url": "https://github.com/user/my-awesome-app",
    "commit_sha": "abc123def456",
    "files_pushed": 25,
    "branch": "main"
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Deploy to Vercel

**Endpoint**: `POST /api/v1/deploy/vercel`

**Request Body**:
```json
{
  "project_id": "proj_123456",
  "vercel_token": "vercel_token_xxx",
  "project_name": "my-awesome-app",
  "framework": "react"
}
```

**Response**:
```json
{
  "success": true,
  "data": {
    "success": true,
    "deployment_url": "https://my-awesome-app-abc123.vercel.app",
    "deployment_id": "dpl_abc123def456",
    "build_logs_url": "https://vercel.com/user/my-awesome-app",
    "status": "ready"
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

## Utility Endpoints

### Health Check

**Endpoint**: `GET /api/v1/health`

**Response**:
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "version": "0.1.0",
    "uptime": 3600,
    "services": {
      "ai": "healthy",
      "database": "healthy",
      "sandbox": "healthy"
    }
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Get Metrics

**Endpoint**: `GET /api/v1/metrics`

**Response**:
```json
{
  "success": true,
  "data": {
    "requests_total": 1000,
    "requests_per_second": 10.5,
    "response_time_avg": 150,
    "errors_total": 5,
    "sandboxes_active": 3,
    "projects_total": 25,
    "memory_usage": 45.2,
    "cpu_usage": 15.8
  },
  "error": null,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

## Error Handling

All endpoints return consistent error responses:

```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "NOT_FOUND",
    "message": "Project not found",
    "details": {
      "project_id": "proj_123456"
    }
  },
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Error Codes

- `NOT_FOUND`: Resource not found
- `INVALID_REQUEST`: Invalid request parameters
- `UNAUTHORIZED`: Authentication required
- `FORBIDDEN`: Access denied
- `INTERNAL_ERROR`: Internal server error
- `RATE_LIMITED`: Rate limit exceeded
- `SERVICE_UNAVAILABLE`: Service temporarily unavailable

## Rate Limiting

API endpoints are rate-limited based on the following tiers:

- **Free**: 100 requests/hour
- **Pro**: 1000 requests/hour
- **Enterprise**: Unlimited

Rate limit information is included in response headers:

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1705329600
```

## Webhooks

Rust Lovable supports webhooks for real-time notifications:

### Webhook Events

- `project.created`: New project created
- `project.updated`: Project updated
- `deployment.completed`: Deployment finished
- `deployment.failed`: Deployment failed
- `ai.generation.completed`: AI code generation completed

### Webhook Payload

```json
{
  "event": "project.created",
  "timestamp": "2024-01-15T10:30:00Z",
  "data": {
    "project_id": "proj_123456",
    "name": "My Awesome App"
  }
}
```

## SDKs and Libraries

Official SDKs are available for:

- **JavaScript/TypeScript**: `npm install @rust-lovable/sdk`
- **Python**: `pip install rust-lovable`
- **Rust**: `cargo add rust-lovable-sdk`

## Examples

### Creating a Project with JavaScript SDK

```javascript
import { RustLovable } from '@rust-lovable/sdk';

const client = new RustLovable({
  apiKey: 'your-api-key',
  baseURL: 'https://api.rust-lovable.com/v1'
});

// Create a new project
const project = await client.projects.create({
  name: 'My Awesome App',
  description: 'A modern web application',
  platform: 'web',
  template: 'react-vite'
});

console.log('Project created:', project.id);
```

### Processing AI Messages

```javascript
// Send a message to AI
const response = await client.ai.process({
  project_id: project.id,
  message: 'Create a modern landing page'
});

console.log('AI Response:', response.response);
console.log('UI Changes:', response.ui_changes);
```

### Exporting to GitHub

```javascript
// Export project to GitHub
const exportResult = await client.export.github({
  project_id: project.id,
  repo_name: 'my-awesome-app',
  github_token: 'ghp_xxxxxxxxxxxx',
  is_private: false
});

console.log('GitHub repo:', exportResult.repo_url);
```

## Changelog

### v0.1.0 (Current)

- Initial API release
- Project management endpoints
- AI integration
- Sandbox management
- File operations
- Export capabilities
- Real-time streaming

## Support

For support and questions:

- **Documentation**: https://docs.rust-lovable.com
- **GitHub Issues**: https://github.com/yourusername/rust-lovable/issues
- **Discord**: https://discord.gg/rust-lovable
- **Email**: support@rust-lovable.com