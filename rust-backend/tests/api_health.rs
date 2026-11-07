//! Tests d'intégration HTTP du backend (skill rust-tdd : dossier `tests/`,
//! `actix_web::test`). Le pool est "lazy" : aucune base réelle n'est requise,
//! `check_connection` renvoie alors `Ok(false)` (statut Disconnected).

use actix_web::{http::StatusCode, test, web, App, HttpRequest, HttpResponse, Responder};
use palet_backend::{config::AppConfig, db::Database, handlers::health_handler, AppState};
use palet_shared::ApiResponse;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

fn test_state() -> web::Data<AppState> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_millis(500))
        .connect_lazy("postgres://palet:palet@localhost:5432/palet")
        .expect("création du pool lazy");
    web::Data::new(AppState {
        db: Arc::new(Database::new(pool)),
        config: Arc::new(AppConfig::default()),
    })
}

async fn not_found(req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().json(ApiResponse::<()> {
        data: (),
        message: Some(format!("Route {} not found", req.uri().path())),
        success: false,
    })
}

#[actix_web::test]
async fn health_check_renvoie_200_et_success() {
    let app = test::init_service(
        App::new()
            .app_data(test_state())
            .route("/health", web::get().to(health_handler::health_check)),
    )
    .await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], serde_json::json!(true));
    assert_eq!(body["data"]["statut"], serde_json::json!("OK"));
    // Sans base de données, le statut est Disconnected mais l'endpoint répond 200.
    assert_eq!(body["data"]["database"], serde_json::json!("disconnected"));
}

#[actix_web::test]
async fn route_inconnue_renvoie_404() {
    let app = test::init_service(
        App::new()
            .app_data(test_state())
            .route("/health", web::get().to(health_handler::health_check))
            .default_service(web::route().to(not_found)),
    )
    .await;

    let req = test::TestRequest::get().uri("/nimporte-quoi").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], serde_json::json!(false));
}
