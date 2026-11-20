//! État d'authentification global (context Dioxus), persisté en localStorage.

use crate::services::api::{self, ApiService};
use dioxus::prelude::*;
use palet_shared::{LoginRequest, RegisterRequest, UserProfile};

#[derive(Clone, PartialEq, Default)]
pub struct AuthState {
    pub user: Option<UserProfile>,
    pub error: Option<String>,
}

impl AuthState {
    pub fn load() -> Self {
        Self {
            user: api::stored_user(),
            error: None,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        api::stored_access_token().is_some() && self.user.is_some()
    }
}

/// Fournit le signal global d'auth (appelé une fois dans App).
pub fn provide_auth() {
    use_context_provider(|| use_signal(AuthState::load));
}

/// Accès au signal global d'auth depuis n'importe quel composant.
pub fn use_auth() -> Signal<AuthState> {
    use_context::<Signal<AuthState>>()
}

pub async fn login(email: &str, password: &str) -> Result<(), String> {
    let mut auth = use_auth();
    let response = ApiService::new()
        .login(&LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        })
        .await?;
    api::save_session(&response.token, &response.user);
    auth.set(AuthState {
        user: Some(response.user),
        error: None,
    });
    Ok(())
}

pub async fn register(request: RegisterRequest) -> Result<(), String> {
    let mut auth = use_auth();
    let response = ApiService::new().register(&request).await?;
    api::save_session(&response.token, &response.user);
    auth.set(AuthState {
        user: Some(response.user),
        error: None,
    });
    Ok(())
}

pub async fn logout() {
    let mut auth = use_auth();
    if let Some(refresh) = api::stored_refresh_token() {
        let _ = ApiService::new().logout(&refresh).await;
    }
    api::clear_session();
    auth.set(AuthState::default());
}
