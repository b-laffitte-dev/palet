//! Footer component for Palet Vendéen

use dioxus::prelude::*;

/// Footer component
#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "bg-brand-green-900/80 border-t border-brand-green-800",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                div { class: "flex flex-col md:flex-row justify-between items-center",
                    div { class: "mb-4 md:mb-0",
                        p { class: "text-brand-green-200 text-sm", "© 2024 Palet Vendéen. Tous droits réservés." }
                    }
                    div { class: "flex space-x-6",
                        a { class: "text-brand-green-200 hover:text-white transition-colors text-sm", href: "/mentions-legales", "Mentions légales" }
                        a { class: "text-brand-green-200 hover:text-white transition-colors text-sm", href: "/contact", "Contact" }
                        a { class: "text-brand-green-200 hover:text-white transition-colors text-sm", href: "https://github.com/b-laffitte-dev/palet", "GitHub" }
                    }
                }
            }
        }
    }
}
