//! Page recherche globale — GET /recherche?q= (route /recherche/:q)

use crate::components::layout::Layout;
use crate::services::api::ApiService;
use crate::utils::format::format_date_fr;
use dioxus::prelude::*;

#[component]
pub fn Recherche(q: String) -> Element {
    let api = ApiService::new();
    let query = q.clone();
    let results = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            let q = query.clone();
            async move { api.search(&q).await }
        }
    });

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    h1 { class: "page-title", "Recherche" }
                    p { class: "page-sub", "Résultats pour « {q} »" }
                    match &*results.read() {
                        Some(Ok(r)) => rsx! {
                            div { class: "page-detail",
                                if !r.clubs.is_empty() {
                                    section {
                                        h2 { class: "page-title", "Clubs ({r.clubs.len()})" }
                                        div { class: "page-grid",
                                            for c in &r.clubs {
                                                Link { to: "/clubs/{c.id}", key: "{c.id}", class: "page-card",
                                                    h3 { "{c.name}" }
                                                    p { class: "muted", "{c.code} — {c.city} ({c.league}, {c.division})" }
                                                }
                                            }
                                        }
                                    }
                                }
                                if !r.players.is_empty() {
                                    section {
                                        h2 { class: "page-title", "Joueurs ({r.players.len()})" }
                                        div { class: "page-grid",
                                            for p in &r.players {
                                                Link { to: "/joueurs/{p.id}", key: "{p.id}", class: "page-card",
                                                    h3 { "{p.full_name}" }
                                                    p { class: "muted", "{p.category} · licence {p.license_number}" }
                                                }
                                            }
                                        }
                                    }
                                }
                                if !r.teams.is_empty() {
                                    section {
                                        h2 { class: "page-title", "Équipes ({r.teams.len()})" }
                                        div { class: "page-grid",
                                            for t in &r.teams {
                                                div { key: "{t.id}", class: "page-card",
                                                    h3 { "{t.name}" }
                                                    p { class: "muted", "{t.status:?}" }
                                                }
                                            }
                                        }
                                    }
                                }
                                if !r.competitions.is_empty() {
                                    section {
                                        h2 { class: "page-title", "Compétitions ({r.competitions.len()})" }
                                        div { class: "page-grid",
                                            for c in &r.competitions {
                                                Link { to: "/tournois/{c.id}", key: "{c.id}", class: "page-card",
                                                    h3 { "{c.name}" }
                                                    p { class: "muted", "{c.season} · {c.registered_teams} équipes" }
                                                }
                                            }
                                        }
                                    }
                                }
                                if !r.news.is_empty() {
                                    section {
                                        h2 { class: "page-title", "Actualités ({r.news.len()})" }
                                        div { class: "page-grid",
                                            for a in &r.news {
                                                Link { to: "/actualites/{a.id}", key: "{a.id}", class: "page-card",
                                                    span { class: "badge", "{a.category}" }
                                                    h3 { "{a.title}" }
                                                    if let Some(published) = &a.published_at {
                                                        p { class: "muted", "Publié le {format_date_fr(published)}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                if r.clubs.is_empty() && r.players.is_empty() && r.teams.is_empty()
                                    && r.competitions.is_empty() && r.news.is_empty() {
                                    p { class: "page-empty", "Aucun résultat pour « {q} »." }
                                }
                            }
                        },
                        Some(Err(e)) => rsx! { p { class: "page-error", "Erreur : {e}" } },
                        None => rsx! { p { class: "page-empty", "Recherche en cours…" } },
                    }
                }
            }
        }
    }
}
