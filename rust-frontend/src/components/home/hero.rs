//! Hero: featured match display (Top 14 style), fed by the live API match

use super::model::HeroMatch;
use dioxus::prelude::*;

#[component]
pub fn HeroSection(hero: Option<HeroMatch>) -> Element {
    let Some(hero) = hero else {
        return rsx! {
            section { class: "pv-hero",
                div { class: "pv-container pv-hero-empty",
                    p { class: "pv-hero-kicker", "PALET VENDÉEN" }
                    p { class: "pv-hero-sub", "Aucun match en direct pour le moment — retrouvez les prochains matchs dans la rubrique à venir." }
                }
            }
        };
    };

    let badge_class = if hero.live {
        "pv-live-badge live"
    } else if hero.finished {
        "pv-live-badge finished"
    } else {
        "pv-live-badge upcoming"
    };

    rsx! {
        section { class: "pv-hero",
            div { class: "pv-container",
                p { class: "pv-hero-kicker", "{hero.kicker}" }
                p { class: "pv-hero-sub", "{hero.subtitle}" }
                div { class: "pv-match",
                    TeamBlock { side: hero.team1.clone() }
                    div { class: "pv-scorebox",
                        div { class: "pv-score-row",
                            span { class: "pv-score", "{hero.team1.score.unwrap_or(0)}" }
                            span { class: "pv-score-sep" }
                            span { class: "pv-score gold", "{hero.team2.score.unwrap_or(0)}" }
                        }
                        span { class: "{badge_class}", "{hero.badge}" }
                    }
                    TeamBlock { side: hero.team2.clone() }
                }
                p { class: "pv-hero-info", "{hero.info}" }
            }
        }
    }
}

#[component]
fn TeamBlock(side: super::model::TeamSide) -> Element {
    rsx! {
        div { class: "pv-team",
            div { class: "pv-team-badge",
                span { class: "pv-team-abbr", "{side.abbr}" }
            }
            p { class: "pv-team-name", "{side.name}" }
            p { class: "pv-team-sub", "{side.subtitle}" }
        }
    }
}
