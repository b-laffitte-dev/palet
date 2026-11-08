//! Championship standings section with one tab per division.
//!
//! Divisions are dynamic: `tabs` is built from ALL the divisions registered
//! in the federal space (via `GET /divisions`, every league), each tab
//! showing the classification of its division.

use super::model::{DivisionTab, StandingRow, Zone};
use dioxus::prelude::*;
use palet_shared::MatchResult;

#[component]
pub fn Standings(
    title: String,
    subtitle: String,
    tabs: Vec<DivisionTab>,
    active: String,
    on_select: EventHandler<String>,
) -> Element {
    // Fall back to the first tab when `active` matches none (initial load).
    let current = tabs
        .iter()
        .find(|t| t.label == active)
        .or_else(|| tabs.first())
        .cloned();

    rsx! {
        section { class: "pv-section",
            div { class: "pv-container",
                div { class: "pv-section-head",
                    h2 { class: "pv-section-title", "{title}" }
                    span { class: "pv-section-sub", "{subtitle}" }
                    Link { to: "/championnats", class: "pv-section-link", "Tout le classement →" }
                }
                if tabs.is_empty() {
                    p { class: "pv-empty", "Aucune division configurée pour le moment." }
                } else {
                    div { class: "pv-tabs", role: "tablist",
                        for tab in &tabs {
                            TabButton {
                                key: "{tab.league}::{tab.name}",
                                name: tab.label.clone(),
                                active: current.as_ref().is_some_and(|c| c.label == tab.label),
                                on_select: on_select,
                            }
                        }
                    }
                    match &current {
                        Some(tab) if !tab.rows.is_empty() => rsx! {
                            StandingsTable { rows: tab.rows.clone() }
                        },
                        Some(_) => rsx! {
                            p { class: "pv-empty", "Le classement de cette division sera disponible dès la première journée." }
                        },
                        None => rsx! {},
                    }
                }
            }
        }
    }
}

#[component]
fn TabButton(name: String, active: bool, on_select: EventHandler<String>) -> Element {
    let tab_class = if active { "pv-tab active" } else { "pv-tab" };
    let label = name.clone();
    rsx! {
        button {
            class: "{tab_class}",
            role: "tab",
            aria_selected: "{active}",
            onclick: move |_| on_select.call(label.clone()),
            "{name}"
        }
    }
}

#[component]
fn StandingsTable(rows: Vec<StandingRow>) -> Element {
    rsx! {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn tab(name: &str, rows: Vec<StandingRow>) -> DivisionTab {
        tab_in(name, "Deux-Sèvres", name, rows)
    }

    fn tab_in(name: &str, league: &str, label: &str, rows: Vec<StandingRow>) -> DivisionTab {
        DivisionTab {
            name: name.to_string(),
            league: league.to_string(),
            label: label.to_string(),
            journee: Some(7),
            rows,
        }
    }

    fn row(club: &str) -> StandingRow {
        StandingRow {
            position: 1,
            club: club.to_string(),
            abbr: "LP".to_string(),
            played: 6,
            wins: 5,
            losses: 1,
            diff: 17,
            points: 15,
            form: vec![MatchResult::Win],
            zone: Zone::Qualif,
        }
    }

    fn render(active: &str, tabs: Vec<DivisionTab>) -> String {
        #[component]
        fn TestApp(active: String, tabs: Vec<DivisionTab>) -> Element {
            rsx! {
                Standings {
                    title: "Championnat Deux-Sèvres".to_string(),
                    subtitle: "Classement après 7 journées".to_string(),
                    tabs,
                    active,
                    on_select: move |_| {},
                }
            }
        }
        let mut dom = VirtualDom::new_with_props(
            TestApp,
            TestAppProps {
                active: active.to_string(),
                tabs,
            },
        );
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn rend_un_onglet_par_division() {
        let html = render(
            "D1",
            vec![
                tab("D1", vec![row("Luçon")]),
                tab("D2", vec![]),
                tab("D3", vec![]),
            ],
        );

        assert!(html.contains("pv-tabs"), "got: {html}");
        assert!(html.contains(">D1<"), "onglet D1 manquant: {html}");
        assert!(html.contains(">D2<"), "onglet D2 manquant: {html}");
        assert!(html.contains(">D3<"), "onglet D3 manquant: {html}");
        // D1 actif: un seul onglet a la classe active, et son tableau est rendu.
        assert_eq!(html.matches("pv-tab active").count(), 1, "got: {html}");
        assert!(html.contains("pv-table"), "tableau manquant: {html}");
        assert!(
            html.contains("Luçon"),
            "ligne du classement manquante: {html}"
        );
    }

    #[test]
    fn onglet_sans_classement_affiche_un_placeholder() {
        let html = render("D2", vec![tab("D1", vec![row("Luçon")]), tab("D2", vec![])]);

        assert!(html.contains("pv-tab active"), "got: {html}");
        assert!(
            html.contains("sera disponible dès la première journée"),
            "placeholder manquant: {html}"
        );
        assert!(!html.contains("pv-table"), "pas de tableau attendu: {html}");
    }

    #[test]
    fn aucune_division_affiche_un_message() {
        let html = render("", vec![]);

        assert!(html.contains("Aucune division configurée"), "got: {html}");
        assert!(!html.contains("pv-tabs"), "got: {html}");
    }

    #[test]
    fn rend_un_onglet_par_division_de_toutes_les_ligues() {
        let html = render(
            "D1 (Vendée)",
            vec![
                tab_in("D1", "Deux-Sèvres", "D1 (Deux-Sèvres)", vec![row("Luçon")]),
                tab_in("D1", "Vendée", "D1 (Vendée)", vec![row("Challans")]),
            ],
        );

        // Deux onglets "D1" de ligues différentes, désambiguïsés par le label.
        assert!(html.contains(">D1 (Deux-Sèvres)<"), "got: {html}");
        assert!(html.contains(">D1 (Vendée)<"), "got: {html}");
        assert_eq!(html.matches("pv-tab active").count(), 1, "got: {html}");
        // L'onglet actif est le D1 Vendée: son classement est rendu.
        assert!(html.contains("Challans"), "got: {html}");
        assert!(!html.contains("Luçon"), "got: {html}");
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
