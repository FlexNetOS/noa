# db Module

Database persistence layer using SQLite and SQLx.

**Location**: `sys/core/src/db/`  
**Always Available**: Yes (no feature flag required)

## Overview

Provides async database access with:

- SQLite as primary data store
- SQLx for compile-time query verification
- Store pattern for entity access
- Migration management

## Key Types

### Database

Main database connection pool.

```rust
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn connect(url: &str) -> Result<Self>;
    pub async fn run_migrations(&self) -> Result<()>;
}
```

### Store Trait

Common interface for entity stores.

```rust
#[async_trait]
pub trait Store {
    type Entity;
    type Id;
    
    async fn get(&self, id: Self::Id) -> Result<Option<Self::Entity>>;
    async fn create(&self, entity: &Self::Entity) -> Result<Self::Id>;
    async fn update(&self, entity: &Self::Entity) -> Result<()>;
    async fn delete(&self, id: Self::Id) -> Result<()>;
    async fn list(&self) -> Result<Vec<Self::Entity>>;
}
```

## Entity Stores

| Store | Entity | Table |
|-------|--------|-------|
| `AgentStore` | `Agent` | `agents` |
| `TaskStore` | `Task` | `tasks` |
| `ConversationStore` | `Conversation` | `conversations` |
| `MemoryStore` | `Memory` | `memories` |
| `ConfigStore` | `ConfigEntry` | `config` |

## Schema

See [database.yaml](../../../../config/database.yaml) for full schema.

```sql
CREATE TABLE agents (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    kind TEXT NOT NULL,
    status TEXT NOT NULL,
    config TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    agent_id TEXT REFERENCES agents(id),
    status TEXT NOT NULL,
    input TEXT,
    output TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

## Usage

```rust
use noa_core::db::{Database, AgentStore};

async fn example() -> Result<()> {
    let db = Database::connect("sqlite://noa.db").await?;
    db.run_migrations().await?;
    
    let agent_store = AgentStore::new(&db);
    let agents = agent_store.list().await?;
    
    Ok(())
}
```

## See Also

- [config module](config.md) — Database URL configuration
- [agents module](agents.md) — Agent entity
