//! Live scores band: current scores of the journee (maquette ticker)

use super::model::LiveItem;
use dioxus::prelude::*;

#[component]
pub fn LiveScores(items: Vec<LiveItem>, journee: Option<i32>) -> Element {
    let title = match journee {
        Some(j) => format!("SCORES EN DIRECT — {j}e journée"),
        None => "SCORES EN DIRECT".to_string(),
    };

    rsx! {
        section { class: "pv-liveband",
            div { class: "pv-container",
                p { class: "pv-liveband-title", "{title}" }
                if items.is_empty() {
                    p { class: "pv-liveband-empty", "Aucun score disponible pour le moment." }
                } else {
                    div { class: "pv-livecards",
                        for item in items {
                            div { class: "pv-livecard",
                                div { class: "pv-livecard-teams",
                                    span { class: "pv-livecard-team", "{item.team1}" }
                                    span { class: "pv-livecard-score",
                                        "{item.score1.unwrap_or(0)} — {item.score2.unwrap_or(0)}"
                                    }
                                    span { class: "pv-livecard-team", "{item.team2}" }
                                }
                                p { class: if item.live { "pv-livecard-status live" } else { "pv-livecard-status" },
                                    "{item.status}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
