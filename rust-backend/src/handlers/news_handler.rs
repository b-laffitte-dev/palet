//! News endpoints — actualités : lecture publique des articles publiés,
//! écriture réservée aux administrateurs.

use crate::db::DatabaseError;
use crate::middleware::auth::AdminUser;
use crate::services::auth_service::AuthError;
use crate::AppState;
use actix_web::{web, HttpResponse};
use palet_shared::{ApiResponse, CreateNewsRequest, NewsArticle, UpdateNewsRequest};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct NewsQuery {
    pub category: Option<String>,
    pub q: Option<String>,
}

fn db_error_response(e: DatabaseError) -> HttpResponse {
    match e {
        DatabaseError::ValidationError(msg) => HttpResponse::BadRequest().json(ApiResponse::<()> {
            data: (),
            message: Some(msg),
            success: false,
        }),
        DatabaseError::NotFound(msg) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some(msg),
            success: false,
        }),
        other => {
            log::error!("news db error: {other:?}");
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Erreur interne".to_string()),
                success: false,
            })
        }
    }
}

/// Liste des actualités publiées (public)
#[utoipa::path(
    get,
    path = "/actualites",
    tag = "Actualites",
    params(NewsQuery),
    responses(
        (status = 200, description = "Actualités publiées", body = Vec<NewsArticle>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_actualites(
    state: web::Data<AppState>,
    query: web::Query<NewsQuery>,
) -> HttpResponse {
    match state
        .db
        .news
        .get_published_articles(query.category.clone(), query.q.clone())
        .await
    {
        Ok(articles) => HttpResponse::Ok().json(ApiResponse {
            data: articles,
            message: Some("Actualités récupérées".to_string()),
            success: true,
        }),
        Err(e) => db_error_response(e),
    }
}

/// Détail d'une actualité publiée (public ; 404 si brouillon)
#[utoipa::path(
    get,
    path = "/actualites/{id}",
    tag = "Actualites",
    params(("id", description = "Article ID")),
    responses(
        (status = 200, description = "Article", body = NewsArticle),
        (status = 404, description = "Article introuvable ou non publié"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_actualite(state: web::Data<AppState>, id: web::Path<Uuid>) -> HttpResponse {
    match state.db.news.get_article(*id).await {
        Ok(Some(article)) if article.published => HttpResponse::Ok().json(ApiResponse {
            data: article,
            message: Some("Article récupéré".to_string()),
            success: true,
        }),
        Ok(Some(_)) | Ok(None) => db_error_response(DatabaseError::NotFound(
            "Article introuvable ou non publié".to_string(),
        )),
        Err(e) => db_error_response(e),
    }
}

/// Publie un article (admin)
#[utoipa::path(
    post,
    path = "/actualites",
    tag = "Actualites",
    request_body = CreateNewsRequest,
    responses(
        (status = 201, description = "Article créé", body = NewsArticle),
        (status = 400, description = "Données invalides"),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Rôle administrateur requis"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_actualite(
    admin: AdminUser,
    state: web::Data<AppState>,
    request: web::Json<CreateNewsRequest>,
) -> Result<HttpResponse, AuthError> {
    let author_id = Uuid::parse_str(&admin.0.sub)
        .map_err(|_| AuthError::Unauthorized("Claim sub invalide".to_string()))?;
    match state
        .db
        .news
        .create_article(request.into_inner(), author_id)
        .await
    {
        Ok(article) => Ok(HttpResponse::Created().json(ApiResponse {
            data: article,
            message: Some("Article créé".to_string()),
            success: true,
        })),
        Err(e) => Ok(db_error_response(e)),
    }
}

/// Met à jour un article (admin)
#[utoipa::path(
    put,
    path = "/actualites/{id}",
    tag = "Actualites",
    params(("id", description = "Article ID")),
    request_body = UpdateNewsRequest,
    responses(
        (status = 200, description = "Article mis à jour", body = NewsArticle),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Rôle administrateur requis"),
        (status = 404, description = "Article introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_actualite(
    _admin: AdminUser,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    request: web::Json<UpdateNewsRequest>,
) -> HttpResponse {
    match state
        .db
        .news
        .update_article(*id, request.into_inner())
        .await
    {
        Ok(Some(article)) => HttpResponse::Ok().json(ApiResponse {
            data: article,
            message: Some("Article mis à jour".to_string()),
            success: true,
        }),
        Ok(None) => db_error_response(DatabaseError::NotFound("Article introuvable".to_string())),
        Err(e) => db_error_response(e),
    }
}

/// Supprime un article (admin)
#[utoipa::path(
    delete,
    path = "/actualites/{id}",
    tag = "Actualites",
    params(("id", description = "Article ID")),
    responses(
        (status = 200, description = "Article supprimé", body = bool),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Rôle administrateur requis"),
        (status = 404, description = "Article introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_actualite(
    _admin: AdminUser,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> HttpResponse {
    match state.db.news.delete_article(*id).await {
        Ok(true) => HttpResponse::Ok().json(ApiResponse {
            data: true,
            message: Some("Article supprimé".to_string()),
            success: true,
        }),
        Ok(false) => db_error_response(DatabaseError::NotFound("Article introuvable".to_string())),
        Err(e) => db_error_response(e),
    }
}
