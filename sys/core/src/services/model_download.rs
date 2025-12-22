//! Model Download Service
//!
//! T119: Implement model download with progress
//! US2: Model download with progress tracking

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Download status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Completed,
    Failed,
    Cancelled,
}

/// Download progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub download_id: Uuid,
    pub model_name: String,
    pub url: String,
    pub status: DownloadStatus,
    pub bytes_downloaded: u64,
    pub total_bytes: Option<u64>,
    pub progress_percent: f64,
    pub error: Option<String>,
}

/// Model download service
pub struct ModelDownloadService {
    active_downloads: Arc<RwLock<std::collections::HashMap<Uuid, DownloadProgress>>>,
}

impl ModelDownloadService {
    /// Create a new model download service
    pub fn new() -> Self {
        Self {
            active_downloads: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Start downloading a model
    pub async fn download_model(
        &self,
        model_name: String,
        url: String,
        output_path: PathBuf,
    ) -> Result<Uuid> {
        let download_id = Uuid::new_v4();

        let progress = DownloadProgress {
            download_id,
            model_name: model_name.clone(),
            url: url.clone(),
            status: DownloadStatus::Pending,
            bytes_downloaded: 0,
            total_bytes: None,
            progress_percent: 0.0,
            error: None,
        };

        {
            let mut downloads = self.active_downloads.write().await;
            downloads.insert(download_id, progress);
        }

        // Start download in background
        let downloads = self.active_downloads.clone();
        let url_clone = url.clone();
        let output_path_clone = output_path.clone();

        tokio::spawn(async move {
            Self::download_task(
                download_id,
                model_name,
                url_clone,
                output_path_clone,
                downloads,
            ).await;
        });

        Ok(download_id)
    }

    /// Download task implementation
    async fn download_task(
        download_id: Uuid,
        model_name: String,
        url: String,
        output_path: PathBuf,
        downloads: Arc<RwLock<std::collections::HashMap<Uuid, DownloadProgress>>>,
    ) {
        // Update status to downloading
        {
            let mut downloads = downloads.write().await;
            if let Some(progress) = downloads.get_mut(&download_id) {
                progress.status = DownloadStatus::Downloading;
            }
        }

        // Create output directory if needed
        if let Some(parent) = output_path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                let mut downloads = downloads.write().await;
                if let Some(progress) = downloads.get_mut(&download_id) {
                    progress.status = DownloadStatus::Failed;
                    progress.error = Some(format!("Failed to create directory: {}", e));
                }
                return;
            }
        }

        // Download file
        let client = reqwest::Client::new();
        match client.get(&url).send().await {
            Ok(response) => {
                let total_bytes = response.content_length();

                // Update total bytes
                {
                    let mut downloads = downloads.write().await;
                    if let Some(progress) = downloads.get_mut(&download_id) {
                        progress.total_bytes = total_bytes;
                    }
                }

                // Create file
                let mut file = match tokio::fs::File::create(&output_path).await {
                    Ok(f) => f,
                    Err(e) => {
                        let mut downloads = downloads.write().await;
                        if let Some(progress) = downloads.get_mut(&download_id) {
                            progress.status = DownloadStatus::Failed;
                            progress.error = Some(format!("Failed to create file: {}", e));
                        }
                        return;
                    }
                };

                // Stream response to file
                let mut stream = response.bytes_stream();
                let mut bytes_downloaded = 0u64;

                use futures::StreamExt;
                while let Some(item) = stream.next().await {
                    match item {
                        Ok(chunk) => {
                            if let Err(e) = tokio::io::AsyncWriteExt::write_all(&mut file, &chunk).await {
                                let mut downloads = downloads.write().await;
                                if let Some(progress) = downloads.get_mut(&download_id) {
                                    progress.status = DownloadStatus::Failed;
                                    progress.error = Some(format!("Write error: {}", e));
                                }
                                return;
                            }

                            bytes_downloaded += chunk.len() as u64;

                            // Update progress
                            {
                                let mut downloads = downloads.write().await;
                                if let Some(progress) = downloads.get_mut(&download_id) {
                                    progress.bytes_downloaded = bytes_downloaded;
                                    if let Some(total) = total_bytes {
                                        progress.progress_percent = (bytes_downloaded as f64 / total as f64) * 100.0;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            let mut downloads = downloads.write().await;
                            if let Some(progress) = downloads.get_mut(&download_id) {
                                progress.status = DownloadStatus::Failed;
                                progress.error = Some(format!("Download error: {}", e));
                            }
                            return;
                        }
                    }
                }

                // Mark as completed
                {
                    let mut downloads = downloads.write().await;
                    if let Some(progress) = downloads.get_mut(&download_id) {
                        progress.status = DownloadStatus::Completed;
                        progress.progress_percent = 100.0;
                    }
                }
            }
            Err(e) => {
                let mut downloads = downloads.write().await;
                if let Some(progress) = downloads.get_mut(&download_id) {
                    progress.status = DownloadStatus::Failed;
                    progress.error = Some(format!("Request failed: {}", e));
                }
            }
        }
    }

    /// Get download progress
    pub async fn get_progress(&self, download_id: &Uuid) -> Option<DownloadProgress> {
        let downloads = self.active_downloads.read().await;
        downloads.get(download_id).cloned()
    }

    /// Cancel a download
    pub async fn cancel_download(&self, download_id: &Uuid) -> Result<()> {
        let mut downloads = self.active_downloads.write().await;
        if let Some(progress) = downloads.get_mut(download_id) {
            progress.status = DownloadStatus::Cancelled;
        }
        Ok(())
    }

    /// List active downloads
    pub async fn list_downloads(&self) -> Vec<DownloadProgress> {
        let downloads = self.active_downloads.read().await;
        downloads.values().cloned().collect()
    }
}

impl Default for ModelDownloadService {
    fn default() -> Self {
        Self::new()
    }
}

