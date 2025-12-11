//! CUDA 13.1+ Tiles Configuration
//!
//! T483: Implement CUDA 13.1+ tiles configuration
//! §3.2: Local-First & Offline-Capable
//! US2: CUDA tiles for optimized GPU utilization

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// CUDA tiles configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CudaTilesConfig {
    pub tile_width: u32,
    pub tile_height: u32,
    pub tile_depth: u32,
    pub warp_tiles: u32,
    pub block_tiles: u32,
}

impl Default for CudaTilesConfig {
    fn default() -> Self {
        Self {
            tile_width: 16,
            tile_height: 16,
            tile_depth: 1,
            warp_tiles: 2,
            block_tiles: 4,
        }
    }
}

/// CUDA tiles manager
pub struct CudaTilesManager {
    config: CudaTilesConfig,
}

impl CudaTilesManager {
    /// Create a new CUDA tiles manager
    pub fn new(config: CudaTilesConfig) -> Self {
        Self { config }
    }

    /// Create default tiles configuration
    pub fn with_defaults() -> Self {
        Self::new(CudaTilesConfig::default())
    }

    /// Configure tiles for a specific compute capability
    pub fn configure_for_compute_capability(&mut self, compute_capability: &str) -> Result<()> {
        // Configure based on compute capability
        // e.g., "8.0" for Ampere, "8.9" for Ada, "9.0" for Hopper
        match compute_capability {
            "9.0" | "9.0a" => {
                // Hopper architecture - optimized tiles
                self.config.tile_width = 32;
                self.config.tile_height = 32;
                self.config.warp_tiles = 4;
                self.config.block_tiles = 8;
            }
            "8.9" | "8.9a" => {
                // Ada architecture
                self.config.tile_width = 16;
                self.config.tile_height = 32;
                self.config.warp_tiles = 2;
                self.config.block_tiles = 4;
            }
            "8.0" | "8.6" => {
                // Ampere architecture
                self.config.tile_width = 16;
                self.config.tile_height = 16;
                self.config.warp_tiles = 2;
                self.config.block_tiles = 4;
            }
            _ => {
                // Default configuration
            }
        }

        Ok(())
    }

    /// Get optimal tile configuration for matrix multiplication
    pub fn get_matrix_tiles(&self, m: usize, n: usize, k: usize) -> Result<TileLayout> {
        Ok(TileLayout {
            m_tiles: (m as f64 / self.config.tile_height as f64).ceil() as usize,
            n_tiles: (n as f64 / self.config.tile_width as f64).ceil() as usize,
            k_tiles: (k as f64 / self.config.tile_depth as f64).ceil() as usize,
            tile_width: self.config.tile_width,
            tile_height: self.config.tile_height,
            tile_depth: self.config.tile_depth,
        })
    }

    /// Get configuration
    pub fn config(&self) -> &CudaTilesConfig {
        &self.config
    }
}

/// Tile layout for matrix operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileLayout {
    pub m_tiles: usize,
    pub n_tiles: usize,
    pub k_tiles: usize,
    pub tile_width: u32,
    pub tile_height: u32,
    pub tile_depth: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let manager = CudaTilesManager::with_defaults();
        assert_eq!(manager.config().tile_width, 16);
    }

    #[test]
    fn test_configure_for_compute_capability() {
        let mut manager = CudaTilesManager::with_defaults();
        manager.configure_for_compute_capability("9.0").unwrap();
        assert_eq!(manager.config().tile_width, 32);
    }

    #[test]
    fn test_get_matrix_tiles() {
        let manager = CudaTilesManager::with_defaults();
        let layout = manager.get_matrix_tiles(128, 256, 512).unwrap();
        assert!(layout.m_tiles > 0);
        assert!(layout.n_tiles > 0);
        assert!(layout.k_tiles > 0);
    }
}
