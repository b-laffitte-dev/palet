//! Test fixtures for Palet Vendéen TDD harness
//!
//! This module provides factory functions for creating test data.

use chrono::Utc;
use palet_shared::{
    Championship, ChampionshipClassification, ChampionshipTeam, ClassificationPrimary,
    ClassificationSecondary, ClassificationTertiary, Club, ClubStatus, CompetitionCategory,
    CompetitionFormat, CompetitionMaterial, CompetitionStatus, CompetitionType, CreateChampionshipRequest,
    CreateMatchRequest, DominantHand, Federation, LicenseStatus, MancheScore, Match, MatchResult,
    MatchStatus, MatchWinner, Player, PlayerStatus, PointsType, SocialLinks, Team,
    TeamClassificationStatus, TeamStats, Tournament, TournamentEligibility, TournamentMatch,
    TournamentPhase, TournamentPhaseType, User, UserRole, UserStatus,
};
use uuid::Uuid;

// ============================================
// CLUB FIXTURES
// ============================================

/// Create a test club with default values
pub fn create_test_club() -> Club {
    Club {
        id: Uuid::new_v4(),
        name: "Test Club".to_string(),
        nickname: Some("TC".to_string()),
        code: "TC001".to_string(),
        logo: None,
        address: Some("123 Test Street".to_string()),
        city: "Test City".to_string(),
        postal_code: Some("12345".to_string()),
        league: "Vendée".to_string(),
        division: "D1".to_string(),
        status: ClubStatus::Active,
        players_count: 10,
        teams_count: 2,
        contact_email: Some("contact@testclub.com".to_string()),
        contact_phone: Some("0123456789".to_string()),
        website: Some("https://testclub.com".to_string()),
        social: SocialLinks::default(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a test club with custom values
pub fn create_test_club_with(name: &str, league: &str, division: &str) -> Club {
    Club {
        id: Uuid::new_v4(),
        name: name.to_string(),
        nickname: None,
        code: format!("{}{}", name.chars().next().unwrap(), division),
        logo: None,
        address: None,
        city: "Test City".to_string(),
        postal_code: None,
        league: league.to_string(),
        division: division.to_string(),
        status: ClubStatus::Active,
        players_count: 0,
        teams_count: 0,
        contact_email: None,
        contact_phone: None,
        website: None,
        social: SocialLinks::default(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create multiple test clubs
pub fn create_test_clubs(count: usize) -> Vec<Club> {
    (0..count)
        .map(|i| Club {
            id: Uuid::new_v4(),
            name: format!("Test Club {}", i + 1),
            nickname: Some(format!("TC{}", i + 1)),
            code: format!("TC{:03}", i + 1),
            logo: None,
            address: Some(format!("123 Test Street {}", i + 1)),
            city: "Test City".to_string(),
            postal_code: Some("12345".to_string()),
            league: "Vendée".to_string(),
            division: format!("D{}", (i % 3) + 1),
            status: ClubStatus::Active,
            players_count: 10,
            teams_count: 2,
            contact_email: Some(format!("contact{}@testclub.com", i + 1)),
            contact_phone: Some("0123456789".to_string()),
            website: Some(format!("https://testclub{}.com", i + 1)),
            social: SocialLinks::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
        .collect()
}

// ============================================
// PLAYER FIXTURES
// ============================================

/// Create a test player with default values
pub fn create_test_player() -> Player {
    Player {
        id: Uuid::new_v4(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        full_name: "John Doe".to_string(),
        alias: Some("JD".to_string()),
        email: "john.doe@test.com".to_string(),
        photo: None,
        status: PlayerStatus::Active,
        birth_date: Utc::now() - chrono::Duration::days(365 * 25),
        age: 25,
        birth_place: Some("Test City".to_string()),
        nationality: "Français".to_string(),
        dominant_hand: DominantHand::Right,
        play_style: Some("Aggressif".to_string()),
        phone: Some("0123456789".to_string()),
        address: Some("123 Test Street".to_string()),
        city: Some("Test City".to_string()),
        postal_code: Some("12345".to_string()),
        club_id: Uuid::new_v4(),
        club_number: Some("TC001".to_string()),
        category: "Senior".to_string(),
        license_number: "LIC001".to_string(),
        license_date: Utc::now(),
        license_status: LicenseStatus::Valid,
        registration_date: Utc::now(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        team_id: None,
    }
}

/// Create a test player with custom values
pub fn create_test_player_with(
    first_name: &str,
    last_name: &str,
    club_id: Uuid,
    category: &str,
) -> Player {
    Player {
        id: Uuid::new_v4(),
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        full_name: format!("{} {}", first_name, last_name),
        alias: None,
        email: format!("{}.{}@test.com", first_name.to_lowercase(), last_name.to_lowercase()),
        photo: None,
        status: PlayerStatus::Active,
        birth_date: Utc::now() - chrono::Duration::days(365 * 25),
        age: 25,
        birth_place: None,
        nationality: "Français".to_string(),
        dominant_hand: DominantHand::Right,
        play_style: None,
        phone: None,
        address: None,
        city: None,
        postal_code: None,
        club_id,
        club_number: None,
        category: category.to_string(),
        license_number: format!("LIC{}", Uuid::new_v4().to_string()[..8].to_uppercase()),
        license_date: Utc::now(),
        license_status: LicenseStatus::Valid,
        registration_date: Utc::now(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        team_id: None,
    }
}

/// Create multiple test players
pub fn create_test_players(count: usize, club_id: Uuid) -> Vec<Player> {
    (0..count)
        .map(|i| Player {
            id: Uuid::new_v4(),
            first_name: format!("Player{}", i + 1),
            last_name: "Test".to_string(),
            full_name: format!("Player{} Test", i + 1),
            alias: None,
            email: format!("player{}@test.com", i + 1),
            photo: None,
            status: PlayerStatus::Active,
            birth_date: Utc::now() - chrono::Duration::days(365 * (20 + i as i64)),
            age: 20 + i as i32,
            birth_place: None,
            nationality: "Français".to_string(),
            dominant_hand: if i % 2 == 0 {
                DominantHand::Right
            } else {
                DominantHand::Left
            },
            play_style: None,
            phone: None,
            address: None,
            city: None,
            postal_code: None,
            club_id,
            club_number: None,
            category: "Senior".to_string(),
            license_number: format!("LIC{:05}", i + 1),
            license_date: Utc::now(),
            license_status: LicenseStatus::Valid,
            registration_date: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            team_id: None,
        })
        .collect()
}

// ============================================
// TEAM FIXTURES
// ============================================

/// Create a test team with default values
pub fn create_test_team() -> Team {
    Team {
        id: Uuid::new_v4(),
        name: "Test Team".to_string(),
        nickname: Some("TT".to_string()),
        code: "TT001".to_string(),
        club_id: Uuid::new_v4(),
        division: "D1".to_string(),
        category: "Senior".to_string(),
        players: Vec::new(),
        captain_id: None,
        stats: TeamStats::default(),
        status: palet_shared::TeamStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a test team with custom values
pub fn create_test_team_with(name: &str, club_id: Uuid, division: &str) -> Team {
    Team {
        id: Uuid::new_v4(),
        name: name.to_string(),
        nickname: None,
        code: format!("{}{}", name.chars().next().unwrap(), division),
        club_id,
        division: division.to_string(),
        category: "Senior".to_string(),
        players: Vec::new(),
        captain_id: None,
        stats: TeamStats::default(),
        status: palet_shared::TeamStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a test team with players
pub fn create_test_team_with_players(name: &str, club_id: Uuid, player_ids: Vec<Uuid>) -> Team {
    Team {
        id: Uuid::new_v4(),
        name: name.to_string(),
        nickname: None,
        code: format!("{}{}", name.chars().next().unwrap(), "D1"),
        club_id,
        division: "D1".to_string(),
        category: "Senior".to_string(),
        players: player_ids,
        captain_id: if !player_ids.is_empty() {
            Some(player_ids[0])
        } else {
            None
        },
        stats: TeamStats::default(),
        status: palet_shared::TeamStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create multiple test teams
pub fn create_test_teams(count: usize, club_id: Uuid) -> Vec<Team> {
    (0..count)
        .map(|i| Team {
            id: Uuid::new_v4(),
            name: format!("Team {}", i + 1),
            nickname: None,
            code: format!("T{:03}", i + 1),
            club_id,
            division: format!("D{}", (i % 3) + 1),
            category: "Senior".to_string(),
            players: Vec::new(),
            captain_id: None,
            stats: TeamStats::default(),
            status: palet_shared::TeamStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
        .collect()
}

// ============================================
// MATCH FIXTURES
// ============================================

/// Create a test match with default values
pub fn create_test_match() -> Match {
    Match {
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
        current_scores: Some(palet_shared::CurrentScores {
            team1: 0,
            team2: 0,
            manche: 0,
        }),
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
    }
}

/// Create a test match with custom values
pub fn create_test_match_with(
    competition_id: Uuid,
    team1_id: Uuid,
    team2_id: Uuid,
    journee: i32,
    competition_type: CompetitionType,
) -> Match {
    Match {
        id: Uuid::new_v4(),
        competition_id,
        competition_type,
        journee: Some(journee),
        phase: None,
        round: None,
        team1_id,
        team2_id,
        team1_score: None,
        team2_score: None,
        winner: None,
        manches: None,
        best_of: Some(13),
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
        date: Utc::now(),
        time: None,
        location: None,
        duration: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        started_at: None,
        ended_at: None,
    }
}

/// Create a test match with scores
pub fn create_test_match_with_scores(
    competition_id: Uuid,
    team1_id: Uuid,
    team2_id: Uuid,
    team1_score: i32,
    team2_score: i32,
    winner: MatchWinner,
) -> Match {
    Match {
        id: Uuid::new_v4(),
        competition_id,
        competition_type: CompetitionType::Championnat,
        journee: Some(1),
        phase: None,
        round: None,
        team1_id,
        team2_id,
        team1_score: Some(team1_score),
        team2_score: Some(team2_score),
        winner: Some(winner),
        manches: None,
        best_of: Some(13),
        current_manche: Some(5),
        current_scores: Some(palet_shared::CurrentScores {
            team1: team1_score,
            team2: team2_score,
            manche: 5,
        }),
        status: if team1_score >= 13 || team2_score >= 13 {
            MatchStatus::Completed
        } else {
            MatchStatus::InProgress
        },
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
        ended_at: if team1_score >= 13 || team2_score >= 13 {
            Some(Utc::now())
        } else {
            None
        },
    }
}

/// Create a tournament final match (15 points)
pub fn create_test_tournament_final_match(team1_id: Uuid, team2_id: Uuid) -> Match {
    Match {
        id: Uuid::new_v4(),
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Tournament,
        journee: None,
        phase: Some("Finale".to_string()),
        round: None,
        team1_id,
        team2_id,
        team1_score: None,
        team2_score: None,
        winner: None,
        manches: None,
        best_of: Some(15), // 15 points for final
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
        date: Utc::now(),
        time: None,
        location: None,
        duration: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        started_at: None,
        ended_at: None,
    }
}

/// Create multiple test matches
pub fn create_test_matches(count: usize, competition_id: Uuid, team_ids: &[Uuid]) -> Vec<Match> {
    (0..count)
        .map(|i| {
            let team1_idx = i * 2;
            let team2_idx = i * 2 + 1;
            
            Match {
                id: Uuid::new_v4(),
                competition_id,
                competition_type: CompetitionType::Championnat,
                journee: Some((i as i32) + 1),
                phase: None,
                round: None,
                team1_id: team_ids[team1_idx % team_ids.len()],
                team2_id: team_ids[team2_idx % team_ids.len()],
                team1_score: None,
                team2_score: None,
                winner: None,
                manches: None,
                best_of: Some(13),
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
                date: Utc::now() + chrono::Duration::days(i as i64),
                time: None,
                location: None,
                duration: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                started_at: None,
                ended_at: None,
            }
        })
        .collect()
}

// ============================================
// MANCHE FIXTURES
// ============================================

/// Create a test manche score
pub fn create_test_manche(number: i32, team1_score: i32, team2_score: i32) -> MancheScore {
    MancheScore {
        number,
        team1: team1_score,
        team2: team2_score,
        winner: if team1_score > team2_score {
            Some(MatchWinner::Team1)
        } else {
            Some(MatchWinner::Team2)
        },
    }
}

/// Create multiple test manches
pub fn create_test_manches(count: i32) -> Vec<MancheScore> {
    (1..=count)
        .map(|i| MancheScore {
            number: i,
            team1: (i % 6) + 1, // Scores between 1-6
            team2: ((i + 1) % 6) + 1,
            winner: if i % 2 == 0 {
                Some(MatchWinner::Team1)
            } else {
                Some(MatchWinner::Team2)
            },
        })
        .collect()
}

// ============================================
// CHAMPIONSHIP FIXTURES
// ============================================

/// Create a test championship with default values
pub fn create_test_championship() -> Championship {
    Championship {
        id: Uuid::new_v4(),
        name: "Test Championship".to_string(),
        code: Some("TC2025".to_string()),
        season: "2025-2026".to_string(),
        description: Some("Test championship for TDD".to_string()),
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: Some("1 journée".to_string()),
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
    }
}

/// Create a test championship with custom values
pub fn create_test_championship_with(
    name: &str,
    league: &str,
    divisions: Vec<&str>,
    total_journees: i32,
) -> Championship {
    Championship {
        id: Uuid::new_v4(),
        name: name.to_string(),
        code: Some(format!("{}{}", name.chars().next().unwrap(), "2025")),
        season: "2025-2026".to_string(),
        description: None,
        league: league.to_string(),
        divisions: divisions.iter().map(|s| s.to_string()).collect(),
        total_journees,
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
    }
}

/// Create a test championship with Palet Vendéen rules
pub fn create_test_palet_vendeen_championship() -> Championship {
    Championship {
        id: Uuid::new_v4(),
        name: "Championnat de Vendée Fonte".to_string(),
        code: Some("CVDP-FONTE-2025".to_string()),
        season: "2025-2026".to_string(),
        description: Some("Championnat officiel de palet fonte en Vendée".to_string()),
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string(), "D2".to_string(), "D3".to_string()],
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
    }
}

/// Create multiple test championships
pub fn create_test_championships(count: usize) -> Vec<Championship> {
    (0..count)
        .map(|i| Championship {
            id: Uuid::new_v4(),
            name: format!("Championnat Test {}", i + 1),
            code: Some(format!("CT{:03}", i + 1)),
            season: format!("202{}-202{}", 5 + i, 6 + i),
            description: Some(format!("Test championship {}", i + 1)),
            league: if i % 2 == 0 {
                "Vendée".to_string()
            } else {
                "Deux-Sèvres".to_string()
            },
            divisions: vec!["D1".to_string()],
            total_journees: 10 + i as i32,
            journee_duration: Some("1 journée".to_string()),
            classification_rules: palet_shared::ClassificationRules {
                primary: ClassificationPrimary::Points,
                secondary: ClassificationSecondary::Diff,
                tertiary: ClassificationTertiary::PointsFor,
            },
            status: match i % 3 {
                0 => CompetitionStatus::Pending,
                1 => CompetitionStatus::InProgress,
                _ => CompetitionStatus::Completed,
            },
            current_journee: if i % 3 == 1 { Some(5) } else { None },
            current_phase: None,
            organizer_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
        .collect()
}

// ============================================
// CHAMPIONSHIP CLASSIFICATION FIXTURES
// ============================================

/// Create a test championship classification
pub fn create_test_classification(championship_id: Uuid, division: &str, journee: i32) -> ChampionshipClassification {
    ChampionshipClassification {
        championship_id,
        division: division.to_string(),
        journee,
        teams: Vec::new(),
        updated_at: Utc::now(),
    }
}

/// Create a test championship classification with teams
pub fn create_test_classification_with_teams(
    championship_id: Uuid,
    division: &str,
    journee: i32,
    teams: Vec<ChampionshipTeam>,
) -> ChampionshipClassification {
    ChampionshipClassification {
        championship_id,
        division: division.to_string(),
        journee,
        teams,
        updated_at: Utc::now(),
    }
}

/// Create a test championship team
pub fn create_test_championship_team(
    position: i32,
    team_id: Uuid,
    points: i32,
    wins: i32,
    draws: i32,
    losses: i32,
    points_for: i32,
    points_against: i32,
) -> ChampionshipTeam {
    let diff = points_for - points_against;
    let matches = wins + draws + losses;
    let win_rate = if matches > 0 {
        wins as f64 / matches as f64
    } else {
        0.0
    };

    ChampionshipTeam {
        position,
        team_id,
        stats: TeamStats {
            matches,
            wins,
            draws,
            losses,
            points_for,
            points_against,
            diff,
            points,
            win_rate,
        },
        form: Vec::new(),
        status: TeamClassificationStatus::Qualified,
    }
}

/// Create test championship teams sorted by Palet Vendéen rules
pub fn create_test_championship_teams_sorted() -> Vec<ChampionshipTeam> {
    vec![
        // Team 1: 13 points, diff +25, 45 points for
        create_test_championship_team(1, Uuid::new_v4(), 13, 4, 1, 0, 45, 20),
        // Team 2: 10 points, diff +15, 40 points for
        create_test_championship_team(2, Uuid::new_v4(), 10, 3, 1, 1, 40, 25),
        // Team 3: 10 points, diff +5, 35 points for
        create_test_championship_team(3, Uuid::new_v4(), 10, 3, 1, 1, 35, 30),
        // Team 4: 8 points
        create_test_championship_team(4, Uuid::new_v4(), 8, 2, 2, 1, 30, 25),
    ]
}

// ============================================
// TEAM STATS FIXTURES
// ============================================

/// Create team stats with specified values
pub fn create_test_team_stats(
    matches: i32,
    wins: i32,
    draws: i32,
    losses: i32,
    points_for: i32,
    points_against: i32,
) -> TeamStats {
    TeamStats {
        matches,
        wins,
        draws,
        losses,
        points_for,
        points_against,
        diff: points_for - points_against,
        points: wins * 3 + draws * 1,
        win_rate: if matches > 0 {
            wins as f64 / matches as f64
        } else {
            0.0
        },
    }
}

// ============================================
// FEDERATION FIXTURES
// ============================================

/// Create a test federation
pub fn create_test_federation() -> Federation {
    Federation {
        id: Uuid::new_v4(),
        name: "Fédération de Palet Vendéen".to_string(),
        slug: "federation-palet-vendeen".to_string(),
        description: Some("Fédération officielle du palet vendéen".to_string()),
        created_at: Utc::now(),
    }
}

// ============================================
// TOURNAMENT FIXTURES
// ============================================

/// Create a test tournament
pub fn create_test_tournament() -> Tournament {
    Tournament {
        id: Uuid::new_v4(),
        name: "Tournoi de Palet".to_string(),
        code: Some("TP2025".to_string()),
        season: "2025".to_string(),
        description: Some("Tournoi test".to_string()),
        category: CompetitionCategory::Senior,
        format: CompetitionFormat::RoundRobin,
        material: CompetitionMaterial::Fonte,
        status: CompetitionStatus::Pending,
        eligibility: TournamentEligibility::Open,
        max_teams: Some(16),
        current_phase: None,
        phases: Vec::new(),
        organizer_id: Uuid::new_v4(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

// ============================================
// USER FIXTURES
// ============================================

/// Create a test user
pub fn create_test_user() -> User {
    User {
        id: Uuid::new_v4(),
        email: "test@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        full_name: "Test User".to_string(),
        role: UserRole::Admin,
        status: UserStatus::Active,
        email_verified: true,
        phone: None,
        address: None,
        city: None,
        postal_code: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        last_login_at: None,
    }
}

// ============================================
// REQUEST FIXTURES
// ============================================

/// Create a test create championship request
pub fn create_test_create_championship_request() -> CreateChampionshipRequest {
    CreateChampionshipRequest {
        name: "Test Championship".to_string(),
        code: Some("TC2025".to_string()),
        season: "2025-2026".to_string(),
        description: Some("Test championship".to_string()),
        league: "Vendée".to_string(),
        divisions: vec!["D1".to_string()],
        total_journees: 10,
        journee_duration: Some("1 journée".to_string()),
        classification_rules: palet_shared::ClassificationRules {
            primary: ClassificationPrimary::Points,
            secondary: ClassificationSecondary::Diff,
            tertiary: ClassificationTertiary::PointsFor,
        },
        organizer_id: Uuid::new_v4(),
    }
}

/// Create a test create match request
pub fn create_test_create_match_request(
    competition_id: Uuid,
    team1_id: Uuid,
    team2_id: Uuid,
) -> CreateMatchRequest {
    CreateMatchRequest {
        competition_id,
        competition_type: CompetitionType::Championnat,
        team1_id,
        team2_id,
        date: Utc::now(),
        location: Some("Test Location".to_string()),
        journee: Some(1),
        phase: None,
        round: None,
        best_of: Some(13),
    }
}

/// Create a test create club request
pub fn create_test_create_club_request() -> CreateClubRequest {
    CreateClubRequest {
        name: "Test Club".to_string(),
        nickname: Some("TC".to_string()),
        code: "TC001".to_string(),
        city: "Test City".to_string(),
        postal_code: Some("12345".to_string()),
        league: "Vendée".to_string(),
        division: "D1".to_string(),
        contact_email: Some("contact@test.com".to_string()),
        contact_phone: Some("0123456789".to_string()),
        website: Some("https://test.com".to_string()),
        federation_id: Uuid::new_v4(),
    }
}

/// Create a test create player request
pub fn create_test_create_player_request(club_id: Uuid) -> CreatePlayerRequest {
    CreatePlayerRequest {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john.doe@test.com".to_string(),
        birth_date: Utc::now() - chrono::Duration::days(365 * 25),
        age: 25,
        birth_place: Some("Test City".to_string()),
        nationality: "Français".to_string(),
        dominant_hand: DominantHand::Right,
        play_style: Some("Aggressif".to_string()),
        phone: Some("0123456789".to_string()),
        address: Some("123 Test Street".to_string()),
        city: Some("Test City".to_string()),
        postal_code: Some("12345".to_string()),
        club_id,
        club_number: Some("TC001".to_string()),
        category: "Senior".to_string(),
        license_number: "LIC001".to_string(),
        license_date: Utc::now(),
    }
}
