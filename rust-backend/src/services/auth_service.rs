//! Service d'authentification : inscription, connexion, refresh (rotation),
//! profil courant. JWT HS256 en access token (stateless), refresh token opaque
//! stocké et révocable en base. Mots de passe hashés en Argon2id.

use crate::config::AppConfig;
use crate::db::DatabaseError;
use crate::models::user::{NewUser, UserAccount};
use crate::repositories::user_repository::UserRepository;
use actix_web::{http::StatusCode, HttpResponse};
use argon2::password_hash::phc::{PasswordHash, SaltString};
use argon2::password_hash::{PasswordHasher, PasswordVerifier};
use argon2::Argon2;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use palet_shared::{
    ApiResponse, AuthResponse, AuthToken, LoginRequest, RefreshTokenRequest, RegisterRequest,
    UserProfile, UserRole, UserStatus,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

pub const ACCESS_TOKEN_TYPE: &str = "access";

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("{0}")]
    Validation(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    Internal(String),
}

impl actix_web::ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::Validation(_) => StatusCode::BAD_REQUEST,
            AuthError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AuthError::Forbidden(_) => StatusCode::FORBIDDEN,
            AuthError::Conflict(_) => StatusCode::CONFLICT,
            AuthError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ApiResponse::<()> {
            data: (),
            message: Some(self.to_string()),
            success: false,
        })
    }
}

impl From<DatabaseError> for AuthError {
    fn from(e: DatabaseError) -> Self {
        match e {
            DatabaseError::ValidationError(msg) => AuthError::Conflict(msg),
            other => AuthError::Internal(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub role: UserRole,
    pub club_id: Option<String>,
    pub token_type: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate();
    Argon2::default()
        .hash_password_with_salt(password.as_bytes(), salt.as_bytes())
        .map(|h| h.to_string())
        .map_err(|e| AuthError::Internal(format!("hash du mot de passe : {e}")))
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok(),
        Err(_) => false,
    }
}

pub fn create_token(secret: &str, claims: &Claims) -> Result<String, AuthError> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AuthError::Internal(format!("création du JWT : {e}")))
}

pub fn validate_token(secret: &str, token: &str, expected_type: &str) -> Result<Claims, AuthError> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AuthError::Unauthorized("Token invalide ou expiré".to_string()))?;
    if data.claims.token_type != expected_type {
        return Err(AuthError::Unauthorized(
            "Type de token inattendu".to_string(),
        ));
    }
    Ok(data.claims)
}

pub struct AuthService {
    users: Arc<dyn UserRepository>,
    jwt_secret: String,
    access_exp_secs: i64,
    refresh_exp_days: i64,
}

impl AuthService {
    pub fn new(users: Arc<dyn UserRepository>, config: &AppConfig) -> Self {
        Self {
            users,
            jwt_secret: config.jwt_secret.clone(),
            access_exp_secs: (config.jwt_expiration_hours as i64) * 3600,
            refresh_exp_days: config.jwt_refresh_expiration_days as i64,
        }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<AuthResponse, AuthError> {
        let email = request.validated().map_err(AuthError::Validation)?;
        if self.users.find_by_email(&email).await?.is_some() {
            return Err(AuthError::Conflict(format!(
                "Un compte existe déjà pour {email}"
            )));
        }
        let password_hash = hash_password(&request.password)?;
        let user = self
            .users
            .create_user(NewUser {
                email,
                password_hash,
                first_name: request.first_name.trim().to_string(),
                last_name: request.last_name.trim().to_string(),
                role: UserRole::Player,
                club_id: request.club_id,
                license_number: request.license_number,
            })
            .await?;
        let token = self.issue_tokens(&user).await?;
        Ok(AuthResponse {
            token,
            user: Self::profile_of(&user),
        })
    }

    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse, AuthError> {
        let email = request.email.trim().to_lowercase();
        let user = self
            .users
            .find_by_email(&email)
            .await?
            .ok_or_else(|| AuthError::Unauthorized("Identifiants invalides".to_string()))?;
        if !verify_password(&request.password, &user.password_hash) {
            return Err(AuthError::Unauthorized(
                "Identifiants invalides".to_string(),
            ));
        }
        self.ensure_active(&user)?;
        self.users.update_last_login(user.id).await?;
        let token = self.issue_tokens(&user).await?;
        Ok(AuthResponse {
            token,
            user: Self::profile_of(&user),
        })
    }

    pub async fn refresh(&self, request: RefreshTokenRequest) -> Result<AuthResponse, AuthError> {
        let stored = self
            .users
            .find_refresh_token(&request.refresh_token)
            .await?
            .ok_or_else(|| AuthError::Unauthorized("Refresh token invalide".to_string()))?;
        if !stored.is_usable(&Utc::now()) {
            return Err(AuthError::Unauthorized(
                "Refresh token expiré ou révoqué".to_string(),
            ));
        }
        let user = self
            .users
            .find_by_id(stored.user_id)
            .await?
            .ok_or_else(|| AuthError::Unauthorized("Compte introuvable".to_string()))?;
        self.ensure_active(&user)?;
        self.users
            .revoke_refresh_token(&request.refresh_token)
            .await?;
        let token = self.issue_tokens(&user).await?;
        Ok(AuthResponse {
            token,
            user: Self::profile_of(&user),
        })
    }

    pub async fn me(&self, user_id: Uuid) -> Result<UserProfile, AuthError> {
        let user = self
            .users
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AuthError::Unauthorized("Compte introuvable".to_string()))?;
        self.ensure_active(&user)?;
        Ok(Self::profile_of(&user))
    }

    pub async fn logout(&self, refresh_token: &str) -> Result<(), AuthError> {
        self.users.revoke_refresh_token(refresh_token).await?;
        Ok(())
    }

    fn ensure_active(&self, user: &UserAccount) -> Result<(), AuthError> {
        if user.status != UserStatus::Active {
            return Err(AuthError::Unauthorized("Compte désactivé".to_string()));
        }
        Ok(())
    }

    async fn issue_tokens(&self, user: &UserAccount) -> Result<AuthToken, AuthError> {
        let now = Utc::now();
        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role: user.role.clone(),
            club_id: user.club_id.map(|c| c.to_string()),
            token_type: ACCESS_TOKEN_TYPE.to_string(),
            iat: now.timestamp(),
            exp: (now + Duration::seconds(self.access_exp_secs)).timestamp(),
        };
        let access_token = create_token(&self.jwt_secret, &claims)?;
        let refresh_token = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
        let expires_at = now + Duration::days(self.refresh_exp_days);
        self.users
            .create_refresh_token(user.id, refresh_token.clone(), expires_at)
            .await?;
        Ok(AuthToken {
            access_token,
            refresh_token,
            expires_in: self.access_exp_secs,
            token_type: "Bearer".to_string(),
        })
    }

    pub fn profile_of(user: &UserAccount) -> UserProfile {
        UserProfile {
            id: user.id,
            email: user.email.clone(),
            username: user.username.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            role: user.role.clone(),
            club_id: user.club_id,
            license_number: user.license_number.clone(),
            category: user.category.clone(),
            status: user.status.clone(),
            email_verified: user.email_verified,
            created_at: user.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::{NewUser, RefreshToken};
    use async_trait::async_trait;
    use chrono::DateTime;
    use std::sync::Mutex;

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
        async fn find_by_license(
            &self,
            license: &str,
        ) -> Result<Option<UserAccount>, DatabaseError> {
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

        async fn update_last_login(&self, _id: Uuid) -> Result<(), DatabaseError> {
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

        async fn find_refresh_token(
            &self,
            token: &str,
        ) -> Result<Option<RefreshToken>, DatabaseError> {
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

    fn service() -> AuthService {
        AuthService::new(Arc::new(MockUserRepository::new()), &AppConfig::default())
    }

    fn register_request() -> RegisterRequest {
        RegisterRequest {
            email: "jean.dupont@example.fr".to_string(),
            password: "motdepasse-solide".to_string(),
            first_name: "Jean".to_string(),
            last_name: "Dupont".to_string(),
            club_id: None,
            license_number: None,
        }
    }

    #[test]
    fn hash_et_verify_password_round_trip() {
        let hash = hash_password("motdepasse-solide").expect("hash");
        assert_ne!(hash, "motdepasse-solide");
        assert!(verify_password("motdepasse-solide", &hash));
        assert!(!verify_password("mauvais-mot-de-passe", &hash));
        assert!(!verify_password("motdepasse-solide", "hash-invalide"));
    }

    #[test]
    fn jwt_round_trip_et_rejets() {
        let secret = "secret-de-test-suffisamment-long-pour-hs256";
        let claims = Claims {
            sub: Uuid::new_v4().to_string(),
            email: "a@b.fr".to_string(),
            role: UserRole::Player,
            club_id: None,
            token_type: ACCESS_TOKEN_TYPE.to_string(),
            iat: Utc::now().timestamp(),
            exp: (Utc::now() + Duration::hours(1)).timestamp(),
        };
        let token = create_token(secret, &claims).expect("token");
        let back = validate_token(secret, &token, ACCESS_TOKEN_TYPE).expect("valide");
        assert_eq!(back.sub, claims.sub);
        assert!(validate_token("autre-secret", &token, ACCESS_TOKEN_TYPE).is_err());
        assert!(validate_token(secret, &token, "refresh").is_err());

        let expired = Claims {
            exp: (Utc::now() - Duration::hours(1)).timestamp(),
            ..claims.clone()
        };
        let expired_token = create_token(secret, &expired).expect("token");
        assert!(validate_token(secret, &expired_token, ACCESS_TOKEN_TYPE).is_err());
    }

    #[actix_web::test]
    async fn register_cree_un_compte_player_et_des_tokens() {
        let service = service();
        let response = service
            .register(register_request())
            .await
            .expect("register");
        assert_eq!(response.user.role, UserRole::Player);
        assert_eq!(response.user.email, "jean.dupont@example.fr");
        assert!(!response.token.access_token.is_empty());
        assert!(!response.token.refresh_token.is_empty());
        assert_eq!(response.token.token_type, "Bearer");
    }

    #[actix_web::test]
    async fn register_rejette_email_invalide_et_doublon() {
        let service = service();
        let mut bad = register_request();
        bad.email = "pas-un-email".to_string();
        assert!(matches!(
            service.register(bad).await,
            Err(AuthError::Validation(_))
        ));

        service
            .register(register_request())
            .await
            .expect("register");
        assert!(matches!(
            service.register(register_request()).await,
            Err(AuthError::Conflict(_))
        ));
    }

    #[actix_web::test]
    async fn login_ok_mauvais_mot_de_passe_et_inconnu() {
        let service = service();
        service
            .register(register_request())
            .await
            .expect("register");

        let ok = service
            .login(LoginRequest {
                email: "JEAN.DUPONT@example.fr".to_string(),
                password: "motdepasse-solide".to_string(),
            })
            .await
            .expect("login");
        assert_eq!(ok.user.first_name, "Jean");

        let wrong = LoginRequest {
            email: "jean.dupont@example.fr".to_string(),
            password: "mauvais".to_string(),
        };
        assert!(matches!(
            service.login(wrong).await,
            Err(AuthError::Unauthorized(_))
        ));

        let unknown = LoginRequest {
            email: "inconnu@example.fr".to_string(),
            password: "motdepasse-solide".to_string(),
        };
        assert!(matches!(
            service.login(unknown).await,
            Err(AuthError::Unauthorized(_))
        ));
    }

    #[actix_web::test]
    async fn refresh_fait_la_rotation_et_revoque_l_ancien_token() {
        let service = service();
        let registered = service
            .register(register_request())
            .await
            .expect("register");

        let refreshed = service
            .refresh(RefreshTokenRequest {
                refresh_token: registered.token.refresh_token.clone(),
            })
            .await
            .expect("refresh");
        assert_ne!(
            refreshed.token.refresh_token,
            registered.token.refresh_token
        );

        let reuse = RefreshTokenRequest {
            refresh_token: registered.token.refresh_token,
        };
        assert!(matches!(
            service.refresh(reuse).await,
            Err(AuthError::Unauthorized(_))
        ));

        let unknown = RefreshTokenRequest {
            refresh_token: "token-inexistant".to_string(),
        };
        assert!(matches!(
            service.refresh(unknown).await,
            Err(AuthError::Unauthorized(_))
        ));
    }

    #[actix_web::test]
    async fn me_renvoie_le_profil_ou_401_si_inconnu() {
        let service = service();
        let registered = service
            .register(register_request())
            .await
            .expect("register");
        let profile = service.me(registered.user.id).await.expect("me");
        assert_eq!(profile.email, "jean.dupont@example.fr");
        assert!(matches!(
            service.me(Uuid::new_v4()).await,
            Err(AuthError::Unauthorized(_))
        ));
    }

    #[actix_web::test]
    async fn logout_revoque_le_refresh_token() {
        let service = service();
        let registered = service
            .register(register_request())
            .await
            .expect("register");
        service
            .logout(&registered.token.refresh_token)
            .await
            .expect("logout");
        assert!(matches!(
            service
                .refresh(RefreshTokenRequest {
                    refresh_token: registered.token.refresh_token,
                })
                .await,
            Err(AuthError::Unauthorized(_))
        ));
    }
}
