//! Footer component for Palet Vendéen

use crate::utils::format::year_of;
use dioxus::prelude::*;
use time::OffsetDateTime;

/// Footer component
#[component]
pub fn Footer() -> Element {
    let year = year_of(&OffsetDateTime::now_utc());
    rsx! {
        footer { class: "bg-brand-green-900/80 border-t border-brand-green-800",
            div { class: "max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8",
                div { class: "flex flex-col md:flex-row justify-between items-center",
                    div { class: "mb-4 md:mb-0",
                        p { class: "text-brand-green-200 text-sm", "© {year} Palet Vendéen. Tous droits réservés." }
                    }
                    div { class: "flex flex-wrap justify-center gap-x-6 gap-y-2",
                        a { class: "text-brand-green-200 hover:text-white transition-colors text-sm", href: "/mentions-legales", "Mentions légales" }
                        a { class: "text-brand-green-200 hover:text-white transition-colors text-sm", href: "/contact", "Contact" }
                        a { class: "text-brand-green-200 hover:text-white transition-colors text-sm", href: "https://www.gestaffil.org/", target: "_blank", rel: "noopener noreferrer", "Gestaffil" }
                        a { class: "text-brand-green-200 hover:text-white transition-colors text-sm", href: "https://www.le-palet.com/", target: "_blank", rel: "noopener noreferrer", "le-palet.com" }
                        a { class: "text-brand-green-200 hover:text-white transition-colors text-sm", href: "https://github.com/b-laffitte-dev/palet", target: "_blank", rel: "noopener noreferrer", "GitHub" }
                    }
                }
            }
        }
    }
}
