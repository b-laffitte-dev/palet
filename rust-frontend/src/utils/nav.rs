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
}
