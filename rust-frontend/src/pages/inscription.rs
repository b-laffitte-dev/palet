//! Page d'inscription — POST /auth/register (auto-login, puis accueil)

use crate::components::common::{Button, ButtonSize, ButtonVariant};
use crate::components::layout::Layout;
use crate::hooks::use_auth::{register, use_auth};
use crate::utils::validation::validate_email;
use dioxus::prelude::*;
use palet_shared::RegisterRequest;

#[component]
pub fn Inscription() -> Element {
    let mut first_name = use_signal(String::new);
    let mut last_name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut license = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm = use_signal(String::new);
    let mut busy = use_signal(|| false);
    let mut local_error = use_signal(|| Option::<String>::None);
    let navigator = use_navigator();

    let on_submit = move |evt: FormEvent| {
        evt.prevent_default();
        let (first, last, email, lic, pw, conf) = (
            first_name.read().clone(),
            last_name.read().clone(),
            email.read().clone(),
            license.read().clone(),
            password.read().clone(),
            confirm.read().clone(),
        );
        let validation = if first.trim().is_empty() || last.trim().is_empty() {
            Some("Prénom et nom requis".to_string())
        } else if !validate_email(&email) {
            Some("Adresse email invalide".to_string())
        } else if pw.len() < 8 {
            Some("Le mot de passe doit contenir au moins 8 caractères".to_string())
        } else if pw != conf {
            Some("Les mots de passe ne correspondent pas".to_string())
        } else {
            None
        };
        if let Some(e) = validation {
            local_error.set(Some(e));
            return;
        }
        local_error.set(None);
        busy.set(true);
        spawn(async move {
            let request = RegisterRequest {
                email,
                password: pw,
                first_name: first,
                last_name: last,
                club_id: None,
                license_number: if lic.trim().is_empty() {
                    None
                } else {
                    Some(lic.trim().to_string())
                },
            };
            match register(request).await {
                Ok(()) => {
                    navigator.push("/");
                }
                Err(e) => {
                    let mut auth = use_auth();
                    auth.write().error = Some(e);
                }
            }
            busy.set(false);
        });
    };

    let error = local_error
        .read()
        .clone()
        .or_else(|| use_auth().read().error.clone());

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    h1 { class: "page-title", "Inscription" }
                    p { class: "page-sub", "Créez votre compte de joueur (licence FNSMR)." }
                    form { class: "page-form", onsubmit: on_submit,
                        label {
                            "Prénom"
                            input { class: "page-input", r#type: "text", value: "{first_name}", oninput: move |evt: FormEvent| first_name.set(evt.value()) }
                        }
                        label {
                            "Nom"
                            input { class: "page-input", r#type: "text", value: "{last_name}", oninput: move |evt: FormEvent| last_name.set(evt.value()) }
                        }
                        label {
                            "Email"
                            input { class: "page-input", r#type: "email", placeholder: "vous@exemple.fr", value: "{email}", oninput: move |evt: FormEvent| email.set(evt.value()) }
                        }
                        label {
                            "Numéro de licence (optionnel)"
                            input { class: "page-input", r#type: "text", placeholder: "79-12345", value: "{license}", oninput: move |evt: FormEvent| license.set(evt.value()) }
                        }
                        label {
                            "Mot de passe (8 caractères min.)"
                            input { class: "page-input", r#type: "password", value: "{password}", oninput: move |evt: FormEvent| password.set(evt.value()) }
                        }
                        label {
                            "Confirmer le mot de passe"
                            input { class: "page-input", r#type: "password", value: "{confirm}", oninput: move |evt: FormEvent| confirm.set(evt.value()) }
                        }
                        if let Some(e) = &error {
                            p { class: "page-error", "{e}" }
                        }
                        Button {
                            variant: ButtonVariant::Primary,
                            size: ButtonSize::L,
                            r#type: "submit".to_string(),
                            disabled: busy(),
                            "Créer mon compte"
                        }
                    }
                    p { class: "page-sub mt-4",
                        "Déjà inscrit ? "
                        Link { to: "/connexion", class: "text-brand-orange-300 underline", "Connectez-vous" }
                    }
                }
            }
        }
    }
}
