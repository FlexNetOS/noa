# NOA Platform Data Flow

> Comprehensive data flow documentation for the NOA platform.
> Last updated: 2026-01-01

## Overview

This document describes how data flows through the NOA platform, from user input to system responses, including all intermediate processing stages.

## High-Level Data Flow

```mermaid
flowchart TB
    subgraph Input["Input Layer"]
        CLI[CLI Commands]
        API[REST API]
        MCP[MCP Protocol]
        UI[Web/Desktop UI]
    end
    
    subgraph Gateway["Gateway Layer"]
        Router[Request Router]
        Auth[Authentication]
        RateLimit[Rate Limiter]
    end
    
    subgraph Orchestrator["Orchestration Layer"]
        TaskMgr[Task Manager]
        AgentMgr[Agent Manager]
        Scheduler[Scheduler]
        WorkflowEngine[Workflow Engine]
    end
    
    subgraph Processing["Processing Layer"]
        AIProviders[AI Providers]
        Embeddings[Embedding Service]
        VectorDB[Vector Database]
        Tools[Tool Execution]
    end
    
    subgraph Storage["Storage Layer"]
        SQLite[(SQLite)]
        FileSystem[(File System)]
        Cache[(Cache)]
        Logs[(Logs)]
    end
    
    subgraph Output["Output Layer"]
        Response[API Response]
        Streams[Event Streams]
        Files[Generated Files]
        Notifications[Notifications]
    end
    
    Input --> Gateway
    Gateway --> Orchestrator
    Orchestrator --> Processing
    Processing --> Storage
    Storage --> Output
    Orchestrator --> Output
```

## Component Data Flows

### 1. CLI Command Flow

```mermaid
sequenceDiagram
    participant User
    participant CLI as tasks-cli
    participant Config as Config Loader
    participant Core as noa-core
    participant DB as SQLite
    participant AI as AI Provider
    
    User->>CLI: Execute command
    CLI->>Config: Load configuration
    Config-->>CLI: Config object
    CLI->>Core: Parse & validate
    Core->>DB: Query/Update state
    
    alt AI-assisted command
        Core->>AI: Send prompt
        AI-->>Core: Response
        Core->>DB: Store result
    end
    
    DB-->>Core: Result
    Core-->>CLI: Formatted output
    CLI-->>User: Display result
```

### 2. MCP Tool Execution Flow

```mermaid
sequenceDiagram
    participant Client as MCP Client
    participant Server as tasks-mcp
    participant Handler as Tool Handler
    participant Executor as Tool Executor
    participant FS as File System
    participant AI as Ollama
    
    Client->>Server: tools/call request
    Server->>Handler: Route to handler
    Handler->>Handler: Validate parameters
    
    alt File operation
        Handler->>FS: Read/Write file
        FS-->>Handler: File content
    end
    
    alt AI operation
        Handler->>AI: Generate embedding
        AI-->>Handler: Vector response
    end
    
    Handler->>Executor: Execute tool logic
    Executor-->>Handler: Execution result
    Handler-->>Server: Tool result
    Server-->>Client: tools/call response
```

### 3. Sweep Pipeline Flow

```mermaid
flowchart LR
    subgraph Discovery
        A[File Discovery] --> B[Filter Extensions]
        B --> C[Build File List]
    end
    
    subgraph Extraction
        C --> D[Symbol Extractor]
        D --> E[Parse AST]
        E --> F[Extract Symbols]
    end
    
    subgraph Enrichment
        F --> G[Doc Cross-Reference]
        G --> H[Find Gaps]
        F --> I[Ollama Embeddings]
        I --> J[768-dim Vectors]
    end
    
    subgraph Storage
        H --> K[(SQLite DB)]
        J --> K
        F --> K
    end
    
    subgraph Visualization
        K --> L[Graph Generator]
        L --> M[Mermaid Diagrams]
        K --> N[Gap Reports]
    end
```

### 4. AI Provider Flow

```mermaid
flowchart TB
    subgraph Request["Request Handling"]
        Input[User Input]
        Prompt[Prompt Builder]
        Context[Context Assembler]
    end
    
    subgraph Routing["Provider Routing"]
        Router{Provider Router}
        Ollama[Ollama Local]
        OpenAI[OpenAI API]
        Anthropic[Anthropic API]
        Azure[Azure OpenAI]
    end
    
    subgraph Processing["Response Processing"]
        Parse[Response Parser]
        Stream[Stream Handler]
        Cache[Response Cache]
    end
    
    Input --> Prompt
    Prompt --> Context
    Context --> Router
    
    Router -->|local| Ollama
    Router -->|cloud| OpenAI
    Router -->|cloud| Anthropic
    Router -->|enterprise| Azure
    
    Ollama --> Parse
    OpenAI --> Stream
    Anthropic --> Stream
    Azure --> Parse
    
    Parse --> Cache
    Stream --> Cache
```

### 5. Embedding Generation Flow

```mermaid
flowchart LR
    subgraph Input
        Symbol[Symbol Data]
        Doc[Documentation]
        Code[Code Snippet]
    end
    
    subgraph Preparation
        Text[Text Formatter]
        Chunk[Chunker]
        Batch[Batch Builder]
    end
    
    subgraph Embedding
        Ollama[Ollama API]
        Model[nomic-embed-text]
    end
    
    subgraph Storage
        Vector[768-dim Vector]
        DB[(Vector DB)]
        Index[HNSW Index]
    end
    
    Symbol --> Text
    Doc --> Text
    Code --> Text
    
    Text --> Chunk
    Chunk --> Batch
    Batch --> Ollama
    Ollama --> Model
    Model --> Vector
    Vector --> DB
    DB --> Index
```

## Data Schemas

### Symbol Record

```json
{
  "id": "uuid",
  "name": "function_name",
  "type": "function|struct|trait|module|class|interface",
  "file_path": "relative/path/to/file.rs",
  "line": 42,
  "visibility": "pub|pub(crate)|private",
  "doc_comment": "/// Documentation comment",
  "generics": ["T", "E"],
  "parameters": ["param1: Type1", "param2: Type2"],
  "return_type": "Result<T, E>",
  "embedding": [0.123, -0.456, ...],  // 768 dimensions
  "created_at": "2026-01-01T00:00:00Z",
  "sweep_id": 1
}
```

### Sweep State Record

```json
{
  "id": 1,
  "sweep_number": 1,
  "start_time": "2026-01-01T02:17:10Z",
  "end_time": "2026-01-01T02:45:30Z",
  "status": "completed",
  "files_processed": 92801,
  "symbols_found": 45230,
  "errors": 0,
  "config": {
    "max_parallel": 8,
    "ollama_model": "nomic-embed-text"
  }
}
```

### Documentation Gap Record

```json
{
  "symbol_name": "process_request",
  "symbol_type": "function",
  "file_path": "sys/core/src/handler.rs",
  "line": 156,
  "missing_in": ["wiki", "api_docs"],
  "suggested_docs": ["runbooks/request-handling.md"],
  "priority": "high"
}
```

## State Management

### SQLite Tables

| Table | Purpose | Key Fields |
|-------|---------|------------|
| `sweep_state` | Track sweep iterations | id, sweep_number, status |
| `file_state` | File processing status | file_path, hash, last_sweep |
| `symbols` | Extracted symbols | name, type, file_path, line |
| `embeddings` | Vector embeddings | symbol_id, vector, model |
| `doc_refs` | Documentation references | symbol_id, doc_path, doc_type |
| `graph_edges` | Symbol relationships | from_id, to_id, relationship |

### Cache Strategy

```mermaid
flowchart TB
    Request[Request] --> Check{Cache Hit?}
    Check -->|Yes| Return[Return Cached]
    Check -->|No| Process[Process Request]
    Process --> Store[Store in Cache]
    Store --> Return2[Return Result]
    
    subgraph TTL["Cache TTL"]
        Embeddings[Embeddings: 24h]
        Symbols[Symbols: 1h]
        Graphs[Graphs: 6h]
    end
```

## Integration Points

### External Services

| Service | Protocol | Data Format | Purpose |
|---------|----------|-------------|---------|
| Ollama | HTTP/REST | JSON | Local LLM & embeddings |
| GitHub | REST/GraphQL | JSON | Repository operations |
| VS Code | MCP | JSON-RPC | Editor integration |
| MinIO | S3 | Binary | Object storage |
| Qdrant | gRPC | Protobuf | Vector search |

### Internal APIs

| Component | Port | Protocol | Purpose |
|-----------|------|----------|---------|
| noa-server | 8080 | HTTP | Main API |
| tasks-mcp | 3000 | MCP | Tool server |
| Ollama | 11434 | HTTP | AI inference |
| Qdrant | 6333 | gRPC | Vector DB |

## Error Handling

```mermaid
flowchart TB
    Error[Error Occurs] --> Classify{Error Type}
    
    Classify -->|Transient| Retry[Retry with Backoff]
    Classify -->|Permanent| Log[Log & Alert]
    Classify -->|Recoverable| Fallback[Use Fallback]
    
    Retry -->|Success| Continue[Continue]
    Retry -->|Max Retries| Log
    
    Fallback --> Continue
    Log --> Report[Error Report]
```

## Performance Considerations

### Throughput

| Operation | Target | Actual |
|-----------|--------|--------|
| Symbol extraction | 1000 files/sec | ~50 files/sec |
| Embedding generation | 100 vectors/sec | ~10 vectors/sec |
| Graph generation | 5 graphs/min | 5 graphs/min |
| Doc cross-reference | 500 symbols/sec | ~200 symbols/sec |

### Bottlenecks

1. **Embedding Generation**: Limited by Ollama inference speed
2. **File I/O**: Large codebase requires efficient batching
3. **Memory**: Symbol storage grows with codebase size

## Related Documentation

- [Directory Structure](../pages/directory-tree.md)
- [Architecture Diagrams](./diagrams/)
- [API Reference](../api/)
- [Runbooks](../runbooks/)
