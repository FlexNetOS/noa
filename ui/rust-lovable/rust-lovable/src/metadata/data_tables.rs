use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTable {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub columns: Vec<ColumnDefinition>,
    pub rows: Vec<Row>,
    pub metadata: TableMetadata,
    pub constraints: Vec<TableConstraint>,
    pub indexes: Vec<TableIndex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDefinition {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub default_value: Option<serde_json::Value>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
    DateTime,
    Json,
    Array(Box<DataType>),
    Binary,
    Reference(String),
    Map(Box<DataType>, Box<DataType>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Row {
    pub id: String,
    pub values: HashMap<String, serde_json::Value>,
    pub metadata: RowMetadata,
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
    pub updated_by: Option<String>,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableMetadata {
    pub row_count: u64,
    pub total_size_bytes: u64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub data_quality: DataQualityMetrics,
    pub lineage: TableLineage,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityMetrics {
    pub completeness: f64,
    pub accuracy: f64,
    pub consistency: f64,
    pub validity: f64,
    pub uniqueness: f64,
    pub overall_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableLineage {
    pub source_tables: Vec<String>,
    pub transformation_steps: Vec<TransformationStep>,
    pub downstream_tables: Vec<String>,
    pub last_refresh: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationStep {
    pub step_id: String,
    pub operation: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub input_checksum: String,
    pub output_checksum: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableConstraint {
    pub name: String,
    pub constraint_type: ConstraintType,
    pub columns: Vec<String>,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConstraintType {
    PrimaryKey,
    ForeignKey,
    Unique,
    Check,
    NotNull,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableIndex {
    pub name: String,
    pub index_type: IndexType,
    pub columns: Vec<String>,
    pub unique: bool,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IndexType {
    BTree,
    Hash,
    Bitmap,
    FullText,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQuery {
    pub filters: Vec<FilterCondition>,
    pub sort_by: Vec<SortCondition>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub aggregates: Vec<AggregateCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    pub column: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilterOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like,
    In,
    NotIn,
    IsNull,
    IsNotNull,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortCondition {
    pub column: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateCondition {
    pub function: AggregateFunction,
    pub column: String,
    pub alias: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AggregateFunction {
    Count,
    Sum,
    Average,
    Min,
    Max,
    DistinctCount,
}

impl DataTable {
    pub fn new(name: String, version: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            name,
            version,
            description: None,
            columns: Vec::new(),
            rows: Vec::new(),
            metadata: TableMetadata {
                row_count: 0,
                total_size_bytes: 0,
                last_updated: now,
                data_quality: DataQualityMetrics {
                    completeness: 1.0,
                    accuracy: 1.0,
                    consistency: 1.0,
                    validity: 1.0,
                    uniqueness: 1.0,
                    overall_score: 1.0,
                },
                lineage: TableLineage {
                    source_tables: Vec::new(),
                    transformation_steps: Vec::new(),
                    downstream_tables: Vec::new(),
                    last_refresh: now,
                },
                tags: Vec::new(),
            },
            constraints: Vec::new(),
            indexes: Vec::new(),
        }
    }
    
    pub fn add_column(&mut self, column: ColumnDefinition) {
        self.columns.push(column);
    }
    
    pub fn add_row(&mut self, row: Row) -> Result<()> {
        // Validate row against schema
        self.validate_row(&row)?;
        
        self.rows.push(row);
        self.metadata.row_count += 1;
        self.metadata.last_updated = chrono::Utc::now();
        
        Ok(())
    }
    
    pub fn update_row(&mut self, row_id: &str, updates: HashMap<String, serde_json::Value>) -> Result<()> {
        if let Some(row) = self.rows.iter_mut().find(|r| r.id == row_id) {
            // Apply updates
            for (column, value) in updates {
                if self.columns.iter().any(|c| c.name == column) {
                    row.values.insert(column, value);
                }
            }
            
            // Update metadata
            row.version += 1;
            row.metadata.updated_at = chrono::Utc::now();
            self.metadata.last_updated = chrono::Utc::now();
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Row not found: {}", row_id))
        }
    }
    
    pub fn delete_row(&mut self, row_id: &str) -> Result<()> {
        if let Some(index) = self.rows.iter().position(|r| r.id == row_id) {
            self.rows.remove(index);
            self.metadata.row_count -= 1;
            self.metadata.last_updated = chrono::Utc::now();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Row not found: {}", row_id))
        }
    }
    
    pub fn query(&self, query: DataQuery) -> Vec<Row> {
        let mut results = self.rows.clone();
        
        // Apply filters
        for filter in &query.filters {
            results.retain(|row| self.matches_filter(row, filter));
        }
        
        // Apply sorting
        if !query.sort_by.is_empty() {
            results.sort_by(|a, b| self.compare_rows(a, b, &query.sort_by));
        }
        
        // Apply pagination
        let start = query.offset.unwrap_or(0) as usize;
        let end = query.limit.map_or(results.len(), |l| start + l as usize);
        
        results.into_iter().skip(start).take(end - start).collect()
    }
    
    pub fn aggregate(&self, query: DataQuery) -> HashMap<String, serde_json::Value> {
        let filtered_rows = self.query(query.clone());
        let mut results = HashMap::new();
        
        for aggregate in &query.aggregates {
            let value = self.calculate_aggregate(&filtered_rows, aggregate);
            results.insert(aggregate.alias.clone(), value);
        }
        
        results
    }
    
    pub fn get_column(&self, name: &str) -> Option<&ColumnDefinition> {
        self.columns.iter().find(|c| c.name == name)
    }
    
    pub fn update_quality_metrics(&mut self) {
        // Calculate completeness
        let total_cells = self.rows.len() * self.columns.len();
        let null_cells = self.rows.iter()
            .flat_map(|row| &row.values)
            .filter(|(_, value)| value.is_null())
            .count();
        
        self.metadata.data_quality.completeness = 
            (total_cells - null_cells) as f64 / total_cells as f64;
        
        // Calculate other metrics based on business rules
        // For now, set them to perfect scores
        self.metadata.data_quality.accuracy = 1.0;
        self.metadata.data_quality.consistency = 1.0;
        self.metadata.data_quality.validity = 1.0;
        self.metadata.data_quality.uniqueness = 1.0;
        
        // Calculate overall score
        self.metadata.data_quality.overall_score = (
            self.metadata.data_quality.completeness +
            self.metadata.data_quality.accuracy +
            self.metadata.data_quality.consistency +
            self.metadata.data_quality.validity +
            self.metadata.data_quality.uniqueness
        ) / 5.0;
    }
    
    fn validate_row(&self, row: &Row) -> Result<()> {
        // Check required columns
        for column in &self.columns {
            if !column.nullable && column.default_value.is_none() {
                if !row.values.contains_key(&column.name) || row.values[&column.name].is_null() {
                    return Err(anyhow::anyhow!("Missing required column: {}", column.name));
                }
            }
        }
        
        // Validate data types
        for (column_name, value) in &row.values {
            if let Some(column) = self.get_column(column_name) {
                self.validate_value_type(value, &column.data_type)?;
            }
        }
        
        Ok(())
    }
    
    fn validate_value_type(&self, value: &serde_json::Value, data_type: &DataType) -> Result<()> {
        match data_type {
            DataType::String => {
                if !value.is_string() && !value.is_null() {
                    return Err(anyhow::anyhow!("Expected string value"));
                }
            }
            DataType::Integer => {
                if !value.is_i64() && !value.is_u64() && !value.is_null() {
                    return Err(anyhow::anyhow!("Expected integer value"));
                }
            }
            DataType::Float => {
                if !value.is_f64() && !value.is_i64() && !value.is_u64() && !value.is_null() {
                    return Err(anyhow::anyhow!("Expected float value"));
                }
            }
            DataType::Boolean => {
                if !value.is_boolean() && !value.is_null() {
                    return Err(anyhow::anyhow!("Expected boolean value"));
                }
            }
            DataType::DateTime => {
                if !value.is_string() && !value.is_null() {
                    return Err(anyhow::anyhow!("Expected datetime string value"));
                }
            }
            DataType::Json => {
                // Any JSON value is valid
            }
            DataType::Array(element_type) => {
                if let serde_json::Value::Array(arr) = value {
                    for element in arr {
                        self.validate_value_type(element, element_type)?;
                    }
                } else if !value.is_null() {
                    return Err(anyhow::anyhow!("Expected array value"));
                }
            }
            DataType::Map(_key_type, value_type) => {
                if let serde_json::Value::Object(map) = value {
                    for (_, v) in map {
                        self.validate_value_type(v, value_type)?;
                    }
                } else if !value.is_null() {
                    return Err(anyhow::anyhow!("Expected object value"));
                }
            }
            DataType::Binary => {
                if !value.is_string() && !value.is_null() {
                    return Err(anyhow::anyhow!("Expected binary string value"));
                }
            }
            DataType::Reference(_) => {
                // References are validated elsewhere
            }
        }
        
        Ok(())
    }
    
    fn matches_filter(&self, row: &Row, filter: &FilterCondition) -> bool {
        let value = row.values.get(&filter.column);
        
        match filter.operator {
            FilterOperator::Equal => {
                value.map_or(false, |v| v == &filter.value)
            }
            FilterOperator::NotEqual => {
                value.map_or(true, |v| v != &filter.value)
            }
            FilterOperator::GreaterThan => {
                self.compare_numeric_values(value, &filter.value) == Some(std::cmp::Ordering::Greater)
            }
            FilterOperator::GreaterThanOrEqual => {
                self.compare_numeric_values(value, &filter.value).map_or(false, |o| o != std::cmp::Ordering::Less)
            }
            FilterOperator::LessThan => {
                self.compare_numeric_values(value, &filter.value) == Some(std::cmp::Ordering::Less)
            }
            FilterOperator::LessThanOrEqual => {
                self.compare_numeric_values(value, &filter.value).map_or(false, |o| o != std::cmp::Ordering::Greater)
            }
            FilterOperator::Like => {
                // Simple string matching for now
                if let (Some(serde_json::Value::String(s)), serde_json::Value::String(pattern)) = (value, &filter.value) {
                    s.contains(pattern)
                } else {
                    false
                }
            }
            FilterOperator::In => {
                if let serde_json::Value::Array(arr) = &filter.value {
                    value.map_or(false, |v| arr.contains(v))
                } else {
                    false
                }
            }
            FilterOperator::NotIn => {
                if let serde_json::Value::Array(arr) = &filter.value {
                    value.map_or(true, |v| !arr.contains(v))
                } else {
                    true
                }
            }
            FilterOperator::IsNull => {
                value.map_or(true, |v| v.is_null())
            }
            FilterOperator::IsNotNull => {
                value.map_or(false, |v| !v.is_null())
            }
        }
    }
    
    fn compare_numeric_values(&self, a: Option<&serde_json::Value>, b: &serde_json::Value) -> Option<std::cmp::Ordering> {
        let a_num = a.and_then(|v| v.as_f64());
        let b_num = b.as_f64();
        
        match (a_num, b_num) {
            (Some(a), Some(b)) => Some(a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)),
            _ => None,
        }
    }
    
    fn compare_rows(&self, a: &Row, b: &Row, sort_conditions: &[SortCondition]) -> std::cmp::Ordering {
        for condition in sort_conditions {
            let a_value = a.values.get(&condition.column);
            let b_value = b.values.get(&condition.column);
            
            let cmp = match (a_value, b_value) {
                (Some(va), Some(vb)) => self.compare_values(va, vb),
                (Some(_), None) => std::cmp::Ordering::Greater,
                (None, Some(_)) => std::cmp::Ordering::Less,
                (None, None) => std::cmp::Ordering::Equal,
            };
            
            if cmp != std::cmp::Ordering::Equal {
                return match condition.direction {
                    SortDirection::Asc => cmp,
                    SortDirection::Desc => cmp.reverse(),
                };
            }
        }
        
        std::cmp::Ordering::Equal
    }
    
    fn compare_values(&self, a: &serde_json::Value, b: &serde_json::Value) -> std::cmp::Ordering {
        match (a, b) {
            (serde_json::Value::String(sa), serde_json::Value::String(sb)) => sa.cmp(sb),
            (serde_json::Value::Number(na), serde_json::Value::Number(nb)) => {
                na.as_f64().unwrap_or(0.0).partial_cmp(&nb.as_f64().unwrap_or(0.0))
                    .unwrap_or(std::cmp::Ordering::Equal)
            }
            (serde_json::Value::Bool(ba), serde_json::Value::Bool(bb)) => ba.cmp(bb),
            _ => std::cmp::Ordering::Equal,
        }
    }
    
    fn calculate_aggregate(&self, rows: &[Row], aggregate: &AggregateCondition) -> serde_json::Value {
        let values: Vec<f64> = rows.iter()
            .filter_map(|row| row.values.get(&aggregate.column))
            .filter_map(|v| v.as_f64())
            .collect();
        
        match aggregate.function {
            AggregateFunction::Count => serde_json::Value::Number(serde_json::Number::from(values.len())),
            AggregateFunction::Sum => {
                let sum: f64 = values.iter().sum();
                serde_json::Value::Number(serde_json::Number::from_f64(sum).unwrap_or(serde_json::Number::from(0)))
            }
            AggregateFunction::Average => {
                if values.is_empty() {
                    serde_json::Value::Number(serde_json::Number::from(0))
                } else {
                    let avg: f64 = values.iter().sum::<f64>() / values.len() as f64;
                    serde_json::Value::Number(serde_json::Number::from_f64(avg).unwrap_or(serde_json::Number::from(0)))
                }
            }
            AggregateFunction::Min => {
                values.iter().cloned().fold(f64::INFINITY, f64::min)
                    .try_into().unwrap_or(serde_json::Value::Number(serde_json::Number::from(0)))
            }
            AggregateFunction::Max => {
                values.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                    .try_into().unwrap_or(serde_json::Value::Number(serde_json::Number::from(0)))
            }
            AggregateFunction::DistinctCount => {
                // Use ordered bits to handle f64 uniqueness since f64 doesn't implement Hash
                let unique: std::collections::HashSet<u64> = values.iter()
                    .map(|v| v.to_bits())
                    .collect();
                serde_json::Value::Number(serde_json::Number::from(unique.len()))
            }
        }
    }
}

impl Row {
    pub fn new(values: HashMap<String, serde_json::Value>, created_by: String) -> Self {
        let now = chrono::Utc::now();
        let checksum = Self::calculate_checksum(&values);
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            values,
            metadata: RowMetadata {
                created_at: now,
                updated_at: now,
                created_by,
                updated_by: None,
                checksum,
            },
            version: 1,
        }
    }
    
    pub fn update(&mut self, updates: HashMap<String, serde_json::Value>, updated_by: String) {
        for (key, value) in updates {
            self.values.insert(key, value);
        }
        
        self.metadata.updated_at = chrono::Utc::now();
        self.metadata.updated_by = Some(updated_by);
        self.metadata.checksum = Self::calculate_checksum(&self.values);
        self.version += 1;
    }
    
    fn calculate_checksum(values: &HashMap<String, serde_json::Value>) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        
        // Sort keys for consistent hashing
        let mut sorted_keys: Vec<_> = values.keys().collect();
        sorted_keys.sort();
        
        for key in sorted_keys {
            key.hash(&mut hasher);
            values[key].to_string().hash(&mut hasher);
        }
        
        format!("{:x}", hasher.finish())
    }
}