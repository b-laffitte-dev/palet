use axum::{routing::get, Json, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod db;

#[derive(utoipa::OpenApi)]
#[openapi(info(title = "API Palet Vendéen", version = "0.1.0"), paths(health))]
struct ApiDoc;

#[derive(serde::Serialize, utoipa::ToSchema)]
struct Sante {
    statut: String,
}

#[utoipa::path(get, path = "/health", responses((status = 200, body = Sante)))]
async fn health() -> Json<Sante> {
    Json(Sante {
        statut: "ok".into(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let pool = db::connect().await?;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("API en écoute sur 0.0.0.0:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
