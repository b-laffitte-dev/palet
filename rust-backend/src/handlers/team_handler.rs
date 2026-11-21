//! Team endpoints — liste/lecture publiques, gestion des équipes et effectifs
//! réservée aux responsables de club (périmètre club) et administrateurs.

use crate::db::DatabaseError;
use crate::middleware::auth::{check_club_scope, ClubManagerUser};
use crate::services::auth_service::AuthError;
use crate::AppState;
use actix_web::{web, HttpResponse};
use palet_shared::{
    AddTeamMemberRequest, ApiResponse, CreateTeamRequest, Team, TeamMemberRole,
    TeamMemberWithPlayer, TeamStatus, UpdateTeamRequest,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct TeamQuery {
    pub club_id: Option<Uuid>,
    pub status: Option<TeamStatus>,
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
            log::error!("team db error: {other:?}");
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Erreur interne".to_string()),
                success: false,
            })
        }
    }
}

async fn team_or_404(state: &web::Data<AppState>, id: Uuid) -> Result<Option<Team>, DatabaseError> {
    state.db.teams.get_team(id).await
}

/// Liste des équipes, filtres optionnels (public)
#[utoipa::path(
    get,
    path = "/equipes",
    tag = "Equipes",
    params(TeamQuery),
    responses(
        (status = 200, description = "Liste des équipes", body = Vec<Team>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_teams(state: web::Data<AppState>, query: web::Query<TeamQuery>) -> HttpResponse {
    match state.db.teams.get_teams().await {
        Ok(mut teams) => {
            if let Some(club_id) = query.club_id {
                teams.retain(|t| t.club_id == club_id);
            }
            if let Some(status) = &query.status {
                teams.retain(|t| &t.status == status);
            }
            if let Some(q) = &query.q {
                let needle = q.to_lowercase();
                teams.retain(|t| t.name.to_lowercase().contains(&needle));
            }
            HttpResponse::Ok().json(ApiResponse {
                data: teams,
                message: Some("Équipes récupérées".to_string()),
                success: true,
            })
        }
        Err(e) => db_error_response(e),
    }
}

/// Fiche d'une équipe (public)
#[utoipa::path(
    get,
    path = "/equipes/{id}",
    tag = "Equipes",
    params(("id", description = "Team ID")),
    responses(
        (status = 200, description = "Équipe", body = Team),
        (status = 404, description = "Équipe introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_team(state: web::Data<AppState>, id: web::Path<Uuid>) -> HttpResponse {
    match team_or_404(&state, *id).await {
        Ok(Some(team)) => HttpResponse::Ok().json(ApiResponse {
            data: team,
            message: Some("Équipe récupérée".to_string()),
            success: true,
        }),
        Ok(None) => db_error_response(DatabaseError::NotFound("Équipe introuvable".to_string())),
        Err(e) => db_error_response(e),
    }
}

/// Effectif d'une équipe avec fiches joueurs (public)
#[utoipa::path(
    get,
    path = "/equipes/{id}/effectif",
    tag = "Equipes",
    params(("id", description = "Team ID")),
    responses(
        (status = 200, description = "Effectif", body = Vec<TeamMemberWithPlayer>),
        (status = 404, description = "Équipe introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_effectif(state: web::Data<AppState>, id: web::Path<Uuid>) -> HttpResponse {
    match state.db.teams.get_team(*id).await {
        Ok(None) => db_error_response(DatabaseError::NotFound("Équipe introuvable".to_string())),
        Ok(Some(_)) => match state.db.teams.get_team_members(*id).await {
            Ok(members) => HttpResponse::Ok().json(ApiResponse {
                data: members,
                message: Some("Effectif récupéré".to_string()),
                success: true,
            }),
            Err(e) => db_error_response(e),
        },
        Err(e) => db_error_response(e),
    }
}

/// Crée une équipe dans un club (responsable du club ou admin)
#[utoipa::path(
    post,
    path = "/equipes",
    tag = "Equipes",
    request_body = CreateTeamRequest,
    responses(
        (status = 201, description = "Équipe créée", body = Team),
        (status = 400, description = "Données invalides"),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Hors périmètre club"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_team(
    manager: ClubManagerUser,
    state: web::Data<AppState>,
    request: web::Json<CreateTeamRequest>,
) -> Result<HttpResponse, AuthError> {
    let request = request.into_inner();
    check_club_scope(&manager.0, request.club_id)?;
    match state
        .db
        .teams
        .create_team(request.name, request.club_id, request.captain_id)
        .await
    {
        Ok(team) => Ok(HttpResponse::Created().json(ApiResponse {
            data: team,
            message: Some("Équipe créée".to_string()),
            success: true,
        })),
        Err(e) => Ok(db_error_response(e)),
    }
}

/// Met à jour une équipe (responsable du club ou admin)
#[utoipa::path(
    put,
    path = "/equipes/{id}",
    tag = "Equipes",
    params(("id", description = "Team ID")),
    request_body = UpdateTeamRequest,
    responses(
        (status = 200, description = "Équipe mise à jour", body = Team),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Hors périmètre club"),
        (status = 404, description = "Équipe introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_team(
    manager: ClubManagerUser,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    request: web::Json<UpdateTeamRequest>,
) -> Result<HttpResponse, AuthError> {
    let request = request.into_inner();
    let Some(team) = team_or_404(&state, *id)
        .await
        .map_err(|e| AuthError::Internal(e.to_string()))?
    else {
        return Ok(db_error_response(DatabaseError::NotFound(
            "Équipe introuvable".to_string(),
        )));
    };
    check_club_scope(&manager.0, team.club_id)?;
    match state
        .db
        .teams
        .update_team(*id, request.name, request.captain_id, None, request.status)
        .await
    {
        Ok(Some(team)) => Ok(HttpResponse::Ok().json(ApiResponse {
            data: team,
            message: Some("Équipe mise à jour".to_string()),
            success: true,
        })),
        Ok(None) => Ok(db_error_response(DatabaseError::NotFound(
            "Équipe introuvable".to_string(),
        ))),
        Err(e) => Ok(db_error_response(e)),
    }
}

/// Dissout une équipe (responsable du club ou admin)
#[utoipa::path(
    delete,
    path = "/equipes/{id}",
    tag = "Equipes",
    params(("id", description = "Team ID")),
    responses(
        (status = 200, description = "Équipe supprimée", body = bool),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Hors périmètre club"),
        (status = 404, description = "Équipe introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_team(
    manager: ClubManagerUser,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, AuthError> {
    let Some(team) = team_or_404(&state, *id)
        .await
        .map_err(|e| AuthError::Internal(e.to_string()))?
    else {
        return Ok(db_error_response(DatabaseError::NotFound(
            "Équipe introuvable".to_string(),
        )));
    };
    check_club_scope(&manager.0, team.club_id)?;
    match state.db.teams.delete_team(*id).await {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse {
            data: true,
            message: Some("Équipe supprimée".to_string()),
            success: true,
        })),
        Ok(false) => Ok(db_error_response(DatabaseError::NotFound(
            "Équipe introuvable".to_string(),
        ))),
        Err(e) => Ok(db_error_response(e)),
    }
}

/// Ajoute un joueur à l'effectif d'une équipe (responsable du club ou admin)
#[utoipa::path(
    post,
    path = "/equipes/{id}/joueurs",
    tag = "Equipes",
    params(("id", description = "Team ID")),
    request_body = AddTeamMemberRequest,
    responses(
        (status = 201, description = "Membre ajouté", body = palet_shared::TeamMember),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Hors périmètre club"),
        (status = 404, description = "Équipe introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn add_team_member(
    manager: ClubManagerUser,
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    request: web::Json<AddTeamMemberRequest>,
) -> Result<HttpResponse, AuthError> {
    let request = request.into_inner();
    let Some(team) = team_or_404(&state, *id)
        .await
        .map_err(|e| AuthError::Internal(e.to_string()))?
    else {
        return Ok(db_error_response(DatabaseError::NotFound(
            "Équipe introuvable".to_string(),
        )));
    };
    check_club_scope(&manager.0, team.club_id)?;
    let role = request.role.unwrap_or(TeamMemberRole::Member);
    match state
        .db
        .teams
        .add_player_to_team(*id, request.player_id, role, request.position)
        .await
    {
        Ok(member) => Ok(HttpResponse::Created().json(ApiResponse {
            data: member,
            message: Some("Membre ajouté à l'effectif".to_string()),
            success: true,
        })),
        Err(e) => Ok(db_error_response(e)),
    }
}

/// Retire un joueur de l'effectif (responsable du club ou admin)
#[utoipa::path(
    delete,
    path = "/equipes/{id}/joueurs/{player_id}",
    tag = "Equipes",
    params(("id", description = "Team ID"), ("player_id", description = "Player ID")),
    responses(
        (status = 200, description = "Membre retiré", body = bool),
        (status = 401, description = "Non authentifié"),
        (status = 403, description = "Hors périmètre club"),
        (status = 404, description = "Équipe ou membre introuvable"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn remove_team_member(
    manager: ClubManagerUser,
    state: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AuthError> {
    let (team_id, player_id) = path.into_inner();
    let Some(team) = team_or_404(&state, team_id)
        .await
        .map_err(|e| AuthError::Internal(e.to_string()))?
    else {
        return Ok(db_error_response(DatabaseError::NotFound(
            "Équipe introuvable".to_string(),
        )));
    };
    check_club_scope(&manager.0, team.club_id)?;
    match state
        .db
        .teams
        .remove_player_from_team(team_id, player_id)
        .await
    {
        Ok(true) => Ok(HttpResponse::Ok().json(ApiResponse {
            data: true,
            message: Some("Membre retiré de l'effectif".to_string()),
            success: true,
        })),
        Ok(false) => Ok(db_error_response(DatabaseError::NotFound(
            "Membre introuvable dans cette équipe".to_string(),
        ))),
        Err(e) => Ok(db_error_response(e)),
    }
}
