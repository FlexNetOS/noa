//! NOA Database Commands
//!
//! Database management commands for NOA.

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use base64::engine::general_purpose;
use base64::Engine;
use clap::Subcommand;
use tracing::{info, warn};

use crate::config::NoaConfig;
use crate::db::{self, ConnectionPool, MigrationRunner};
use crate::error::Result;

/// Database subcommands
#[derive(Subcommand, Debug)]
pub enum DbCommands {
    /// Check database integrity
    Check {
        /// Fix issues if possible
        #[arg(long)]
        fix: bool,
    },

    /// Export database to file
    Export {
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Export format (sql, csv, json)
        #[arg(short, long, default_value = "sql")]
        format: String,

        /// Tables to export (comma-separated, or 'all')
        #[arg(short, long, default_value = "all")]
        tables: String,
    },

    /// Import data from file
    Import {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,
    },

    /// Show migration status
    Migrate {
        /// Apply pending migrations
        #[arg(long)]
        apply: bool,

        /// Rollback last migration
        #[arg(long)]
        rollback: bool,
    },

    /// Show database statistics
    Stats,

    /// Backup database
    Backup {
        /// Backup destination path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Vacuum database to reclaim space
    Vacuum,
}

/// Execute database command
pub async fn execute(command: DbCommands) -> Result<()> {
    let config = NoaConfig::load()?;
    let db_path = config.noa_root.join(&config.database.path);

    match command {
        DbCommands::Check { fix } => check_database(&db_path, fix),
        DbCommands::Export {
            output,
            format,
            tables,
        } => export_database(&db_path, &output, &format, &tables),
        DbCommands::Import { input } => import_database(&db_path, &input),
        DbCommands::Migrate { apply, rollback } => {
            migrate_database(&config.noa_root, &db_path, apply, rollback)
        }
        DbCommands::Stats => show_stats(&db_path),
        DbCommands::Backup { output } => {
            let output_path = output.map(PathBuf::from);
            backup_database(&db_path, output_path.as_ref())
        }
        DbCommands::Vacuum => vacuum_database(&db_path),
    }
}

/// Check database integrity
fn check_database(db_path: &PathBuf, fix: bool) -> Result<()> {
    println!("Checking database: {}", db_path.display());

    if !db_path.exists() {
        println!("Database file not found");
        return Ok(());
    }

    let conn = db::init_database(db_path)?;

    // Run integrity check
    match db::check_integrity(&conn) {
        Ok(true) => println!("✓ Integrity check: PASSED"),
        Ok(false) => {
            println!("✗ Integrity check: FAILED");
            if fix {
                println!("Attempting to fix...");
                // In SQLite, we can try REINDEX
                conn.execute_batch("REINDEX;")?;
                println!("Reindexed database");
            }
        }
        Err(e) => println!("✗ Integrity check error: {}", e),
    }

    // Check foreign keys
    let fk_result: i64 =
        conn.query_row("PRAGMA foreign_key_check", [], |row| row.get(0)).unwrap_or(0);

    if fk_result == 0 {
        println!("✓ Foreign key check: PASSED");
    } else {
        println!("✗ Foreign key violations found: {}", fk_result);
    }

    Ok(())
}

/// Export database
fn export_database(db_path: &PathBuf, output: &PathBuf, format: &str, tables: &str) -> Result<()> {
    println!(
        "Exporting database to {} (format: {})",
        output.display(),
        format
    );

    let conn = db::init_database(db_path)?;

    match format {
        "sql" => export_sql(&conn, output, tables),
        "csv" => export_csv(&conn, output, tables),
        "json" => export_json(&conn, output, tables),
        _ => {
            println!("Unsupported export format: {}", format);
            Ok(())
        }
    }
}

fn export_sql(conn: &rusqlite::Connection, output: &PathBuf, tables: &str) -> Result<()> {
    let mut file = fs::File::create(output)?;

    // Get table list
    let table_list = if tables == "all" {
        get_all_tables(conn)?
    } else {
        tables.split(',').map(|s| s.trim().to_string()).collect()
    };

    writeln!(file, "-- NOA Database Export")?;
    writeln!(file, "-- Generated: {}", chrono::Utc::now().to_rfc3339())?;
    writeln!(file)?;

    for table in table_list {
        // Get CREATE TABLE statement
        let sql: String = conn
            .query_row(
                "SELECT sql FROM sqlite_master WHERE type='table' AND name=?",
                [&table],
                |row| row.get(0),
            )
            .unwrap_or_default();

        if !sql.is_empty() {
            writeln!(file, "{};\n", sql)?;
        }

        // Get data
        let mut stmt = conn.prepare(&format!("SELECT * FROM {}", table))?;
        let column_count = stmt.column_count();
        let columns: Vec<String> = (0..column_count)
            .map(|i| stmt.column_name(i).unwrap_or("").to_string())
            .collect();

        let rows = stmt.query_map([], |row| {
            let values: Vec<String> = (0..column_count)
                .map(|i| {
                    let value: rusqlite::types::Value = row.get(i).unwrap();
                    match value {
                        rusqlite::types::Value::Null => "NULL".to_string(),
                        rusqlite::types::Value::Integer(i) => i.to_string(),
                        rusqlite::types::Value::Real(f) => f.to_string(),
                        rusqlite::types::Value::Text(s) => format!("'{}'", s.replace('\'', "''")),
                        rusqlite::types::Value::Blob(_) => "X'...'".to_string(),
                    }
                })
                .collect();
            Ok(values)
        })?;

        for row in rows {
            let values = row?;
            writeln!(
                file,
                "INSERT INTO {} ({}) VALUES ({});",
                table,
                columns.join(", "),
                values.join(", ")
            )?;
        }
        writeln!(file)?;
    }

    println!("✓ Exported to {}", output.display());
    Ok(())
}

fn export_csv(conn: &rusqlite::Connection, output: &PathBuf, tables: &str) -> Result<()> {
    let table_list = if tables == "all" {
        get_all_tables(conn)?
    } else {
        tables.split(',').map(|s| s.trim().to_string()).collect()
    };

    // Create output directory
    fs::create_dir_all(output)?;

    for table in table_list {
        let file_path = output.join(format!("{}.csv", table));
        let mut file = fs::File::create(&file_path)?;

        let mut stmt = conn.prepare(&format!("SELECT * FROM {}", table))?;
        let column_count = stmt.column_count();
        let columns: Vec<String> = (0..column_count)
            .map(|i| stmt.column_name(i).unwrap_or("").to_string())
            .collect();

        writeln!(file, "{}", columns.join(","))?;

        let rows = stmt.query_map([], |row| {
            let values: Vec<String> = (0..column_count)
                .map(|i| {
                    let value: rusqlite::types::Value = row.get(i).unwrap();
                    match value {
                        rusqlite::types::Value::Null => "".to_string(),
                        rusqlite::types::Value::Integer(i) => i.to_string(),
                        rusqlite::types::Value::Real(f) => f.to_string(),
                        rusqlite::types::Value::Text(s) => {
                            if s.contains(',') || s.contains('"') {
                                format!("\"{}\"", s.replace('"', "\"\""))
                            } else {
                                s
                            }
                        }
                        rusqlite::types::Value::Blob(_) => "[BLOB]".to_string(),
                    }
                })
                .collect();
            Ok(values)
        })?;

        for row in rows {
            writeln!(file, "{}", row?.join(","))?;
        }

        println!("  Exported {}", file_path.display());
    }

    println!("✓ Exported to {}", output.display());
    Ok(())
}

fn export_json(conn: &rusqlite::Connection, output: &PathBuf, tables: &str) -> Result<()> {
    let table_list = if tables == "all" {
        get_all_tables(conn)?
    } else {
        tables.split(',').map(|s| s.trim().to_string()).collect()
    };

    let mut data = serde_json::Map::new();

    for table in table_list {
        let mut stmt = conn.prepare(&format!("SELECT * FROM {}", table))?;
        let column_count = stmt.column_count();
        let columns: Vec<String> = (0..column_count)
            .map(|i| stmt.column_name(i).unwrap_or("").to_string())
            .collect();

        let rows: Vec<serde_json::Value> = stmt
            .query_map([], |row| {
                let mut obj = serde_json::Map::new();
                for (i, col) in columns.iter().enumerate() {
                    let value: rusqlite::types::Value = row.get(i).unwrap();
                    let json_value = match value {
                        rusqlite::types::Value::Null => serde_json::Value::Null,
                        rusqlite::types::Value::Integer(i) => serde_json::json!(i),
                        rusqlite::types::Value::Real(f) => serde_json::json!(f),
                        rusqlite::types::Value::Text(s) => serde_json::json!(s),
                        rusqlite::types::Value::Blob(b) => {
                            serde_json::json!(base64::engine::general_purpose::STANDARD.encode(&b))
                        }
                    };
                    obj.insert(col.clone(), json_value);
                }
                Ok(serde_json::Value::Object(obj))
            })?
            .filter_map(|r| r.ok())
            .collect();

        data.insert(table, serde_json::Value::Array(rows));
    }

    let json = serde_json::to_string_pretty(&serde_json::Value::Object(data))?;
    fs::write(output, json)?;

    println!("✓ Exported to {}", output.display());
    Ok(())
}

fn get_all_tables(conn: &rusqlite::Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
    )?;

    let tables: Vec<String> =
        stmt.query_map([], |row| row.get(0))?.filter_map(|r| r.ok()).collect();

    Ok(tables)
}

/// Import database
fn import_database(db_path: &PathBuf, input: &PathBuf) -> Result<()> {
    println!(
        "Importing from {} to {}",
        input.display(),
        db_path.display()
    );

    let content = fs::read_to_string(input)?;
    let conn = db::init_database(db_path)?;

    conn.execute_batch(&content)?;

    println!("✓ Import complete");
    Ok(())
}

/// Migrate database
fn migrate_database(
    noa_root: &PathBuf,
    db_path: &PathBuf,
    apply: bool,
    rollback: bool,
) -> Result<()> {
    let migrations_dir = noa_root.join("init/migrations");
    let conn = db::init_database(db_path)?;
    let runner = MigrationRunner::new(&migrations_dir);

    let status = runner.status(&conn)?;

    println!("Migration Status");
    println!("================");
    println!("Total migrations: {}", status.total);
    println!("Applied: {}", status.applied);
    println!("Pending: {}", status.pending);

    if !status.pending_versions.is_empty() {
        println!("\nPending migrations:");
        for version in &status.pending_versions {
            println!("  - {}", version);
        }
    }

    if apply && status.pending > 0 {
        println!("\nApplying pending migrations...");
        let applied = runner.apply_pending(&conn)?;
        println!("✓ Applied {} migrations", applied.len());
    }

    if rollback {
        println!("\nRolling back last migration...");
        if let Some(version) = runner.rollback_last(&conn)? {
            println!("✓ Rolled back migration: {}", version);
        } else {
            println!("No migrations to rollback");
        }
    }

    Ok(())
}

/// Show database statistics
fn show_stats(db_path: &PathBuf) -> Result<()> {
    println!("Database Statistics");
    println!("===================");
    println!("Path: {}", db_path.display());

    if !db_path.exists() {
        println!("Database file not found");
        return Ok(());
    }

    let conn = db::init_database(db_path)?;
    let stats = db::get_stats(&conn)?;

    println!(
        "Total size: {} bytes ({:.2} MB)",
        stats.total_size_bytes,
        stats.total_size_bytes as f64 / 1024.0 / 1024.0
    );
    println!(
        "Used size: {} bytes ({:.2} MB)",
        stats.used_size_bytes,
        stats.used_size_bytes as f64 / 1024.0 / 1024.0
    );
    println!("Total pages: {}", stats.total_pages);
    println!("Page size: {} bytes", stats.page_size);
    println!("Free pages: {}", stats.free_pages);

    // Table sizes
    println!("\nTable Sizes:");
    let tables = get_all_tables(&conn)?;
    for table in tables {
        let count: i64 = conn
            .query_row(&format!("SELECT COUNT(*) FROM {}", table), [], |row| {
                row.get(0)
            })
            .unwrap_or(0);
        println!("  {}: {} rows", table, count);
    }

    Ok(())
}

/// Backup database
fn backup_database(db_path: &PathBuf, output: Option<&PathBuf>) -> Result<()> {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let default_output = db_path.with_file_name(format!(
        "{}_backup_{}.db",
        db_path.file_stem().unwrap_or_default().to_string_lossy(),
        timestamp
    ));
    let output_path = output.unwrap_or(&default_output);

    println!("Backing up database to {}", output_path.display());

    // Simple file copy for SQLite
    fs::copy(db_path, output_path)?;

    println!("✓ Backup complete");
    Ok(())
}

/// Vacuum database
fn vacuum_database(db_path: &PathBuf) -> Result<()> {
    println!("Vacuuming database: {}", db_path.display());

    let conn = db::init_database(db_path)?;

    // Get size before
    let stats_before = db::get_stats(&conn)?;

    conn.execute_batch("VACUUM;")?;

    // Get size after
    let stats_after = db::get_stats(&conn)?;

    let freed = stats_before.total_size_bytes.saturating_sub(stats_after.total_size_bytes);

    println!("✓ Vacuum complete");
    println!("  Before: {} bytes", stats_before.total_size_bytes);
    println!("  After: {} bytes", stats_after.total_size_bytes);
    println!("  Freed: {} bytes", freed);

    Ok(())
}
