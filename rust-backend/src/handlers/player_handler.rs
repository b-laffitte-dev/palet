//! Player endpoints — liste/filtres publics, écriture réservée aux
//! responsables de club (périmètre club) et administrateurs.

use crate::db::DatabaseError;
use crate::middleware::auth::{check_club_scope, ClubManagerUser};
use crate::services::auth_service::AuthError;
use crate::AppState;
use actix_web::{web, HttpResponse};
use palet_shared::{ApiResponse, CreatePlayerRequest, LicenseStatus, Player, PlayerStatus};
use serde::Deserialize;
use uuid::Uuid;

/// Filtres de recherche pour GET /joueurs
#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct PlayerQuery {
    pub club_id: Option<Uuid>,
    pub category: Option<String>,
    pub status: Option<PlayerStatus>,
    pub license_status: Option<LicenseStatus>,
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
            log::error!("player db error: {other:?}");
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Erreur interne".to_string()),
                success: false,
            })
        }
    }
}

fn apply_filters(mut players: Vec<Player>, query: &PlayerQuery) -> Vec<Player> {
    if let Some(club_id) = query.club_id {
        players.retain(|p| p.club_id == club_id);
    }
    if let Some(category) = &query.category {
        players.retain(|p| &p.category == category);
    }
    if let Some(status) = &query.status {
        players.retain(|p| &p.status == status);
    }
    if let Some(license_status) = &query.license_status {
        players.retain(|p| &p.license_status == license_status);
    }
    if let Some(q) = &query.q {
        let needle = q.to_lowercase();
        players.retain(|p| {
            p.full_name.to_lowercase().contains(&needle)
                || p.email.to_lowercase().contains(&needle)
                || p.alias
                    .as_deref()
                    .map(|a| a.to_lowercase().contains(&needle))
                    .unwrap_or(false)
        });
    }
    players
}

/// Liste des joueurs, avec filtres optionnels (public)
#[utoipa::path(
    get,
    path = "/joueurs",
    tag = "Joueurs",
    params(PlayerQuery),
    responses(
        (status = 200, description = "Liste des joueurs", body = Vec<Player>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_players(
    state: web::Data<AppState>,
    query: web::Query<PlayerQuery>,
) -> HttpResponse {
    match state.db.players.get_players().await {
        Ok(players) => HttpResponse::Ok().json(ApiResponse {
            data: apply_filters(players, &query),
            message: Some("Joueurs récupérés".to_string()),
            success: true,
        }),
        Err(e) => db_error_response(e),
    }
}

/// Fiche d'un joueur (public)
#[utoipa::path(
    get,
    path = "/joueurs/{id}",
    tag = "Joueurs",
    params(("id", description = "Player ID")),
    responses(
        (status = 200, description = "Joueur", body = Player),
        (status = 404, description = "Joueur introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_player(state: web::Data<AppState>, id: web::Path<Uuid>) -> HttpResponse {
    match state.db.players.get_player(*id).await {
        Ok(Some(player)) => HttpResponse::Ok().json(ApiResponse {
            data: player,
            message: Some("Joueur récupéré".to_string()),
            success: true,
        }),
        Ok(None) => db_error_response(DatabaseError::NotFound("Joueur introuvable".to_string())),
        Err(e) => db_error_response(e),
    }
}

/// Crée un joueur dans un club (responsable du club ou admin)
#[utoipa::path(
    post,
    path = "/joueurs",
    tag = "Joueurs",
    request_body = CreatePlayerRequest,
    responses(
        (status = 201, description = "Joueur créé", body = Player),
        (status = 400, description = "Données invalides"),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Hors périmètre club"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_player(
    manager: ClubManagerUser,
    state: web::Data<AppState>,
    request: web::Json<CreatePlayerRequest>,
) -> Result<HttpResponse, AuthError> {
    let request = request.into_inner();
    check_club_scope(&manager.0, request.club_id)?;
    match state.db.players.create_player(request).await {
        Ok(player) => Ok(HttpResponse::Created().json(ApiResponse {
            data: player,
            message: Some("Joueur créé".to_string()),
            success: true,
        })),
        Err(e) => Ok(db_error_response(e)),
    }
}

/// Met à jour un joueur (responsable de son club ou admin)
#[utoipa::path(
    put,
    path = "/joueurs/{id}",
    tag = "Joueurs",
    params(("id", description = "Player ID")),
    request_body = CreatePlayerRequest,
    responses(
        (status = 200, description = "Joueur mis à jour", body = Player),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Hors périmètre club"),
        (status = 404, description = "Joueur introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_player(
    manager: ClubManagerUser,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    request: web::Json<CreatePlayerRequest>,
) -> Result<HttpResponse, AuthError> {
    let existing = state
        .db
        .players
        .get_player(*id)
        .await
        .map_err(|e| AuthError::Internal(e.to_string()))?;
    let Some(existing) = existing else {
        return Ok(db_error_response(DatabaseError::NotFound(
            "Joueur introuvable".to_string(),
        )));
    };
    check_club_scope(&manager.0, existing.club_id)?;
    match state
        .db
        .players
        .update_player(*id, request.into_inner())
        .await
    {
        Ok(Some(player)) => Ok(HttpResponse::Ok().json(ApiResponse {
            data: player,
            message: Some("Joueur mis à jour".to_string()),
            success: true,
        })),
        Ok(None) => Ok(db_error_response(DatabaseError::NotFound(
            "Joueur introuvable".to_string(),
        ))),
        Err(e) => Ok(db_error_response(e)),
    }
}

/// Supprime un joueur (responsable de son club ou admin)
#[utoipa::path(
    delete,
    path = "/joueurs/{id}",
    tag = "Joueurs",
    params(("id", description = "Player ID")),
    responses(
        (status = 200, description = "Joueur supprimé", body = bool),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Hors périmètre club"),
        (status = 404, description = "Joueur introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_player(
    manager: ClubManagerUser,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, AuthError> {
    let existing = state
        .db
        .players
        .get_player(*id)
        .await
        .map_err(|e| AuthError::Internal(e.to_string()))?;
    let Some(existing) = existing else {
        return Ok(db_error_response(DatabaseError::NotFound(
            "Joueur introuvable".to_string(),
        )));
    };
    check_club_scope(&manager.0, existing.club_id)?;
    match state.db.players.delete_player(*id).await {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse {
            data: true,
            message: Some("Joueur supprimé".to_string()),
            success: true,
        })),
        Ok(false) => Ok(db_error_response(DatabaseError::NotFound(
            "Joueur introuvable".to_string(),
        ))),
        Err(e) => Ok(db_error_response(e)),
    }
}
