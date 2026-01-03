//! Data tables for metadata

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A data table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTable {
    pub id: String,
    pub name: String,
    pub schema_id: String,
    pub columns: Vec<Column>,
    pub row_count: u64,
    pub size_bytes: u64,
    pub partitions: Vec<Partition>,
    pub statistics: TableStatistics,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A column in a data table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub statistics: ColumnStatistics,
}

/// Column statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnStatistics {
    pub null_count: u64,
    pub distinct_count: u64,
    pub min_value: Option<serde_json::Value>,
    pub max_value: Option<serde_json::Value>,
    pub avg_length: Option<f64>,
}

/// A partition of a data table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partition {
    pub id: String,
    pub key: String,
    pub value: String,
    pub row_count: u64,
    pub size_bytes: u64,
    pub path: Option<String>,
}

/// Table-level statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableStatistics {
    pub total_rows: u64,
    pub total_size_bytes: u64,
    pub avg_row_size_bytes: u64,
    pub last_analyzed: DateTime<Utc>,
    pub sample_rows: Vec<serde_json::Value>,
}

impl DataTable {
    /// Create a new data table
    pub fn new(name: impl Into<String>, schema_id: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            schema_id: schema_id.into(),
            columns: Vec::new(),
            row_count: 0,
            size_bytes: 0,
            partitions: Vec::new(),
            statistics: TableStatistics::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Add a column to the table
    pub fn add_column(&mut self, column: Column) -> &mut Self {
        self.columns.push(column);
        self.updated_at = Utc::now();
        self
    }

    /// Add a partition to the table
    pub fn add_partition(&mut self, partition: Partition) -> &mut Self {
        self.partitions.push(partition);
        self.updated_at = Utc::now();
        self
    }
}

impl Default for TableStatistics {
    fn default() -> Self {
        Self {
            total_rows: 0,
            total_size_bytes: 0,
            avg_row_size_bytes: 0,
            last_analyzed: Utc::now(),
            sample_rows: Vec::new(),
        }
    }
}

impl Default for ColumnStatistics {
    fn default() -> Self {
        Self {
            null_count: 0,
            distinct_count: 0,
            min_value: None,
            max_value: None,
            avg_length: None,
        }
    }
}
