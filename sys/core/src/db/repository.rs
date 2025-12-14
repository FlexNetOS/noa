//! Repository Trait Pattern
//!
//! Defines generic repository interface for database operations.
//! §3.1: Core abstractions

use std::fmt;

use crate::error::Result;

/// Repository error type
#[derive(Debug)]
pub enum RepositoryError {
    NotFound { entity: String, id: String },
    AlreadyExists { entity: String, id: String },
    InvalidData { message: String },
    DatabaseError { message: String },
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::NotFound { entity, id } => {
                write!(f, "{} not found: {}", entity, id)
            }
            RepositoryError::AlreadyExists { entity, id } => {
                write!(f, "{} already exists: {}", entity, id)
            }
            RepositoryError::InvalidData { message } => {
                write!(f, "Invalid data: {}", message)
            }
            RepositoryError::DatabaseError { message } => {
                write!(f, "Database error: {}", message)
            }
        }
    }
}

impl std::error::Error for RepositoryError {}

/// Generic repository trait for CRUD operations
pub trait Repository<T, Id> {
    /// Create a new entity
    fn create(&self, entity: &T) -> Result<Id>;

    /// Find entity by ID
    fn find_by_id(&self, id: &Id) -> Result<Option<T>>;

    /// Update an existing entity
    fn update(&self, entity: &T) -> Result<()>;

    /// Delete entity by ID
    fn delete(&self, id: &Id) -> Result<bool>;

    /// Check if entity exists
    fn exists(&self, id: &Id) -> Result<bool>;

    /// Count all entities
    fn count(&self) -> Result<u64>;
}

/// Extended repository trait with query capabilities
pub trait QueryableRepository<T, Id>: Repository<T, Id> {
    /// Find all entities
    fn find_all(&self) -> Result<Vec<T>>;

    /// Find entities with pagination
    fn find_paginated(&self, offset: u64, limit: u64) -> Result<Vec<T>>;

    /// Find entities by filter
    fn find_by_filter(&self, filter: &Filter) -> Result<Vec<T>>;
}

/// Generic filter for queries
#[derive(Debug, Clone)]
pub struct Filter {
    pub conditions: Vec<Condition>,
    pub order_by: Option<OrderBy>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
            order_by: None,
            limit: None,
            offset: None,
        }
    }

    pub fn with_condition(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn with_order_by(mut self, field: impl Into<String>, direction: Direction) -> Self {
        self.order_by = Some(OrderBy {
            field: field.into(),
            direction,
        });
        self
    }

    pub fn with_limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}

/// Query condition
#[derive(Debug, Clone)]
pub struct Condition {
    pub field: String,
    pub operator: Operator,
    pub value: Value,
}

impl Condition {
    pub fn eq(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self {
            field: field.into(),
            operator: Operator::Equal,
            value: value.into(),
        }
    }

    pub fn ne(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self {
            field: field.into(),
            operator: Operator::NotEqual,
            value: value.into(),
        }
    }

    pub fn gt(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self {
            field: field.into(),
            operator: Operator::GreaterThan,
            value: value.into(),
        }
    }

    pub fn lt(field: impl Into<String>, value: impl Into<Value>) -> Self {
        Self {
            field: field.into(),
            operator: Operator::LessThan,
            value: value.into(),
        }
    }

    pub fn contains(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            operator: Operator::Contains,
            value: Value::String(value.into()),
        }
    }

    pub fn is_in(field: impl Into<String>, values: Vec<Value>) -> Self {
        Self {
            field: field.into(),
            operator: Operator::In,
            value: Value::Array(values),
        }
    }
}

/// Query operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Contains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
    IsNull,
    IsNotNull,
}

/// Query value types
#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
    Array(Vec<Value>),
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Integer(n)
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Value::Integer(n as i64)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Float(n)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

/// Order by clause
#[derive(Debug, Clone)]
pub struct OrderBy {
    pub field: String,
    pub direction: Direction,
}

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Ascending,
    Descending,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Ascending
    }
}

/// Paginated result
#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub offset: u64,
    pub limit: u64,
    pub has_more: bool,
}

impl<T> PaginatedResult<T> {
    pub fn new(items: Vec<T>, total: u64, offset: u64, limit: u64) -> Self {
        let has_more = offset + (items.len() as u64) < total;
        Self {
            items,
            total,
            offset,
            limit,
            has_more,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_builder() {
        let filter = Filter::new()
            .with_condition(Condition::eq("status", "active"))
            .with_condition(Condition::gt("priority", 5))
            .with_order_by("created_at", Direction::Descending)
            .with_limit(10)
            .with_offset(0);

        assert_eq!(filter.conditions.len(), 2);
        assert!(filter.order_by.is_some());
        assert_eq!(filter.limit, Some(10));
    }

    #[test]
    fn test_paginated_result() {
        let items = vec![1, 2, 3, 4, 5];
        let result = PaginatedResult::new(items, 100, 0, 10);

        assert_eq!(result.items.len(), 5);
        assert!(result.has_more);
    }
}

