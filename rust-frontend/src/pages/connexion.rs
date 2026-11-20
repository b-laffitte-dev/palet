//! Page de connexion (email + mot de passe) — POST /auth/login

use crate::components::common::{Button, ButtonSize, ButtonVariant};
use crate::components::layout::Layout;
use crate::hooks::use_auth::{login, use_auth};
use crate::utils::validation::validate_email;
use dioxus::prelude::*;

#[component]
pub fn Connexion() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut busy = use_signal(|| false);
    let navigator = use_navigator();

    let on_submit = move |evt: FormEvent| {
        evt.prevent_default();
        let (email, password) = (email.read().clone(), password.read().clone());
        if !validate_email(&email) || password.is_empty() {
            let mut auth = use_auth();
            auth.write().error = Some("Email ou mot de passe invalide".to_string());
            return;
        }
        busy.set(true);
        spawn(async move {
            match login(&email, &password).await {
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

    let error = use_auth().read().error.clone();

    rsx! {
        Layout {
            main { class: "flex-1 px-4 py-6 lg:px-8 lg:py-8",
                div { class: "max-w-7xl mx-auto",
                    h1 { class: "page-title", "Connexion" }
                    p { class: "page-sub", "Accédez à votre espace joueur, club ou arbitre." }
                    form { class: "page-form", onsubmit: on_submit,
                        label {
                            "Email"
                            input {
                                class: "page-input",
                                r#type: "email",
                                placeholder: "vous@exemple.fr",
                                value: "{email}",
                                oninput: move |evt: FormEvent| email.set(evt.value()),
                            }
                        }
                        label {
                            "Mot de passe"
                            input {
                                class: "page-input",
                                r#type: "password",
                                placeholder: "••••••••",
                                value: "{password}",
                                oninput: move |evt: FormEvent| password.set(evt.value()),
                            }
                        }
                        if let Some(e) = &error {
                            p { class: "page-error", "{e}" }
                        }
                        Button {
                            variant: ButtonVariant::Primary,
                            size: ButtonSize::L,
                            r#type: "submit".to_string(),
                            disabled: busy(),
                            "Se connecter"
                        }
                    }
                    p { class: "page-sub mt-4",
                        "Pas encore de compte ? "
                        Link { to: "/inscription", class: "text-brand-orange-300 underline", "Inscrivez-vous" }
                    }
                }
            }
        }
    }
}
