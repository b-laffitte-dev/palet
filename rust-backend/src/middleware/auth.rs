//! Extracteurs Actix pour protéger les routes :
//! - `AuthUser` : JWT access valide (n'importe quel rôle connecté)
//! - `AdminUser` : rôle commission_admin ou super_admin (espace fédéral)
//! - `OfficialUser` : arbitre, commission_admin ou super_admin (saisie des matchs)

use crate::services::auth_service::{validate_token, AuthError, Claims, ACCESS_TOKEN_TYPE};
use crate::AppState;
use actix_web::dev::Payload;
use actix_web::http::header;
use actix_web::{web, Error, FromRequest, HttpRequest};
use palet_shared::UserRole;
use std::future::{ready, Ready};
use uuid::Uuid;

pub fn extract_access_claims(req: &HttpRequest) -> Result<Claims, AuthError> {
    let Some(state) = req.app_data::<web::Data<AppState>>() else {
        return Err(AuthError::Internal(
            "État de l'application indisponible".to_string(),
        ));
    };
    let Some(auth_header) = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
    else {
        return Err(AuthError::Unauthorized(
            "En-tête Authorization manquant".to_string(),
        ));
    };
    let Some(token) = auth_header.strip_prefix("Bearer ") else {
        return Err(AuthError::Unauthorized(
            "Format attendu : Bearer <token>".to_string(),
        ));
    };
    validate_token(&state.config.jwt_secret, token, ACCESS_TOKEN_TYPE)
        .map_err(|_| AuthError::Unauthorized("Token invalide ou expiré".to_string()))
}

pub fn is_admin(role: &UserRole) -> bool {
    matches!(role, UserRole::CommissionAdmin | UserRole::SuperAdmin)
}

pub fn is_match_official(role: &UserRole) -> bool {
    matches!(
        role,
        UserRole::Arbitre | UserRole::CommissionAdmin | UserRole::SuperAdmin
    )
}

/// Gestionnaire de club : club_admin (périmètre = son club_id, vérifié dans le
/// handler) ou administrateur.
pub fn is_club_manager(role: &UserRole) -> bool {
    matches!(
        role,
        UserRole::ClubAdmin | UserRole::CommissionAdmin | UserRole::SuperAdmin
    )
}

/// ClubAdmin/admin : autorisé à gérer les joueurs/équipes d'un club.
/// Le handler vérifie que `claims.club_id` correspond au club concerné.
pub struct ClubManagerUser(pub Claims);

impl FromRequest for ClubManagerUser {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(match extract_access_claims(req) {
            Ok(claims) if is_club_manager(&claims.role) => Ok(ClubManagerUser(claims)),
            Ok(_) => Err(Error::from(AuthError::Forbidden(
                "Rôle responsable de club ou administrateur requis".to_string(),
            ))),
            Err(e) => Err(Error::from(e)),
        })
    }
}

/// Vérifie le périmètre club d'un club_admin : admins passent toujours,
/// un club_admin doit agir sur son propre club.
pub fn check_club_scope(claims: &Claims, club_id: Uuid) -> Result<(), AuthError> {
    if is_admin(&claims.role) {
        return Ok(());
    }
    match claims
        .club_id
        .as_deref()
        .and_then(|c| Uuid::parse_str(c).ok())
    {
        Some(own) if own == club_id => Ok(()),
        _ => Err(AuthError::Forbidden(
            "Hors de votre club : opération réservée au responsable de ce club".to_string(),
        )),
    }
}

/// Utilisateur authentifié (JWT access valide), quel que soit son rôle.
pub struct AuthUser(pub Claims);

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(
            extract_access_claims(req)
                .map(AuthUser)
                .map_err(Error::from),
        )
    }
}

/// Administrateur départemental/national (création clubs, championnats, divisions).
pub struct AdminUser(pub Claims);

impl FromRequest for AdminUser {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(match extract_access_claims(req) {
            Ok(claims) if is_admin(&claims.role) => Ok(AdminUser(claims)),
            Ok(_) => Err(Error::from(AuthError::Forbidden(
                "Rôle administrateur (commission) requis".to_string(),
            ))),
            Err(e) => Err(Error::from(e)),
        })
    }
}

/// Arbitre ou administrateur : autorisé à créer/modifier/saisir des matchs.
pub struct OfficialUser(pub Claims);

impl FromRequest for OfficialUser {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(match extract_access_claims(req) {
            Ok(claims) if is_match_official(&claims.role) => Ok(OfficialUser(claims)),
            Ok(_) => Err(Error::from(AuthError::Forbidden(
                "Rôle arbitre ou administrateur requis".to_string(),
            ))),
            Err(e) => Err(Error::from(e)),
        })
    }
}
