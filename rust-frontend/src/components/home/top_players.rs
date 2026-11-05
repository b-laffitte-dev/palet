//! Top of the championship section (maquette "Les meilleurs joueurs" cards,
//! fed with the leading teams of the classification — the only stats the
//! API exposes today)

use super::model::{Accent, BadgeKind, TopCard};
use dioxus::prelude::*;

#[component]
pub fn TopPlayers(cards: Vec<TopCard>) -> Element {
    rsx! {
        section { class: "pv-section",
            div { class: "pv-container",
                div { class: "pv-section-head",
                    h2 { class: "pv-section-title", "LE TOP DU CHAMPIONNAT" }
                    span { class: "pv-section-sub", "Statistiques de la saison" }
                }
                if cards.is_empty() {
                    p { class: "pv-empty", "Les statistiques arrivent dès les premiers matchs joués." }
                } else {
                    div { class: "pv-cards3",
                        for card in cards {
                            TopCardView { key: "{card.name}", card }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TopCardView(card: TopCard) -> Element {
    let accent_class = match card.accent {
        Accent::Gold => "pv-pcard-accent gold",
        Accent::Green => "pv-pcard-accent green",
        Accent::Amber => "pv-pcard-accent amber",
    };
    let badge_class = match card.badge_kind {
        BadgeKind::Mvp => "pv-pbadge mvp",
        BadgeKind::TopScorer => "pv-pbadge top",
        BadgeKind::Revelation => "pv-pbadge revelation",
    };

    rsx! {
        article { class: "pv-pcard",
            div { class: "{accent_class}" }
            div { class: "pv-pcard-top",
                div { class: "pv-avatar", "{card.initials}" }
                div {
                    h3 { class: "pv-pname", "{card.name}" }
                    p { class: "pv-pclub", "{card.club}" }
                    span { class: "{badge_class}", "{card.badge}" }
                }
            }
            div { class: "pv-pstats",
                for (label, value) in &card.stats {
                    p { class: "pv-pstat", "{label} : " b { "{value}" } }
                }
            }
        }
    }
}
