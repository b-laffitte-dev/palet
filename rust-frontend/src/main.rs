//! Palet Vendéen Frontend - Rust/Yew Implementation
//!
//! Main entry point for the web application

use log::info;
use palet_shared::{ApiResponse, Championship, Club, Federation, Match, Player, Tournament};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

// Import all components and pages
mod components;
mod hooks;
mod pages;
mod services;
mod types;
mod utils;

use components::{common::*, layout::Layout};
use pages::{
    actualites::Actualites, championship_detail::ChampionshipDetail, championships::Championships,
    club_detail::ClubDetail, clubs::Clubs, connexion::Connexion, federation::Federation,
    home::Home, inscription::Inscription, not_found::NotFound, player_detail::PlayerDetail,
    players::Players, tournament_detail::TournamentDetail, tournaments::Tournaments,
};
use services::api::ApiService;

// Route definitions
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/championnats")]
    Championnats,
    #[at("/championnats/:id")]
    ChampionnatDetail { id: String },
    #[at("/tournois")]
    Tournois,
    #[at("/tournois/:id")]
    TournamentDetail { id: String },
    #[at("/clubs")]
    Clubs,
    #[at("/clubs/:id")]
    ClubDetail { id: String },
    #[at("/joueurs")]
    Joueurs,
    #[at("/joueurs/:id")]
    PlayerDetail { id: String },
    #[at("/actualites")]
    Actualites,
    #[at("/federation")]
    Federation,
    #[at("/connexion")]
    Connexion,
    #[at("/inscription")]
    Inscription,
    #[at("/404")]
    NotFound,
}

impl Route {
    /// Get the current route as a string
    pub fn to_string(&self) -> String {
        match self {
            Route::Home => "/".to_string(),
            Route::Championnats => "/championnats".to_string(),
            Route::ChampionnatDetail { id } => format!("/championnats/{}", id),
            Route::Tournois => "/tournois".to_string(),
            Route::TournamentDetail { id } => format!("/tournois/{}", id),
            Route::Clubs => "/clubs".to_string(),
            Route::ClubDetail { id } => format!("/clubs/{}", id),
            Route::Joueurs => "/joueurs".to_string(),
            Route::PlayerDetail { id } => format!("/joueurs/{}", id),
            Route::Actualites => "/actualites".to_string(),
            Route::Federation => "/federation".to_string(),
            Route::Connexion => "/connexion".to_string(),
            Route::Inscription => "/inscription".to_string(),
            Route::NotFound => "/404".to_string(),
            Route::NotFoundRoute => "/404".to_string(),
        }
    }
}

/// Switch component for routing
fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Championnats => html! { <Championships /> },
        Route::ChampionnatDetail { id } => html! { <ChampionshipDetail id={id.clone()} /> },
        Route::Tournois => html! { <Tournaments /> },
        Route::TournamentDetail { id } => html! { <TournamentDetail id={id.clone()} /> },
        Route::Clubs => html! { <Clubs /> },
        Route::ClubDetail { id } => html! { <ClubDetail id={id.clone()} /> },
        Route::Joueurs => html! { <Players /> },
        Route::PlayerDetail { id } => html! { <PlayerDetail id={id.clone()} /> },
        Route::Actualites => html! { <Actualites /> },
        Route::Federation => html! { <Federation /> },
        Route::Connexion => html! { <Connexion /> },
        Route::Inscription => html! { <Inscription /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

/// Main App component
#[function_component(App)]
pub fn app() -> Html {
    let api_service = use_memo(|_| ApiService::new(), ());

    // Provide API service via context
    let api_context = use_context_provider(|| api_service);

    html! {
        <BrowserRouter>
            <Layout>
                <Switch<Route> render={switch} />
            </Layout>
        </BrowserRouter>
    }
}

/// Main entry point for WASM
#[wasm_bindgen(start)]
pub fn run_app() {
    // Initialize logging
    wasm_logger::init(wasm_logger::Config::default());

    info!("Starting Palet Vendéen Frontend...");

    // Mount the app
    yew::Renderer::<App>::new().render();
}

// Re-export for testing
#[cfg(test)]
pub use pages::*;
