//! Palet Vendéen Backend - Rust Implementation
//!
//! Main entry point for the Actix-web API server

use actix_cors::Cors;
use actix_web::{
    dev::ServiceRequest, error, get, http::header, middleware, web, App, HttpRequest, HttpResponse,
    HttpServer, Responder,
};
use chrono::Utc;
use dotenv::dotenv;
use log::{error, info};
use palet_shared::{
    ApiResponse, DatabaseStatus, HealthCheck, CacheStatus,
    Club, CreateClubRequest, Federation, Player, Tournament, Match, Championship,
};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Re-export handlers for modular imports
pub mod config;
pub mod db;
pub mod handlers;
pub mod models;
pub mod routes;

use config::AppConfig;
use db::Database;
use handlers::{club_handler, health_handler};

// App state shared across all routes
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub config: Arc<AppConfig>,
}

// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        health_handler::health_check,
        club_handler::get_clubs,
        club_handler::create_club,
        club_handler::get_club,
    ),
    components(),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Clubs", description = "Club management endpoints"),
        (name = "Players", description = "Player management endpoints"),
        (name = "Tournaments", description = "Tournament management endpoints"),
        (name = "Championnats", description = "Championnat management endpoints"),
    ),
    info(
        title = "Palet Vendéen API",
        description = "API pour la fédération des joueurs, clubs et fédérations de palet vendéen",
        version = "0.1.0",
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        ),
        contact(
            name = "Palet Vendéen Team",
            email = "contact@palet-vendeen.fr"
        ),
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development"),
        (url = "https://api.palet-vendeen.fr", description = "Production"),
    ),
)]
struct ApiDoc;

/// Initialize database connection pool
async fn init_db(config: &AppConfig) -> Result<PgPool, sqlx::Error> {
    let db_url = &config.database_url;
    
    info!("Connecting to database: {}", db_url);
    
    PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .acquire_timeout(std::time::Duration::from_secs(config.db_timeout_secs))
        .connect(db_url)
        .await
}

/// Initialize application configuration
fn init_config() -> Result<AppConfig, config::ConfigError> {
    dotenv().ok();
    
    let settings = config::Config::builder()
        .add_source(config::File::with_name("config/default"))
        .add_source(config::File::with_name("config/local").required(false))
        .add_source(config::Environment::with_prefix("PALET").separator("__"))
        .build()?;
    
    Ok(AppConfig::from_settings(settings)?)
}

/// CORS middleware configuration
fn configure_cors(config: &AppConfig) -> Cors {
    Cors::default()
        .allow_any_origin() // For development - restrict in production
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
            header::ORIGIN,
        ])
        .expose_headers(vec![header::CONTENT_LENGTH])
        .max_age(config.cors_max_age)
}

/// Logging middleware
async fn logger_middleware(
    req: ServiceRequest,
    srv: actix_web::dev::Service<impl actix_web::dev::ServiceFactory>,
) -> Result<actix_web::dev::ServiceResponse, actix_web::Error> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    
    info!("{} {}", method, path);
    
    let res = srv.call(req).await?;
    
    let status = res.response().status();
    info!("{} {} - {}", method, path, status);
    
    Ok(res)
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

/// Error handler for internal server errors
async fn internal_error_handler(err: error::InternalError<actix_web::error::BoxedError>) -> impl Responder {
    error!("Internal Server Error: {:?}", err);
    HttpResponse::InternalServerError().json(ApiResponse::<()> {
        data: (),
        message: Some("Internal server error".to_string()),
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
    let config = match init_config() {
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
    let server = HttpServer::new(move || {
        let cors = configure_cors(&app_config);

        App::new()
            // App state
            .app_data(web::Data::new(app_state.clone()))
            
            // Middleware
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::ErrorHandlers::new()
                .handler(error::InternalError::class_of(), internal_error_handler))
            
            // Swagger UI for API documentation
            .service(SwaggerUi::new("/api-docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
            
            // Health check endpoint
            .route("/health", get().to(health_handler::health_check))
            
            // Club endpoints
            .service(
                web::scope("/clubs")
                    .route("", get().to(club_handler::get_clubs))
                    .route("", web::post().to(club_handler::create_club))
                    .route("/{id}", get().to(club_handler::get_club)),
            )
            
            // Default service for unmatched routes
            .default_service(web::route().to(not_found_handler))
    });

    // Start server
    let port = app_config.port;
    let host = app_config.host.clone();
    
    info!("Starting server on {}:{}", host, port);
    
    server
        .bind((host, port))?
        .run()
        .await
        .map_err(|e| {
            error!("Server error: {:?}", e);
            e
        })?;

    Ok(())
}

// Re-export for use in tests
#[cfg(test)]
pub use db::tests::*;
