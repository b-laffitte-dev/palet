//! Championship endpoint handlers for Palet Venden
//!
//! Handles all championship-related operations with TDD approach

use actix_web::{web, HttpResponse, Responder};
use palet_shared::{
    ApiResponse, Championship, ChampionshipClassification, ClassificationPrimary,
    ClassificationRules, ClassificationSecondary, ClassificationTertiary, CompetitionStatus,
    CreateChampionshipRequest, PaginatedResponse, Pagination, UpdateChampionshipRequest,
};
use uuid::Uuid;

use crate::AppState;

/// Get all championships
#[utoipa::path(
    get,
    path = "/championnats",
    tag = "Championnats",
    responses(
        (status = 200, description = "List of all championships", body = PaginatedResponse<Championship>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_championships(state: web::Data<AppState>) -> impl Responder {
    match state.db.championships.get_championships().await {
        Ok(championships) => {
            let pagination = Pagination {
                current: 1,
                total: 1,
                page_size: championships.len() as i64,
                total_items: championships.len() as i64,
                has_next: false,
                has_previous: false,
            };

            HttpResponse::Ok().json(ApiResponse {
                data: PaginatedResponse {
                    data: championships,
                    pagination,
                },
                message: Some("Championnats retrieved successfully".to_string()),
                success: true,
            })
        }
        Err(e) => {
            log::error!("Failed to get championships: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get championships".to_string()),
                success: false,
            })
        }
    }
}

/// Get championship by ID
#[utoipa::path(
    get,
    path = "/championnats/{id}",
    tag = "Championnats",
    params(
        ("id", description = "Championship ID")
    ),
    responses(
        (status = 200, description = "Championship details", body = Championship),
        (status = 404, description = "Championship not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_championship(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    match state.db.championships.get_championship(*id).await {
        Ok(Some(championship)) => HttpResponse::Ok().json(ApiResponse {
            data: championship,
            message: Some("Championnat retrieved successfully".to_string()),
            success: true,
        }),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Championnat not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to get championship: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get championship".to_string()),
                success: false,
            })
        }
    }
}

/// Get championships by league
#[utoipa::path(
    get,
    path = "/championnats/league/{league}",
    tag = "Championnats",
    params(
        ("league", description = "League name")
    ),
    responses(
        (status = 200, description = "List of championships by league", body = Vec<Championship>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_championships_by_league(
    state: web::Data<AppState>,
    league: web::Path<String>,
) -> impl Responder {
    match state
        .db
        .championships
        .get_championships_by_league(league.to_string())
        .await
    {
        Ok(championships) => HttpResponse::Ok().json(ApiResponse {
            data: championships,
            message: Some("Championnats retrieved successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to get championships by league: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get championships by league".to_string()),
                success: false,
            })
        }
    }
}

/// Get championships by season
#[utoipa::path(
    get,
    path = "/championnats/season/{season}",
    tag = "Championnats",
    params(
        ("season", description = "Season (e.g., 2025-2026)")
    ),
    responses(
        (status = 200, description = "List of championships by season", body = Vec<Championship>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_championships_by_season(
    state: web::Data<AppState>,
    season: web::Path<String>,
) -> impl Responder {
    match state
        .db
        .championships
        .get_championships_by_season(season.to_string())
        .await
    {
        Ok(championships) => HttpResponse::Ok().json(ApiResponse {
            data: championships,
            message: Some("Championnats retrieved successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to get championships by season: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get championships by season".to_string()),
                success: false,
            })
        }
    }
}

/// Create a new championship
#[utoipa::path(
    post,
    path = "/championnats",
    tag = "Championnats",
    request_body = CreateChampionshipRequest,
    responses(
        (status = 201, description = "Championship created", body = Championship),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_championship(
    state: web::Data<AppState>,
    request: web::Json<CreateChampionshipRequest>,
) -> impl Responder {
    match state
        .db
        .championships
        .create_championship(request.into_inner())
        .await
    {
        Ok(championship) => HttpResponse::Created().json(ApiResponse {
            data: championship,
            message: Some("Championnat created successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to create championship: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to create championship".to_string()),
                success: false,
            })
        }
    }
}

/// Update a championship
#[utoipa::path(
    put,
    path = "/championnats/{id}",
    tag = "Championnats",
    params(
        ("id", description = "Championship ID")
    ),
    request_body = UpdateChampionshipRequest,
    responses(
        (status = 200, description = "Championship updated", body = Championship),
        (status = 404, description = "Championship not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_championship(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    request: web::Json<UpdateChampionshipRequest>,
) -> impl Responder {
    match state
        .db
        .championships
        .update_championship(*id, request.into_inner())
        .await
    {
        Ok(Some(championship)) => HttpResponse::Ok().json(ApiResponse {
            data: championship,
            message: Some("Championnat updated successfully".to_string()),
            success: true,
        }),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Championnat not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to update championship: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to update championship".to_string()),
                success: false,
            })
        }
    }
}

/// Delete a championship
#[utoipa::path(
    delete,
    path = "/championnats/{id}",
    tag = "Championnats",
    params(
        ("id", description = "Championship ID")
    ),
    responses(
        (status = 200, description = "Championship deleted", body = ApiResponse<bool>),
        (status = 404, description = "Championship not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_championship(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    match state.db.championships.delete_championship(*id).await {
        Ok(deleted) => {
            if deleted {
                HttpResponse::Ok().json(ApiResponse {
                    data: deleted,
                    message: Some("Championnat deleted successfully".to_string()),
                    success: true,
                })
            } else {
                HttpResponse::NotFound().json(ApiResponse::<()> {
                    data: (),
                    message: Some("Championnat not found".to_string()),
                    success: false,
                })
            }
        }
        Err(e) => {
            log::error!("Failed to delete championship: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to delete championship".to_string()),
                success: false,
            })
        }
    }
}

/// Start a championship
#[utoipa::path(
    post,
    path = "/championnats/{id}/start",
    tag = "Championnats",
    params(
        ("id", description = "Championship ID")
    ),
    responses(
        (status = 200, description = "Championship started", body = Championship),
        (status = 400, description = "Invalid operation"),
        (status = 404, description = "Championship not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn start_championship(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    match state.db.championships.start_championship(*id).await {
        Ok(championship) => HttpResponse::Ok().json(ApiResponse {
            data: championship,
            message: Some("Championnat started successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to start championship: {:?}", e);
            HttpResponse::BadRequest().json(ApiResponse::<()> {
                data: (),
                message: Some(format!("Failed to start championship: {}", e)),
                success: false,
            })
        }
    }
}

/// Move to next journee
#[utoipa::path(
    post,
    path = "/championnats/{id}/next-journee",
    tag = "Championnats",
    params(
        ("id", description = "Championship ID")
    ),
    responses(
        (status = 200, description = "Moved to next journee", body = Championship),
        (status = 400, description = "Invalid operation"),
        (status = 404, description = "Championship not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn next_journee(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    match state.db.championships.next_journee(*id).await {
        Ok(championship) => HttpResponse::Ok().json(ApiResponse {
            data: championship,
            message: Some("Journ\u{00e9}e avanc\u{00e9}e avec succ\u{00e8}s".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to move to next journee: {:?}", e);
            HttpResponse::BadRequest().json(ApiResponse::<()> {
                data: (),
                message: Some(format!("Failed to move to next journee: {}", e)),
                success: false,
            })
        }
    }
}

/// Get championship classification
#[utoipa::path(
    get,
    path = "/championnats/{id}/classification",
    tag = "Championnats",
    params(
        ("id", description = "Championship ID")
    ),
    responses(
        (status = 200, description = "Championship classification", body = Vec<ChampionshipClassification>),
        (status = 404, description = "Championship not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_championship_classification(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    match state
        .db
        .championships
        .get_championship_classification(*id, None)
        .await
    {
        Ok(classifications) => HttpResponse::Ok().json(ApiResponse {
            data: classifications,
            message: Some("Classification retrieved successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to get championship classification: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get championship classification".to_string()),
                success: false,
            })
        }
    }
}

/// Update championship classification
#[utoipa::path(
    put,
    path = "/championnats/{id}/classification",
    tag = "Championnats",
    params(
        ("id", description = "Championship ID")
    ),
    request_body = ChampionshipClassification,
    responses(
        (status = 200, description = "Classification updated", body = ChampionshipClassification),
        (status = 404, description = "Championship not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_championship_classification(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    request: web::Json<ChampionshipClassification>,
) -> impl Responder {
    match state
        .db
        .championships
        .update_championship_classification(*id, request.division.clone(), request.into_inner())
        .await
    {
        Ok(classification) => HttpResponse::Ok().json(ApiResponse {
            data: classification,
            message: Some("Classification updated successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to update championship classification: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to update championship classification".to_string()),
                success: false,
            })
        }
    }
}
