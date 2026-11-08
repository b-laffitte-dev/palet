//! Tests d'intégration HTTP des endpoints /divisions (skill rust-tdd).
//!
//! Le pool est "lazy" (aucune base réelle requise) : on remplace le
//! repository `divisions` par un mock en mémoire via le champ public de
//! `Database`, ce qui teste le wiring HTTP + la logique métier (validation,
//! unicité, position auto, filtre par ligue).

use actix_web::{http::StatusCode, test, web, App};
use async_trait::async_trait;
use chrono::Utc;
use palet_backend::{
    config::AppConfig,
    db::{Database, DatabaseError, DivisionRepository},
    handlers::division_handler,
    AppState,
};
use palet_shared::{ApiResponse, CreateDivisionRequest, Division};
use sqlx::postgres::PgPoolOptions;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

struct MockDivisionRepository {
    divisions: Mutex<Vec<Division>>,
}

impl MockDivisionRepository {
    fn new() -> Self {
        Self {
            divisions: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl DivisionRepository for MockDivisionRepository {
    async fn create_division(
        &self,
        request: CreateDivisionRequest,
    ) -> Result<Division, DatabaseError> {
        let (name, league, season) = request
            .validated()
            .map_err(DatabaseError::ValidationError)?;
        let mut divisions = self.divisions.lock().unwrap();
        if divisions
            .iter()
            .any(|d| d.league == league && d.season == season && d.name == name)
        {
            return Err(DatabaseError::ValidationError(format!(
                "La division {name} existe déjà pour {league} (saison {season})"
            )));
        }
        let position = divisions
            .iter()
            .filter(|d| d.league == league && d.season == season)
            .map(|d| d.position)
            .max()
            .unwrap_or(0)
            + 1;
        let now = Utc::now();
        let division = Division {
            id: Uuid::new_v4(),
            name,
            league,
            season,
            position,
            created_at: now,
            updated_at: now,
        };
        divisions.push(division.clone());
        Ok(division)
    }

    async fn get_division(&self, id: Uuid) -> Result<Option<Division>, DatabaseError> {
        Ok(self
            .divisions
            .lock()
            .unwrap()
            .iter()
            .find(|d| d.id == id)
            .cloned())
    }

    async fn get_divisions(&self, league: Option<String>) -> Result<Vec<Division>, DatabaseError> {
        let mut divisions = self.divisions.lock().unwrap().clone();
        if let Some(league) = &league {
            divisions.retain(|d| &d.league == league);
        }
        divisions.sort_by(|a, b| a.position.cmp(&b.position).then(a.name.cmp(&b.name)));
        Ok(divisions)
    }

    async fn delete_division(&self, id: Uuid) -> Result<bool, DatabaseError> {
        let mut divisions = self.divisions.lock().unwrap();
        let before = divisions.len();
        divisions.retain(|d| d.id != id);
        Ok(divisions.len() != before)
    }
}

fn test_state() -> web::Data<AppState> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_millis(500))
        .connect_lazy("postgres://palet:palet@localhost:5432/palet")
        .expect("création du pool lazy");
    let mut db = Database::new(pool);
    db.divisions = Arc::new(MockDivisionRepository::new());
    web::Data::new(AppState {
        db: Arc::new(db),
        config: Arc::new(AppConfig::default()),
    })
}

macro_rules! init_app {
    () => {
        test::init_service(
            App::new().app_data(test_state()).service(
                web::scope("/divisions")
                    .route("", web::get().to(division_handler::get_divisions))
                    .route("", web::post().to(division_handler::create_division))
                    .route("/{id}", web::get().to(division_handler::get_division))
                    .route("/{id}", web::delete().to(division_handler::delete_division)),
            ),
        )
        .await
    };
}

fn division_body(name: &str) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "league": "Deux-Sèvres",
        "season": "2026-2027"
    })
}

#[actix_web::test]
async fn get_divisions_returns_empty_list_initially() {
    let app = init_app!();

    let req = test::TestRequest::get().uri("/divisions").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<Vec<Division>> = test::read_body_json(resp).await;
    assert!(body.success);
    assert!(body.data.is_empty());
}

#[actix_web::test]
async fn create_division_returns_201_with_normalized_name_and_position() {
    let app = init_app!();

    let req = test::TestRequest::post()
        .uri("/divisions")
        .set_json(division_body(" d1 "))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: ApiResponse<Division> = test::read_body_json(resp).await;
    assert_eq!(body.data.name, "D1");
    assert_eq!(body.data.league, "Deux-Sèvres");
    assert_eq!(body.data.season, "2026-2027");
    assert_eq!(body.data.position, 1);
}

#[actix_web::test]
async fn create_several_divisions_appends_positions_and_lists_them_ordered() {
    let app = init_app!();

    for name in ["D1", "D2", "D3"] {
        let req = test::TestRequest::post()
            .uri("/divisions")
            .set_json(division_body(name))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    let req = test::TestRequest::get()
        .uri("/divisions?league=Deux-S%C3%A8vres")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<Vec<Division>> = test::read_body_json(resp).await;
    let names: Vec<&str> = body.data.iter().map(|d| d.name.as_str()).collect();
    assert_eq!(names, vec!["D1", "D2", "D3"]);
    let positions: Vec<i32> = body.data.iter().map(|d| d.position).collect();
    assert_eq!(positions, vec![1, 2, 3]);
}

#[actix_web::test]
async fn create_division_rejects_duplicate_with_400() {
    let app = init_app!();

    let req = test::TestRequest::post()
        .uri("/divisions")
        .set_json(division_body("D1"))
        .to_request();
    test::call_service(&app, req).await;

    let req = test::TestRequest::post()
        .uri("/divisions")
        .set_json(division_body("D1"))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let body: ApiResponse<()> = test::read_body_json(resp).await;
    assert!(!body.success);
}

#[actix_web::test]
async fn create_division_rejects_empty_name_with_400() {
    let app = init_app!();

    let req = test::TestRequest::post()
        .uri("/divisions")
        .set_json(division_body("   "))
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn league_filter_returns_only_matching_divisions() {
    let app = init_app!();

    let req = test::TestRequest::post()
        .uri("/divisions")
        .set_json(division_body("D1"))
        .to_request();
    test::call_service(&app, req).await;

    let req = test::TestRequest::get()
        .uri("/divisions?league=Vend%C3%A9e")
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: ApiResponse<Vec<Division>> = test::read_body_json(resp).await;
    assert!(body.data.is_empty());
}

#[actix_web::test]
async fn delete_division_returns_200_then_404_and_get_404_after_delete() {
    let app = init_app!();

    let req = test::TestRequest::post()
        .uri("/divisions")
        .set_json(division_body("D1"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let created: ApiResponse<Division> = test::read_body_json(resp).await;
    let id = created.data.id;

    let req = test::TestRequest::get()
        .uri(&format!("/divisions/{id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let req = test::TestRequest::delete()
        .uri(&format!("/divisions/{id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<bool> = test::read_body_json(resp).await;
    assert!(body.data);

    let req = test::TestRequest::delete()
        .uri(&format!("/divisions/{id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    let req = test::TestRequest::get()
        .uri(&format!("/divisions/{id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
