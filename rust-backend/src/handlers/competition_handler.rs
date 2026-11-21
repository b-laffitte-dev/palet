//! Competition endpoints — compétitions "événement" (tournois/coupe/open),
//! inscriptions d'équipes et génération du calendrier.

use crate::db::DatabaseError;
use crate::middleware::auth::{
    check_club_scope, extract_access_claims, is_admin, AdminUser, ClubManagerUser,
};
use crate::services::auth_service::AuthError;
use crate::services::competition_service::generate_matches;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use palet_shared::{
    ApiResponse, CompetitionEvent, CompetitionStatus, CompetitionType, CreateCompetitionRequest,
    Match, RegisterTeamRequest, Registration, UpdateCompetitionRequest,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct CompetitionQuery {
    #[serde(rename = "type")]
    pub competition_type: Option<CompetitionType>,
    pub status: Option<CompetitionStatus>,
    pub season: Option<String>,
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
            log::error!("competition db error: {other:?}");
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Erreur interne".to_string()),
                success: false,
            })
        }
    }
}

/// Liste des compétitions (public)
#[utoipa::path(
    get,
    path = "/competitions",
    tag = "Competitions",
    params(CompetitionQuery),
    responses(
        (status = 200, description = "Liste des compétitions", body = Vec<CompetitionEvent>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_competitions(
    state: web::Data<AppState>,
    query: web::Query<CompetitionQuery>,
) -> HttpResponse {
    match state
        .db
        .competitions
        .get_competitions(
            query.competition_type.clone(),
            query.status.clone(),
            query.season.clone(),
        )
        .await
    {
        Ok(events) => HttpResponse::Ok().json(ApiResponse {
            data: events,
            message: Some("Compétitions récupérées".to_string()),
            success: true,
        }),
        Err(e) => db_error_response(e),
    }
}

/// Détail d'une compétition (public)
#[utoipa::path(
    get,
    path = "/competitions/{id}",
    tag = "Competitions",
    params(("id", description = "Competition ID")),
    responses(
        (status = 200, description = "Compétition", body = CompetitionEvent),
        (status = 404, description = "Compétition introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_competition(state: web::Data<AppState>, id: web::Path<Uuid>) -> HttpResponse {
    match state.db.competitions.get_competition(*id).await {
        Ok(Some(event)) => HttpResponse::Ok().json(ApiResponse {
            data: event,
            message: Some("Compétition récupérée".to_string()),
            success: true,
        }),
        Ok(None) => db_error_response(DatabaseError::NotFound(
            "Compétition introuvable".to_string(),
        )),
        Err(e) => db_error_response(e),
    }
}

/// Crée une compétition (admin)
#[utoipa::path(
    post,
    path = "/competitions",
    tag = "Competitions",
    request_body = CreateCompetitionRequest,
    responses(
        (status = 201, description = "Compétition créée", body = CompetitionEvent),
        (status = 400, description = "Données invalides"),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Rôle administrateur requis"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_competition(
    _admin: AdminUser,
    state: web::Data<AppState>,
    request: web::Json<CreateCompetitionRequest>,
) -> HttpResponse {
    match state
        .db
        .competitions
        .create_competition(request.into_inner())
        .await
    {
        Ok(event) => HttpResponse::Created().json(ApiResponse {
            data: event,
            message: Some("Compétition créée".to_string()),
            success: true,
        }),
        Err(e) => db_error_response(e),
    }
}

/// Met à jour une compétition (admin)
#[utoipa::path(
    put,
    path = "/competitions/{id}",
    tag = "Competitions",
    params(("id", description = "Competition ID")),
    request_body = UpdateCompetitionRequest,
    responses(
        (status = 200, description = "Compétition mise à jour", body = CompetitionEvent),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Rôle administrateur requis"),
        (status = 404, description = "Compétition introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_competition(
    _admin: AdminUser,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    request: web::Json<UpdateCompetitionRequest>,
) -> HttpResponse {
    match state
        .db
        .competitions
        .update_competition(*id, request.into_inner())
        .await
    {
        Ok(Some(event)) => HttpResponse::Ok().json(ApiResponse {
            data: event,
            message: Some("Compétition mise à jour".to_string()),
            success: true,
        }),
        Ok(None) => db_error_response(DatabaseError::NotFound(
            "Compétition introuvable".to_string(),
        )),
        Err(e) => db_error_response(e),
    }
}

/// Supprime une compétition (admin)
#[utoipa::path(
    delete,
    path = "/competitions/{id}",
    tag = "Competitions",
    params(("id", description = "Competition ID")),
    responses(
        (status = 200, description = "Compétition supprimée", body = bool),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Rôle administrateur requis"),
        (status = 404, description = "Compétition introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_competition(
    _admin: AdminUser,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> HttpResponse {
    match state.db.competitions.delete_competition(*id).await {
        Ok(true) => HttpResponse::Ok().json(ApiResponse {
            data: true,
            message: Some("Compétition supprimée".to_string()),
            success: true,
        }),
        Ok(false) => db_error_response(DatabaseError::NotFound(
            "Compétition introuvable".to_string(),
        )),
        Err(e) => db_error_response(e),
    }
}

/// Inscrit une équipe à une compétition (responsable du club de l'équipe ou admin)
#[utoipa::path(
    post,
    path = "/competitions/{id}/inscrire",
    tag = "Competitions",
    params(("id", description = "Competition ID")),
    request_body = RegisterTeamRequest,
    responses(
        (status = 201, description = "Équipe inscrite (en attente)", body = Registration),
        (status = 400, description = "Déjà inscrite"),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Hors périmètre club"),
        (status = 404, description = "Compétition ou équipe introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn register_team(
    manager: ClubManagerUser,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    request: web::Json<RegisterTeamRequest>,
) -> Result<HttpResponse, AuthError> {
    let competition_id = *id;
    let team_id = request.team_id;
    let Some(team) = state
        .db
        .teams
        .get_team(team_id)
        .await
        .map_err(|e| AuthError::Internal(e.to_string()))?
    else {
        return Ok(db_error_response(DatabaseError::NotFound(
            "Équipe introuvable".to_string(),
        )));
    };
    check_club_scope(&manager.0, team.club_id)?;
    let user_id = Uuid::parse_str(&manager.0.sub)
        .map_err(|_| AuthError::Unauthorized("Claim sub invalide".to_string()))?;
    match state
        .db
        .competitions
        .register_team(competition_id, team_id, user_id)
        .await
    {
        Ok(registration) => Ok(HttpResponse::Created().json(ApiResponse {
            data: registration,
            message: Some("Équipe inscrite, en attente de confirmation".to_string()),
            success: true,
        })),
        Err(e) => Ok(db_error_response(e)),
    }
}

/// Liste des inscriptions d'une compétition (public si compétition publique,
/// sinon admin uniquement)
#[utoipa::path(
    get,
    path = "/competitions/{id}/inscriptions",
    tag = "Competitions",
    params(("id", description = "Competition ID")),
    responses(
        (status = 200, description = "Inscriptions", body = Vec<Registration>),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Compétition non publique"),
        (status = 404, description = "Compétition introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_inscriptions(
    req: HttpRequest,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, AuthError> {
    let Some(event) = state
        .db
        .competitions
        .get_competition(*id)
        .await
        .map_err(|e| AuthError::Internal(e.to_string()))?
    else {
        return Ok(db_error_response(DatabaseError::NotFound(
            "Compétition introuvable".to_string(),
        )));
    };
    if !event.is_public {
        let claims = extract_access_claims(&req)?;
        if !is_admin(&claims.role) {
            return Err(AuthError::Forbidden(
                "Inscriptions d'une compétition non publique : administrateur requis".to_string(),
            ));
        }
    }
    match state.db.competitions.get_registrations(*id).await {
        Ok(registrations) => Ok(HttpResponse::Ok().json(ApiResponse {
            data: registrations,
            message: Some("Inscriptions récupérées".to_string()),
            success: true,
        })),
        Err(e) => Ok(db_error_response(e)),
    }
}

/// Confirme une inscription (admin)
#[utoipa::path(
    post,
    path = "/competitions/{id}/inscriptions/{team_id}/confirmer",
    tag = "Competitions",
    params(("id", description = "Competition ID"), ("team_id", description = "Team ID")),
    responses(
        (status = 200, description = "Inscription confirmée", body = Registration),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Rôle administrateur requis"),
        (status = 404, description = "Inscription introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn confirm_inscription(
    admin: AdminUser,
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AuthError> {
    let (competition_id, team_id) = path.into_inner();
    let admin_id = Uuid::parse_str(&admin.0.sub)
        .map_err(|_| AuthError::Unauthorized("Claim sub invalide".to_string()))?;
    match state
        .db
        .competitions
        .confirm_registration(competition_id, team_id, admin_id)
        .await
    {
        Ok(Some(registration)) => {
            // Notification au demandeur : inscription confirmée
            let _ = state
                .db
                .notifications
                .create(
                    registration.registered_by,
                    "inscription_confirmee",
                    "Inscription confirmée",
                    "Votre équipe est confirmée pour cette compétition.",
                    Some(&format!("/tournois/{}", registration.competition_id)),
                )
                .await;
            Ok(HttpResponse::Ok().json(ApiResponse {
                data: registration,
                message: Some("Inscription confirmée".to_string()),
                success: true,
            }))
        }
        Ok(None) => Ok(db_error_response(DatabaseError::NotFound(
            "Inscription introuvable".to_string(),
        ))),
        Err(e) => Ok(db_error_response(e)),
    }
}

/// Génère le calendrier round-robin des équipes confirmées (admin)
#[utoipa::path(
    post,
    path = "/competitions/{id}/generer",
    tag = "Competitions",
    params(("id", description = "Competition ID")),
    responses(
        (status = 200, description = "Matchs générés", body = Vec<Match>),
        (status = 400, description = "Pas assez d'équipes confirmées"),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Rôle administrateur requis"),
        (status = 404, description = "Compétition introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn generer_calendrier(
    _admin: AdminUser,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, AuthError> {
    let matches = generate_matches(&state.db, *id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        data: matches,
        message: Some("Calendrier généré".to_string()),
        success: true,
    }))
}
