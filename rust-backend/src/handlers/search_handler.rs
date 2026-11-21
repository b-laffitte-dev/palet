//! Recherche globale — GET /recherche?q= (clubs, joueurs, équipes, compétitions, actualités)

use actix_web::{web, HttpResponse, Responder};
use palet_shared::{ApiResponse, SearchResults};
use serde::Deserialize;

use crate::services::stats_service::matches_query;
use crate::AppState;

/// Nombre max de résultats par section.
const SEARCH_LIMIT: usize = 8;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct SearchQuery {
    pub q: String,
}

/// Recherche globale dans les clubs, joueurs, équipes, compétitions et actualités publiées.
///
/// Filtrage en mémoire (les volumes restent faibles) ; à passer en SQL ILIKE
/// si les tables grossissent beaucoup.
#[utoipa::path(
    get,
    path = "/recherche",
    tag = "Recherche",
    params(
        ("q", Query, description = "Texte recherché (2 caractères minimum)")
    ),
    responses(
        (status = 200, description = "Résultats de recherche", body = SearchResults),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn search(state: web::Data<AppState>, query: web::Query<SearchQuery>) -> impl Responder {
    let q = query.q.trim().to_string();
    if q.chars().count() < 2 {
        return HttpResponse::Ok().json(ApiResponse {
            data: SearchResults::default(),
            message: Some("Requête trop courte (2 caractères minimum)".to_string()),
            success: true,
        });
    }

    let clubs = state.db.clubs.get_clubs().await.unwrap_or_default();
    let players = state.db.players.get_players().await.unwrap_or_default();
    let teams = state.db.teams.get_teams().await.unwrap_or_default();
    let competitions = state
        .db
        .competitions
        .get_competitions(None, None, None)
        .await
        .unwrap_or_default();
    let news = state
        .db
        .news
        .get_published_articles(None, None)
        .await
        .unwrap_or_default();

    let take =
        |v: Vec<palet_shared::Club>| -> Vec<_> { v.into_iter().take(SEARCH_LIMIT).collect() };

    let results = SearchResults {
        clubs: take(
            clubs
                .into_iter()
                .filter(|c| {
                    matches_query(&c.name, &q)
                        || matches_query(&c.city, &q)
                        || matches_query(&c.code, &q)
                        || matches_query(&c.league, &q)
                })
                .collect(),
        ),
        players: players
            .into_iter()
            .filter(|p| {
                matches_query(&p.full_name, &q)
                    || matches_query(&p.license_number, &q)
                    || matches_query(&p.email, &q)
            })
            .take(SEARCH_LIMIT)
            .collect(),
        teams: teams
            .into_iter()
            .filter(|t| matches_query(&t.name, &q))
            .take(SEARCH_LIMIT)
            .collect(),
        competitions: competitions
            .into_iter()
            .filter(|c| matches_query(&c.name, &q) || matches_query(&c.season, &q))
            .take(SEARCH_LIMIT)
            .collect(),
        news: news
            .into_iter()
            .filter(|n| matches_query(&n.title, &q) || matches_query(&n.content, &q))
            .take(SEARCH_LIMIT)
            .collect(),
    };

    HttpResponse::Ok().json(ApiResponse {
        data: results,
        message: Some("Résultats de recherche".to_string()),
        success: true,
    })
}
