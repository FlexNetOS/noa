use axum::{extract::Path, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

use crate::api::server::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Promotion {
    pub id: String,
    pub status: String,
    pub target_plane: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromotionList {
    pub promotions: Vec<Promotion>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/promotions", get(list_promotions))
        .route("/promotions/:id/status", get(promotion_status))
}

async fn list_promotions() -> Json<PromotionList> {
    Json(PromotionList {
        promotions: vec![Promotion {
            id: "promo-1".into(),
            status: "pending".into(),
            target_plane: "deployed".into(),
        }],
    })
}

async fn promotion_status(Path(id): Path<String>) -> Json<Promotion> {
    Json(Promotion {
        id: id.clone(),
        status: "pending".into(),
        target_plane: "deployed".into(),
    })
}
