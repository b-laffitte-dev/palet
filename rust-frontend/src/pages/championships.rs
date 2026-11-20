//! Page liste des championnats — GET /championnats, recherche côté client

use crate::components::layout::Layout;
use crate::services::api::ApiService;
use crate::utils::labels::competition_status_label;
use dioxus::prelude::*;

/// Libellé des divisions ("D1, D2") — le littéral ", " vit hors du rsx.
fn divisions_label(divisions: &[String]) -> String {
    divisions.join(", ")
}

#[component]
pub fn Championnats() -> Element {
    let api = ApiService::new();
    let mut q = use_signal(String::new);
    let championships = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move { api.get_championships().await }
        }
    });

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    h1 { class: "page-title", "Championnats" }
                    p { class: "page-sub", "Les championnats départementaux du palet vendéen." }
                    div { class: "page-filters",
                        input {
                            class: "page-input",
                            r#type: "search",
                            placeholder: "Rechercher un championnat, un département…",
                            value: "{q}",
                            oninput: move |evt: FormEvent| q.set(evt.value()),
                        }
                    }
                    match &*championships.read() {
                        Some(Ok(list)) => {
                            let needle = q.read().to_lowercase();
                            let filtered: Vec<_> = list.iter()
                                .filter(|c| needle.is_empty()
                                    || c.name.to_lowercase().contains(&needle)
                                    || c.league.to_lowercase().contains(&needle)
                                    || c.season.to_lowercase().contains(&needle))
                                .collect();
                            if filtered.is_empty() {
                                rsx! { p { class: "page-empty", "Aucun championnat ne correspond." } }
                            } else {
                                rsx! {
                                    div { class: "page-grid",
                                        for c in filtered {
                                            Link { to: "/championnats/{c.id}", key: "{c.id}", class: "page-card",
                                                span { class: "badge", "{competition_status_label(&c.status)}" }
                                                h3 { "{c.name}" }
                                                p { class: "muted", "{c.league} · saison {c.season}" }
                                                if !c.divisions.is_empty() {
                                                    p { class: "muted", "Divisions : {divisions_label(&c.divisions)}" }
                                                }
                                                if let Some(j) = c.current_journee {
                                                    p { class: "muted", "Journée {j} / {c.total_journees}" }
                                                }
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
