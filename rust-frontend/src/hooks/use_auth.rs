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
///
/// Ne pas appeler de hook (`use_signal`) dans la closure de
/// `use_context_provider` : en Dioxus 0.7, `use_hook` garde la liste des
/// hooks empruntée pendant l'initialiseur → BorrowMutError au premier
/// rendu (page noire).
pub fn provide_auth() {
    let auth = use_signal(AuthState::load);
    use_context_provider(|| auth);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[component]
    fn AuthProbe() -> Element {
        let auth = use_auth();
        let connecte = auth.read().is_authenticated();
        rsx! { div { id: "auth-probe", "connecté:{connecte}" } }
    }

    #[component]
    fn TestApp() -> Element {
        provide_auth();
        rsx! { AuthProbe {} }
    }

    /// Le provider d'auth doit rendre sans panic et fournir le contexte
    /// aux composants enfants (régression : hook dans l'initialiseur de
    /// `use_context_provider` → BorrowMutError → page noire).
    #[test]
    fn provide_auth_fournit_le_contexte_sans_panic() {
        let mut dom = VirtualDom::new(TestApp);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("auth-probe"), "rendu vide: {html}");
        assert!(html.contains("connecté:false"), "rendu inattendu: {html}");
    }
}
