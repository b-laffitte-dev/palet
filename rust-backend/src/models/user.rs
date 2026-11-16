//! Modèles de persistance pour les comptes utilisateurs (table `users`) et les
//! refresh tokens (table `refresh_tokens`). Le hash de mot de passe ne sort
//! jamais de ce module : l'API n'expose que `palet_shared::UserProfile`.

use chrono::{DateTime, Utc};
use palet_shared::{UserRole, UserStatus};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct UserAccount {
    pub id: Uuid,
    pub email: String,
    pub username: Option<String>,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub club_id: Option<Uuid>,
    pub license_number: Option<String>,
    pub category: String,
    pub status: UserStatus,
    pub email_verified: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Données de création d'un compte (le hash est déjà calculé par le service).
#[derive(Debug, Clone)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub club_id: Option<Uuid>,
    pub license_number: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl RefreshToken {
    pub fn is_usable(&self, now: &DateTime<Utc>) -> bool {
        self.revoked_at.is_none() && &self.expires_at > now
    }
}
