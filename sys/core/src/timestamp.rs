//! Timestamp Utilities
//!
//! Provides accurate, consistent timestamp handling across NOA.
//! Uses `filetime` for file metadata and `httpdate` for HTTP-standard formatting.

use chrono::{DateTime, Local, Utc};
use filetime::FileTime;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

/// Metadata timestamp information for a file or document.
#[derive(Debug, Clone)]
pub struct TimestampMeta {
    /// Created timestamp (if available).
    pub created: Option<DateTime<Utc>>,
    /// Last modified timestamp.
    pub modified: DateTime<Utc>,
    /// Last accessed timestamp (if available).
    pub accessed: Option<DateTime<Utc>>,
}

impl TimestampMeta {
    /// Format the modified timestamp as HTTP date (RFC 7231).
    /// Example: "Tue, 31 Dec 2025 17:30:00 GMT"
    pub fn http_date(&self) -> String {
        httpdate::fmt_http_date(self.modified.into())
    }

    /// Format as ISO 8601 with timezone.
    /// Example: "2025-12-31T17:30:00Z"
    pub fn iso8601(&self) -> String {
        self.modified.format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }

    /// Format as local time for display.
    /// Example: "2025-12-31 11:30:00 CST"
    pub fn local_display(&self) -> String {
        let local: DateTime<Local> = self.modified.into();
        local.format("%Y-%m-%d %H:%M:%S %Z").to_string()
    }

    /// Format as clean metadata header for markdown files.
    /// Example: "Last Updated: 2025-12-31 17:30:00 UTC"
    pub fn markdown_header(&self) -> String {
        format!("Last Updated: {}", self.iso8601())
    }

    /// Format created and modified for full metadata block.
    pub fn full_metadata(&self) -> String {
        let mut meta = String::new();
        if let Some(created) = self.created {
            meta.push_str(&format!("Created: {}\n", created.format("%Y-%m-%dT%H:%M:%SZ")));
        }
        meta.push_str(&format!("Modified: {}\n", self.iso8601()));
        meta
    }
}

/// Get timestamp metadata for a file.
pub fn get_file_timestamps<P: AsRef<Path>>(path: P) -> std::io::Result<TimestampMeta> {
    let metadata = fs::metadata(path)?;

    let modified = metadata.modified()?;
    let modified_dt = DateTime::<Utc>::from(modified);

    let created = metadata.created().ok().map(DateTime::<Utc>::from);
    let accessed = metadata.accessed().ok().map(DateTime::<Utc>::from);

    Ok(TimestampMeta {
        created,
        modified: modified_dt,
        accessed,
    })
}

/// Set the modified timestamp for a file.
pub fn set_file_modified<P: AsRef<Path>>(path: P, time: DateTime<Utc>) -> std::io::Result<()> {
    let system_time: SystemTime = time.into();
    let file_time = FileTime::from_system_time(system_time);
    filetime::set_file_mtime(path, file_time)
}

/// Set both access and modified timestamps for a file.
pub fn set_file_times<P: AsRef<Path>>(
    path: P,
    accessed: DateTime<Utc>,
    modified: DateTime<Utc>,
) -> std::io::Result<()> {
    let atime = FileTime::from_system_time(accessed.into());
    let mtime = FileTime::from_system_time(modified.into());
    filetime::set_file_times(path, atime, mtime)
}

/// Get the current time as a formatted string.
pub fn now_iso8601() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

/// Get the current time as HTTP date.
pub fn now_http_date() -> String {
    httpdate::fmt_http_date(SystemTime::now())
}

/// Get the current time in local timezone for display.
pub fn now_local() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string()
}

/// Parse an HTTP date string into DateTime<Utc>.
pub fn parse_http_date(s: &str) -> Result<DateTime<Utc>, httpdate::Error> {
    let system_time = httpdate::parse_http_date(s)?;
    Ok(DateTime::<Utc>::from(system_time))
}

/// Generate a metadata YAML frontmatter block for markdown files.
pub fn generate_frontmatter(
    title: &str,
    description: Option<&str>,
    created: Option<DateTime<Utc>>,
    modified: DateTime<Utc>,
) -> String {
    let mut fm = String::from("---\n");
    fm.push_str(&format!("title: \"{}\"\n", title));
    if let Some(desc) = description {
        fm.push_str(&format!("description: \"{}\"\n", desc));
    }
    if let Some(c) = created {
        fm.push_str(&format!("created: {}\n", c.format("%Y-%m-%dT%H:%M:%SZ")));
    }
    fm.push_str(&format!("modified: {}\n", modified.format("%Y-%m-%dT%H:%M:%SZ")));
    fm.push_str(&format!("generated: {}\n", now_iso8601()));
    fm.push_str("---\n");
    fm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_iso8601() {
        let ts = now_iso8601();
        assert!(ts.ends_with('Z'));
        assert!(ts.contains('T'));
    }

    #[test]
    fn test_http_date_roundtrip() {
        let http_date = now_http_date();
        let parsed = parse_http_date(&http_date);
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_frontmatter() {
        let fm = generate_frontmatter(
            "Test Document",
            Some("A test"),
            None,
            Utc::now(),
        );
        assert!(fm.starts_with("---\n"));
        assert!(fm.ends_with("---\n"));
        assert!(fm.contains("title: \"Test Document\""));
    }
}
