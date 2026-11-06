//! Championship standings table (maquette classification section)

use super::model::{StandingRow, Zone};
use dioxus::prelude::*;
use palet_shared::MatchResult;

#[component]
pub fn Standings(title: String, subtitle: String, rows: Vec<StandingRow>) -> Element {
    rsx! {
        section { class: "pv-section",
            div { class: "pv-container",
                div { class: "pv-section-head",
                    h2 { class: "pv-section-title", "{title}" }
                    span { class: "pv-section-sub", "{subtitle}" }
                    Link { to: "/championnats", class: "pv-section-link", "Tout le classement →" }
                }
                if rows.is_empty() {
                    p { class: "pv-empty", "Le classement sera disponible dès la première journée." }
                } else {
                    div { class: "pv-table-wrap",
                        table { class: "pv-table",
                            thead {
                                tr {
                                    th { class: "pv-th pos", "POS" }
                                    th { class: "pv-th club", "CLUB" }
                                    th { class: "pv-th num", "MJ" }
                                    th { class: "pv-th num", "G" }
                                    th { class: "pv-th num", "P" }
                                    th { class: "pv-th num", "DIFF" }
                                    th { class: "pv-th num", "PTS" }
                                    th { class: "pv-th form", "FORME" }
                                }
                            }
                            tbody {
                                for (i, row) in rows.iter().enumerate() {
                                    RowView { key: "{row.position}", row: row.clone(), alt: i % 2 == 0 }
                                }
                            }
                        }
                    }
                    div { class: "pv-legend",
                        span { class: "pv-legend-item",
                            span { class: "pv-legend-swatch qualif" }
                            "Finale"
                        }
                        span { class: "pv-legend-item",
                            span { class: "pv-legend-swatch barrage" }
                            "Barrage"
                        }
                        span { class: "pv-legend-item",
                            span { class: "pv-legend-swatch releg" }
                            "Descente D2"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RowView(row: StandingRow, alt: bool) -> Element {
    let zone_class = match row.zone {
        Zone::Qualif => "pv-row qualif",
        Zone::Barrage => "pv-row barrage",
        Zone::Releg => "pv-row releg",
        Zone::Safe => "pv-row",
    };
    let row_class = if alt && row.zone == Zone::Safe {
        format!("{zone_class} alt")
    } else {
        zone_class.to_string()
    };
    let diff = if row.diff >= 0 {
        format!("+{}", row.diff)
    } else {
        format!("{}", row.diff)
    };

    rsx! {
        tr { class: "{row_class}",
            td { class: "pv-td pos", "{row.position}" }
            td { class: "pv-td club",
                span { class: "pv-club-logo", "{row.abbr}" }
                span { class: "pv-club-name", "{row.club}" }
            }
            td { class: "pv-td num", "{row.played}" }
            td { class: "pv-td num", "{row.wins}" }
            td { class: "pv-td num", "{row.losses}" }
            td { class: "pv-td num", "{diff}" }
            td { class: "pv-td num pts", "{row.points}" }
            td { class: "pv-td form",
                div { class: "pv-form",
                    for result in &row.form {
                        span {
                            class: match result {
                                MatchResult::Win => "pv-form-badge win",
                                MatchResult::Draw => "pv-form-badge draw",
                                MatchResult::Loss => "pv-form-badge loss",
                            },
                        }
                    }
                }
            }
        }
    }
}
