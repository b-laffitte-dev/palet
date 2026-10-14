//! Palet Vendéen Mobile - Dioxus Implementation
//!
//! Main entry point for the mobile application (iOS and Android)

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_mobile::launch;
use log::info;

mod components;
mod pages;
mod services;
mod hooks;
mod utils;

use pages::{
    home::Home, clubs::Clubs, club_detail::ClubDetail,
    players::Joueurs, player_detail::PlayerDetail,
    tournaments::Tournois, tournament_detail::TournamentDetail,
    championships::Championnats, championship_detail::ChampionnatDetail,
    federation::Federation, actualites::Actualites,
    connexion::Connexion, inscription::Inscription, not_found::NotFound,
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
    dioxus_mobile::init_logging();
    
    info!("Starting Palet Vendéen Mobile with Dioxus...");
    
    // Launch the app
    launch(App);
}
