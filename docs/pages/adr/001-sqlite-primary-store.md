# ADR-001: SQLite as Primary Store

## Status

Accepted

## Context

NOA needs a persistent storage solution for:
- Configuration
- Agent state
- Task history
- User preferences
- Embeddings (with vector extensions)

Options considered:
1. PostgreSQL
2. SQLite
3. RocksDB
4. In-memory only

## Decision

Use SQLite as the primary data store.

## Rationale

1. **Local-first**: No external dependencies
2. **Zero configuration**: Works out of the box
3. **Portable**: Single file, easy backup
4. **Embedded**: No separate process
5. **ACID compliant**: Full transaction support
6. **Well-tested**: Battle-tested across billions of deployments
7. **Vector extensions**: sqlite-vss for embeddings

## Consequences

### Positive
- Simple deployment
- Fast reads
- Easy backup/restore
- Works offline

### Negative
- Limited concurrency
- Not suitable for large-scale deployments
- No built-in replication

## Mitigations

- Use connection pooling
- Implement WAL mode
- Consider PostgreSQL for enterprise deployments

## References

- [SQLite Documentation](https://sqlite.org/docs.html)
- [sqlite-vss](https://github.com/asg017/sqlite-vss)
