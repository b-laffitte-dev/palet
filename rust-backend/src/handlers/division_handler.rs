//! Division endpoint handlers for Palet Vendéen
//!
//! Divisions départementales (D1, D2, D3…) gérées par l'espace fédéral.

use actix_web::{web, HttpResponse, Responder};
use palet_shared::{ApiResponse, CreateDivisionRequest, Division};
use serde::Deserialize;
use uuid::Uuid;

use crate::db::DatabaseError;
use crate::AppState;

/// Query params for `GET /divisions`
#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct DivisionQuery {
    /// Filter by league (département), e.g. "Deux-Sèvres"
    pub league: Option<String>,
}

/// Get divisions, optionally filtered by league
#[utoipa::path(
    get,
    path = "/divisions",
    tag = "Divisions",
    params(DivisionQuery),
    responses(
        (status = 200, description = "List of divisions", body = Vec<Division>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_divisions(
    state: web::Data<AppState>,
    query: web::Query<DivisionQuery>,
) -> impl Responder {
    match state.db.divisions.get_divisions(query.league.clone()).await {
        Ok(divisions) => HttpResponse::Ok().json(ApiResponse {
            data: divisions,
            message: Some("Divisions retrieved successfully".to_string()),
            success: true,
        }),
        Err(e) => {
            log::error!("Failed to get divisions: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get divisions".to_string()),
                success: false,
            })
        }
    }
}

/// Get a division by ID
#[utoipa::path(
    get,
    path = "/divisions/{id}",
    tag = "Divisions",
    params(
        ("id", description = "Division ID")
    ),
    responses(
        (status = 200, description = "Division details", body = Division),
        (status = 404, description = "Division not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_division(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    match state.db.divisions.get_division(*id).await {
        Ok(Some(division)) => HttpResponse::Ok().json(ApiResponse {
            data: division,
            message: Some("Division retrieved successfully".to_string()),
            success: true,
        }),
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Division not found".to_string()),
            success: false,
        }),
        Err(e) => {
            log::error!("Failed to get division: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to get division".to_string()),
                success: false,
            })
        }
    }
}

/// Create a new division (espace fédéral)
#[utoipa::path(
    post,
    path = "/divisions",
    tag = "Divisions",
    request_body = CreateDivisionRequest,
    responses(
        (status = 201, description = "Division created", body = Division),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_division(
    state: web::Data<AppState>,
    request: web::Json<CreateDivisionRequest>,
) -> impl Responder {
    match state
        .db
        .divisions
        .create_division(request.into_inner())
        .await
    {
        Ok(division) => HttpResponse::Created().json(ApiResponse {
            data: division,
            message: Some("Division created successfully".to_string()),
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
            log::error!("Failed to create division: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to create division".to_string()),
                success: false,
            })
        }
    }
}

/// Delete a division (espace fédéral)
#[utoipa::path(
    delete,
    path = "/divisions/{id}",
    tag = "Divisions",
    params(
        ("id", description = "Division ID")
    ),
    responses(
        (status = 200, description = "Division deleted", body = ApiResponse<bool>),
        (status = 404, description = "Division not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_division(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    match state.db.divisions.delete_division(*id).await {
        Ok(deleted) => {
            if deleted {
                HttpResponse::Ok().json(ApiResponse {
                    data: deleted,
                    message: Some("Division deleted successfully".to_string()),
                    success: true,
                })
            } else {
                HttpResponse::NotFound().json(ApiResponse::<()> {
                    data: (),
                    message: Some("Division not found".to_string()),
                    success: false,
                })
            }
        }
        Err(e) => {
            log::error!("Failed to delete division: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to delete division".to_string()),
                success: false,
            })
        }
    }
}
