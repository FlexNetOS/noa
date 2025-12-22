use axum::{routing::{get, post}, Json, Router, extract::Path};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct PlaneInfo {
    pub name: String,
    pub status: String,
    pub health: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaneList {
    pub planes: Vec<PlaneInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwitchRequest {
    pub target: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwitchResponse {
    pub message: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/planes", get(list_planes))
        .route("/planes/:name/health", get(plane_health))
        .route("/planes/switch", post(switch_plane))
}

async fn list_planes() -> Json<PlaneList> {
    Json(PlaneList {
        planes: vec![
            PlaneInfo {
                name: "sandbox".into(),
                status: "active".into(),
                health: "healthy".into(),
            },
            PlaneInfo {
                name: "deployed".into(),
                status: "standby".into(),
                health: "healthy".into(),
            },
            PlaneInfo {
                name: "coordinator".into(),
                status: "standby".into(),
                health: "healthy".into(),
            },
        ],
    })
}

async fn plane_health(Path(name): Path<String>) -> Json<PlaneInfo> {
    Json(PlaneInfo {
        name: name.clone(),
        status: "active".into(),
        health: format!("{} plane healthy", name),
    })
}

async fn switch_plane(Json(body): Json<SwitchRequest>) -> Json<SwitchResponse> {
    Json(SwitchResponse {
        message: format!("switch triggered to {}", body.target),
    })
}
