# NOA Database Operations

## Database Backend Options

NOA supports two database backends:

| Backend | Use Case | Vector Search | Feature Flag |
|---------|----------|---------------|--------------|
| SQLite | Desktop, local-first, single-user | sqlite-vss (experimental) | default |
| PostgreSQL | Server deployments, multi-user | RuVector | `--features full` |

## Schema Parity

### Intentional Differences

The SQLite and PostgreSQL schemas are mostly compatible but have intentional differences:

| Feature | SQLite | PostgreSQL |
|---------|--------|------------|
| **ID Types** | TEXT (UUID string) | UUID native |
| **Timestamps** | TEXT (ISO 8601) | TIMESTAMPTZ |
| **JSON** | TEXT (JSON string) | JSONB |
| **Arrays** | TEXT (JSON array) | Native arrays |
| **Vectors** | BLOB (f32 bytes) | ruvector(384) |
| **Foreign Keys** | PRAGMA enabled | Native |
| **Migrations** | `_migrations` table | `_sqlx_migrations` |

### Core Tables (Both Backends)

All backends must have these tables:

```
memory          - Stored interactions, decisions, learnings
model           - Registered AI models
agent           - Agent definitions
agent_log       - Agent activity audit log
task            - Task queue items
task_event      - Task lifecycle events
embedding       - Vector embeddings
knowledge_node  - Knowledge graph nodes
knowledge_edge  - Knowledge graph edges
digest_source   - Content ingestion sources
device          - Registered devices
sync_state      - Synchronization state
```

### PostgreSQL-Only Tables

```
auth_users              - User authentication (when auth enabled)
auth_accounts           - OAuth provider links
auth_sessions           - Active sessions
auth_verification_tokens - Email verification
auth_oauth_states       - PKCE/CSRF state
```

---

## Migration System

### SQLite Migrations

Location: `init/migrations/*.sql`

Migrations are numbered sequentially:
- `001_initial.sql` - Core schema
- `002_indexes.sql` - Performance indexes
- `003_vectors.sql` - Vector search setup
- `004_providers.sql` - AI provider configuration
- `005_autonomous.sql` - Autonomous operation tables
- `006_auth.sql` - Authentication tables

Run migrations:
```bash
noa db migrate --sqlite
```

### PostgreSQL Migrations

Location: `init/migrations/pg/*.sql`

PostgreSQL migrations use the same numbering but with PG-specific syntax:
- `001_core.sql` - Core schema with RuVector
- `002_search.sql` - Full-text search setup
- `003_vector.sql` - Vector indexes and functions
- `004_pgvector_compat.sql` - pgvector compatibility layer
- `006_auth.sql` - Authentication tables

Run migrations:
```bash
noa db migrate --postgres --url "postgres://..."
```

---

## Migration Guarantees

### Forward-Only Policy

**Migrations are forward-only.** There is no automated rollback support.

Rationale:
1. Data loss risks with automated rollbacks
2. Production data integrity is paramount
3. Rollback should be deliberate, not automatic

### Recovery Procedure

If a migration fails:

1. **Stop all NOA processes**
2. **Restore from backup** (see Backup Policy below)
3. **Fix the migration script**
4. **Re-apply migrations**

### Version Checking

At startup, NOA checks:
1. Current migration version
2. Required migration version
3. Blocks startup if versions don't match

To check migration status:
```bash
noa db status
```

### Migrate-on-Start vs Explicit

**Default behavior**: Migrations run automatically on start.

To require explicit migration:
```bash
# Disable auto-migrate
noa start --no-auto-migrate

# Run migrations explicitly
noa db migrate
```

For production, we recommend explicit migrations:
```bash
# Production startup sequence
noa db backup
noa db migrate
noa start --no-auto-migrate
```

---

## Health Checks

### SQLite Health

| Check | What It Validates |
|-------|-------------------|
| Connection | Can open and query database |
| Integrity | `PRAGMA integrity_check` passes |
| Schema | Required tables exist |
| Stats | Page count, size, free pages |

```bash
noa db health --sqlite
```

### PostgreSQL Health

| Check | What It Validates |
|-------|-------------------|
| Connection | `SELECT 1` succeeds |
| Extension | RuVector extension installed |
| Schema | Required tables exist |
| Version | Migration version matches |

```bash
noa db health --postgres
```

### API Health Endpoints

| Endpoint | Purpose |
|----------|---------|
| `/health/live` | Process is running (always 200 if reachable) |
| `/health/ready` | DB connected + migrations applied |
| `/health` | Full status with component details |

---

## Backup Policy

### SQLite Backup

SQLite databases can be backed up while running (WAL mode):

```bash
# Copy the database file
cp $NOA_DATA/noa.db $BACKUP_DIR/noa-$(date +%Y%m%d).db

# Or use SQLite backup API
sqlite3 $NOA_DATA/noa.db ".backup $BACKUP_DIR/noa-$(date +%Y%m%d).db"
```

### PostgreSQL Backup

Use standard pg_dump:

```bash
pg_dump -Fc $DATABASE_URL > noa-$(date +%Y%m%d).dump
```

### Backup Frequency

| Data Criticality | Backup Frequency |
|------------------|------------------|
| Development | Daily |
| Staging | Daily + before deployments |
| Production | Hourly + before any migration |

---

## Vector Search

### SQLite Vector Search

SQLite uses in-memory brute-force search (no ANN index):

```sql
-- Vectors stored as BLOB
SELECT id, content FROM memory
JOIN embedding ON memory.embedding_id = embedding.id
-- Similarity computed in application code
```

For production SQLite vector search, consider sqlite-vss extension (experimental).

### PostgreSQL Vector Search (RuVector)

PostgreSQL uses RuVector for ANN search:

```sql
-- Find similar memories
SELECT memory_id, (e.vector <-> $1::ruvector(384)) AS distance
FROM memory m
JOIN embedding e ON e.id = m.embedding_id
ORDER BY e.vector <-> $1::ruvector(384)
LIMIT 10;
```

Index configuration:
```sql
-- HNSW index for fast ANN
CREATE INDEX idx_embedding_vector_hnsw
ON embedding USING ruhnsw (vector ruvector_l2_ops);
```

### Hybrid Search

Hybrid search combines keyword and vector search:

```sql
-- Hybrid: keyword filter + vector ranking
SELECT m.id, m.content,
       (e.vector <-> $query_vector) AS vector_dist,
       ts_rank(to_tsvector('english', m.content), plainto_tsquery('english', $keyword)) AS keyword_rank
FROM memory m
JOIN embedding e ON e.id = m.embedding_id
WHERE to_tsvector('english', m.content) @@ plainto_tsquery('english', $keyword)
ORDER BY vector_dist
LIMIT 10;
```

---

## Production Configuration

### Connection Pooling

| Setting | SQLite | PostgreSQL |
|---------|--------|------------|
| Max connections | 10 | 20 |
| Min idle | 2 | 5 |
| Connection timeout | 30s | 30s |
| Idle timeout | 600s | 300s |
| Max lifetime | 3600s | 1800s |

### Environment Variables

```bash
# PostgreSQL
DATABASE_URL="postgres://user:pass@host:5432/noa"
DATABASE_MAX_CONNECTIONS=20

# SQLite
NOA_DATA="/var/lib/noa"
NOA_DB_PATH="$NOA_DATA/noa.db"
```

### Recommended PostgreSQL Extensions

```sql
-- Required
CREATE EXTENSION IF NOT EXISTS ruvector;

-- Recommended
CREATE EXTENSION IF NOT EXISTS pg_trgm;  -- Fuzzy text search
CREATE EXTENSION IF NOT EXISTS btree_gin; -- GIN indexes for JSONB
```

---

## Troubleshooting

### "Database locked" (SQLite)

Cause: Another process has an exclusive lock.

Fix:
1. Ensure only one NOA instance per database
2. Check for zombie processes: `ps aux | grep noa`
3. Enable WAL mode (default): `PRAGMA journal_mode = WAL;`

### "Connection refused" (PostgreSQL)

Cause: PostgreSQL not running or wrong host.

Fix:
1. Verify PostgreSQL is running: `pg_isready`
2. Check connection string
3. Verify firewall allows connection

### "Extension not found: ruvector"

Cause: RuVector not installed in PostgreSQL.

Fix:
1. Install RuVector extension
2. Run: `CREATE EXTENSION IF NOT EXISTS ruvector;`
3. Verify: `SELECT * FROM pg_extension WHERE extname = 'ruvector';`

### "Migration version mismatch"

Cause: Database has different migration version than code expects.

Fix:
1. Check current version: `noa db status`
2. Run pending migrations: `noa db migrate`
3. If downgrade needed: restore from backup

---

## Performance Tuning

### SQLite

```sql
-- Already configured by default
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA cache_size = -64000;  -- 64MB
PRAGMA mmap_size = 268435456; -- 256MB
```

### PostgreSQL

```sql
-- Recommended for NOA workload
ALTER SYSTEM SET shared_buffers = '256MB';
ALTER SYSTEM SET effective_cache_size = '1GB';
ALTER SYSTEM SET maintenance_work_mem = '128MB';
ALTER SYSTEM SET work_mem = '16MB';

-- For vector search
ALTER SYSTEM SET max_parallel_workers_per_gather = 4;
```
