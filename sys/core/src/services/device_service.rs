//! Device Service
//!
//! T236: Implement device registration
//! US6: P2P Hive-Mind Device Federation
//! §3.8: P2P Hive-Mind

use crate::db::repositories::{DeviceRepository, Device, DeviceType, Platform, DeviceStatus};
use crate::db::Connection;
use crate::error::{NoaError, Result};
use chrono::Utc;
use serde_json::{Map, Value};
use uuid::Uuid;

/// Device service for device registration and management
pub struct DeviceService {
    repo: DeviceRepository,
}

impl DeviceService {
    /// Create a new device service
    pub fn new(conn: Connection) -> Self {
        Self {
            repo: DeviceRepository::new(conn),
        }
    }

    /// Register a new device
    ///
    /// Implements T236: Implement device registration
    pub fn register_device(
        &self,
        name: String,
        device_type: DeviceType,
        platform: Platform,
        peer_id: String,
        capabilities: Option<Map<String, Value>>,
        resources: Option<Map<String, Value>>,
        is_local: bool,
    ) -> Result<Uuid> {
        let device = Device {
            id: Uuid::new_v4(),
            name,
            device_type,
            platform,
            peer_id,
            status: DeviceStatus::Online,
            last_seen: Some(Utc::now()),
            capabilities,
            resources,
            is_local,
        };

        self.repo.create(&device)?;
        Ok(device.id)
    }

    /// Update device status
    pub fn update_status(&self, device_id: &Uuid, status: DeviceStatus) -> Result<()> {
        self.repo.update_status(device_id, status)?;
        Ok(())
    }

    /// Update last seen timestamp
    pub fn update_last_seen(&self, device_id: &Uuid) -> Result<()> {
        self.repo.update_last_seen(device_id, Utc::now())?;
        Ok(())
    }

    /// Get device by ID
    pub fn get_device(&self, device_id: &Uuid) -> Result<Option<Device>> {
        self.repo.find_by_id(device_id)
    }

    /// Get device by peer ID
    pub fn get_device_by_peer_id(&self, peer_id: &str) -> Result<Option<Device>> {
        self.repo.find_by_peer_id(peer_id)
    }

    /// Get local device
    pub fn get_local_device(&self) -> Result<Option<Device>> {
        self.repo.find_local()
    }

    /// List all devices
    pub fn list_devices(&self) -> Result<Vec<Device>> {
        self.repo.list()
    }

    /// List online devices
    pub fn list_online_devices(&self) -> Result<Vec<Device>> {
        let all = self.repo.list()?;
        Ok(all
            .into_iter()
            .filter(|d| matches!(d.status, DeviceStatus::Online))
            .collect())
    }
}

