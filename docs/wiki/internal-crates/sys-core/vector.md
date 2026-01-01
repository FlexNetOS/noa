# vector Module

Vector embeddings and similarity search.

**Location**: `sys/core/src/vector/`  
**Feature**: `full`

## Overview

Vector storage and retrieval for semantic search:

- Embedding storage
- Cosine similarity search
- HNSW index for fast retrieval
- Integration with SQLite

## Key Types

### VectorStore

Main vector storage interface.

```rust
pub struct VectorStore {
    db: Database,
    index: HnswIndex,
    dimension: usize,
}

impl VectorStore {
    pub async fn insert(&mut self, id: &str, embedding: Vec<f32>, metadata: Value) -> NoaResult<()>;
    pub async fn search(&self, query: Vec<f32>, k: usize) -> NoaResult<Vec<SearchResult>>;
    pub async fn delete(&mut self, id: &str) -> NoaResult<()>;
}
```

### SearchResult

Search result with score.

```rust
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub metadata: Value,
}
```

### HnswIndex

Hierarchical Navigable Small World graph.

```rust
pub struct HnswIndex {
    dimension: usize,
    m: usize,           // Max connections per layer
    ef_construction: usize,
    ef_search: usize,
}
```

## Usage

```rust
use noa_core::vector::VectorStore;
use noa_core::neural::NeuralModule;

async fn example(neural: &NeuralModule, vectors: &mut VectorStore) -> NoaResult<()> {
    // Generate embedding
    let text = "Rust programming language";
    let embedding = neural.embed(model, text).await?;
    
    // Store
    vectors.insert("doc-1", embedding.clone(), json!({"text": text})).await?;
    
    // Search
    let query = neural.embed(model, "systems programming").await?;
    let results = vectors.search(query, 5).await?;
    
    for result in results {
        println!("Score: {}, ID: {}", result.score, result.id);
    }
    
    Ok(())
}
```

## See Also

- [neural module](neural.md) — Embedding generation
- [memory module](memory.md) — Semantic memory
- [db module](db.md) — SQLite storage
