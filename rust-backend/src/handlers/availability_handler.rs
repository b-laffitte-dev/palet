//! Disponibilités des joueurs pour un match (Module 8 SportEasy, E3).
//!
//! - GET  /matchs/{id}/dispos  (public) : récap + liste des dispo
//! - POST /matchs/{id}/dispo  (connecté) : déclare sa dispo (ou celle d'un
//!   joueur de son club pour un responsable de club)

use actix_web::{web, HttpResponse, Responder};
use palet_shared::{ApiResponse, AvailabilitySummary, MatchAvailability, SetAvailabilityRequest};
use uuid::Uuid;

use crate::db::DatabaseError;
use crate::middleware::auth::extract_access_claims;
use crate::services::auth_service::AuthError;
use crate::services::stats_service::summarize_availabilities;
use crate::AppState;

/// Récap des disponibilités déclarées pour un match.
#[utoipa::path(
    get,
    path = "/matchs/{id}/dispos",
    tag = "Disponibilités",
    params(("id", description = "Match ID")),
    responses(
        (status = 200, description = "Disponibilités du match", body = AvailabilitySummary),
        (status = 404, description = "Match not found")
    )
)]
pub async fn get_dispos(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    if state
        .db
        .matches
        .get_match(*id)
        .await
        .ok()
        .flatten()
        .is_none()
    {
        return HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Match introuvable".to_string()),
            success: false,
        });
    }
    match state.db.disponibilites.get_for_match(*id).await {
        Ok(list) => {
            let (total, dispo, indispo, incertain) = summarize_availabilities(&list);
            HttpResponse::Ok().json(ApiResponse {
                data: AvailabilitySummary {
                    total,
                    disponibles: dispo,
                    indisponibles: indispo,
                    incertains: incertain,
                    disponibilites: list,
                },
                message: Some("Disponibilités récupérées".to_string()),
                success: true,
            })
        }
        Err(e) => {
            log::error!("Failed to get disponibilites: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Erreur serveur".to_string()),
                success: false,
            })
        }
    }
}

/// Déclare ou met à jour la disponibilité d'un joueur pour un match.
///
/// Autorisé si le joueur se déclare lui-même (même numéro de licence que le
/// compte connecté) ou si l'utilisateur gère le club du joueur (périmètre club).
#[utoipa::path(
    post,
    path = "/matchs/{id}/dispo",
    tag = "Disponibilités",
    params(("id", description = "Match ID")),
    request_body = SetAvailabilityRequest,
    responses(
        (status = 200, description = "Disponibilité enregistrée", body = MatchAvailability),
        (status = 400, description = "Requête invalide"),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Hors périmètre"),
        (status = 404, description = "Match ou joueur introuvable")
    )
)]
pub async fn set_dispo(
    req: actix_web::HttpRequest,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    body: web::Json<SetAvailabilityRequest>,
) -> impl Responder {
    let claims = match extract_access_claims(&req) {
        Ok(c) => c,
        Err(e) => return auth_error_response(e),
    };
    let body = body.into_inner();

    // Le match doit exister
    if state
        .db
        .matches
        .get_match(*id)
        .await
        .ok()
        .flatten()
        .is_none()
    {
        return HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Match introuvable".to_string()),
            success: false,
        });
    }
    // Le joueur doit exister
    let Some(player) = state
        .db
        .players
        .get_player(body.player_id)
        .await
        .ok()
        .flatten()
    else {
        return HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Joueur introuvable".to_string()),
            success: false,
        });
    };

    // Autorisation : soi-même (même licence) ou responsable du club du joueur
    let self_license = state
        .db
        .users
        .find_by_id(Uuid::parse_str(&claims.sub).unwrap_or_default())
        .await
        .ok()
        .flatten()
        .and_then(|u| u.license_number);
    let is_self = self_license.as_deref() == Some(player.license_number.as_str());
    let club_id = claims
        .club_id
        .as_deref()
        .and_then(|s| Uuid::parse_str(s).ok());
    let in_club_scope = club_id == Some(player.club_id);
    if !is_self && !in_club_scope {
        return HttpResponse::Forbidden().json(ApiResponse::<()> {
            data: (),
            message: Some(
                "Vous ne pouvez déclarer une dispo que pour vous-même ou un joueur de votre club"
                    .to_string(),
            ),
            success: false,
        });
    }

    match state
        .db
        .disponibilites
        .upsert(*id, body.player_id, body.status, body.commentaire)
        .await
    {
        Ok(dispo) => HttpResponse::Ok().json(ApiResponse {
            data: dispo,
            message: Some("Disponibilité enregistrée".to_string()),
            success: true,
        }),
        Err(DatabaseError::ValidationError(msg)) => {
            HttpResponse::BadRequest().json(ApiResponse::<()> {
                data: (),
                message: Some(msg),
                success: false,
            })
        }
        Err(e) => {
            log::error!("Failed to set dispo: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Erreur serveur".to_string()),
                success: false,
            })
        }
    }
}

fn auth_error_response(e: AuthError) -> HttpResponse {
    let status = match e {
        AuthError::Unauthorized(_) => actix_web::http::StatusCode::UNAUTHORIZED,
        AuthError::Forbidden(_) => actix_web::http::StatusCode::FORBIDDEN,
        _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
    };
    HttpResponse::build(status).json(ApiResponse::<()> {
        data: (),
        message: Some(e.to_string()),
        success: false,
    })
}
