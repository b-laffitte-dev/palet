//! Palet Vendéen Mobile - Dioxus Implementation
//!
//! Main entry point for the mobile application (iOS and Android)

use dioxus::prelude::*;
use dioxus_router::*;
use log::info;

mod components;
mod hooks;
mod pages;
mod services;
mod utils;

use pages::{
    actualites::Actualites, championship_detail::ChampionnatDetail, championships::Championnats,
    club_detail::ClubDetail, clubs::Clubs, connexion::Connexion, federation::Federation,
    home::Home, inscription::Inscription, not_found::NotFound, player_detail::PlayerDetail,
    players::Joueurs, tournament_detail::TournamentDetail, tournaments::Tournois,
};

/// Route definitions
#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home,
    #[route("/championnats")]
    Championnats,
    #[route("/championnats/:id")]
    ChampionnatDetail { id: String },
    #[route("/tournois")]
    Tournois,
    #[route("/tournois/:id")]
    TournamentDetail { id: String },
    #[route("/clubs")]
    Clubs,
    #[route("/clubs/:id")]
    ClubDetail { id: String },
    #[route("/joueurs")]
    Joueurs,
    #[route("/joueurs/:id")]
    PlayerDetail { id: String },
    #[route("/actualites")]
    Actualites,
    #[route("/federation")]
    Federation,
    #[route("/connexion")]
    Connexion,
    #[route("/inscription")]
    Inscription,
    #[route("/404")]
    NotFound,
}

/// Main App component
#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

/// Main entry point for mobile
#[no_mangle]
pub fn dioxus_main() {
    // Initialize logging
    use dioxus_desktop::launch;

    info!("Starting Palet Vendéen Mobile with Dioxus...");

    // Launch the app
    launch(App);
}

/// Standard main function for binary builds
#[cfg(not(target_os = "android"))]
#[cfg(not(target_os = "ios"))]
fn main() {
    dioxus_main();
}
