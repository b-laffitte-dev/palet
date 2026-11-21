//! Statistiques (analytics) — GET /stats/joueur/{id}, /stats/club/{id},
//! /stats/competition/{id}. Calculs purs dans `stats_service`.

use actix_web::{web, HttpResponse, Responder};
use palet_shared::{ApiResponse, ClubStats, CompetitionStats, PlayerStats};
use uuid::Uuid;

use crate::services::stats_service::{
    compute_club_stats, compute_competition_stats, compute_player_stats,
};
use crate::AppState;

/// Statistiques d'un joueur (matchs joués, victoires, points, dispo).
#[utoipa::path(
    get,
    path = "/stats/joueur/{id}",
    tag = "Statistiques",
    params(("id", description = "Player ID")),
    responses(
        (status = 200, description = "Stats du joueur", body = PlayerStats),
        (status = 404, description = "Joueur introuvable")
    )
)]
pub async fn player_stats(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    if state
        .db
        .players
        .get_player(*id)
        .await
        .ok()
        .flatten()
        .is_none()
    {
        return HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Joueur introuvable".to_string()),
            success: false,
        });
    }
    let team_ids: Vec<Uuid> = state
        .db
        .teams
        .get_teams_of_player(*id)
        .await
        .unwrap_or_default()
        .iter()
        .map(|t| t.id)
        .collect();
    let mut matches = Vec::new();
    for t in &team_ids {
        if let Ok(ms) = state.db.matches.get_matches_by_team(*t).await {
            matches.extend(ms);
        }
    }
    let dispo = state
        .db
        .disponibilites
        .get_for_player(*id)
        .await
        .unwrap_or_default();
    let stats = compute_player_stats(*id, &team_ids, &matches, &dispo);
    HttpResponse::Ok().json(ApiResponse {
        data: stats,
        message: Some("Statistiques joueur".to_string()),
        success: true,
    })
}

/// Statistiques d'un club (toutes ses équipes).
#[utoipa::path(
    get,
    path = "/stats/club/{id}",
    tag = "Statistiques",
    params(("id", description = "Club ID")),
    responses(
        (status = 200, description = "Stats du club", body = ClubStats),
        (status = 404, description = "Club introuvable")
    )
)]
pub async fn club_stats(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    let Some(club) = state.db.clubs.get_club(*id).await.ok().flatten() else {
        return HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Club introuvable".to_string()),
            success: false,
        });
    };
    let teams = state
        .db
        .teams
        .get_teams_by_club(club.id)
        .await
        .unwrap_or_default();
    let team_ids: Vec<Uuid> = teams.iter().map(|t| t.id).collect();
    let mut matches = Vec::new();
    for t in &team_ids {
        if let Ok(ms) = state.db.matches.get_matches_by_team(*t).await {
            matches.extend(ms);
        }
    }
    let stats = compute_club_stats(
        club.id,
        &team_ids,
        teams.len() as i64,
        club.players_count as i64,
        &matches,
    );
    HttpResponse::Ok().json(ApiResponse {
        data: stats,
        message: Some("Statistiques club".to_string()),
        success: true,
    })
}

/// Statistiques d'une compétition (statuts des matchs, scoring).
#[utoipa::path(
    get,
    path = "/stats/competition/{id}",
    tag = "Statistiques",
    params(("id", description = "Competition ID")),
    responses(
        (status = 200, description = "Stats de la compétition", body = CompetitionStats),
        (status = 404, description = "Compétition introuvable")
    )
)]
pub async fn competition_stats(state: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    if state
        .db
        .competitions
        .get_competition(*id)
        .await
        .ok()
        .flatten()
        .is_none()
    {
        return HttpResponse::NotFound().json(ApiResponse::<()> {
            data: (),
            message: Some("Compétition introuvable".to_string()),
            success: false,
        });
    }
    let matches = state
        .db
        .matches
        .get_matches_by_competition(*id)
        .await
        .unwrap_or_default();
    let stats = compute_competition_stats(*id, &matches);
    HttpResponse::Ok().json(ApiResponse {
        data: stats,
        message: Some("Statistiques compétition".to_string()),
        success: true,
    })
}
