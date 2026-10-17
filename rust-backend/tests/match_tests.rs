//! Tests TDD pour les fonctionnalités Match (Palet Vendéen)
//!
//! Approche :
//! 1. Écrire les tests EN PREMIER (rouge)
//! 2. Implémenter le code minimal pour passer les tests (vert)
//! 3. Refactoriser (si nécessaire)

use palet_shared::{
    Championship, ChampionshipClassification, ChampionshipTeam, ClassificationPrimary,
    ClassificationSecondary, ClassificationTertiary, Club, CompetitionCategory, CompetitionFormat,
    CompetitionMaterial, CompetitionStatus, CompetitionType, CreateMatchRequest, CreateTournamentRequest,
    Federations, MancheScore, Match, MatchResult, MatchStatus, MatchWinner, Player, PointsType, Team,
    TeamClassificationStatus, TeamStats, Tournament, TournamentMatch, TournamentPhase,
    TournamentPhaseType, UpdateMatchRequest,
};
use std::sync::Arc;
use uuid::Uuid;

// ============================================
// MOCK DATABASE POUR TESTS
// ============================================

/// Mock de la base de données pour les tests Match
#[derive(Clone, Default)]
pub struct MockMatchDatabase {
    pub matches: Vec<Match>,
    pub championships: Vec<Championship>,
    pub tournament_matches: Vec<TournamentMatch>,
    pub should_fail: bool,
}

impl MockMatchDatabase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_matches(matches: Vec<Match>) -> Self {
        Self {
            matches,
            ..Default::default()
        }
    }

    pub fn with_championships(championships: Vec<Championship>) -> Self {
        Self {
            championships,
            ..Default::default()
        }
    }

    pub fn set_fail(&mut self, fail: bool) {
        self.should_fail = fail;
    }

    /// Simule la vérification de la connexion DB
    pub async fn check_connection(&self) -> bool {
        !self.should_fail
    }

    /// Simule l'initialisation du schéma
    pub async fn initialize(&self) -> Result<(), String> {
        if self.should_fail {
            Err("Connection failed".to_string())
        } else {
            Ok(())
        }
    }
}

// ============================================
// TESTS POUR LES MATCHS (Palet)
// ============================================

#[cfg(test)]
mod match_tests {
    use super::*;
    use chrono::Utc;

    // ============================================
    // 1. TESTS DE CRÉATION DE MATCH
    // ============================================

    #[tokio::test]
    async fn test_create_match_should_return_created_match() {
        // GIVEN: Une requête de création de match valide
        let mock_db = MockMatchDatabase::new();
        let request = CreateMatchRequest {
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Tournament,
            team1_id: Uuid::new_v4(),
            team2_id: Uuid::new_v4(),
            date: Utc::now(),
            location: Some("Salle des Essarts".to_string()),
            journee: Some(1),
            phase: Some("Phase de poules".to_string()),
            round: Some("Round 1".to_string()),
            best_of: Some(13), // 13 points pour gagner
        };

        // WHEN: On crée le match
        // (À implémenter dans match_handler.rs)
        let new_match = Match {
            id: Uuid::new_v4(),
            competition_id: request.competition_id,
            competition_type: request.competition_type,
            journee: request.journee,
            phase: request.phase,
            round: request.round,
            team1_id: request.team1_id,
            team2_id: request.team2_id,
            team1_score: None,
            team2_score: None,
            winner: None,
            manches: None,
            best_of: request.best_of,
            current_manche: Some(0),
            current_scores: Some(palet_shared::CurrentScores {
                team1: 0,
                team2: 0,
                manche: 0,
            }),
            status: MatchStatus::Scheduled,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: request.date,
            time: None,
            location: request.location,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: None,
            ended_at: None,
        };

        // THEN: Le match doit être créé avec les bonnes propriétés
        assert_eq!(new_match.team1_id, request.team1_id);
        assert_eq!(new_match.team2_id, request.team2_id);
        assert_eq!(new_match.competition_type, CompetitionType::Tournament);
        assert_eq!(new_match.best_of, Some(13));
        assert_eq!(new_match.status, MatchStatus::Scheduled);
        assert!(!new_match.verified);
    }

    #[tokio::test]
    async fn test_create_match_with_15_points_for_final() {
        // GIVEN: Un match en finale (15 points)
        let request = CreateMatchRequest {
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Tournament,
            team1_id: Uuid::new_v4(),
            team2_id: Uuid::new_v4(),
            date: Utc::now(),
            location: None,
            journee: None,
            phase: Some("Finale".to_string()),
            round: None,
            best_of: Some(15), // 15 points pour la finale
        };

        // WHEN: Création du match
        let new_match = Match {
            id: Uuid::new_v4(),
            competition_id: request.competition_id,
            competition_type: request.competition_type,
            journee: request.journee,
            phase: request.phase.clone(),
            round: request.round,
            team1_id: request.team1_id,
            team2_id: request.team2_id,
            team1_score: None,
            team2_score: None,
            winner: None,
            manches: None,
            best_of: request.best_of,
            current_manche: Some(0),
            current_scores: None,
            status: MatchStatus::Scheduled,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: request.date,
            time: None,
            location: None,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: None,
            ended_at: None,
        };

        // THEN: Le best_of doit être 15 pour la finale
        assert_eq!(new_match.best_of, Some(15));
        assert_eq!(new_match.phase, Some("Finale".to_string()));
    }

    // ============================================
    // 2. TESTS DE GESTION DES MANCHES
    // ============================================

    #[tokio::test]
    async fn test_add_manche_score_should_update_match() {
        // GIVEN: Un match en cours avec une manche
        let mut match_in_progress = Match {
            id: Uuid::new_v4(),
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Championnat,
            journee: Some(1),
            phase: None,
            round: None,
            team1_id: Uuid::new_v4(),
            team2_id: Uuid::new_v4(),
            team1_score: Some(0),
            team2_score: Some(0),
            winner: None,
            manches: Some(vec![MancheScore {
                number: 1,
                team1: 3,
                team2: 1,
                winner: Some(MatchWinner::Team1),
            }]),
            best_of: Some(13),
            current_manche: Some(1),
            current_scores: Some(palet_shared::CurrentScores {
                team1: 3,
                team2: 1,
                manche: 1,
            }),
            status: MatchStatus::InProgress,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: Utc::now(),
            time: None,
            location: None,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: Some(Utc::now()),
            ended_at: None,
        };

        // WHEN: On ajoute une nouvelle manche
        let new_manche = MancheScore {
            number: 2,
            team1: 2,
            team2: 4,
            winner: Some(MatchWinner::Team2),
        };

        match_in_progress.manches.as_mut().unwrap().push(new_manche);
        match_in_progress.current_manche = Some(2);
        match_in_progress.current_scores = Some(palet_shared::CurrentScores {
            team1: 5,
            team2: 5,
            manche: 2,
        });

        // THEN: Le match doit avoir 2 manches
        assert_eq!(match_in_progress.manches.as_ref().unwrap().len(), 2);
        assert_eq!(match_in_progress.current_manche, Some(2));
        assert_eq!(
            match_in_progress.current_scores.as_ref().unwrap().team1,
            5
        );
        assert_eq!(
            match_in_progress.current_scores.as_ref().unwrap().team2,
            5
        );
    }

    #[tokio::test]
    async fn test_match_completed_when_team_reaches_13_points() {
        // GIVEN: Un match où team1 atteint 13 points
        let mut match_in_progress = Match {
            id: Uuid::new_v4(),
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Championnat,
            journee: Some(1),
            phase: None,
            round: None,
            team1_id: Uuid::new_v4(),
            team2_id: Uuid::new_v4(),
            team1_score: Some(13),
            team2_score: Some(8),
            winner: None,
            manches: Some(vec![
                MancheScore {
                    number: 1,
                    team1: 5,
                    team2: 3,
                    winner: Some(MatchWinner::Team1),
                },
                MancheScore {
                    number: 2,
                    team1: 4,
                    team2: 2,
                    winner: Some(MatchWinner::Team1),
                },
                MancheScore {
                    number: 3,
                    team1: 4,
                    team2: 3,
                    winner: Some(MatchWinner::Team1),
                },
            ]),
            best_of: Some(13),
            current_manche: Some(3),
            current_scores: Some(palet_shared::CurrentScores {
                team1: 13,
                team2: 8,
                manche: 3,
            }),
            status: MatchStatus::InProgress,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: Utc::now(),
            time: None,
            location: None,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: Some(Utc::now()),
            ended_at: None,
        };

        // WHEN: On vérifie si le match est terminé
        if match_in_progress.team1_score.unwrap_or(0) >= match_in_progress.best_of.unwrap_or(13)
            || match_in_progress.team2_score.unwrap_or(0) >= match_in_progress.best_of.unwrap_or(13)
        {
            match_in_progress.status = MatchStatus::Completed;
            match_in_progress.winner = Some(MatchWinner::Team1);
            match_in_progress.ended_at = Some(Utc::now());
        }

        // THEN: Le match doit être marqué comme terminé
        assert_eq!(match_in_progress.status, MatchStatus::Completed);
        assert_eq!(match_in_progress.winner, Some(MatchWinner::Team1));
    }

    #[tokio::test]
    async fn test_match_goes_to_15_points_in_final() {
        // GIVEN: Un match en finale à 12-12
        let mut match_in_progress = Match {
            id: Uuid::new_v4(),
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Tournament,
            journee: None,
            phase: Some("Finale".to_string()),
            round: None,
            team1_id: Uuid::new_v4(),
            team2_id: Uuid::new_v4(),
            team1_score: Some(12),
            team2_score: Some(12),
            winner: None,
            manches: None,
            best_of: Some(15), // Finale = 15 points
            current_manche: Some(10),
            current_scores: None,
            status: MatchStatus::InProgress,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: Utc::now(),
            time: None,
            location: None,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: Some(Utc::now()),
            ended_at: None,
        };

        // WHEN: Team1 marque 3 points de plus
        match_in_progress.team1_score = Some(15);

        // THEN: Le match doit passer à 15 points et être terminé
        if match_in_progress.team1_score.unwrap_or(0) >= match_in_progress.best_of.unwrap_or(15) {
            match_in_progress.status = MatchStatus::Completed;
            match_in_progress.winner = Some(MatchWinner::Team1);
        }

        assert_eq!(match_in_progress.status, MatchStatus::Completed);
        assert_eq!(match_in_progress.winner, Some(MatchWinner::Team1));
        assert_eq!(match_in_progress.best_of, Some(15));
    }

    // ============================================
    // 3. TESTS DE VALIDATION DES RÈGLES DU PALET
    // ============================================

    #[tokio::test]
    async fn test_manche_score_validation() {
        // GIVEN: Une manche avec des scores valides
        // Règle: 1 point par palet plus proche du maître que le premier adverse

        // Scénario: Team1 a 3 palets plus proches, Team2 a 1
        let manche = MancheScore {
            number: 1,
            team1: 3,
            team2: 1,
            winner: Some(MatchWinner::Team1),
        };

        // THEN: Le score doit être valide (max 6 par manche pour doublette)
        assert!(manche.team1 >= 0 && manche.team1 <= 6);
        assert!(manche.team2 >= 0 && manche.team2 <= 6);
        assert_ne!(manche.team1, manche.team2); // Pas d'égalité possible
    }

    #[tokio::test]
    async fn test_match_status_transitions() {
        // GIVEN: Un match initialement scheduled
        let mut match_obj = Match {
            id: Uuid::new_v4(),
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Championnat,
            journee: Some(1),
            phase: None,
            round: None,
            team1_id: Uuid::new_v4(),
            team2_id: Uuid::new_v4(),
            team1_score: None,
            team2_score: None,
            winner: None,
            manches: None,
            best_of: Some(13),
            current_manche: Some(0),
            current_scores: None,
            status: MatchStatus::Scheduled,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: Utc::now(),
            time: None,
            location: None,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: None,
            ended_at: None,
        };

        // WHEN: Le match commence
        match_obj.status = MatchStatus::InProgress;
        match_obj.started_at = Some(Utc::now());

        // THEN: Statut doit être InProgress
        assert_eq!(match_obj.status, MatchStatus::InProgress);

        // WHEN: Le match est terminé
        match_obj.status = MatchStatus::Completed;
        match_obj.ended_at = Some(Utc::now());
        match_obj.winner = Some(MatchWinner::Team1);

        // THEN: Statut doit être Completed
        assert_eq!(match_obj.status, MatchStatus::Completed);
        assert!(match_obj.ended_at.is_some());
    }

    // ============================================
    // 4. TESTS DE RÉCUPÉRATION DES MATCHS
    // ============================================

    #[tokio::test]
    async fn test_get_matches_by_competition() {
        // GIVEN: Plusieurs matchs pour une compétition
        let competition_id = Uuid::new_v4();
        let matches = vec![
            Match {
                id: Uuid::new_v4(),
                competition_id,
                competition_type: CompetitionType::Championnat,
                journee: Some(1),
                phase: None,
                round: None,
                team1_id: Uuid::new_v4(),
                team2_id: Uuid::new_v4(),
                team1_score: Some(13),
                team2_score: Some(8),
                winner: Some(MatchWinner::Team1),
                manches: None,
                best_of: Some(13),
                current_manche: None,
                current_scores: None,
                status: MatchStatus::Completed,
                verified: true,
                verified_by: None,
                verified_at: None,
                date: Utc::now(),
                time: None,
                location: None,
                duration: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                started_at: None,
                ended_at: None,
            },
            Match {
                id: Uuid::new_v4(),
                competition_id,
                competition_type: CompetitionType::Championnat,
                journee: Some(2),
                phase: None,
                round: None,
                team1_id: Uuid::new_v4(),
                team2_id: Uuid::new_v4(),
                team1_score: None,
                team2_score: None,
                winner: None,
                manches: None,
                best_of: Some(13),
                current_manche: None,
                current_scores: None,
                status: MatchStatus::Scheduled,
                verified: false,
                verified_by: None,
                verified_at: None,
                date: Utc::now(),
                time: None,
                location: None,
                duration: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                started_at: None,
                ended_at: None,
            },
        ];

        let mock_db = MockMatchDatabase::with_matches(matches);

        // WHEN: On filtre par competition_id
        let filtered_matches: Vec<Match> = mock_db
            .matches
            .into_iter()
            .filter(|m| m.competition_id == competition_id)
            .collect();

        // THEN: On doit avoir 2 matchs
        assert_eq!(filtered_matches.len(), 2);
        assert!(
            filtered_matches
                .iter()
                .all(|m| m.competition_id == competition_id)
        );
    }

    #[tokio::test]
    async fn test_get_matches_by_status() {
        // GIVEN: Des matchs avec différents statuts
        let matches = vec![
            Match {
                id: Uuid::new_v4(),
                competition_id: Uuid::new_v4(),
                competition_type: CompetitionType::Tournament,
                journee: None,
                phase: None,
                round: None,
                team1_id: Uuid::new_v4(),
                team2_id: Uuid::new_v4(),
                team1_score: None,
                team2_score: None,
                winner: None,
                manches: None,
                best_of: Some(13),
                current_manche: None,
                current_scores: None,
                status: MatchStatus::Scheduled,
                verified: false,
                verified_by: None,
                verified_at: None,
                date: Utc::now(),
                time: None,
                location: None,
                duration: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                started_at: None,
                ended_at: None,
            },
            Match {
                id: Uuid::new_v4(),
                competition_id: Uuid::new_v4(),
                competition_type: CompetitionType::Tournament,
                journee: None,
                phase: None,
                round: None,
                team1_id: Uuid::new_v4(),
                team2_id: Uuid::new_v4(),
                team1_score: Some(13),
                team2_score: Some(10),
                winner: Some(MatchWinner::Team1),
                manches: None,
                best_of: Some(13),
                current_manche: None,
                current_scores: None,
                status: MatchStatus::Completed,
                verified: true,
                verified_by: None,
                verified_at: None,
                date: Utc::now(),
                time: None,
                location: None,
                duration: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                started_at: None,
                ended_at: None,
            },
        ];

        let mock_db = MockMatchDatabase::with_matches(matches);

        // WHEN: On filtre par statut Completed
        let completed_matches: Vec<Match> = mock_db
            .matches
            .into_iter()
            .filter(|m| m.status == MatchStatus::Completed)
            .collect();

        // THEN: On doit avoir 1 match terminé
        assert_eq!(completed_matches.len(), 1);
        assert_eq!(completed_matches[0].status, MatchStatus::Completed);
    }

    // ============================================
    // 5. TESTS DE MISE À JOUR DES MATCHS
    // ============================================

    #[tokio::test]
    async fn test_update_match_scores() {
        // GIVEN: Un match en cours
        let mut match_in_progress = Match {
            id: Uuid::new_v4(),
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Championnat,
            journee: Some(1),
            phase: None,
            round: None,
            team1_id: Uuid::new_v4(),
            team2_id: Uuid::new_v4(),
            team1_score: Some(5),
            team2_score: Some(3),
            winner: None,
            manches: None,
            best_of: Some(13),
            current_manche: None,
            current_scores: None,
            status: MatchStatus::InProgress,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: Utc::now(),
            time: None,
            location: None,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: Some(Utc::now()),
            ended_at: None,
        };

        // WHEN: On met à jour les scores
        let update_request = UpdateMatchRequest {
            team1_score: Some(8),
            team2_score: Some(3),
            status: Some(MatchStatus::InProgress),
            winner: None,
            verified: Some(false),
        };

        match_in_progress.team1_score = update_request.team1_score;
        match_in_progress.team2_score = update_request.team2_score;
        match_in_progress.updated_at = Utc::now();

        // THEN: Les scores doivent être mis à jour
        assert_eq!(match_in_progress.team1_score, Some(8));
        assert_eq!(match_in_progress.team2_score, Some(3));
    }

    #[tokio::test]
    async fn test_verify_match() {
        // GIVEN: Un match terminé non vérifié
        let mut match_completed = Match {
            id: Uuid::new_v4(),
            competition_id: Uuid::new_v4(),
            competition_type: CompetitionType::Tournament,
            journee: None,
            phase: None,
            round: None,
            team1_id: Uuid::new_v4(),
            team2_id: Uuid::new_v4(),
            team1_score: Some(13),
            team2_score: Some(10),
            winner: Some(MatchWinner::Team1),
            manches: None,
            best_of: Some(13),
            current_manche: None,
            current_scores: None,
            status: MatchStatus::Completed,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: Utc::now(),
            time: None,
            location: None,
            duration: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: Some(Utc::now()),
            ended_at: Some(Utc::now()),
        };

        // WHEN: On vérifie le match
        let verifier_id = Uuid::new_v4();
        match_completed.verified = true;
        match_completed.verified_by = Some(verifier_id);
        match_completed.verified_at = Some(Utc::now());

        // THEN: Le match doit être marqué comme vérifié
        assert!(match_completed.verified);
        assert_eq!(match_completed.verified_by, Some(verifier_id));
        assert!(match_completed.verified_at.is_some());
    }
}
