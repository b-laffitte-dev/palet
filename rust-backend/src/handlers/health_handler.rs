//! Health check endpoint handler

use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use palet_shared::{ApiResponse, CacheStatus, DatabaseStatus, HealthCheck};

use crate::AppState;

/// Health check handler
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "API is healthy", body = palet_shared::HealthCheck),
        (status = 503, description = "Service unavailable")
    )
)]
pub async fn health_check(state: web::Data<AppState>) -> impl Responder {
    let database_status = if state.db.check_connection().await {
        DatabaseStatus::Connected
    } else {
        DatabaseStatus::Disconnected
    };

    let health = HealthCheck {
        statut: "ok".to_string(),
        version: "0.1.0".to_string(),
        timestamp: Utc::now(),
        database: database_status,
        cache: CacheStatus::Connected,
    };

    HttpResponse::Ok().json(ApiResponse {
        data: health,
        message: Some("API is healthy".to_string()),
        success: true,
    })
}
