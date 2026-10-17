//! Club endpoint handlers for Palet Venden
//!
//! Handles all club-related operations

use actix_web::{web, HttpResponse, Responder};
use palet_shared::{ApiResponse, Club, CreateClubRequest, PaginatedResponse, Pagination};
use uuid::Uuid;
use crate::repositories::club_repository::ClubRepository;

use crate::AppState;

/// Get all clubs
#[utoipa::path(
    get,
    path = "/clubs",
    tag = "Clubs",
    responses(
        (status = 200, description = "List of all clubs", body = PaginatedResponse<Club>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_clubs(state: web::Data<AppState>) -> impl Responder {
    match state.db.clubs.get_clubs().await {
        Ok(clubs) => {
            let pagination = Pagination {
                current: 1,
                total: 1,
                page_size: clubs.len() as i64,
                total_items: clubs.len() as i64,
                has_next: false,
                has_previous: false,
            };

            HttpResponse::Ok().json(ApiResponse {
                data: PaginatedResponse {
                    data: clubs,
                    pagination,
                },
                message: Some("Clubs retrieved successfully".to_string()),
                success: true,
            })
        }
        Err(e) => {
            log::error!("Failed to get clubs: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get clubs".to_string()),
                success: false,
            })
        }
    }
}

/// Get club by ID
#[utoipa::path(
    get,
    path = "/clubs/{id}",
    tag = "Clubs",
    params(
        ("id", description = "Club ID")
    ),
    responses(
        (status = 200, description = "Club details", body = Club),
        (status = 404, description = "Club not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_club(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    match state.db.clubs.get_club(*id).await {
        Ok(Some(club)) => HttpResponse::Ok().json(ApiResponse {
            data: club,
            message: Some("Club retrieved successfully".to_string()),
            success: true,
        }),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Club not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to get club: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get club".to_string()),
                success: false,
            })
        }
    }
}

/// Create a new club
#[utoipa::path(
    post,
    path = "/clubs",
    tag = "Clubs",
    request_body = CreateClubRequest,
    responses(
        (status = 201, description = "Club created", body = Club),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_club(
    state: web::Data<AppState>,
    request: web::Json<CreateClubRequest>,
) -> impl Responder {
    match state.db.clubs.create_club(request.into_inner()).await {
        Ok(club) => HttpResponse::Created().json(ApiResponse {
            data: club,
            message: Some("Club created successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to create club: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to create club".to_string()),
                success: false,
            })
        }
    }
}
