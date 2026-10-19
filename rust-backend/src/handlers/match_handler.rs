//! Match endpoint handlers for Palet Venden
//!
//! Handles all match-related operations with TDD approach

use actix_web::{web, HttpResponse, Responder};
use palet_shared::{
    ApiResponse, CreateMatchRequest, MancheScore, Match, MatchStatus, MatchWinner,
    PaginatedResponse, Pagination, UpdateMatchRequest,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

/// Get all matches
#[utoipa::path(
    get,
    path = "/matchs",
    tag = "Matchs",
    responses(
        (status = 200, description = "List of all matches", body = PaginatedResponse<Match>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_matches(state: web::Data<AppState>) -> impl Responder {
    match state.db.matches.get_matches().await {
        Ok(matches) => {
            let pagination = Pagination {
                current: 1,
                total: 1,
                page_size: matches.len() as i64,
                total_items: matches.len() as i64,
                has_next: false,
                has_previous: false,
            };

            HttpResponse::Ok().json(ApiResponse {
                data: PaginatedResponse {
                    data: matches,
                    pagination,
                },
                message: Some("Matchs retrieved successfully".to_string()),
                success: true,
            })
        }
        Err(e) => {
            log::error!("Failed to get matches: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get matches".to_string()),
                success: false,
            })
        }
    }
}

/// Get match by ID
#[utoipa::path(
    get,
    path = "/matchs/{id}",
    tag = "Matchs",
    params(
        ("id", description = "Match ID")
    ),
    responses(
        (status = 200, description = "Match details", body = Match),
        (status = 404, description = "Match not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_match(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    match state.db.matches.get_match(*id).await {
        Ok(Some(match_obj)) => HttpResponse::Ok().json(ApiResponse {
            data: match_obj,
            message: Some("Match retrieved successfully".to_string()),
            success: true,
        }),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Match not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to get match: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get match".to_string()),
                success: false,
            })
        }
    }
}

/// Get matches by competition
#[utoipa::path(
    get,
    path = "/matchs/competition/{competition_id}",
    tag = "Matchs",
    params(
        ("competition_id", description = "Competition ID")
    ),
    responses(
        (status = 200, description = "List of matches by competition", body = Vec<Match>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_matches_by_competition(
    state: web::Data<AppState>,
    competition_id: web::Path<Uuid>,
) -> impl Responder {
    match state
        .db
        .matches
        .get_matches_by_competition(*competition_id)
        .await
    {
        Ok(matches) => HttpResponse::Ok().json(ApiResponse {
            data: matches,
            message: Some("Matchs retrieved successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to get matches by competition: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get matches by competition".to_string()),
                success: false,
            })
        }
    }
}

/// Get matches by status
#[utoipa::path(
    get,
    path = "/matchs/status/{status}",
    tag = "Matchs",
    params(
        ("status", description = "Match status (scheduled, in_progress, completed, etc.)")
    ),
    responses(
        (status = 200, description = "List of matches by status", body = Vec<Match>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_matches_by_status(
    state: web::Data<AppState>,
    status: web::Path<String>,
) -> impl Responder {
    // Parse status from string
    let match_status = match status.as_str() {
        "scheduled" => MatchStatus::Scheduled,
        "in_progress" => MatchStatus::InProgress,
        "completed" => MatchStatus::Completed,
        "postponed" => MatchStatus::Postponed,
        "cancelled" => MatchStatus::Cancelled,
        "disputed" => MatchStatus::Disputed,
        _ => MatchStatus::Scheduled,
    };

    match state.db.matches.get_matches_by_status(match_status).await {
        Ok(matches) => HttpResponse::Ok().json(ApiResponse {
            data: matches,
            message: Some("Matchs retrieved successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to get matches by status: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get matches by status".to_string()),
                success: false,
            })
        }
    }
}

/// Create a new match
#[utoipa::path(
    post,
    path = "/matchs",
    tag = "Matchs",
    request_body = CreateMatchRequest,
    responses(
        (status = 201, description = "Match created", body = Match),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_match(
    state: web::Data<AppState>,
    request: web::Json<CreateMatchRequest>,
) -> impl Responder {
    match state.db.matches.create_match(request.into_inner()).await {
        Ok(match_obj) => HttpResponse::Created().json(ApiResponse {
            data: match_obj,
            message: Some("Match created successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to create match: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to create match".to_string()),
                success: false,
            })
        }
    }
}

/// Update a match
#[utoipa::path(
    put,
    path = "/matchs/{id}",
    tag = "Matchs",
    params(
        ("id", description = "Match ID")
    ),
    request_body = UpdateMatchRequest,
    responses(
        (status = 200, description = "Match updated", body = Match),
        (status = 404, description = "Match not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_match(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    request: web::Json<UpdateMatchRequest>,
) -> impl Responder {
    match state
        .db
        .matches
        .update_match(*id, request.into_inner())
        .await
    {
        Ok(Some(match_obj)) => HttpResponse::Ok().json(ApiResponse {
            data: match_obj,
            message: Some("Match updated successfully".to_string()),
            success: true,
        }),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Match not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to update match: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to update match".to_string()),
                success: false,
            })
        }
    }
}

/// Delete a match
#[utoipa::path(
    delete,
    path = "/matchs/{id}",
    tag = "Matchs",
    params(
        ("id", description = "Match ID")
    ),
    responses(
        (status = 200, description = "Match deleted", body = ApiResponse<bool>),
        (status = 404, description = "Match not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_match(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    match state.db.matches.delete_match(*id).await {
        Ok(deleted) => {
            if deleted {
                HttpResponse::Ok().json(ApiResponse {
                    data: deleted,
                    message: Some("Match deleted successfully".to_string()),
                    success: true,
                })
            } else {
                HttpResponse::NotFound().json(ApiResponse::<()> {
                    data: (),
                    message: Some("Match not found".to_string()),
                    success: false,
                })
            }
        }
        Err(e) => {
            log::error!("Failed to delete match: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to delete match".to_string()),
                success: false,
            })
        }
    }
}

/// Start a match
#[utoipa::path(
    post,
    path = "/matchs/{id}/start",
    tag = "Matchs",
    params(
        ("id", description = "Match ID")
    ),
    responses(
        (status = 200, description = "Match started", body = Match),
        (status = 400, description = "Invalid operation"),
        (status = 404, description = "Match not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn start_match(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    match state.db.matches.start_match(*id).await {
        Ok(match_obj) => HttpResponse::Ok().json(ApiResponse {
            data: match_obj,
            message: Some("Match started successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to start match: {:?}", e);
            HttpResponse::BadRequest().json(ApiResponse::<()> {
                data: (),
                message: Some(format!("Failed to start match: {}", e)),
                success: false,
            })
        }
    }
}

/// Complete a match
#[utoipa::path(
    post,
    path = "/matchs/{id}/complete",
    tag = "Matchs",
    params(
        ("id", description = "Match ID")
    ),
    responses(
        (status = 200, description = "Match completed", body = Match),
        (status = 400, description = "Invalid operation"),
        (status = 404, description = "Match not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn complete_match(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    winner: web::Query<MatchWinner>,
) -> impl Responder {
    match state.db.matches.complete_match(*id, winner.0.clone()).await {
        Ok(match_obj) => HttpResponse::Ok().json(ApiResponse {
            data: match_obj,
            message: Some("Match completed successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to complete match: {:?}", e);
            HttpResponse::BadRequest().json(ApiResponse::<()> {
                data: (),
                message: Some(format!("Failed to complete match: {}", e)),
                success: false,
            })
        }
    }
}

/// Add a manche to a match
#[utoipa::path(
    post,
    path = "/matchs/{id}/manche",
    tag = "Matchs",
    params(
        ("id", description = "Match ID")
    ),
    request_body = AddMancheRequest,
    responses(
        (status = 200, description = "Manche added", body = Match),
        (status = 400, description = "Invalid operation"),
        (status = 404, description = "Match not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn add_manche(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    request: web::Json<AddMancheRequest>,
) -> impl Responder {
    match state
        .db
        .matches
        .add_manche(
            *id,
            MancheScore {
                number: request.current_manche + 1,
                team1: request.team1_score,
                team2: request.team2_score,
                winner: None,
            },
        )
        .await
    {
        Ok(match_obj) => HttpResponse::Ok().json(ApiResponse {
            data: match_obj,
            message: Some("Manche added successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to add manche: {:?}", e);
            HttpResponse::BadRequest().json(ApiResponse::<()> {
                data: (),
                message: Some(format!("Failed to add manche: {}", e)),
                success: false,
            })
        }
    }
}

// ============================================
// REQUEST/RESPONSE TYPES
// ============================================

/// Request to add a manche
#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AddMancheRequest {
    pub team1_score: i32,
    pub team2_score: i32,
    pub current_manche: i32,
}
