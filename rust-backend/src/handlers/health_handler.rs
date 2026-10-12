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
        (status = 200, description = "API is healthy", body = HealthCheck),
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use palet_shared::HealthCheck;
    use std::sync::Arc;

    #[actix_rt::test]
    async fn test_health_check() {
        let db = Arc::new(crate::db::tests::create_test_db().await);
        let config = Arc::new(crate::config::AppConfig::default());
        let state = AppState { db, config };

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state))
                .route("/health", web::get().to(health_check)),
        )
        .await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: ApiResponse<HealthCheck> = test::read_body_json(resp).await;
        assert!(body.success);
        assert_eq!(body.data.statut, "ok");
    }
}
