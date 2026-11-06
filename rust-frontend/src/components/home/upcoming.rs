//! Upcoming events section (maquette "Tournois à venir" cards)

use super::model::{EventCard, EventTone};
use dioxus::prelude::*;

#[component]
pub fn Upcoming(events: Vec<EventCard>) -> Element {
    rsx! {
        section { class: "pv-section pv-section-alt",
            div { class: "pv-container",
                div { class: "pv-section-head",
                    h2 { class: "pv-section-title", "MATCHS & TOURNOIS À VENIR" }
                    span { class: "pv-section-sub", "Les prochaines rencontres programmées" }
                }
                if events.is_empty() {
                    p { class: "pv-empty", "Aucune rencontre programmée pour le moment." }
                } else {
                    div { class: "pv-cards3",
                        for event in events {
                            EventCardView { key: "{event.title}", event }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EventCardView(event: EventCard) -> Element {
    let head_class = match event.tone {
        EventTone::Open => "pv-tcard-head open",
        EventTone::Championship => "pv-tcard-head championship",
        EventTone::Federal => "pv-tcard-head federal",
    };

    rsx! {
        article { class: "pv-tcard",
            div { class: "{head_class}",
                div {
                    p { class: "pv-tag", "{event.tag}" }
                    h3 { class: "pv-tcard-title", "{event.title}" }
                }
                span { class: "pv-pill", "{event.pill}" }
            }
            div { class: "pv-tcard-body",
                p { class: "pv-detail strong", "{event.date}" }
                p { class: "pv-detail", "{event.location}" }
                for detail in &event.details {
                    p { class: "pv-detail", "{detail}" }
                }
                Link { to: event.href.clone(), class: "pv-btn-dark", "{event.action}" }
            }
        }
    }
}
