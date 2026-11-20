//! Page liste des actualités — GET /actualites?category=&q=

use crate::components::layout::Layout;
use crate::services::api::ApiService;
use crate::utils::format::format_date_fr;
use dioxus::prelude::*;

#[component]
pub fn Actualites() -> Element {
    let api = ApiService::new();
    let mut q = use_signal(String::new);
    let mut category = use_signal(|| Option::<String>::None);

    let articles = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            let q = q.read().clone();
            let cat = category.read().clone();
            async move {
                api.get_actualites(
                    cat.as_deref(),
                    if q.is_empty() { None } else { Some(q.as_str()) },
                )
                .await
            }
        }
    });

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    h1 { class: "page-title", "Actualités" }
                    p { class: "page-sub", "Toute l'actualité du palet vendéen." }
                    div { class: "page-filters",
                        input {
                            class: "page-input",
                            r#type: "search",
                            placeholder: "Rechercher une actualité…",
                            value: "{q}",
                            oninput: move |evt: FormEvent| q.set(evt.value()),
                        }
                        select {
                            class: "page-input",
                            onchange: move |evt: FormEvent| {
                                let v = evt.value();
                                category.set(if v.is_empty() { None } else { Some(v) });
                            },
                            option { value: "", "Toutes les catégories" }
                            option { value: "championnat", "Championnat" }
                            option { value: "tournoi", "Tournoi" }
                            option { value: "coupe", "Coupe" }
                            option { value: "reglement", "Règlement" }
                        }
                    }
                    match &*articles.read() {
                        Some(Ok(list)) if !list.is_empty() => rsx! {
                            div { class: "page-grid",
                                for a in list {
                                    Link { to: "/actualites/{a.id}", key: "{a.id}", class: "page-card",
                                        span { class: "badge", "{a.category}" }
                                        h3 { "{a.title}" }
                                        if let Some(excerpt) = &a.excerpt {
                                            p { class: "muted", "{excerpt}" }
                                        }
                                        if let Some(published) = &a.published_at {
                                            p { class: "muted", "Publié le {format_date_fr(published)}" }
                                        }
                                    }
                                }
                            }
                        },
                        Some(Ok(_)) => rsx! { p { class: "page-empty", "Aucune actualité pour le moment." } },
                        Some(Err(e)) => rsx! { p { class: "page-error", "Erreur : {e}" } },
                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                    }
                }
            }
        }
    }
}
