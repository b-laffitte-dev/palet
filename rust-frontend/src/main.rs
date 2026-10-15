//! Palet Vendéen Frontend - Dioxus Implementation
//!
//! Main entry point for the web application

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_web::launch;
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

/// Main entry point for WASM
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    // Initialize logging
    wasm_logger::init(wasm_logger::Config::default());

    info!("Starting Palet Vendéen Frontend with Dioxus...");

    // Launch the app
    launch(App);
}
