//! Page liste des clubs — GET /clubs, recherche côté client

use crate::components::layout::Layout;
use crate::services::api::ApiService;
use dioxus::prelude::*;

#[component]
pub fn Clubs() -> Element {
    let api = ApiService::new();
    let mut q = use_signal(String::new);
    let clubs = use_resource(move || {
        let api = api.clone();
        async move { api.get_clubs().await }
    });

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    h1 { class: "page-title", "Clubs" }
                    p { class: "page-sub", "Les clubs affiliés du palet vendéen." }
                    div { class: "page-filters",
                        input {
                            class: "page-input",
                            r#type: "search",
                            placeholder: "Rechercher un club, une ville…",
                            value: "{q}",
                            oninput: move |evt: FormEvent| q.set(evt.value()),
                        }
                    }
                    match &*clubs.read() {
                        Some(Ok(list)) => {
                            let needle = q.read().to_lowercase();
                            let filtered: Vec<_> = list.iter()
                                .filter(|c| needle.is_empty()
                                    || c.name.to_lowercase().contains(&needle)
                                    || c.city.to_lowercase().contains(&needle)
                                    || c.code.to_lowercase().contains(&needle))
                                .collect();
                            if filtered.is_empty() {
                                rsx! { p { class: "page-empty", "Aucun club ne correspond." } }
                            } else {
                                rsx! {
                                    div { class: "page-grid",
                                        for c in filtered {
                                            Link { to: "/clubs/{c.id}", key: "{c.id}", class: "page-card",
                                                h3 { "{c.name}" }
                                                p { class: "muted", "{c.code} — {c.city} ({c.league}, {c.division})" }
                                                p { class: "muted", "{c.players_count} joueurs · {c.teams_count} équipes" }
                                            }
                                        }
                                    }
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
