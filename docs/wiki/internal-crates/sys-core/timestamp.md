# timestamp Module

Time utilities for consistent timestamp handling.

**Location**: `sys/core/src/timestamp.rs`  
**Always Available**: Yes (no feature flag required)  
**Added**: 2026-01-01

## Overview

Provides consistent timestamp formatting across NOA:

- File modification timestamps via `filetime`
- HTTP date formatting via `httpdate` (RFC 7231)
- ISO 8601 formatting
- Markdown frontmatter generation

## Key Types

### TimestampMeta

Metadata extracted from file timestamps.

```rust
pub struct TimestampMeta {
    pub modified: SystemTime,
    pub accessed: SystemTime,
    pub created: Option<SystemTime>,
}
```

## Key Functions

### get_file_timestamps

Get file timestamps using `filetime` crate.

```rust
pub fn get_file_timestamps<P: AsRef<Path>>(path: P) -> io::Result<TimestampMeta>;
```

### now_iso8601

Current time in ISO 8601 format.

```rust
pub fn now_iso8601() -> String;
// Returns: "2026-01-01T00:00:00Z"
```

### now_http_date

Current time in HTTP date format (RFC 7231).

```rust
pub fn now_http_date() -> String;
// Returns: "Wed, 01 Jan 2026 00:00:00 GMT"
```

### now_local_formatted

Current time in local timezone with readable format.

```rust
pub fn now_local_formatted() -> String;
// Returns: "2026-01-01 00:00:00"
```

### generate_frontmatter

Generate YAML frontmatter with timestamps.

```rust
pub fn generate_frontmatter(title: &str, author: Option<&str>) -> String;
```

Output:
```yaml
---
title: "My Document"
author: "AI"
created: "2026-01-01T00:00:00Z"
modified: "2026-01-01T00:00:00Z"
---
```

## Dependencies

```toml
[dependencies]
filetime = "0.2.26"  # Cross-platform file timestamps
httpdate = "1.0.3"   # HTTP date formatting (RFC 7231)
chrono = "0.4"       # Date/time utilities
```

## Usage

```rust
use noa_core::timestamp::{
    get_file_timestamps, 
    now_iso8601, 
    now_http_date,
    generate_frontmatter
};

fn example() -> io::Result<()> {
    // Get file timestamps
    let meta = get_file_timestamps("document.md")?;
    println!("Modified: {:?}", meta.modified);
    
    // Format current time
    println!("ISO 8601: {}", now_iso8601());
    println!("HTTP: {}", now_http_date());
    
    // Generate frontmatter
    let frontmatter = generate_frontmatter("README", Some("NOA"));
    println!("{}", frontmatter);
    
    Ok(())
}
```

## See Also

- [api module](api.md) — HTTP date headers
- [db module](db.md) — Database timestamps
