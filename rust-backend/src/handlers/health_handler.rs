//! Health check endpoint handler

use actix_web::{get, HttpResponse, Responder};
use chrono::Utc;
use palet_shared::{ApiResponse, CacheStatus, DatabaseStatus, HealthCheck};
use std::sync::Arc;

use crate::db::Database;

/// Health check handler
///
/// Returns the current health status of the API and its dependencies
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    summary = "Health check endpoint",
    description = "Returns the current health status of the API, database, and cache",
    responses(
        (status = 200, description = "API is healthy", body = HealthCheck),
        (status = 503, description = "Service unavailable")
    )
)]
#[get("/health")]
pub async fn health_check(db: Arc<Database>) -> impl Responder {
    let database_status = if db.check_connection().await {
        DatabaseStatus::Connected
    } else {
        DatabaseStatus::Disconnected
    };

    let health = HealthCheck {
        statut: "ok".to_string(),
        version: "0.1.0".to_string(),
        timestamp: Utc::now(),
        database: database_status,
        cache: CacheStatus::Connected, // For now, always connected
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

    #[actix_rt::test]
    async fn test_health_check() {
        let db = Arc::new(crate::db::tests::create_test_db().await);

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(db.clone()))
                .service(health_check),
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
