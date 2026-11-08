//! Carrousel des matchs des journées en cours, une slide par division.
//!
//! Remplace l'ancienne bande « scores en direct » : on navigue d'une
//! division à l'autre (flèches + dots), chaque slide liste les matchs de la
//! journée en cours de la ligue concernée (live, à venir et terminés).

use super::model::{CarouselMatch, DivisionSlide};
use dioxus::prelude::*;

#[component]
pub fn JourneeCarousel(slides: Vec<DivisionSlide>) -> Element {
    let mut index = use_signal(|| 0usize);
    let count = slides.len();
    let current = clamp_index(*index.read(), count);

    let go_prev = move |_| index.set(clamp_index(current.saturating_sub(1), count));
    let go_next = move |_| index.set(clamp_index(current + 1, count));

    rsx! {
        section { class: "pv-carousel",
            div { class: "pv-container",
                div { class: "pv-carousel-head",
                    div {
                        p { class: "pv-carousel-kicker", "Journées en cours" }
                        h2 { class: "pv-carousel-title", "Matchs par division" }
                    }
                    if count > 1 {
                        div { class: "pv-carousel-nav",
                            button {
                                class: "pv-carousel-btn",
                                aria_label: "Division précédente",
                                disabled: current == 0,
                                onclick: go_prev,
                                "‹"
                            }
                            button {
                                class: "pv-carousel-btn",
                                aria_label: "Division suivante",
                                disabled: current >= count - 1,
                                onclick: go_next,
                                "›"
                            }
                        }
                    }
                }
                if slides.is_empty() {
                    p { class: "pv-carousel-empty",
                        "Aucun match sur les journées en cours pour le moment."
                    }
                } else {
                    div { class: "pv-carousel-viewport",
                        div {
                            class: "pv-carousel-track",
                            style: "transform: translateX(-{current * 100}%);",
                            for (i, slide) in slides.iter().enumerate() {
                                SlideView {
                                    key: "{slide.league}::{slide.name}",
                                    slide: slide.clone(),
                                    visible: i == current,
                                }
                            }
                        }
                    }
                    if count > 1 {
                        div { class: "pv-carousel-dots", role: "tablist",
                            for (i, slide) in slides.iter().enumerate() {
                                button {
                                    key: "{slide.league}::{slide.name}",
                                    class: if i == current { "pv-dot active" } else { "pv-dot" },
                                    role: "tab",
                                    aria_selected: "{i == current}",
                                    aria_label: "Aller à {slide.label}",
                                    onclick: move |_| index.set(i),
                                    "{slide.label}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Ramène l'index dans les bornes quand le nombre de slides change.
fn clamp_index(i: usize, count: usize) -> usize {
    if count == 0 {
        0
    } else {
        i.min(count - 1)
    }
}

#[component]
fn SlideView(slide: DivisionSlide, visible: bool) -> Element {
    let journee_label = slide
        .journee
        .map(|j| format!("Journée {j}"))
        .unwrap_or_default();
    rsx! {
        div {
            class: "pv-slide",
            aria_hidden: "{!visible}",
            div { class: "pv-slide-head",
                span { class: "pv-slide-division", "{slide.label}" }
                if !journee_label.is_empty() {
                    span { class: "pv-slide-journee", "{journee_label}" }
                }
                if !slide.league.is_empty() {
                    span { class: "pv-slide-league", "{slide.league}" }
                }
            }
            div { class: "pv-slide-matches",
                for (i, m) in slide.matches.iter().enumerate() {
                    MatchCard { key: "{i}", m: m.clone() }
                }
            }
        }
    }
}

#[component]
fn MatchCard(m: CarouselMatch) -> Element {
    let score = match (m.score1, m.score2) {
        (Some(a), Some(b)) => format!("{a} — {b}"),
        _ => "– · –".to_string(),
    };
    rsx! {
        div { class: "pv-match-card",
            div { class: "pv-match-top",
                span {
                    class: if m.live { "pv-match-status live" } else { "pv-match-status" },
                    "{m.status_label}"
                }
                if !m.meta.is_empty() {
                    span { class: "pv-match-meta", "{m.meta}" }
                }
            }
            div { class: "pv-match-teams",
                span { class: "pv-match-team",
                    span { class: "pv-match-abbr", "{m.abbr1}" }
                    span { class: "pv-match-name", "{m.team1}" }
                }
                span { class: "pv-match-score", "{score}" }
                span { class: "pv-match-team right",
                    span { class: "pv-match-name", "{m.team2}" }
                    span { class: "pv-match-abbr", "{m.abbr2}" }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slide(name: &str, league: &str, journee: i32, matches: Vec<CarouselMatch>) -> DivisionSlide {
        DivisionSlide {
            name: name.to_string(),
            league: league.to_string(),
            label: name.to_string(),
            journee: Some(journee),
            matches,
        }
    }

    fn card(team1: &str, team2: &str, status: &str, live: bool) -> CarouselMatch {
        CarouselMatch {
            team1: team1.to_string(),
            team2: team2.to_string(),
            abbr1: "AA".to_string(),
            abbr2: "BB".to_string(),
            score1: Some(9),
            score2: Some(11),
            status_label: status.to_string(),
            live,
            meta: "Manche 3".to_string(),
        }
    }

    fn render(slides: Vec<DivisionSlide>) -> String {
        #[component]
        fn TestApp(slides: Vec<DivisionSlide>) -> Element {
            rsx! { JourneeCarousel { slides } }
        }
        let mut dom = VirtualDom::new_with_props(TestApp, TestAppProps { slides });
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn clamp_index_reste_dans_les_bornes() {
        assert_eq!(clamp_index(0, 0), 0);
        assert_eq!(clamp_index(5, 0), 0);
        assert_eq!(clamp_index(0, 3), 0);
        assert_eq!(clamp_index(2, 3), 2);
        assert_eq!(clamp_index(9, 3), 2);
    }

    #[test]
    fn rend_une_slide_par_division_avec_nav_et_dots() {
        let html = render(vec![
            slide(
                "D1",
                "Vendée",
                7,
                vec![card("Challans", "Herbiers", "EN DIRECT", true)],
            ),
            slide(
                "D2",
                "Deux-Sèvres",
                3,
                vec![card("Airvault", "Cerizay", "À VENIR", false)],
            ),
        ]);

        assert!(html.contains("pv-carousel"), "got: {html}");
        assert!(html.contains("Matchs par division"), "got: {html}");
        assert!(html.contains(">D1<"), "dot D1 manquant: {html}");
        assert!(html.contains(">D2<"), "dot D2 manquant: {html}");
        assert_eq!(html.matches("pv-dot").count(), 2, "got: {html}");
        assert_eq!(html.matches("pv-dot active").count(), 1, "got: {html}");
        assert!(
            html.contains("pv-carousel-btn"),
            "flèches manquantes: {html}"
        );
        assert!(html.contains("Journée 7"), "badge journée manquant: {html}");
        assert!(html.contains("EN DIRECT"), "statut live manquant: {html}");
        assert!(html.contains("9 — 11"), "score manquant: {html}");
        assert!(html.contains("Challans"), "équipe manquante: {html}");
        // La piste est positionnée sur la première slide.
        assert!(html.contains("translateX(-0%)"), "got: {html}");
    }

    #[test]
    fn une_seule_slide_cache_nav_et_dots() {
        let html = render(vec![slide(
            "D1",
            "Vendée",
            7,
            vec![card("A", "B", "À VENIR", false)],
        )]);

        assert!(!html.contains("pv-carousel-btn"), "pas de flèches: {html}");
        assert!(!html.contains("pv-dot"), "pas de dots: {html}");
        assert!(html.contains("pv-match-card"), "got: {html}");
    }

    #[test]
    fn aucun_match_affiche_un_message() {
        let html = render(vec![]);

        assert!(
            html.contains("Aucun match sur les journées en cours"),
            "got: {html}"
        );
        assert!(!html.contains("pv-carousel-track"), "got: {html}");
    }
}
