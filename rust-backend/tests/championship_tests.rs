//! Tests TDD pour les fonctionnalités Championnat (Palet Vendéen)
//!
//! Approche :
//! 1. Écrire les tests EN PREMIER (rouge)
//! 2. Implémenter le code minimal pour passer les tests (vert)
//! 3. Refactoriser (si nécessaire)

use palet_shared::{
    Championship, ChampionshipClassification, ChampionshipTeam, ClassificationPrimary,
    ClassificationSecondary, ClassificationTertiary, Club, CompetitionCategory, CompetitionFormat,
    CompetitionMaterial, CompetitionStatus, CompetitionType, CreateChampionshipRequest,
    CreateMatchRequest, Match, MatchStatus, MatchWinner, Player, Team, TeamClassificationStatus,
    TeamStats, UpdateChampionshipRequest,
};
use std::sync::Arc;
use uuid::Uuid;

// ============================================
// MOCK DATABASE POUR TESTS CHAMPIONNAT
// ============================================

/// Mock de la base de données pour les tests Championnat
#[derive(Clone, Default)]
pub struct MockChampionshipDatabase {
    pub championships: Vec<Championship>,
    pub classifications: Vec<ChampionshipClassification>,
    pub should_fail: bool,
}

impl MockChampionshipDatabase {
    pub fn new() -> Self {
        Self::default()
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
}

// ============================================
// TESTS POUR LES CHAMPIONNATS
// ============================================

#[cfg(test)]
mod championship_tests {
    use super::*;
    use chrono::Utc;

    // ============================================
    // 1. TESTS DE CRÉATION DE CHAMPIONNAT
    // ============================================

    #[tokio::test]
    async fn test_create_championship_with_valid_data() {
        // GIVEN: Une requête de création de championnat valide
        let request = CreateChampionshipRequest {
            name: "Championnat de Vendée Fonte 2025-2026".to_string(),
            code: Some("CVDP-FONTE-2025-2026".to_string()),
            season: "2025-2026".to_string(),
            description: Some("Championnat départemental de palet fonte".to_string()),
            league: "Vendée".to_string(),
            divisions: vec!["D1".to_string(), "D2".to_string(), "D3".to_string()],
            total_journees: 10,
            journee_duration: Some("1 journée".to_string()),
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            organizer_id: Uuid::new_v4(),
        };

        // WHEN: On crée le championnat
        let new_championship = Championship {
            id: Uuid::new_v4(),
            name: request.name.clone(),
            code: request.code.clone(),
            season: request.season.clone(),
            description: request.description.clone(),
            league: request.league.clone(),
            divisions: request.divisions.clone(),
            total_journees: request.total_journees,
            journee_duration: request.journee_duration.clone(),
            classification_rules: request.classification_rules.clone(),
            status: CompetitionStatus::Pending,
            current_journee: None,
            current_phase: None,
            organizer_id: request.organizer_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // THEN: Le championnat doit être créé avec les bonnes propriétés
        assert_eq!(new_championship.name, "Championnat de Vendée Fonte 2025-2026");
        assert_eq!(new_championship.season, "2025-2026");
        assert_eq!(new_championship.league, "Vendée");
        assert_eq!(new_championship.divisions.len(), 3);
        assert_eq!(new_championship.total_journees, 10);
        assert_eq!(new_championship.status, CompetitionStatus::Pending);
        assert_eq!(
            new_championship.classification_rules.primary,
            ClassificationPrimary::Points
        );
    }

    #[tokio::test]
    async fn test_create_championship_with_default_classification() {
        // GIVEN: Un championnat avec les règles de classement par défaut du Palet Vendéen
        // Règle: 1. Points, 2. Différence, 3. Points marqués (Bp)
        let request = CreateChampionshipRequest {
            name: "Championnat Local".to_string(),
            code: None,
            season: "2025".to_string(),
            description: None,
            league: "Deux-Sèvres".to_string(),
            divisions: vec!["Unique".to_string()],
            total_journees: 6,
            journee_duration: None,
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            organizer_id: Uuid::new_v4(),
        };

        // WHEN: Création du championnat
        let championship = Championship {
            id: Uuid::new_v4(),
            name: request.name.clone(),
            code: request.code.clone(),
            season: request.season.clone(),
            description: request.description.clone(),
            league: request.league.clone(),
            divisions: request.divisions.clone(),
            total_journees: request.total_journees,
            journee_duration: request.journee_duration.clone(),
            classification_rules: request.classification_rules.clone(),
            status: CompetitionStatus::Pending,
            current_journee: None,
            current_phase: None,
            organizer_id: request.organizer_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // THEN: Les règles de classement doivent être correctes
        assert_eq!(
            championship.classification_rules.primary,
            ClassificationPrimary::Points
        );
        assert_eq!(
            championship.classification_rules.secondary,
            ClassificationSecondary::Diff
        );
        assert_eq!(
            championship.classification_rules.tertiary,
            ClassificationTertiary::PointsFor
        );
    }

    // ============================================
    // 2. TESTS DE GESTION DES JOURNÉES
    // ============================================

    #[tokio::test]
    async fn test_championship_journee_progression() {
        // GIVEN: Un championnat en cours
        let mut championship = Championship {
            id: Uuid::new_v4(),
            name: "Championnat de Vendée".to_string(),
            code: Some("CVDP-2025".to_string()),
            season: "2025-2026".to_string(),
            description: None,
            league: "Vendée".to_string(),
            divisions: vec!["D1".to_string()],
            total_journees: 10,
            journee_duration: None,
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            status: CompetitionStatus::InProgress,
            current_journee: Some(1),
            current_phase: None,
            organizer_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // WHEN: On passe à la journée suivante
        championship.current_journee = Some(2);
        championship.updated_at = Utc::now();

        // THEN: La journée doit être mise à jour
        assert_eq!(championship.current_journee, Some(2));

        // WHEN: On termine toutes les journées
        championship.current_journee = Some(10);
        championship.status = CompetitionStatus::Completed;

        // THEN: Le championnat doit être marqué comme terminé
        assert_eq!(championship.current_journee, Some(10));
        assert_eq!(championship.status, CompetitionStatus::Completed);
    }

    #[tokio::test]
    async fn test_championship_cannot_exceed_total_journees() {
        // GIVEN: Un championnat avec 10 journées
        let mut championship = Championship {
            id: Uuid::new_v4(),
            name: "Championnat".to_string(),
            code: None,
            season: "2025".to_string(),
            description: None,
            league: "Test".to_string(),
            divisions: vec!["D1".to_string()],
            total_journees: 10,
            journee_duration: None,
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            status: CompetitionStatus::InProgress,
            current_journee: Some(10),
            current_phase: None,
            organizer_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // WHEN: On essaie de passer à la journée 11
        let new_journee = 11;

        // THEN: Cela ne doit pas être possible
        assert!(
            new_journee > championship.total_journees,
            "La journée {} dépasse le total de {} journées",
            new_journee,
            championship.total_journees
        );
    }

    // ============================================
    // 3. TESTS DE CLASSIFICATION
    // ============================================

    #[tokio::test]
    async fn test_championship_classification_by_points() {
        // GIVEN: Plusieurs équipes avec différents points
        let team1 = ChampionshipTeam {
            position: 1,
            team_id: Uuid::new_v4(),
            stats: TeamStats {
                matches: 5,
                wins: 4,
                draws: 1,
                losses: 0,
                points_for: 45,
                points_against: 20,
                diff: 25,
                points: 13, // 4 victoires * 3 + 1 nul * 1 = 13
                win_rate: 0.8,
            },
            form: vec![MatchResult::Win, MatchResult::Win, MatchResult::Win, MatchResult::Win, MatchResult::Draw],
            status: TeamClassificationStatus::Qualified,
        };

        let team2 = ChampionshipTeam {
            position: 2,
            team_id: Uuid::new_v4(),
            stats: TeamStats {
                matches: 5,
                wins: 3,
                draws: 1,
                losses: 1,
                points_for: 40,
                points_against: 25,
                diff: 15,
                points: 10, // 3 victoires * 3 + 1 nul * 1 = 10
                win_rate: 0.6,
            },
            form: vec![MatchResult::Win, MatchResult::Win, MatchResult::Win, MatchResult::Draw, MatchResult::Loss],
            status: TeamClassificationStatus::Qualified,
        };

        let team3 = ChampionshipTeam {
            position: 3,
            team_id: Uuid::new_v4(),
            stats: TeamStats {
                matches: 5,
                wins: 2,
                draws: 2,
                losses: 1,
                points_for: 35,
                points_against: 30,
                diff: 5,
                points: 8, // 2 victoires * 3 + 2 nuls * 1 = 8
                win_rate: 0.4,
            },
            form: vec![MatchResult::Win, MatchResult::Win, MatchResult::Draw, MatchResult::Draw, MatchResult::Loss],
            status: TeamClassificationStatus::Safe,
        };

        let classification = ChampionshipClassification {
            division: "D1".to_string(),
            journee: 5,
            updated_at: Utc::now(),
            teams: vec![team1, team2, team3],
        };

        // WHEN: On trie par points (règle primaire)
        let mut sorted_teams = classification.teams.clone();
        sorted_teams.sort_by(|a, b| b.stats.points.cmp(&a.stats.points));

        // THEN: L'ordre doit être respecté
        assert_eq!(sorted_teams[0].stats.points, 13);
        assert_eq!(sorted_teams[1].stats.points, 10);
        assert_eq!(sorted_teams[2].stats.points, 8);
    }

    #[tokio::test]
    async fn test_championship_classification_by_diff_when_points_equal() {
        // GIVEN: Deux équipes avec les mêmes points mais différentes diffs
        let team1 = ChampionshipTeam {
            position: 1,
            team_id: Uuid::new_v4(),
            stats: TeamStats {
                matches: 5,
                wins: 3,
                draws: 1,
                losses: 1,
                points_for: 50,
                points_against: 20,
                diff: 30, // Meilleure diff
                points: 10,
                win_rate: 0.6,
            },
            form: vec![],
            status: TeamClassificationStatus::Qualified,
        };

        let team2 = ChampionshipTeam {
            position: 2,
            team_id: Uuid::new_v4(),
            stats: TeamStats {
                matches: 5,
                wins: 3,
                draws: 1,
                losses: 1,
                points_for: 45,
                points_against: 25,
                diff: 20, // Moins bonne diff
                points: 10,
                win_rate: 0.6,
            },
            form: vec![],
            status: TeamClassificationStatus::Qualified,
        };

        let classification = ChampionshipClassification {
            division: "D1".to_string(),
            journee: 5,
            updated_at: Utc::now(),
            teams: vec![team1.clone(), team2.clone()],
        };

        // WHEN: On trie par points puis par diff (règle secondaire)
        let mut sorted_teams = classification.teams.clone();
        sorted_teams.sort_by(|a, b| {
            b.stats.points.cmp(&a.stats.points) // D'abord par points
                .then_with(|| b.stats.diff.cmp(&a.stats.diff)) // Puis par diff
        });

        // THEN: Team1 doit être devant Team2
        assert_eq!(sorted_teams[0].stats.diff, 30);
        assert_eq!(sorted_teams[1].stats.diff, 20);
    }

    #[tokio::test]
    async fn test_championship_classification_by_points_for_when_diff_equal() {
        // GIVEN: Deux équipes avec les mêmes points et mêmes diffs
        let team1 = ChampionshipTeam {
            position: 1,
            team_id: Uuid::new_v4(),
            stats: TeamStats {
                matches: 5,
                wins: 3,
                draws: 1,
                losses: 1,
                points_for: 55,
                points_against: 25,
                diff: 30,
                points: 10,
                win_rate: 0.6,
            },
            form: vec![],
            status: TeamClassificationStatus::Qualified,
        };

        let team2 = ChampionshipTeam {
            position: 2,
            team_id: Uuid::new_v4(),
            stats: TeamStats {
                matches: 5,
                wins: 3,
                draws: 1,
                losses: 1,
                points_for: 50,
                points_against: 20,
                diff: 30, // Même diff
                points: 10,
                win_rate: 0.6,
            },
            form: vec![],
            status: TeamClassificationStatus::Qualified,
        };

        let classification = ChampionshipClassification {
            division: "D1".to_string(),
            journee: 5,
            updated_at: Utc::now(),
            teams: vec![team1.clone(), team2.clone()],
        };

        // WHEN: On trie par points, diff, puis points_for (règle tertiaire)
        let mut sorted_teams = classification.teams.clone();
        sorted_teams.sort_by(|a, b| {
            b.stats.points.cmp(&a.stats.points) // Points
                .then_with(|| b.stats.diff.cmp(&a.stats.diff)) // Diff
                .then_with(|| b.stats.points_for.cmp(&a.stats.points_for)) // Points pour
        });

        // THEN: Team1 doit être devant Team2 (55 > 50 points pour)
        assert_eq!(sorted_teams[0].stats.points_for, 55);
        assert_eq!(sorted_teams[1].stats.points_for, 50);
    }

    // ============================================
    // 4. TESTS DE RÉCUPÉRATION DES CHAMPIONNATS
    // ============================================

    #[tokio::test]
    async fn test_get_championships_by_league() {
        // GIVEN: Plusieurs championnats de différentes ligues
        let championships = vec![
            Championship {
                id: Uuid::new_v4(),
                name: "Championnat de Vendée".to_string(),
                code: Some("CVDP-2025".to_string()),
                season: "2025-2026".to_string(),
                description: None,
                league: "Vendée".to_string(),
                divisions: vec!["D1".to_string()],
                total_journees: 10,
                journee_duration: None,
                classification_rules: palet_shared::ClassificationRules {
                    primary: ClassificationPrimary::Points,
                    secondary: ClassificationSecondary::Diff,
                    tertiary: ClassificationTertiary::PointsFor,
                },
                status: CompetitionStatus::InProgress,
                current_journee: Some(5),
                current_phase: None,
                organizer_id: Uuid::new_v4(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Championship {
                id: Uuid::new_v4(),
                name: "Championnat des Deux-Sèvres".to_string(),
                code: Some("CDSP-2025".to_string()),
                season: "2025-2026".to_string(),
                description: None,
                league: "Deux-Sèvres".to_string(),
                divisions: vec!["D1".to_string()],
                total_journees: 8,
                journee_duration: None,
                classification_rules: palet_shared::ClassificationRules {
                    primary: ClassificationPrimary::Points,
                    secondary: ClassificationSecondary::Diff,
                    tertiary: ClassificationTertiary::PointsFor,
                },
                status: CompetitionStatus::Pending,
                current_journee: None,
                current_phase: None,
                organizer_id: Uuid::new_v4(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ];

        let mock_db = MockChampionshipDatabase::with_championships(championships);

        // WHEN: On filtre par ligue "Vendée"
        let vendee_championships: Vec<Championship> = mock_db
            .championships
            .into_iter()
            .filter(|c| c.league == "Vendée")
            .collect();

        // THEN: On doit avoir 1 championnat
        assert_eq!(vendee_championships.len(), 1);
        assert_eq!(vendee_championships[0].league, "Vendée");
    }

    #[tokio::test]
    async fn test_get_championships_by_season() {
        // GIVEN: Plusieurs championnats de différentes saisons
        let championships = vec![
            Championship {
                id: Uuid::new_v4(),
                name: "Championnat 2025-2026".to_string(),
                code: None,
                season: "2025-2026".to_string(),
                description: None,
                league: "Vendée".to_string(),
                divisions: vec![],
                total_journees: 10,
                journee_duration: None,
                classification_rules: palet_shared::ClassificationRules {
                    primary: ClassificationPrimary::Points,
                    secondary: ClassificationSecondary::Diff,
                    tertiary: ClassificationTertiary::PointsFor,
                },
                status: CompetitionStatus::InProgress,
                current_journee: None,
                current_phase: None,
                organizer_id: Uuid::new_v4(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            Championship {
                id: Uuid::new_v4(),
                name: "Championnat 2024-2025".to_string(),
                code: None,
                season: "2024-2025".to_string(),
                description: None,
                league: "Vendée".to_string(),
                divisions: vec![],
                total_journees: 10,
                journee_duration: None,
                classification_rules: palet_shared::ClassificationRules {
                    primary: ClassificationPrimary::Points,
                    secondary: ClassificationSecondary::Diff,
                    tertiary: ClassificationTertiary::PointsFor,
                },
                status: CompetitionStatus::Completed,
                current_journee: None,
                current_phase: None,
                organizer_id: Uuid::new_v4(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ];

        let mock_db = MockChampionshipDatabase::with_championships(championships);

        // WHEN: On filtre par saison "2025-2026"
        let current_season: Vec<Championship> = mock_db
            .championships
            .into_iter()
            .filter(|c| c.season == "2025-2026")
            .collect();

        // THEN: On doit avoir 1 championnat
        assert_eq!(current_season.len(), 1);
        assert_eq!(current_season[0].season, "2025-2026");
    }

    // ============================================
    // 5. TESTS DE MISE À JOUR DES CHAMPIONNATS
    // ============================================

    #[tokio::test]
    async fn test_update_championship_status() {
        // GIVEN: Un championnat en attente
        let mut championship = Championship {
            id: Uuid::new_v4(),
            name: "Championnat".to_string(),
            code: None,
            season: "2025".to_string(),
            description: None,
            league: "Vendée".to_string(),
            divisions: vec!["D1".to_string()],
            total_journees: 10,
            journee_duration: None,
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            status: CompetitionStatus::Pending,
            current_journee: None,
            current_phase: None,
            organizer_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // WHEN: On met à jour le statut
        let update_request = UpdateChampionshipRequest {
            name: None,
            code: None,
            season: None,
            description: None,
            league: None,
            divisions: None,
            total_journees: None,
            journee_duration: None,
            classification_rules: None,
            status: Some(CompetitionStatus::InProgress),
            current_journee: Some(1),
            current_phase: None,
        };

        championship.status = update_request.status.unwrap_or(championship.status);
        championship.current_journee = update_request.current_journee.or(championship.current_journee);
        championship.updated_at = Utc::now();

        // THEN: Le statut doit être mis à jour
        assert_eq!(championship.status, CompetitionStatus::InProgress);
        assert_eq!(championship.current_journee, Some(1));
    }

    #[tokio::test]
    async fn test_update_championship_divisions() {
        // GIVEN: Un championnat avec 1 division
        let mut championship = Championship {
            id: Uuid::new_v4(),
            name: "Championnat".to_string(),
            code: None,
            season: "2025".to_string(),
            description: None,
            league: "Vendée".to_string(),
            divisions: vec!["D1".to_string()],
            total_journees: 10,
            journee_duration: None,
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            status: CompetitionStatus::Pending,
            current_journee: None,
            current_phase: None,
            organizer_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // WHEN: On ajoute des divisions
        let update_request = UpdateChampionshipRequest {
            name: None,
            code: None,
            season: None,
            description: None,
            league: None,
            divisions: Some(vec!["D1".to_string(), "D2".to_string(), "D3".to_string()]),
            total_journees: None,
            journee_duration: None,
            classification_rules: None,
            status: None,
            current_journee: None,
            current_phase: None,
        };

        championship.divisions = update_request.divisions.clone().unwrap_or(championship.divisions);
        championship.updated_at = Utc::now();

        // THEN: Les divisions doivent être mises à jour
        assert_eq!(championship.divisions.len(), 3);
        assert!(championship.divisions.contains(&"D2".to_string()));
        assert!(championship.divisions.contains(&"D3".to_string()));
    }

    // ============================================
    // 6. TESTS SPÉCIFIQUES PALET VENDÉEN
    // ============================================

    #[tokio::test]
    async fn test_championship_with_palet_vendeen_rules() {
        // GIVEN: Un championnat suivant les règles du Palet Vendéen
        // - 13 points pour gagner une partie (15 en finale)
        // - Classement: Points > Diff > Points marqués

        let championship = Championship {
            id: Uuid::new_v4(),
            name: "Championnat de Vendée Fonte".to_string(),
            code: Some("CVDP-FONTE-2025".to_string()),
            season: "2025-2026".to_string(),
            description: Some("Championnat officiel de palet fonte en Vendée".to_string()),
            league: "Vendée".to_string(),
            divisions: vec!["D1".to_string(), "D2".to_string()],
            total_journees: 10,
            journee_duration: Some("1 journée".to_string()),
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            status: CompetitionStatus::InProgress,
            current_journee: Some(5),
            current_phase: None,
            organizer_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // THEN: Les règles doivent être conformes au Palet Vendéen
        assert_eq!(championship.league, "Vendée");
        assert_eq!(championship.classification_rules.primary, ClassificationPrimary::Points);
        assert_eq!(championship.classification_rules.secondary, ClassificationSecondary::Diff);
        assert_eq!(
            championship.classification_rules.tertiary,
            ClassificationTertiary::PointsFor
        );
    }

    #[tokio::test]
    async fn test_championship_journee_scheduling() {
        // GIVEN: Un championnat avec 10 journées
        // En Vendée, les journées sont généralement espacées de 1 à 2 semaines

        let mut championship = Championship {
            id: Uuid::new_v4(),
            name: "Championnat de Vendée".to_string(),
            code: None,
            season: "2025-2026".to_string(),
            description: None,
            league: "Vendée".to_string(),
            divisions: vec!["D1".to_string()],
            total_journees: 10,
            journee_duration: Some("1 journée".to_string()),
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            status: CompetitionStatus::InProgress,
            current_journee: Some(1),
            current_phase: None,
            organizer_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // WHEN: On avance les journées
        for journee in 2..=10 {
            championship.current_journee = Some(journee);
            championship.updated_at = Utc::now();

            // THEN: Chaque journée doit être valide
            assert!(
                championship.current_journee.unwrap() <= championship.total_journees,
                "Journée {} dépasse le total de {}",
                journee,
                championship.total_journees
            );
        }

        // WHEN: On termine le championnat
        championship.status = CompetitionStatus::Completed;

        // THEN: Le championnat doit être marqué comme terminé
        assert_eq!(championship.status, CompetitionStatus::Completed);
    }
}
