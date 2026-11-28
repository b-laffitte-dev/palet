//! Helpers de navigation : section du menu active selon la route courante.

use crate::Route;

/// Href de la section de menu active pour une route donnée.
///
/// Les pages détail activent la section parente (ex. `/clubs/{id}` →
/// `/clubs`). La home et les pages sans section (404, compte, connexion…)
/// renvoient `""` : aucun lien actif.
pub(crate) fn active_section(route: &Route) -> &'static str {
    match route {
        Route::Championnats | Route::ChampionnatDetail { .. } => "/championnats",
        Route::Tournois | Route::TournamentDetail { .. } => "/tournois",
        Route::Clubs | Route::ClubDetail { .. } => "/clubs",
        Route::Joueurs | Route::PlayerDetail { .. } => "/joueurs",
        Route::Actualites | Route::ActualiteDetail { .. } => "/actualites",
        Route::Federation => "/federation",
        _ => "",
    }
}

/// Action décidée par le garde de route pour la route courante.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum GuardAction {
    /// La route est accessible telle quelle.
    Allow,
    /// Rediriger vers cette route.
    Redirect(Route),
}

/// Décide de l'accès à une route selon l'état de connexion.
///
/// - Pages publiques (fédération, actualités + détail, connexion, inscription,
///   404) : accessibles sans être connecté ; `connexion`/`inscription`
///   redirigent vers la home quand on est déjà connecté.
/// - Toute autre route exige d'être connecté ; sinon, redirection vers la
///   page fédération (page d'accueil publique).
pub(crate) fn guard_action(route: &Route, authenticated: bool) -> GuardAction {
    let public = matches!(
        route,
        Route::Federation
            | Route::Actualites
            | Route::ActualiteDetail { .. }
            | Route::Connexion
            | Route::Inscription
            | Route::NotFound
            | Route::CatchAll { .. }
    );
    match (public, authenticated) {
        (true, true) if matches!(route, Route::Connexion | Route::Inscription) => {
            GuardAction::Redirect(Route::Home)
        }
        (true, _) => GuardAction::Allow,
        (false, true) => GuardAction::Allow,
        (false, false) => GuardAction::Redirect(Route::Federation),
    }
}

/// Liens du menu principal selon l'état de connexion : un visiteur anonyme
/// ne voit que la fédération et les actualités.
const NAV_LINKS_ALL: &[(&str, &str)] = &[
    ("Championnat", "/championnats"),
    ("Tournois", "/tournois"),
    ("Clubs", "/clubs"),
    ("Joueurs", "/joueurs"),
    ("Fédération", "/federation"),
    ("Actualités", "/actualites"),
];

const NAV_LINKS_PUBLIC: &[(&str, &str)] =
    &[("Fédération", "/federation"), ("Actualités", "/actualites")];

pub(crate) fn nav_links(connected: bool) -> &'static [(&'static str, &'static str)] {
    if connected {
        NAV_LINKS_ALL
    } else {
        NAV_LINKS_PUBLIC
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chaque_section_active_sa_route() {
        let cases: Vec<(Route, &str)> = vec![
            (Route::Championnats, "/championnats"),
            (Route::Tournois, "/tournois"),
            (Route::Clubs, "/clubs"),
            (Route::Joueurs, "/joueurs"),
            (Route::Actualites, "/actualites"),
            (Route::Federation, "/federation"),
        ];
        for (route, expected) in cases {
            assert_eq!(active_section(&route), expected);
        }
    }

    #[test]
    fn les_pages_detail_activent_la_section_parente() {
        assert_eq!(
            active_section(&Route::ChampionnatDetail { id: "x".into() }),
            "/championnats"
        );
        assert_eq!(
            active_section(&Route::TournamentDetail { id: "x".into() }),
            "/tournois"
        );
        assert_eq!(
            active_section(&Route::ClubDetail { id: "x".into() }),
            "/clubs"
        );
        assert_eq!(
            active_section(&Route::PlayerDetail { id: "x".into() }),
            "/joueurs"
        );
        assert_eq!(
            active_section(&Route::ActualiteDetail { id: "x".into() }),
            "/actualites"
        );
    }

    #[test]
    fn home_et_pages_sans_section_n_activent_rien() {
        assert_eq!(active_section(&Route::Home), "");
        assert_eq!(active_section(&Route::Connexion), "");
        assert_eq!(active_section(&Route::NotFound), "");
        assert_eq!(
            active_section(&Route::CatchAll {
                rest: vec!["x".into()]
            }),
            ""
        );
    }

    #[test]
    fn anonyme_n_accede_qu_a_federation_et_actualites() {
        let publiques = [
            Route::Federation,
            Route::Actualites,
            Route::ActualiteDetail { id: "1".into() },
            Route::Connexion,
            Route::Inscription,
            Route::NotFound,
            Route::CatchAll {
                rest: vec!["x".into()],
            },
        ];
        for route in publiques {
            assert_eq!(
                guard_action(&route, false),
                GuardAction::Allow,
                "{route:?} devrait être publique"
            );
        }
    }

    #[test]
    fn anonyme_est_redirige_vers_federation_sur_les_routes_protegees() {
        let protegees = [
            Route::Home,
            Route::Championnats,
            Route::ChampionnatDetail { id: "1".into() },
            Route::Tournois,
            Route::TournamentDetail { id: "1".into() },
            Route::Clubs,
            Route::ClubDetail { id: "1".into() },
            Route::Joueurs,
            Route::PlayerDetail { id: "1".into() },
            Route::MonCompte,
            Route::MonClub,
            Route::Arbitrage,
            Route::ArbitrageMatch { id: "1".into() },
            Route::Recherche { q: "x".into() },
            Route::Notifications,
        ];
        for route in protegees {
            assert_eq!(
                guard_action(&route, false),
                GuardAction::Redirect(Route::Federation),
                "{route:?} devrait rediriger vers /federation"
            );
        }
    }

    #[test]
    fn connecte_accede_a_toutes_les_routes() {
        let routes = [
            Route::Home,
            Route::Championnats,
            Route::Clubs,
            Route::Joueurs,
            Route::Tournois,
            Route::Actualites,
            Route::Federation,
            Route::MonCompte,
            Route::MonClub,
            Route::Arbitrage,
            Route::Notifications,
            Route::Recherche { q: "x".into() },
        ];
        for route in routes {
            assert_eq!(
                guard_action(&route, true),
                GuardAction::Allow,
                "{route:?} devrait être accessible connecté"
            );
        }
    }

    #[test]
    fn connecte_sur_connexion_ou_inscription_est_redirige_vers_home() {
        assert_eq!(
            guard_action(&Route::Connexion, true),
            GuardAction::Redirect(Route::Home)
        );
        assert_eq!(
            guard_action(&Route::Inscription, true),
            GuardAction::Redirect(Route::Home)
        );
    }

    #[test]
    fn menu_anonyme_ne_contient_que_federation_et_actualites() {
        let liens = nav_links(false);
        assert_eq!(
            liens,
            &[("Fédération", "/federation"), ("Actualités", "/actualites")]
        );
    }

    #[test]
    fn menu_connecte_contient_toutes_les_sections() {
        let liens = nav_links(true);
        assert!(liens.contains(&("Championnat", "/championnats")));
        assert!(liens.contains(&("Tournois", "/tournois")));
        assert!(liens.contains(&("Clubs", "/clubs")));
        assert!(liens.contains(&("Joueurs", "/joueurs")));
        assert!(liens.contains(&("Fédération", "/federation")));
        assert!(liens.contains(&("Actualités", "/actualites")));
    }
}
