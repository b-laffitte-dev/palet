//! Page liste des joueurs — GET /joueurs?q=&category=

use crate::components::layout::Layout;
use crate::services::api::ApiService;
use dioxus::prelude::*;

#[component]
pub fn Joueurs() -> Element {
    let api = ApiService::new();
    let mut q = use_signal(String::new);
    let mut category = use_signal(|| Option::<String>::None);

    let players = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            let q = q.read().clone();
            let cat = category.read().clone();
            async move {
                api.get_joueurs(
                    if q.is_empty() { None } else { Some(q.as_str()) },
                    None,
                    cat.as_deref(),
                )
                .await
            }
        }
    });

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    h1 { class: "page-title", "Joueurs" }
                    p { class: "page-sub", "Tous les licenciés du palet vendéen." }
                    div { class: "page-filters",
                        input {
                            class: "page-input",
                            r#type: "search",
                            placeholder: "Rechercher un joueur…",
                            value: "{q}",
                            oninput: move |evt: FormEvent| q.set(evt.value()),
                        }
                        select {
                            class: "page-input",
                            onchange: move |evt: FormEvent| {
                                let v = evt.value();
                                category.set(if v.is_empty() { None } else { Some(v) });
                            },
                            option { value: "", "Toutes catégories" }
                            option { value: "senior", "Senior" }
                            option { value: "veteran", "Vétéran" }
                            option { value: "junior", "Junior" }
                            option { value: "feminin", "Féminin" }
                        }
                    }
                    match &*players.read() {
                        Some(Ok(list)) if !list.is_empty() => rsx! {
                            p { class: "page-sub", "{list.len()} joueur(s)" }
                            div { class: "page-grid",
                                for p in list {
                                    Link { to: "/joueurs/{p.id}", key: "{p.id}", class: "page-card",
                                        h3 { "{p.full_name}" }
                                        p { class: "muted", "{p.category} · {p.license_status:?} · {p.license_number}" }
                                        p { class: "muted", "{p.age} ans — main {p.dominant_hand:?}" }
                                    }
                                }
                            }
                        },
                        Some(Ok(_)) => rsx! { p { class: "page-empty", "Aucun joueur ne correspond." } },
                        Some(Err(e)) => rsx! { p { class: "page-error", "Erreur : {e}" } },
                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                    }
                }
            }
        }
    }
}
