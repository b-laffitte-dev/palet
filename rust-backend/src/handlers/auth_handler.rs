//! Auth endpoints : register / login / refresh / me / logout

use crate::middleware::auth::AuthUser;
use crate::services::auth_service::{AuthError, AuthService};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use palet_shared::{
    ApiResponse, AuthResponse, LoginRequest, RefreshTokenRequest, RegisterRequest, UserProfile,
};
use uuid::Uuid;

fn service(state: &web::Data<AppState>) -> AuthService {
    AuthService::new(state.db.users.clone(), &state.config)
}

/// Inscription d'un nouveau compte (auto-login : tokens renvoyés directement)
#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Compte créé", body = AuthResponse),
        (status = 400, description = "Données invalides"),
        (status = 409, description = "Email déjà utilisé"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn register(
    state: web::Data<AppState>,
    request: web::Json<RegisterRequest>,
) -> Result<impl Responder, AuthError> {
    let response = service(&state).register(request.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse {
        data: response,
        message: Some("Compte créé avec succès".to_string()),
        success: true,
    }))
}

/// Connexion email + mot de passe
#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Connecté", body = AuthResponse),
        (status = 401, description = "Identifiants invalides"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn login(
    state: web::Data<AppState>,
    request: web::Json<LoginRequest>,
) -> Result<impl Responder, AuthError> {
    let response = service(&state).login(request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        data: response,
        message: Some("Connexion réussie".to_string()),
        success: true,
    }))
}

/// Rotation du refresh token (l'ancien est révoqué)
#[utoipa::path(
    post,
    path = "/auth/refresh",
    tag = "Auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Tokens renouvelés", body = AuthResponse),
        (status = 401, description = "Refresh token invalide, expiré ou révoqué"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn refresh(
    state: web::Data<AppState>,
    request: web::Json<RefreshTokenRequest>,
) -> Result<impl Responder, AuthError> {
    let response = service(&state).refresh(request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        data: response,
        message: Some("Tokens renouvelés".to_string()),
        success: true,
    }))
}

/// Profil de l'utilisateur connecté (JWT access requis)
#[utoipa::path(
    get,
    path = "/auth/me",
    tag = "Auth",
    responses(
        (status = 200, description = "Profil courant", body = UserProfile),
        (status = 401, description = "Non authentifié"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn me(auth: AuthUser, state: web::Data<AppState>) -> Result<impl Responder, AuthError> {
    let user_id = Uuid::parse_str(&auth.0.sub)
        .map_err(|_| AuthError::Unauthorized("Claim sub invalide".to_string()))?;
    let profile = service(&state).me(user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        data: profile,
        message: Some("Profil récupéré".to_string()),
        success: true,
    }))
}

/// Révocation d'un refresh token (déconnexion)
#[utoipa::path(
    post,
    path = "/auth/logout",
    tag = "Auth",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Déconnecté", body = bool),
        (status = 401, description = "Non authentifié"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn logout(
    _auth: AuthUser,
    state: web::Data<AppState>,
    request: web::Json<RefreshTokenRequest>,
) -> Result<impl Responder, AuthError> {
    service(&state).logout(&request.refresh_token).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        data: true,
        message: Some("Déconnexion effectuée".to_string()),
        success: true,
    }))
}
