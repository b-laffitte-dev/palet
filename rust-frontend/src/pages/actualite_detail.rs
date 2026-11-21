//! Page détail d'une actualité — GET /actualites/{id} (publiée uniquement)

use crate::components::layout::Layout;
use crate::services::api::ApiService;
use crate::utils::format::format_date_fr;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn ActualiteDetail(id: String) -> Element {
    let api = ApiService::new();
    let article = use_resource({
        let api = api.clone();
        let id = id.clone();
        move || {
            let api = api.clone();
            let id = id.clone();
            async move {
                match Uuid::parse_str(&id) {
                    Ok(u) => api.get_actualite(u).await,
                    Err(_) => Err("Identifiant d'article invalide".to_string()),
                }
            }
        }
    });

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-3xl mx-auto",
                    Link { to: "/actualites", class: "page-sub", "← Retour aux actualités" }
                    match &*article.read() {
                        Some(Ok(a)) => rsx! {
                            h1 { class: "page-title", "{a.title}" }
                            p { class: "page-sub",
                                span { class: "badge", "{a.category}" }
                                if let Some(published) = &a.published_at {
                                    span { "  — publié le {format_date_fr(published)}" }
                                }
                            }
                            article { class: "page-card",
                                for (i, para) in a.content.split("\n\n").enumerate() {
                                    if !para.trim().is_empty() {
                                        p { key: "{i}", class: "muted", style: "margin-bottom: .75rem; color: #fff;", "{para}" }
                                    }
                                }
                            }
                        },
                        Some(Err(e)) => rsx! { p { class: "page-error", "Article introuvable : {e}" } },
                        None => rsx! { p { class: "page-empty", "Chargement…" } },
                    }
                }
            }
        }
    }
}
