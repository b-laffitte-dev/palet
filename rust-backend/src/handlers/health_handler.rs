//! Health check endpoint handlers for Palet Venden
//!
//! Handles health check and status operations

use actix_web::{web, HttpResponse, Responder};
use palet_shared::{ApiResponse, DatabaseStatus, HealthCheck};

use crate::AppState;

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Health check status", body = HealthCheck),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn health_check(state: web::Data<AppState>) -> impl Responder {
    let database_status = match state.db.check_connection().await {
        Ok(true) => DatabaseStatus::Connected,
        Ok(false) => DatabaseStatus::Disconnected,
        Err(_) => DatabaseStatus::Disconnected,
    };

    let health = HealthCheck {
        statut: "OK".to_string(),
        version: "0.1.0".to_string(),
        timestamp: chrono::Utc::now(),
        database: database_status,
        cache: palet_shared::CacheStatus::Connected,
    };

    HttpResponse::Ok().json(ApiResponse {
        data: health,
        message: Some("Health check successful".to_string()),
        success: true,
    })
}
