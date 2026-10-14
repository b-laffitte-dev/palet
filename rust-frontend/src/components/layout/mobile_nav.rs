//! Mobile navigation component for Palet Vendéen

use dioxus::prelude::*;

/// Mobile navigation component
#[component]
pub fn MobileNav() -> Element {
    rsx! {
        nav { class: "md:hidden fixed bottom-0 left-0 right-0 bg-brand-green-900/90 backdrop-blur-sm border-t border-brand-green-800 z-50",
            div { class: "flex justify-around items-center h-16",
                Link { to: "/", class: "flex flex-col items-center text-brand-green-200 hover:text-white transition-colors",
                    svg { class: "w-6 h-6 mb-1", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" }
                    }
                    span { class: "text-xs", "Accueil" }
                }
                Link { to: "/clubs", class: "flex flex-col items-center text-brand-green-200 hover:text-white transition-colors",
                    svg { class: "w-6 h-6 mb-1", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" }
                    }
                    span { class: "text-xs", "Clubs" }
                }
                Link { to: "/tournois", class: "flex flex-col items-center text-brand-green-200 hover:text-white transition-colors",
                    svg { class: "w-6 h-6 mb-1", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" }
                    }
                    span { class: "text-xs", "Tournois" }
                }
                Link { to: "/joueurs", class: "flex flex-col items-center text-brand-green-200 hover:text-white transition-colors",
                    svg { class: "w-6 h-6 mb-1", xmlns: "http://www.w3.org/2000/svg", fill: "none", view_box: "0 0 24 24", stroke: "currentColor",
                        path { stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2", d: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" }
                    }
                    span { class: "text-xs", "Joueurs" }
                }
            }
        }
    }
}
