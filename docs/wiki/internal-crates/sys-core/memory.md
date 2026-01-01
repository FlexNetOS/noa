# memory Module

Semantic memory and knowledge retrieval.

**Location**: `sys/core/src/memory/`  
**Feature**: `full`

## Overview

Long-term memory storage for agents:

- Semantic memory with embeddings
- Episodic memory for conversations
- Working memory for active context
- Memory consolidation

## Key Types

### MemoryBank

Central memory management.

```rust
pub struct MemoryBank {
    semantic: SemanticMemory,
    episodic: EpisodicMemory,
    working: WorkingMemory,
}

impl MemoryBank {
    pub async fn store(&mut self, memory: Memory) -> NoaResult<MemoryId>;
    pub async fn recall(&self, query: &str, limit: usize) -> NoaResult<Vec<Memory>>;
    pub async fn consolidate(&mut self) -> NoaResult<()>;
}
```

### Memory

Memory entry.

```rust
pub struct Memory {
    pub id: MemoryId,
    pub kind: MemoryKind,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
    pub metadata: MemoryMetadata,
    pub timestamp: DateTime<Utc>,
}

pub enum MemoryKind {
    Fact,           // Declarative knowledge
    Episode,        // Conversation/event
    Procedure,      // How-to knowledge
    Preference,     // User preferences
}
```

### MemoryMetadata

```rust
pub struct MemoryMetadata {
    pub source: String,
    pub confidence: f32,
    pub access_count: u32,
    pub last_accessed: DateTime<Utc>,
}
```

## Memory Types

| Type | Purpose | Retention |
|------|---------|-----------|
| Semantic | Facts and knowledge | Permanent |
| Episodic | Conversations | 30 days |
| Working | Active context | Session |

## Usage

```rust
use noa_core::memory::{MemoryBank, Memory, MemoryKind};

async fn example(bank: &mut MemoryBank) -> NoaResult<()> {
    // Store a fact
    let memory = Memory {
        kind: MemoryKind::Fact,
        content: "User prefers dark mode".into(),
        ..Default::default()
    };
    bank.store(memory).await?;
    
    // Recall relevant memories
    let memories = bank.recall("user preferences", 5).await?;
    
    Ok(())
}
```

## See Also

- [vector module](vector.md) — Embedding storage
- [neural module](neural.md) — Embedding generation
- [agents module](agents.md) — Agent memory access
