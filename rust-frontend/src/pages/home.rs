//! Home page for Palet Vendéen
//!
//! Layout follows `docs/maquettes/homepage.png`: top bar, header, live match
//! hero, live scores band, standings, upcoming events, top of the
//! championship, spaces and footer. All dynamic sections are fed from the
//! backend API through [`ApiService`] (see [`HomeData::load`]).

use crate::components::home::{
    HeroSection, HomeData, HomeFooter, LiveScores, SiteHeader, Spaces, Standings, TopBar,
    TopPlayers, Upcoming,
};
use crate::services::api::ApiService;
use dioxus::prelude::*;

/// Home page component
#[component]
pub fn Home() -> Element {
    let data = use_resource(|| {
        let api = ApiService::new();
        async move { HomeData::load(&api).await }
    });

    rsx! {
        div { class: "pv-page",
            TopBar {}
            SiteHeader {}
            main {
                if let Some(home) = &*data.read() {
                    HeroSection { hero: home.hero.clone() }
                    LiveScores { items: home.live.clone(), journee: home.journee }
                    Standings {
                        title: home.championship_name.clone(),
                        subtitle: home.subtitle.clone(),
                        rows: home.standings.clone(),
                    }
                    Upcoming { events: home.events.clone() }
                    TopPlayers { cards: home.top.clone() }
                } else {
                    div { class: "pv-loading",
                        p { "Chargement des données…" }
                    }
                }
                Spaces {}
            }
            HomeFooter {}
        }
    }
}
