//! Espace fédéral — gestion des divisions départementales.
//!
//! Les divisions (D1, D2, D3…) affichées en onglets sur la home sont
//! dynamiques : on peut en ajouter autant que souhaité ici, et les supprimer.

use crate::components::common::{Button, ButtonSize, ButtonVariant};
use crate::components::home::model::HOME_LEAGUE;
use crate::components::layout::Layout;
use crate::services::api::ApiService;
use crate::utils::format::current_season;
use chrono::Utc;
use dioxus::prelude::*;
use palet_shared::CreateDivisionRequest;

/// Page component
#[component]
pub fn Federation() -> Element {
    let api = ApiService::new();
    let divisions = use_resource({
        let api = api.clone();
        move || {
            let api = api.clone();
            async move { api.get_divisions(Some(HOME_LEAGUE)).await }
        }
    });
    let mut new_name = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut busy = use_signal(|| false);

    let api_form = api.clone();
    let on_submit = move |evt: FormEvent| {
        evt.prevent_default();
        let name = new_name.read().clone();
        if name.trim().is_empty() {
            message.set("Indiquez un nom de division (ex. D4).".to_string());
            return;
        }
        busy.set(true);
        message.set(String::new());
        let api = api_form.clone();
        let mut divisions = divisions;
        spawn(async move {
            let request = CreateDivisionRequest {
                name,
                league: HOME_LEAGUE.to_string(),
                season: current_season(&Utc::now()),
            };
            match api.create_division(&request).await {
                Ok(d) => {
                    new_name.set(String::new());
                    message.set(format!("Division {} ajoutée.", d.name));
                }
                Err(e) => message.set(e),
            }
            busy.set(false);
            divisions.restart();
        });
    };

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    h1 { class: "text-heading-xl font-bold text-white uppercase tracking-wide mb-8", "Espace fédéral" }
                    p { class: "text-brand-green-200 mb-8",
                        "Gestion des divisions départementales — {HOME_LEAGUE} (79). "
                        "Chaque division ajoutée apparaît comme onglet sur la page d'accueil."
                    }
                    section { class: "fed-panel",
                        h2 { class: "text-white font-bold mb-4", "Divisions" }
                        match &*divisions.read() {
                            Some(Ok(list)) if !list.is_empty() => {
                                let list = list.clone();
                                rsx! {
                                    ul { class: "fed-list",
                                        for d in list {
                                            li { key: "{d.id}", class: "fed-item",
                                                span { class: "fed-item-name", "{d.name}" }
                                                span { class: "fed-item-season", "Saison {d.season}" }
                                                Button {
                                                    variant: ButtonVariant::Danger,
                                                    size: ButtonSize::S,
                                                    onclick: {
                                                        let api = api.clone();
                                                        move |_| {
                                                            let api = api.clone();
                                                            let mut divisions = divisions;
                                                            spawn(async move {
                                                                match api.delete_division(d.id).await {
                                                                    Ok(_) => message.set("Division supprimée.".to_string()),
                                                                    Err(e) => message.set(format!("Suppression impossible : {e}")),
                                                                }
                                                                divisions.restart();
                                                            });
                                                        }
                                                    },
                                                    "Supprimer"
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            Some(Ok(_)) => rsx! {
                                p { class: "text-brand-green-200 mb-4", "Aucune division pour le moment." }
                            },
                            Some(Err(e)) => rsx! {
                                p { class: "fed-error mb-4", "Erreur de chargement : {e}" }
                            },
                            None => rsx! {
                                p { class: "text-brand-green-200 mb-4", "Chargement…" }
                            },
                        }
                        form { class: "fed-form", onsubmit: on_submit,
                            input {
                                class: "fed-input",
                                r#type: "text",
                                placeholder: "Nouvelle division (ex. D4)",
                                maxlength: 12,
                                value: "{new_name}",
                                oninput: move |evt: FormEvent| new_name.set(evt.value()),
                            }
                            Button {
                                variant: ButtonVariant::Primary,
                                r#type: "submit".to_string(),
                                disabled: busy(),
                                "Ajouter"
                            }
                        }
                        if !message.read().is_empty() {
                            p { class: "text-brand-green-200 text-sm mt-2", "{message}" }
                        }
                    }
                }
            }
        }
    }
}
