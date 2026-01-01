# api Module

REST and gRPC API endpoints.

**Location**: `sys/core/src/api/`  
**Feature**: `full`

## Overview

Exposes NOA functionality via HTTP APIs:

- RESTful endpoints using Axum
- WebSocket for real-time events
- JWT authentication
- OpenAPI documentation

## Architecture

```
api/
├── mod.rs          # Module root
├── routes/         # Route handlers
│   ├── admin.rs    # Admin bootstrap
│   ├── agents.rs   # Agent CRUD
│   ├── tasks.rs    # Task execution
│   ├── health.rs   # Health checks
│   └── auth.rs     # Authentication
├── middleware/     # Axum middleware
│   ├── auth.rs     # JWT validation
│   ├── logging.rs  # Request logging
│   └── cors.rs     # CORS handling
└── openapi.rs      # OpenAPI spec
```

## Key Types

### Router

Main Axum router setup.

```rust
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1", api_routes())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
```

### AppState

Shared application state.

```rust
pub struct AppState {
    pub db: Database,
    pub agent_registry: AgentRegistry,
    pub governor: Governor,
    pub config: AppConfig,
}
```

## Endpoints

### Agents

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/agents` | List all agents |
| POST | `/api/v1/agents` | Spawn new agent |
| GET | `/api/v1/agents/:id` | Get agent details |
| POST | `/api/v1/agents/:id/execute` | Execute task |
| DELETE | `/api/v1/agents/:id` | Stop and remove agent |

### Tasks

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/tasks` | List tasks |
| GET | `/api/v1/tasks/:id` | Get task status |
| POST | `/api/v1/tasks/:id/cancel` | Cancel running task |

### Health

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Basic health check |
| GET | `/health/ready` | Readiness probe |
| GET | `/health/live` | Liveness probe |

### Admin

| Method | Path | Description |
|--------|------|-------------|
| POST | `/api/v1/admin/bootstrap` | First-run setup |
| POST | `/api/v1/admin/users` | Create admin user |

## Authentication

JWT-based authentication:

```rust
// Login
POST /api/v1/auth/login
{
    "username": "admin",
    "password": "secret"
}

// Response
{
    "token": "eyJ...",
    "expires_at": "2026-01-02T00:00:00Z"
}

// Authenticated request
GET /api/v1/agents
Authorization: Bearer eyJ...
```

## Usage

```rust
use noa_core::api::{create_router, AppState};

async fn main() -> NoaResult<()> {
    let state = AppState::new().await?;
    let router = create_router(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, router).await?;
    
    Ok(())
}
```

## See Also

- [services module](services.md) — Background services
- [auth routes](routes/auth.md) — Authentication details
- [timestamp module](timestamp.md) — HTTP date headers
