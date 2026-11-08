//! Carrousel des matchs des journées en cours, une slide par division.
//!
//! On navigue d'une division à l'autre (flèches + dots) ; chaque slide liste
//! les matchs de la journée en cours de la ligue concernée (live, à venir et
//! terminés). Un clic sur une carte remonte la sélection vers la home
//! ([`CarouselSelection`]), qui l'affiche dans le hero.

use super::model::{CarouselMatch, CarouselSelection, DivisionSlide};
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn JourneeCarousel(
    slides: Vec<DivisionSlide>,
    on_select: EventHandler<CarouselSelection>,
) -> Element {
    let mut index = use_signal(|| 0usize);
    let mut selected = use_signal(|| Option::<Uuid>::None);
    let count = slides.len();
    let current = clamp_index(*index.read(), count);

    let go_prev = move |_| index.set(clamp_index(current.saturating_sub(1), count));
    let go_next = move |_| index.set(clamp_index(current + 1, count));

    // Clic sur une carte : on la marque sélectionnée et on remonte le match
    // vers la home (affiché dans le hero).
    let select = move |sel: CarouselSelection| {
        selected.set(Some(sel.m.id));
        on_select.call(sel);
    };

    // Match dont la carte est marquée « selected » : celui cliqué, sinon le
    // premier live, sinon le premier match.
    let all: Vec<&CarouselMatch> = slides.iter().flat_map(|s| s.matches.iter()).collect();
    let clicked_id = *selected.read();
    let selected_id = all
        .iter()
        .find(|m| Some(m.id) == clicked_id)
        .or_else(|| all.iter().find(|m| m.live))
        .or_else(|| all.first())
        .map(|m| m.id);

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
                                    selected_id,
                                    on_select: select,
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
fn SlideView(
    slide: DivisionSlide,
    visible: bool,
    selected_id: Option<Uuid>,
    on_select: EventHandler<CarouselSelection>,
) -> Element {
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
                    MatchCard {
                        key: "{i}",
                        m: m.clone(),
                        selected: Some(m.id) == selected_id,
                        selection: CarouselSelection {
                            division: slide.label.clone(),
                            journee: slide.journee,
                            m: m.clone(),
                        },
                        on_select,
                    }
                }
            }
        }
    }
}

#[component]
fn MatchCard(
    m: CarouselMatch,
    selected: bool,
    selection: CarouselSelection,
    on_select: EventHandler<CarouselSelection>,
) -> Element {
    let score = match (m.score1, m.score2) {
        (Some(a), Some(b)) => format!("{a} — {b}"),
        _ => "– · –".to_string(),
    };
    let card_class = if selected {
        "pv-match-card selected"
    } else {
        "pv-match-card"
    };
    rsx! {
        button {
            class: "{card_class}",
            aria_pressed: "{selected}",
            onclick: move |_| on_select.call(selection.clone()),
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
            id: Uuid::new_v4(),
            team1: team1.to_string(),
            team2: team2.to_string(),
            abbr1: "AA".to_string(),
            abbr2: "BB".to_string(),
            score1: Some(9),
            score2: Some(11),
            status_label: status.to_string(),
            live,
            finished: status == "TERMINÉ",
            meta: "Manche 3".to_string(),
            when: "Manche 3".to_string(),
            location: "Boulodrome".to_string(),
            date_label: "08/11/2026".to_string(),
            best_of: Some(13),
            manches: vec![],
            winner_name: None,
        }
    }

    fn render(slides: Vec<DivisionSlide>) -> String {
        #[component]
        fn TestApp(slides: Vec<DivisionSlide>) -> Element {
            rsx! {
                JourneeCarousel {
                    slides,
                    on_select: move |_: CarouselSelection| {},
                }
            }
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
        // Toutes les cartes sont des boutons cliquables.
        assert_eq!(html.matches("aria-pressed").count(), 2, "got: {html}");
    }

    #[test]
    fn la_carte_du_premier_match_live_est_selectionnee_par_defaut() {
        let html = render(vec![
            slide(
                "D1",
                "Vendée",
                7,
                vec![card("Challans", "Herbiers", "À VENIR", false)],
            ),
            slide(
                "D2",
                "Deux-Sèvres",
                3,
                vec![card("Airvault", "Cerizay", "EN DIRECT", true)],
            ),
        ]);

        // Une seule carte sélectionnée : celle du match live (slide D2).
        assert_eq!(
            html.matches("pv-match-card selected").count(),
            1,
            "got: {html}"
        );
        assert_eq!(
            html.matches("aria-pressed=\"true\"").count(),
            1,
            "got: {html}"
        );
        // Plus de zone détail dans le carrousel : elle vit dans le hero.
        assert!(!html.contains("pv-carousel-detail"), "got: {html}");
    }

    #[test]
    fn sans_match_live_la_premiere_carte_est_selectionnee() {
        let html = render(vec![slide(
            "D1",
            "Vendée",
            7,
            vec![card("Challans", "Herbiers", "À VENIR", false)],
        )]);

        assert_eq!(
            html.matches("pv-match-card selected").count(),
            1,
            "got: {html}"
        );
        assert!(html.contains("pv-match-card"), "got: {html}");
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
