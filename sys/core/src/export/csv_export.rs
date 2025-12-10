//! CSV Export Service
//!
//! Implements CSV export for all NOA entities (agents, tasks, memory, etc.)
//! T041: CSV export service for all entities
//! §3.5: Transparent & Auditable

use crate::error::Result;
use rusqlite::Connection;
use std::fs;
use std::io::Write;
use std::path::Path;

/// CSV exporter for NOA entities
pub struct CsvExporter;

impl CsvExporter {
    /// Export a table to CSV
    pub fn export_table(conn: &Connection, table: &str, output_path: &Path) -> Result<()> {
        let mut file = fs::File::create(output_path)?;

        // Get column names
        let stmt = conn.prepare(&format!("SELECT * FROM {} LIMIT 0", table))?;
        let column_count = stmt.column_count();
        let columns: Vec<String> = (0..column_count)
            .map(|i| stmt.column_name(i).unwrap_or("").to_string())
            .collect();

        // Write header
        writeln!(file, "{}", columns.join(","))?;

        // Write rows
        let mut stmt = conn.prepare(&format!("SELECT * FROM {}", table))?;
        let rows = stmt.query_map([], |row| {
            let values: Vec<String> = (0..column_count)
                .map(|i| {
                    let value: rusqlite::types::Value = row.get(i).unwrap();
                    match value {
                        rusqlite::types::Value::Null => String::new(),
                        rusqlite::types::Value::Integer(i) => i.to_string(),
                        rusqlite::types::Value::Real(f) => f.to_string(),
                        rusqlite::types::Value::Text(s) => {
                            format!("\"{}\"", s.replace("\"", "\"\""))
                        }
                        rusqlite::types::Value::Blob(_) => "BLOB".to_string(),
                    }
                })
                .collect();
            Ok(values)
        })?;

        for row in rows {
            let values = row?;
            writeln!(file, "{}", values.join(","))?;
        }

        Ok(())
    }

    /// Export all tables to CSV files in a directory
    pub fn export_all(conn: &Connection, output_dir: &Path) -> Result<()> {
        fs::create_dir_all(output_dir)?;

        let tables = Self::get_all_tables(conn)?;
        for table in tables {
            let output_path = output_dir.join(format!("{}.csv", table));
            Self::export_table(conn, &table, &output_path)?;
        }

        Ok(())
    }

    fn get_all_tables(conn: &Connection) -> Result<Vec<String>> {
        let mut stmt = conn.prepare(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name"
        )?;

        let tables = stmt
            .query_map([], |row| Ok(row.get::<_, String>(0)?))?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(tables)
    }
}
