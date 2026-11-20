//! Page liste des compétitions ponctuelles — GET /competitions (tournois & coupes)

use crate::components::layout::Layout;
use crate::services::api::ApiService;
use crate::utils::format::format_date_fr;
use crate::utils::labels::{
    competition_category_label, competition_material_label, competition_status_label,
    competition_type_label,
};
use dioxus::prelude::*;
use palet_shared::{CompetitionEvent, CompetitionStatus, CompetitionType};

fn card(c: &CompetitionEvent) -> Element {
    let dates = match (c.start_date.as_ref(), c.end_date.as_ref()) {
        (Some(s), Some(e)) => format!("Du {} au {}", format_date_fr(s), format_date_fr(e)),
        (Some(s), None) => format!("Le {}", format_date_fr(s)),
        _ => "Dates à définir".to_string(),
    };
    let teams = match c.max_teams {
        Some(max) => format!("{} / {max} équipes", c.registered_teams),
        None => format!("{} équipes inscrites", c.registered_teams),
    };
    rsx! {
        Link { to: "/tournois/{c.id}", key: "{c.id}", class: "page-card",
            if c.status == CompetitionStatus::Registration {
                span { class: "badge orange", "{competition_status_label(&c.status)}" }
            } else {
                span { class: "badge", "{competition_status_label(&c.status)}" }
            }
            h3 { "{c.name}" }
            p { class: "muted", "{competition_type_label(&c.competition_type)} · {competition_category_label(&c.category)} · palets {competition_material_label(&c.material)}" }
            if let Some(loc) = &c.location {
                p { class: "muted", "{loc}" }
            }
            p { class: "muted", "{dates} · saison {c.season} · {teams}" }
        }
    }
}

#[component]
pub fn Tournois() -> Element {
    let api = ApiService::new();
    let competitions = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move { api.get_competitions(None, None).await }
        }
    });

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    h1 { class: "page-title", "Tournois & coupes" }
                    p { class: "page-sub", "Les compétitions ponctuelles du palet vendéen." }
                    match &*competitions.read() {
                        Some(Ok(list)) => {
                            let tournois: Vec<&CompetitionEvent> = list.iter()
                                .filter(|c| c.competition_type == CompetitionType::Tournament)
                                .collect();
                            let coupes: Vec<&CompetitionEvent> = list.iter()
                                .filter(|c| c.competition_type == CompetitionType::Cup)
                                .collect();
                            let n_tournois = tournois.len();
                            let n_coupes = coupes.len();
                            rsx! {
                                if n_tournois > 0 {
                                    section {
                                        h2 { class: "page-title", "Tournois ({n_tournois})" }
                                        div { class: "page-grid",
                                            for c in tournois.iter() { {card(c)} }
                                        }
                                    }
                                }
                                if n_coupes > 0 {
                                    section {
                                        h2 { class: "page-title", "Coupes ({n_coupes})" }
                                        div { class: "page-grid",
                                            for c in coupes.iter() { {card(c)} }
                                        }
                                    }
                                }
                                if tournois.is_empty() && coupes.is_empty() {
                                    p { class: "page-empty", "Aucune compétition ponctuelle pour le moment." }
                                }
                            }
                        }
                        Some(Err(e)) => rsx! { p { class: "page-error", "Erreur : {e}" } },
                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                    }
                }
            }
        }
    }
}
