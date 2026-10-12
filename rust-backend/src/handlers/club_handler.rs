//! Club endpoint handlers

use actix_web::{get, post, web, HttpResponse, Responder};
use palet_shared::{ApiResponse, Club, CreateClubRequest, PaginatedResponse, Pagination};
use std::sync::Arc;

use crate::{db::Database, AppState};

/// Get all clubs
///
/// Returns a paginated list of all clubs
#[utoipa::path(
    get,
    path = "/clubs",
    tag = "Clubs",
    summary = "Get all clubs",
    description = "Returns a paginated list of all registered clubs",
    responses(
        (status = 200, description = "List of clubs", body = PaginatedResponse<Club>),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/clubs")]
pub async fn get_clubs(state: web::Data<AppState>) -> impl Responder {
    let clubs = state.db.get_clubs().await.map_err(|e| {
        log::error!("Failed to get clubs: {:?}", e);
        HttpResponse::InternalServerError().json(ApiResponse::<()> {
            data: (),
            message: Some("Failed to get clubs".to_string()),
            success: false,
        })
    })?;

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

/// Get a specific club by ID
///
/// Returns detailed information about a specific club
#[utoipa::path(
    get,
    path = "/clubs/{id}",
    tag = "Clubs",
    summary = "Get club by ID",
    description = "Returns detailed information about a specific club",
    params(
        ("id" = UUID, description = "Club ID")
    ),
    responses(
        (status = 200, description = "Club details", body = Club),
        (status = 404, description = "Club not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/clubs/{id}")]
pub async fn get_club(state: web::Data<AppState>, id: web::Path<uuid::Uuid>) -> impl Responder {
    let club = state.db.get_club(*id).await.map_err(|e| {
        log::error!("Failed to get club: {:?}", e);
        HttpResponse::InternalServerError().json(ApiResponse::<()> {
            data: (),
            message: Some("Failed to get club".to_string()),
            success: false,
        })
    })?;

    match club {
        Some(club) => HttpResponse::Ok().json(ApiResponse {
            data: club,
            message: Some("Club retrieved successfully".to_string()),
            success: true,
        }),
        None => HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Club not found".to_string()),
            success: false,
        }),
    }
}

/// Create a new club
///
/// Creates a new club with the provided information
#[utoipa::path(
    post,
    path = "/clubs",
    tag = "Clubs",
    summary = "Create a new club",
    description = "Creates a new club with the provided information",
    request_body = CreateClubRequest,
    responses(
        (status = 201, description = "Club created", body = Club),
        (status = 400, description = "Invalid request data"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/clubs")]
pub async fn create_club(
    state: web::Data<AppState>,
    request: web::Json<CreateClubRequest>,
) -> impl Responder {
    let club = state
        .db
        .create_club(request.into_inner())
        .await
        .map_err(|e| {
            log::error!("Failed to create club: {:?}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                data: (),
                message: Some("Failed to create club".to_string()),
                success: false,
            })
        })?;

    HttpResponse::Created().json(ApiResponse {
        data: club,
        message: Some("Club created successfully".to_string()),
        success: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use palet_shared::{Club, CreateClubRequest, PaginatedResponse};

    #[actix_rt::test]
    async fn test_get_clubs() {
        let db = Arc::new(crate::db::tests::create_test_db().await);
        db.initialize().await.expect("Failed to initialize");

        let state = AppState {
            db,
            config: Arc::new(crate::config::AppConfig::default()),
        };

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state))
                .service(get_clubs),
        )
        .await;

        let req = test::TestRequest::get().uri("/clubs").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: ApiResponse<PaginatedResponse<Club>> = test::read_body_json(resp).await;
        assert!(body.success);
    }

    #[actix_rt::test]
    async fn test_create_club() {
        let db = Arc::new(crate::db::tests::create_test_db().await);
        db.initialize().await.expect("Failed to initialize");

        let state = AppState {
            db,
            config: Arc::new(crate::config::AppConfig::default()),
        };

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state))
                .service(create_club),
        )
        .await;

        let request = CreateClubRequest {
            name: "Test Club".to_string(),
            nickname: Some("TC".to_string()),
            code: "TC001".to_string(),
            city: "Test City".to_string(),
            postal_code: Some("12345".to_string()),
            league: "Deux-Sèvres".to_string(),
            division: "D1".to_string(),
            federation_id: uuid::Uuid::nil(),
            contact_email: Some("test@test.com".to_string()),
            contact_phone: Some("1234567890".to_string()),
            website: Some("https://test.com".to_string()),
        };

        let req = test::TestRequest::post()
            .uri("/clubs")
            .set_json(&request)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body: ApiResponse<Club> = test::read_body_json(resp).await;
        assert!(body.success);
        assert_eq!(body.data.name, "Test Club");
    }
}
