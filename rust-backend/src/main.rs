//! Palet Vendeen Backend - Rust Implementation
//!
//! Main entry point for the Actix-web API server

use actix_cors::Cors;
use actix_web::{
    http::header, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use log::{error, info};
use palet_shared::ApiResponse;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Import modules
mod config;
mod db;
mod handlers;

use config::load_config;
use db::Database;
use handlers::{championship_handler, club_handler, health_handler, match_handler};

// App state shared across all routes

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub config: Arc<config::AppConfig>,
}

// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        health_handler::health_check,
        club_handler::get_clubs,
        club_handler::create_club,
        club_handler::get_club,
        match_handler::get_matches,
        match_handler::get_match,
        match_handler::get_matches_by_competition,
        match_handler::create_match,
        match_handler::update_match,
        match_handler::delete_match,
        match_handler::start_match,
        match_handler::complete_match,
        match_handler::add_manche,
        championship_handler::get_championships,
        championship_handler::get_championship,
        championship_handler::get_championships_by_league,
        championship_handler::get_championships_by_season,
        championship_handler::create_championship,
        championship_handler::update_championship,
        championship_handler::delete_championship,
        championship_handler::start_championship,
        championship_handler::next_journee,
        championship_handler::get_championship_classification,
        championship_handler::update_championship_classification,
    ),
    components(),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Clubs", description = "Club management endpoints"),
        (name = "Matchs", description = "Match management endpoints"),
        (name = "Championnats", description = "Championship management endpoints"),
    ),
    info(
        title = "Palet Vendéen API",
        description = "API pour la fédération des joueurs, clubs et fédérations de palet vendéen",
        version = "0.1.0",
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development"),
        (url = "https://api.palet-vendeen.fr", description = "Production"),
    ),
)]
struct ApiDoc;

/// Initialize database connection pool
async fn init_db(config: &config::AppConfig) -> Result<PgPool, sqlx::Error> {
    let db_url = &config.database_url;

    info!("Connecting to database: {}", db_url);

    PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .acquire_timeout(std::time::Duration::from_secs(config.db_timeout_secs))
        .connect(db_url)
        .await
}

/// CORS middleware configuration
fn configure_cors(config: &config::AppConfig) -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
            header::ORIGIN,
        ])
        .expose_headers(vec![header::CONTENT_LENGTH])
        .max_age(config.cors_max_age as usize)
}

/// Error handler for 404 Not Found
async fn not_found_handler(req: HttpRequest) -> impl Responder {
    error!("Not Found: {}", req.uri());
    HttpResponse::NotFound().json(ApiResponse::<()> {
        data: (),
        message: Some(format!("Route {} not found", req.uri().path())),
        success: false,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info,actix_web=info"),
    )
    .init();

    info!("Starting Palet Vendéen Backend...");

    // Load configuration
    let config = match load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load configuration: {:?}", e);
            std::process::exit(1);
        }
    };

    info!("Configuration loaded successfully");

    // Initialize database
    let pool = match init_db(&config).await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to connect to database: {:?}", e);
            std::process::exit(1);
        }
    };

    info!("Database connection established");

    // Create database wrapper
    let database = Arc::new(Database::new(pool));
    let app_config = Arc::new(config);

    // Build application state
    let app_state = AppState {
        db: database.clone(),
        config: app_config.clone(),
    };

    info!("App state initialized");

    // Build HTTP server
    let app_config_clone = app_config.clone();
    let server = HttpServer::new(move || {
        App::new()
            // App state
            .app_data(web::Data::new(app_state.clone()))
            // Middleware
            .wrap(configure_cors(&app_config_clone))
            .wrap(middleware::Logger::default())
            // Swagger UI for API documentation
            .service(SwaggerUi::new("/api-docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
            // Health check endpoint
            .route("/health", web::get().to(health_handler::health_check))
            // Club endpoints
            .service(
                web::scope("/clubs")
                    .route("", web::get().to(club_handler::get_clubs))
                    .route("", web::post().to(club_handler::create_club))
                    .route("/{id}", web::get().to(club_handler::get_club)),
            )
            // Default service for unmatched routes
            .default_service(web::route().to(not_found_handler))
    });

    // Start server
    let port = app_config.port;
    let host = app_config.host.clone();

    info!("Starting server on {}:{}", host, port);

    server.bind((host, port))?.run().await.map_err(|e| {
        error!("Server error: {:?}", e);
        e
    })?;

    Ok(())
}
