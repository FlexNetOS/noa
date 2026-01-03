//! Device Repository
//!
//! T235: §3.8 Implement Device repository
//! US6: P2P Hive-Mind Device Federation
//! FR-019: P2P device federation

use crate::db::Connection;
use crate::error::{DatabaseError, NoaError, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Row};
use serde_json::{Map, Value};
use uuid::Uuid;

/// Device entity representing a P2P network device
#[derive(Debug, Clone)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub device_type: DeviceType,
    pub platform: Platform,
    pub peer_id: String,
    pub status: DeviceStatus,
    pub last_seen: Option<DateTime<Utc>>,
    pub capabilities: Option<Map<String, Value>>,
    pub resources: Option<Map<String, Value>>,
    pub is_local: bool,
}

/// Device type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Desktop,
    Laptop,
    Mobile,
    Server,
}

impl DeviceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeviceType::Desktop => "desktop",
            DeviceType::Laptop => "laptop",
            DeviceType::Mobile => "mobile",
            DeviceType::Server => "server",
        }
    }

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "desktop" => Ok(DeviceType::Desktop),
            "laptop" => Ok(DeviceType::Laptop),
            "mobile" => Ok(DeviceType::Mobile),
            "server" => Ok(DeviceType::Server),
            _ => Err(NoaError::Validation(crate::error::ValidationError::new(
                "device_type",
                format!("Invalid device type: {}", s),
                "INVALID_DEVICE_TYPE",
            ))),
        }
    }
}

/// Platform enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
    IOS,
    Android,
}

impl Platform {
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Windows => "windows",
            Platform::MacOS => "macos",
            Platform::Linux => "linux",
            Platform::IOS => "ios",
            Platform::Android => "android",
        }
    }

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "windows" => Ok(Platform::Windows),
            "macos" => Ok(Platform::MacOS),
            "linux" => Ok(Platform::Linux),
            "ios" => Ok(Platform::IOS),
            "android" => Ok(Platform::Android),
            _ => Err(NoaError::Validation(crate::error::ValidationError::new(
                "platform",
                format!("Invalid platform: {}", s),
                "INVALID_PLATFORM",
            ))),
        }
    }
}

/// Device status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceStatus {
    Online,
    Offline,
    Syncing,
}

impl DeviceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeviceStatus::Online => "online",
            DeviceStatus::Offline => "offline",
            DeviceStatus::Syncing => "syncing",
        }
    }

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "online" => Ok(DeviceStatus::Online),
            "offline" => Ok(DeviceStatus::Offline),
            "syncing" => Ok(DeviceStatus::Syncing),
            _ => Err(NoaError::Validation(crate::error::ValidationError::new(
                "status",
                format!("Invalid device status: {}", s),
                "INVALID_STATUS",
            ))),
        }
    }
}

/// Device repository for CRUD operations
pub struct DeviceRepository {
    conn: Connection,
}

impl DeviceRepository {
    /// Create a new device repository
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Create a new device
    pub fn create(&self, device: &Device) -> Result<Uuid> {
        let capabilities_json = device
            .capabilities
            .as_ref()
            .map(|c| serde_json::to_string(c))
            .transpose()
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "serialize capabilities".to_string(),
                    error: e.to_string(),
                })
            })?;

        let resources_json = device
            .resources
            .as_ref()
            .map(|r| serde_json::to_string(r))
            .transpose()
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "serialize resources".to_string(),
                    error: e.to_string(),
                })
            })?;

        self.conn
            .execute(
                r#"
                INSERT INTO device (
                    id, name, type, platform, peer_id, status,
                    last_seen, capabilities, resources, is_local
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                "#,
                params![
                    device.id.to_string(),
                    device.name,
                    device.device_type.as_str(),
                    device.platform.as_str(),
                    device.peer_id,
                    device.status.as_str(),
                    device.last_seen.map(|d| d.to_rfc3339()),
                    capabilities_json,
                    resources_json,
                    if device.is_local { 1 } else { 0 },
                ],
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "INSERT INTO device".to_string(),
                    error: e.to_string(),
                })
            })?;

        Ok(device.id)
    }

    /// Find device by ID
    pub fn find_by_id(&self, id: &Uuid) -> Result<Option<Device>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, name, type, platform, peer_id, status,
                       last_seen, capabilities, resources, is_local
                FROM device
                WHERE id = ?1
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM device".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut rows = stmt
            .query_map(params![id.to_string()], |row| self.row_to_device(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM device".to_string(),
                    error: e.to_string(),
                })
            })?;

        match rows.next() {
            Some(Ok(device)) => Ok(Some(device)),
            Some(Err(e)) => Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT FROM device".to_string(),
                error: e.to_string(),
            })),
            None => Ok(None),
        }
    }

    /// Find device by peer ID
    pub fn find_by_peer_id(&self, peer_id: &str) -> Result<Option<Device>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, name, type, platform, peer_id, status,
                       last_seen, capabilities, resources, is_local
                FROM device
                WHERE peer_id = ?1
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM device".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut rows = stmt
            .query_map(params![peer_id], |row| self.row_to_device(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM device".to_string(),
                    error: e.to_string(),
                })
            })?;

        match rows.next() {
            Some(Ok(device)) => Ok(Some(device)),
            Some(Err(e)) => Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT FROM device".to_string(),
                error: e.to_string(),
            })),
            None => Ok(None),
        }
    }

    /// Find local device
    pub fn find_local(&self) -> Result<Option<Device>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, name, type, platform, peer_id, status,
                       last_seen, capabilities, resources, is_local
                FROM device
                WHERE is_local = 1
                LIMIT 1
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM device".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut rows = stmt
            .query_map(params![], |row| self.row_to_device(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM device".to_string(),
                    error: e.to_string(),
                })
            })?;

        match rows.next() {
            Some(Ok(device)) => Ok(Some(device)),
            Some(Err(e)) => Err(NoaError::Database(DatabaseError::QueryFailed {
                query: "SELECT FROM device".to_string(),
                error: e.to_string(),
            })),
            None => Ok(None),
        }
    }

    /// List all devices
    pub fn list(&self) -> Result<Vec<Device>> {
        let mut stmt = self
            .conn
            .prepare(
                r#"
                SELECT id, name, type, platform, peer_id, status,
                       last_seen, capabilities, resources, is_local
                FROM device
                ORDER BY last_seen DESC
                "#,
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM device".to_string(),
                    error: e.to_string(),
                })
            })?;

        let rows = stmt
            .query_map(params![], |row| self.row_to_device(row))
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "SELECT FROM device".to_string(),
                    error: e.to_string(),
                })
            })?;

        let mut devices = Vec::new();
        for row in rows {
            devices.push(row?);
        }
        Ok(devices)
    }

    /// Update device status
    pub fn update_status(&self, id: &Uuid, status: DeviceStatus) -> Result<()> {
        self.conn
            .execute(
                "UPDATE device SET status = ?1 WHERE id = ?2",
                params![status.as_str(), id.to_string()],
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "UPDATE device".to_string(),
                    error: e.to_string(),
                })
            })?;
        Ok(())
    }

    /// Update last seen timestamp
    pub fn update_last_seen(&self, id: &Uuid, last_seen: DateTime<Utc>) -> Result<()> {
        self.conn
            .execute(
                "UPDATE device SET last_seen = ?1 WHERE id = ?2",
                params![last_seen.to_rfc3339(), id.to_string()],
            )
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "UPDATE device".to_string(),
                    error: e.to_string(),
                })
            })?;
        Ok(())
    }

    /// Delete device
    pub fn delete(&self, id: &Uuid) -> Result<()> {
        self.conn
            .execute("DELETE FROM device WHERE id = ?1", params![id.to_string()])
            .map_err(|e| {
                NoaError::Database(DatabaseError::QueryFailed {
                    query: "DELETE FROM device".to_string(),
                    error: e.to_string(),
                })
            })?;
        Ok(())
    }

    /// Convert database row to Device
    fn row_to_device(&self, row: &Row) -> rusqlite::Result<Device> {
        let id_str: String = row.get(0)?;
        let id = Uuid::parse_str(&id_str).map_err(|_e| {
            rusqlite::Error::InvalidColumnType(0, "UUID".to_string(), rusqlite::types::Type::Text)
        })?;

        let name: String = row.get(1)?;
        let type_str: String = row.get(2)?;
        let device_type = DeviceType::from_str(&type_str).map_err(|_e| {
            rusqlite::Error::InvalidColumnType(2, "device_type".to_string(), rusqlite::types::Type::Text)
        })?;

        let platform_str: String = row.get(3)?;
        let platform = Platform::from_str(&platform_str).map_err(|_e| {
            rusqlite::Error::InvalidColumnType(3, "platform".to_string(), rusqlite::types::Type::Text)
        })?;

        let peer_id: String = row.get(4)?;
        let status_str: String = row.get(5)?;
        let status = DeviceStatus::from_str(&status_str).map_err(|_e| {
            rusqlite::Error::InvalidColumnType(5, "status".to_string(), rusqlite::types::Type::Text)
        })?;

        let last_seen_str: Option<String> = row.get(6)?;
        let last_seen = last_seen_str
            .map(|s| DateTime::parse_from_rfc3339(&s).map(|d| d.with_timezone(&Utc)))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(6, "timestamp".to_string(), rusqlite::types::Type::Text)
            })?;

        let capabilities_json: Option<String> = row.get(7)?;
        let capabilities = capabilities_json
            .map(|json| serde_json::from_str(&json))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(7, "JSON".to_string(), rusqlite::types::Type::Text)
            })?;

        let resources_json: Option<String> = row.get(8)?;
        let resources = resources_json
            .map(|json| serde_json::from_str(&json))
            .transpose()
            .map_err(|_| {
                rusqlite::Error::InvalidColumnType(8, "JSON".to_string(), rusqlite::types::Type::Text)
            })?;

        let is_local: i32 = row.get(9)?;

        Ok(Device {
            id,
            name,
            device_type,
            platform,
            peer_id,
            status,
            last_seen,
            capabilities,
            resources,
            is_local: is_local != 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn setup_test_db() -> Connection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = Connection::open(&db_path).unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS device (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                type TEXT NOT NULL,
                platform TEXT NOT NULL,
                peer_id TEXT NOT NULL,
                status TEXT NOT NULL,
                last_seen TEXT,
                capabilities TEXT,
                resources TEXT,
                is_local INTEGER NOT NULL DEFAULT 0
            );
            "#,
        ).unwrap();
        std::mem::forget(dir);
        conn
    }

    #[test]
    fn test_device_create_and_find() {
        let conn = setup_test_db();
        let repo = DeviceRepository::new(conn);

        let device = Device {
            id: Uuid::new_v4(),
            name: "Test Workstation".to_string(),
            device_type: DeviceType::Desktop,
            platform: Platform::Windows,
            peer_id: "12D3KooWTest".to_string(),
            status: DeviceStatus::Online,
            last_seen: Some(Utc::now()),
            capabilities: None,
            resources: None,
            is_local: true,
        };

        let id = repo.create(&device).unwrap();
        assert_eq!(id, device.id);

        let found = repo.find_by_id(&device.id).unwrap().unwrap();
        assert_eq!(found.name, "Test Workstation");
        assert_eq!(found.device_type, DeviceType::Desktop);
        assert_eq!(found.platform, Platform::Windows);
        assert!(found.is_local);
    }

    #[test]
    fn test_device_types() {
        assert_eq!(DeviceType::Desktop.as_str(), "desktop");
        assert_eq!(DeviceType::Laptop.as_str(), "laptop");
        assert_eq!(DeviceType::Mobile.as_str(), "mobile");
        assert_eq!(DeviceType::Server.as_str(), "server");
        
        assert!(matches!(DeviceType::from_str("desktop"), Ok(DeviceType::Desktop)));
        assert!(DeviceType::from_str("invalid").is_err());
    }

    #[test]
    fn test_platform_types() {
        assert_eq!(Platform::Windows.as_str(), "windows");
        assert_eq!(Platform::MacOS.as_str(), "macos");
        assert_eq!(Platform::Linux.as_str(), "linux");
        assert_eq!(Platform::IOS.as_str(), "ios");
        assert_eq!(Platform::Android.as_str(), "android");
        
        assert!(matches!(Platform::from_str("windows"), Ok(Platform::Windows)));
        assert!(Platform::from_str("invalid").is_err());
    }

    #[test]
    fn test_device_status_types() {
        assert_eq!(DeviceStatus::Online.as_str(), "online");
        assert_eq!(DeviceStatus::Offline.as_str(), "offline");
        assert_eq!(DeviceStatus::Syncing.as_str(), "syncing");
        
        assert!(matches!(DeviceStatus::from_str("online"), Ok(DeviceStatus::Online)));
        assert!(DeviceStatus::from_str("invalid").is_err());
    }

    #[test]
    fn test_device_find_nonexistent() {
        let conn = setup_test_db();
        let repo = DeviceRepository::new(conn);

        let result = repo.find_by_id(&Uuid::new_v4()).unwrap();
        assert!(result.is_none());
    }
}

