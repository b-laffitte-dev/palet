//! Service des compétitions "événement" : génération du calendrier round-robin
//! à partir des équipes inscrites et confirmées.

use crate::db::Database;
use crate::services::auth_service::AuthError;
use palet_shared::{CompetitionStatus, CreateMatchRequest, Match, UpdateCompetitionRequest};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

/// Génère les appariements d'un round-robin (méthode du cercle).
/// Si le nombre d'équipes est impair, un "bye" (Uuid nil) est ajouté et filtré :
/// chaque équipe joue exactement n-1 matchs (n pair) ou n-1 matchs aussi (n impair,
/// une équipe est exempte par journée).
pub fn round_robin_pairs(team_ids: &[Uuid]) -> Vec<(Uuid, Uuid)> {
    let n = team_ids.len();
    if n < 2 {
        return Vec::new();
    }
    let mut ids: Vec<Uuid> = team_ids.to_vec();
    if n % 2 == 1 {
        ids.push(Uuid::nil());
    }
    let m = ids.len();
    let rounds = m - 1;
    let mut rotated = ids;
    let mut pairs = Vec::new();
    for _ in 0..rounds {
        for i in 0..m / 2 {
            let a = rotated[i];
            let b = rotated[m - 1 - i];
            if a != Uuid::nil() && b != Uuid::nil() {
                pairs.push((a, b));
            }
        }
        let last = rotated.pop().expect("m >= 2");
        rotated.insert(1, last);
    }
    pairs
}

/// Crée les matchs round-robin d'une compétition à partir de ses inscriptions
/// confirmées, et passe la compétition en `in_progress`.
pub async fn generate_matches(
    db: &Database,
    competition_id: Uuid,
) -> Result<Vec<Match>, AuthError> {
    let event = db
        .competitions
        .get_competition(competition_id)
        .await
        .map_err(|e| AuthError::Internal(e.to_string()))?
        .ok_or_else(|| AuthError::Unauthorized("Compétition introuvable".to_string()))?;
    let team_ids = db
        .competitions
        .get_confirmed_team_ids(competition_id)
        .await
        .map_err(|e| AuthError::Internal(e.to_string()))?;
    let pairs = round_robin_pairs(&team_ids);
    if pairs.is_empty() {
        return Err(AuthError::Validation(
            "Au moins 2 équipes confirmées sont requises pour générer le calendrier".to_string(),
        ));
    }
    let base_date = event
        .start_date
        .unwrap_or_else(|| OffsetDateTime::now_utc() + Duration::days(7));
    let mut created = Vec::with_capacity(pairs.len());
    for (index, (team1_id, team2_id)) in pairs.into_iter().enumerate() {
        let journee = (index as i32) + 1;
        let request = CreateMatchRequest {
            competition_id,
            competition_type: event.competition_type.clone(),
            team1_id,
            team2_id,
            date: base_date + Duration::days((index as i64) * 7),
            location: event.location.clone(),
            journee: Some(journee),
            phase: None,
            round: Some(format!("J{journee}")),
            best_of: Some(13),
        };
        let m = db
            .matches
            .create_match(request)
            .await
            .map_err(|e| AuthError::Internal(e.to_string()))?;
        created.push(m);
    }
    let _ = db
        .competitions
        .update_competition(
            competition_id,
            UpdateCompetitionRequest {
                status: Some(CompetitionStatus::InProgress),
                ..Default::default()
            },
        )
        .await
        .map_err(|e| AuthError::Internal(e.to_string()))?;
    Ok(created)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(n: usize) -> Vec<Uuid> {
        (0..n).map(|_| Uuid::new_v4()).collect()
    }

    #[test]
    fn round_robin_4_equipes_donne_6_matchs_uniques_3_par_equipe() {
        let teams = ids(4);
        let pairs = round_robin_pairs(&teams);
        assert_eq!(pairs.len(), 6);
        let mut seen = std::collections::HashSet::new();
        for (a, b) in &pairs {
            assert!(teams.contains(a) && teams.contains(b));
            assert_ne!(a, b);
            let key = if a < b { (*a, *b) } else { (*b, *a) };
            assert!(seen.insert(key), "match en double : {a} vs {b}");
        }
        for t in &teams {
            let played = pairs.iter().filter(|(a, b)| a == t || b == t).count();
            assert_eq!(played, 3);
        }
    }

    #[test]
    fn round_robin_3_equipes_avec_bye_donne_3_matchs() {
        let teams = ids(3);
        let pairs = round_robin_pairs(&teams);
        assert_eq!(pairs.len(), 3);
        for (a, b) in &pairs {
            assert_ne!(a, &Uuid::nil());
            assert_ne!(b, &Uuid::nil());
        }
    }

    #[test]
    fn round_robin_5_equipes_donne_10_matchs_4_par_equipe() {
        let teams = ids(5);
        let pairs = round_robin_pairs(&teams);
        assert_eq!(pairs.len(), 10);
        for t in &teams {
            let played = pairs.iter().filter(|(a, b)| a == t || b == t).count();
            assert_eq!(played, 4);
        }
    }

    #[test]
    fn round_robin_moins_de_2_equipes_ne_genere_rien() {
        assert!(round_robin_pairs(&[]).is_empty());
        assert!(round_robin_pairs(&ids(1)).is_empty());
    }
}
