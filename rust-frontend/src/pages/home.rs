//! Home page for Palet Vendéen
//!
//! Layout follows `docs/maquettes/homepage.png`: top bar, header, live match
//! hero, journee carousel (matches by division), standings, upcoming events,
//! top of the championship, spaces and footer. All dynamic sections are fed
//! from the backend API through [`ApiService`] (see [`HomeData::load`]).

use crate::components::home::model::{hero_from_carousel, CarouselSelection};
use crate::components::home::{
    HeroSection, HomeData, HomeFooter, JourneeCarousel, SiteHeader, Spaces, Standings, TopBar,
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
    // Active standings tab (division name); falls back to the first tab.
    let mut active = use_signal(String::new);
    // Match sélectionné dans le carrousel des journées en cours : affiché
    // dans le hero à la place du match en vedette par défaut.
    let mut selection = use_signal(|| Option::<CarouselSelection>::None);

    let active_name = {
        let selected = active.read().clone();
        let tabs = data
            .read()
            .as_ref()
            .map(|h| h.tabs.clone())
            .unwrap_or_default();
        if tabs.iter().any(|t| t.label == selected) {
            selected
        } else {
            tabs.first().map(|t| t.label.clone()).unwrap_or_default()
        }
    };

    // Hero : match sélectionné dans le carrousel, sinon match en vedette.
    let hero = match &*selection.read() {
        Some(sel) => Some(hero_from_carousel(sel)),
        None => data.read().as_ref().and_then(|h| h.hero.clone()),
    };

    rsx! {
        div { class: "pv-page",
            TopBar {}
            SiteHeader {}
            main {
                if let Some(home) = &*data.read() {
                    JourneeCarousel {
                        slides: home.slides.clone(),
                        on_select: move |sel: CarouselSelection| selection.set(Some(sel)),
                    }
                    HeroSection { hero }
                    Standings {
                        title: home.championship_name.clone(),
                        subtitle: home.subtitle.clone(),
                        tabs: home.tabs.clone(),
                        active: active_name.clone(),
                        on_select: move |name: String| active.set(name),
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
