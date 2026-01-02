use axum::{
    extract::{Path, State},
    response::sse::{Event, Sse},
};
use futures::stream::Stream;
use std::collections::HashMap;
use std::convert::Infallible;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::AppState;

pub struct StreamingService {
    channels: Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>,
}

impl StreamingService {
    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub async fn create_channel(&self, channel_id: String) -> broadcast::Receiver<String> {
        let (sender, receiver) = broadcast::channel(100);
        let mut channels = self.channels.lock().await;
        channels.insert(channel_id, sender);
        receiver
    }
    
    pub async fn broadcast(&self, channel_id: &str, message: String) -> Result<(), String> {
        let channels = self.channels.lock().await;
        if let Some(sender) = channels.get(channel_id) {
            sender.send(message).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
    
    pub async fn cleanup_channel(&self, channel_id: &str) {
        let mut channels = self.channels.lock().await;
        channels.remove(channel_id);
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct StreamEvent {
    pub event_type: String,
    pub data: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub async fn generate_ai_code_stream(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(project_id): Path<String>,
) -> Sse<Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>> {
    let stream_id = Uuid::new_v4().to_string();
    let streaming_service = StreamingService::new();
    let receiver = streaming_service.create_channel(stream_id.clone()).await;
    
    // Spawn task to generate code and stream updates
    tokio::spawn(async move {
        let steps = vec![
            "Analyzing requirements...",
            "Generating component structure...",
            "Creating styling...",
            "Adding interactions...",
            "Optimizing performance...",
            "Finalizing code...",
        ];
        
        for (i, step) in steps.iter().enumerate() {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            
            let event = StreamEvent {
                event_type: "progress".to_string(),
                data: serde_json::json!({
                    "step": i + 1,
                    "total": steps.len(),
                    "message": step,
                    "percentage": ((i + 1) * 100) / steps.len()
                }),
                timestamp: chrono::Utc::now(),
            };
            
            let _ = streaming_service.broadcast(&stream_id, serde_json::to_string(&event).unwrap()).await;
        }
        
        // Send completion event
        let completion_event = StreamEvent {
            event_type: "complete".to_string(),
            data: serde_json::json!({
                "code": "// Generated code would go here",
                "components": ["Button", "Form", "Modal"],
                "dependencies": ["dioxus", "tailwind"]
            }),
            timestamp: chrono::Utc::now(),
        };
        
        let _ = streaming_service.broadcast(&stream_id, serde_json::to_string(&completion_event).unwrap()).await;
        
        // Cleanup after stream ends
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        streaming_service.cleanup_channel(&stream_id).await;
    });
    
    let stream = receiver.map(|message| {
        Ok(Event::default().data(message))
    });
    
    Sse::new(Box::pin(stream))
}

pub async fn apply_ai_code_stream(
    State(state): State<Arc<Mutex<AppState>>>,
    Path((project_id, component_id)): Path<(String, String)>,
) -> Sse<Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>> {
    let stream_id = Uuid::new_v4().to_string();
    let streaming_service = StreamingService::new();
    let receiver = streaming_service.create_channel(stream_id.clone()).await;
    
    tokio::spawn(async move {
        let changes = vec![
            "Analyzing current component...",
            "Generating diff...",
            "Applying changes to properties...",
            "Updating styles...",
            "Adding event handlers...",
            "Validating component...",
        ];
        
        for (i, change) in changes.iter().enumerate() {
            tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
            
            let event = StreamEvent {
                event_type: "change_progress".to_string(),
                data: serde_json::json!({
                    "step": i + 1,
                    "total": changes.len(),
                    "message": change,
                    "component_id": component_id.clone(),
                    "percentage": ((i + 1) * 100) / changes.len()
                }),
                timestamp: chrono::Utc::now(),
            };
            
            let _ = streaming_service.broadcast(&stream_id, serde_json::to_string(&event).unwrap()).await;
        }
        
        let completion_event = StreamEvent {
            event_type: "change_complete".to_string(),
            data: serde_json::json!({
                "component_id": component_id.clone(),
                "success": true,
                "changes_applied": 5,
                "preview_url": format!("/preview/{}/{}", project_id, component_id)
            }),
            timestamp: chrono::Utc::now(),
        };
        
        let _ = streaming_service.broadcast(&stream_id, serde_json::to_string(&completion_event).unwrap()).await;
        
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        streaming_service.cleanup_channel(&stream_id).await;
    });
    
    let stream = receiver.map(|message| {
        Ok(Event::default().data(message))
    });
    
    Sse::new(Box::pin(stream))
}

pub async fn monitor_vite_logs(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(sandbox_id): Path<String>,
) -> Sse<Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>> {
    let stream_id = Uuid::new_v4().to_string();
    let streaming_service = StreamingService::new();
    let receiver = streaming_service.create_channel(stream_id.clone()).await;
    
    tokio::spawn(async move {
        let log_messages = vec![
            "[vite] Starting development server...",
            "[vite] Server running at http://localhost:5173",
            "[vite] Hot module replacement enabled",
            "[vite] Build completed in 1.2s",
            "[vite] 3 modules transformed",
        ];
        
        for message in log_messages {
            tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
            
            let event = StreamEvent {
                event_type: "vite_log".to_string(),
                data: serde_json::json!({
                    "level": "info",
                    "message": message,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }),
                timestamp: chrono::Utc::now(),
            };
            
            let _ = streaming_service.broadcast(&stream_id, serde_json::to_string(&event).unwrap()).await;
        }
        
        // Simulate error
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        let error_event = StreamEvent {
            event_type: "vite_error".to_string(),
            data: serde_json::json!({
                "level": "error",
                "message": "[vite] Build failed: Module not found",
                "file": "/src/components/Button.tsx",
                "line": 15,
                "column": 8,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }),
            timestamp: chrono::Utc::now(),
        };
        
        let _ = streaming_service.broadcast(&stream_id, serde_json::to_string(&error_event).unwrap()).await;
        
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        streaming_service.cleanup_channel(&stream_id).await;
    });
    
    let stream = receiver.map(|message| {
        Ok(Event::default().data(message))
    });
    
    Sse::new(Box::pin(stream))
}