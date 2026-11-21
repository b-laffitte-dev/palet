//! Tests d'intégration HTTP des endpoints /auth + protection RBAC des routes
//! d'écriture (skill rust-tdd). Pool "lazy" : pas de base réelle requise, le
//! repository `users` est remplacé par un mock en mémoire.

use actix_web::{http::StatusCode, test, web, App};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use palet_backend::{
    config::AppConfig,
    db::{Database, DatabaseError, DivisionRepository, UserRepository},
    handlers::{auth_handler, division_handler},
    models::user::{NewUser, RefreshToken, UserAccount},
    services::auth_service::{create_token, Claims, ACCESS_TOKEN_TYPE},
    AppState,
};
use palet_shared::{
    ApiResponse, AuthResponse, CreateDivisionRequest, Division, UserProfile, UserRole, UserStatus,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// ============================================
// MOCKS
// ============================================

struct MockUserRepository {
    users: Mutex<Vec<UserAccount>>,
    tokens: Mutex<Vec<RefreshToken>>,
}

impl MockUserRepository {
    fn new() -> Self {
        Self {
            users: Mutex::new(Vec::new()),
            tokens: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn find_by_license(&self, license: &str) -> Result<Option<UserAccount>, DatabaseError> {
        Ok(self
            .users
            .lock()
            .unwrap()
            .iter()
            .find(|u| u.license_number.as_deref() == Some(license))
            .cloned())
    }

    async fn create_user(&self, new_user: NewUser) -> Result<UserAccount, DatabaseError> {
        let NewUser {
            email,
            password_hash,
            first_name,
            last_name,
            role,
            club_id,
            license_number,
        } = new_user;
        if self.users.lock().unwrap().iter().any(|u| u.email == email) {
            return Err(DatabaseError::ValidationError(format!(
                "Un compte existe déjà pour {email}"
            )));
        }
        let now = Utc::now();
        let user = UserAccount {
            id: Uuid::new_v4(),
            email,
            username: None,
            password_hash,
            first_name,
            last_name,
            role,
            club_id,
            license_number,
            category: "senior".to_string(),
            status: UserStatus::Active,
            email_verified: false,
            last_login: None,
            created_at: now,
            updated_at: now,
        };
        self.users.lock().unwrap().push(user.clone());
        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserAccount>, DatabaseError> {
        Ok(self
            .users
            .lock()
            .unwrap()
            .iter()
            .find(|u| u.email == email)
            .cloned())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserAccount>, DatabaseError> {
        Ok(self
            .users
            .lock()
            .unwrap()
            .iter()
            .find(|u| u.id == id)
            .cloned())
    }

    async fn update_last_login(&self, id: Uuid) -> Result<(), DatabaseError> {
        let mut users = self.users.lock().unwrap();
        if let Some(u) = users.iter_mut().find(|u| u.id == id) {
            u.last_login = Some(Utc::now());
        }
        Ok(())
    }

    async fn create_refresh_token(
        &self,
        user_id: Uuid,
        token: String,
        expires_at: DateTime<Utc>,
    ) -> Result<RefreshToken, DatabaseError> {
        let rt = RefreshToken {
            id: Uuid::new_v4(),
            user_id,
            token,
            expires_at,
            revoked_at: None,
            created_at: Utc::now(),
        };
        self.tokens.lock().unwrap().push(rt.clone());
        Ok(rt)
    }

    async fn find_refresh_token(&self, token: &str) -> Result<Option<RefreshToken>, DatabaseError> {
        Ok(self
            .tokens
            .lock()
            .unwrap()
            .iter()
            .find(|t| t.token == token)
            .cloned())
    }

    async fn revoke_refresh_token(&self, token: &str) -> Result<(), DatabaseError> {
        let mut tokens = self.tokens.lock().unwrap();
        if let Some(t) = tokens.iter_mut().find(|t| t.token == token) {
            t.revoked_at = Some(Utc::now());
        }
        Ok(())
    }

    async fn revoke_all_user_tokens(&self, _user_id: Uuid) -> Result<(), DatabaseError> {
        Ok(())
    }

    async fn delete_expired_tokens(&self, _now: DateTime<Utc>) -> Result<u64, DatabaseError> {
        Ok(0)
    }
}

struct MockDivisionRepository {
    divisions: Mutex<Vec<Division>>,
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
        let now = Utc::now();
        let division = Division {
            id: Uuid::new_v4(),
            name,
            league,
            season,
            position: 1,
            created_at: now,
            updated_at: now,
        };
        self.divisions.lock().unwrap().push(division.clone());
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

    async fn get_divisions(&self, _league: Option<String>) -> Result<Vec<Division>, DatabaseError> {
        Ok(self.divisions.lock().unwrap().clone())
    }

    async fn delete_division(&self, id: Uuid) -> Result<bool, DatabaseError> {
        let mut divisions = self.divisions.lock().unwrap();
        let before = divisions.len();
        divisions.retain(|d| d.id != id);
        Ok(divisions.len() != before)
    }
}

// ============================================
// APP DE TEST
// ============================================

fn test_state() -> web::Data<AppState> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_millis(500))
        .connect_lazy("postgres://palet:palet@localhost:5432/palet")
        .expect("création du pool lazy");
    let mut db = Database::new(pool);
    db.users = Arc::new(MockUserRepository::new());
    db.divisions = Arc::new(MockDivisionRepository {
        divisions: Mutex::new(Vec::new()),
    });
    web::Data::new(AppState {
        db: Arc::new(db),
        config: Arc::new(AppConfig::default()),
    })
}

macro_rules! init_app {
    () => {
        test::init_service(
            App::new()
                .app_data(test_state())
                .service(
                    web::scope("/auth")
                        .route("/register", web::post().to(auth_handler::register))
                        .route("/login", web::post().to(auth_handler::login))
                        .route("/refresh", web::post().to(auth_handler::refresh))
                        .route("/me", web::get().to(auth_handler::me))
                        .route("/logout", web::post().to(auth_handler::logout)),
                )
                .service(
                    web::scope("/divisions")
                        .route("", web::post().to(division_handler::create_division)),
                ),
        )
        .await
    };
}

macro_rules! post_json {
    ($app:expr, $uri:expr, $body:expr, $token:expr) => {{
        let mut req = test::TestRequest::post().uri($uri).set_json($body);
        if let Some(token) = $token {
            req = req.insert_header(("Authorization", format!("Bearer {token}")));
        }
        test::call_service($app, req.to_request()).await
    }};
}

fn claims_for(role: UserRole) -> Claims {
    Claims {
        sub: Uuid::new_v4().to_string(),
        email: "test@example.fr".to_string(),
        role,
        club_id: None,
        token_type: ACCESS_TOKEN_TYPE.to_string(),
        iat: Utc::now().timestamp(),
        exp: (Utc::now() + Duration::hours(1)).timestamp(),
    }
}

fn token_for(role: UserRole) -> String {
    create_token(&AppConfig::default().jwt_secret, &claims_for(role)).expect("token")
}

fn register_body() -> serde_json::Value {
    serde_json::json!({
        "email": "jean.dupont@example.fr",
        "password": "motdepasse-solide",
        "first_name": "Jean",
        "last_name": "Dupont",
        "club_id": null,
        "license_number": "79-00001"
    })
}

// ============================================
// TESTS
// ============================================

#[actix_web::test]
async fn register_returns_201_with_tokens_and_profile() {
    let app = init_app!();

    let resp = post_json!(&app, "/auth/register", register_body(), None::<String>);
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body: ApiResponse<AuthResponse> = test::read_body_json(resp).await;
    assert!(body.success);
    assert_eq!(body.data.user.email, "jean.dupont@example.fr");
    assert_eq!(body.data.user.role, UserRole::Player);
    assert!(!body.data.token.access_token.is_empty());
    assert!(!body.data.token.refresh_token.is_empty());
    assert_eq!(body.data.token.token_type, "Bearer");
}

#[actix_web::test]
async fn register_rejects_invalid_email_with_400() {
    let app = init_app!();

    let mut body = register_body();
    body["email"] = serde_json::json!("pas-un-email");
    let resp = post_json!(&app, "/auth/register", body, None::<String>);
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn register_conflict_on_duplicate_email_with_409() {
    let app = init_app!();

    let resp = post_json!(&app, "/auth/register", register_body(), None::<String>);
    assert_eq!(resp.status(), StatusCode::CREATED);
    let resp = post_json!(&app, "/auth/register", register_body(), None::<String>);
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}

#[actix_web::test]
async fn login_returns_200_then_401_on_wrong_password_or_unknown_email() {
    let app = init_app!();
    post_json!(&app, "/auth/register", register_body(), None::<String>);

    let ok = serde_json::json!({
        "email": "jean.dupont@example.fr",
        "password": "motdepasse-solide"
    });
    let resp = post_json!(&app, "/auth/login", ok, None::<String>);
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<AuthResponse> = test::read_body_json(resp).await;
    assert_eq!(body.data.user.first_name, "Jean");

    let ko = serde_json::json!({
        "email": "jean.dupont@example.fr",
        "password": "mauvais-mot-de-passe"
    });
    let resp = post_json!(&app, "/auth/login", ko, None::<String>);
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let unknown = serde_json::json!({
        "email": "inconnu@example.fr",
        "password": "motdepasse-solide"
    });
    let resp = post_json!(&app, "/auth/login", unknown, None::<String>);
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn me_returns_profile_with_access_token_401_without_or_with_refresh_token() {
    let app = init_app!();
    let resp = post_json!(&app, "/auth/register", register_body(), None::<String>);
    let registered: ApiResponse<AuthResponse> = test::read_body_json(resp).await;
    let access = registered.data.token.access_token.clone();
    let refresh = registered.data.token.refresh_token.clone();

    let req = test::TestRequest::get()
        .uri("/auth/me")
        .insert_header(("Authorization", format!("Bearer {access}")))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<UserProfile> = test::read_body_json(resp).await;
    assert_eq!(body.data.email, "jean.dupont@example.fr");

    let req = test::TestRequest::get().uri("/auth/me").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let req = test::TestRequest::get()
        .uri("/auth/me")
        .insert_header(("Authorization", format!("Bearer {refresh}")))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn refresh_rotates_tokens_and_rejects_reuse_of_old_token() {
    let app = init_app!();
    let resp = post_json!(&app, "/auth/register", register_body(), None::<String>);
    let registered: ApiResponse<AuthResponse> = test::read_body_json(resp).await;
    let old_refresh = registered.data.token.refresh_token;

    let resp = post_json!(
        &app,
        "/auth/refresh",
        serde_json::json!({ "refresh_token": old_refresh }),
        None::<String>
    );
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<AuthResponse> = test::read_body_json(resp).await;
    assert!(!body.data.token.refresh_token.is_empty());

    let resp = post_json!(
        &app,
        "/auth/refresh",
        serde_json::json!({ "refresh_token": old_refresh }),
        None::<String>
    );
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn logout_revokes_refresh_token() {
    let app = init_app!();
    let resp = post_json!(&app, "/auth/register", register_body(), None::<String>);
    let registered: ApiResponse<AuthResponse> = test::read_body_json(resp).await;
    let access = registered.data.token.access_token;
    let refresh = registered.data.token.refresh_token;

    let resp = post_json!(
        &app,
        "/auth/logout",
        serde_json::json!({ "refresh_token": refresh }),
        Some(access)
    );
    assert_eq!(resp.status(), StatusCode::OK);

    let resp = post_json!(
        &app,
        "/auth/refresh",
        serde_json::json!({ "refresh_token": refresh }),
        None::<String>
    );
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn protected_write_requires_admin_role() {
    let app = init_app!();
    let body = serde_json::json!({
        "name": "D1",
        "league": "Deux-Sèvres",
        "season": "2026-2027"
    });

    let resp = post_json!(&app, "/divisions", body.clone(), None::<String>);
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let resp = post_json!(
        &app,
        "/divisions",
        body.clone(),
        Some(token_for(UserRole::Player))
    );
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    let resp = post_json!(
        &app,
        "/divisions",
        body,
        Some(token_for(UserRole::CommissionAdmin))
    );
    assert_eq!(resp.status(), StatusCode::CREATED);
}
