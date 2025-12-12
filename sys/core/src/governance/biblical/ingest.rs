use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

/// Supported source languages.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptureLanguage {
    Greek,
    Hebrew,
    Unknown,
}

impl ScriptureLanguage {
    fn from_filename(path: &Path) -> Self {
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();

        if name.starts_with("grc") || name.contains("greek") {
            ScriptureLanguage::Greek
        } else if name.starts_with("hbo") || name.contains("hebrew") {
            ScriptureLanguage::Hebrew
        } else {
            ScriptureLanguage::Unknown
        }
    }
}

/// Metadata for a single ingested source file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiblicalSource {
    pub path: PathBuf,
    pub checksum: String,
    pub language: ScriptureLanguage,
    pub book: Option<String>,
    pub ingested_at: DateTime<Utc>,
}

/// Ingestion summary.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IngestionReport {
    pub ingested: Vec<BiblicalSource>,
    pub skipped: Vec<String>,
}

/// Greek/Hebrew text ingestor.
pub struct BiblicalIngestor {
    root: PathBuf,
}

impl BiblicalIngestor {
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        Self { root: root.into() }
    }

    /// Ingest all sources in the root directory, producing an index and report.
    pub fn ingest(&self) -> Result<IngestionReport> {
        fs::create_dir_all(&self.root)?;

        let mut report = IngestionReport::default();
        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() || path.file_name().is_none() {
                continue;
            }

            match self.process_file(&path) {
                Ok(Some(source)) => {
                    info!(
                        target: "governance::ingest",
                        file = %path.display(),
                        lang = ?source.language,
                        checksum = %source.checksum,
                        "Ingested biblical source"
                    );
                    report.ingested.push(source);
                }
                Ok(None) => {
                    report.skipped.push(path.file_name().unwrap().to_string_lossy().to_string());
                }
                Err(err) => {
                    warn!(
                        target: "governance::ingest",
                        file = %path.display(),
                        error = %err,
                        "Failed to ingest biblical source"
                    );
                    report.skipped.push(path.file_name().unwrap().to_string_lossy().to_string());
                }
            }
        }

        let index_path = self.root.join("index.json");
        let mut index_file = fs::File::create(index_path)?;
        serde_json::to_writer_pretty(&mut index_file, &report.ingested)?;

        Ok(report)
    }

    fn process_file(&self, path: &Path) -> Result<Option<BiblicalSource>> {
        let mut contents = String::new();
        fs::File::open(path)?.read_to_string(&mut contents)?;

        let language = self.detect_language(path, &contents);
        if language == ScriptureLanguage::Unknown {
            warn!(
                target: "governance::ingest",
                file = %path.display(),
                "Skipping source with unknown language"
            );
            return Ok(None);
        }

        let checksum = format!("{:x}", Sha256::digest(contents.as_bytes()));
        let book = self.extract_book_hint(path, &contents);

        Ok(Some(BiblicalSource {
            path: path.to_path_buf(),
            checksum,
            language,
            book,
            ingested_at: Utc::now(),
        }))
    }

    fn detect_language(&self, path: &Path, contents: &str) -> ScriptureLanguage {
        let from_name = ScriptureLanguage::from_filename(path);
        if from_name != ScriptureLanguage::Unknown {
            return from_name;
        }

        let mut greek = 0usize;
        let mut hebrew = 0usize;

        for ch in contents.chars().take(2048) {
            if is_greek(ch) {
                greek += 1;
            } else if is_hebrew(ch) {
                hebrew += 1;
            }
        }

        match (greek, hebrew) {
            (g, h) if g > h && g > 8 => ScriptureLanguage::Greek,
            (g, h) if h > g && h > 8 => ScriptureLanguage::Hebrew,
            _ => ScriptureLanguage::Unknown,
        }
    }

    fn extract_book_hint(&self, path: &Path, contents: &str) -> Option<String> {
        // Try first line as JSON metadata: {"book": "Genesis", "chapter": 1}
        if let Some(first_line) = contents.lines().next() {
            if first_line.trim_start().starts_with('{') {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(first_line) {
                    if let Some(book) = value.get("book").and_then(|b| b.as_str()) {
                        return Some(book.to_string());
                    }
                }
            }
        }

        // Fallback: use filename stem.
        path.file_stem().and_then(|s| s.to_str()).map(|s| s.to_string())
    }
}

fn is_greek(ch: char) -> bool {
    matches!(ch as u32, 0x0370..=0x03FF | 0x1F00..=0x1FFF)
}

fn is_hebrew(ch: char) -> bool {
    matches!(ch as u32, 0x0590..=0x05FF)
}
