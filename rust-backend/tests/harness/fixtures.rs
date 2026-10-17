//! Test fixtures for Palet Vendéen TDD harness
//!
//! This module provides factory functions for creating test data.

use chrono::Utc;
use palet_shared::{
    Championship, ChampionshipClassification, ChampionshipTeam, ClassificationPrimary,
    ClassificationSecondary, ClassificationTertiary, Club, ClubStatus, CompetitionCategory,
    CompetitionFormat, CompetitionMaterial, CompetitionStatus, CompetitionType, CreateChampionshipRequest,
    CreateMatchRequest, DominantHand, Federation, LicenseStatus, MancheScore, Match,
    MatchResult, MatchStatus, MatchWinner, Player, PlayerStatus, PointsType, SocialLinks,
    Team, TeamClassificationStatus, TeamStats, Tournament, TournamentEligibility,
    TournamentMatch, TournamentPhase, TournamentPhaseType, User, UserRole, UserStatus,
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
        birth_date: Utc::now() - chrono::Duration::days(365 * 25), // 25 years old
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
        category: "Sénior".to_string(),
        license_number: "LIC001".to_string(),
        license_date: Utc::now(),
        license_status: LicenseStatus::Valid,
        registration_date: Utc::now(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
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
        email: format!("{}.{}@test.com", first_name, last_name).to_lowercase(),
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
    }
}

// ============================================
// TEAM FIXTURES
// ============================================

/// Create a test team with default values
pub fn create_test_team() -> Team {
    Team {
        id: Uuid::new_v4(),
        name: "Test Team".to_string(),
        club_id: Uuid::new_v4(),
        captain_id: None,
        logo: None,
        stats: TeamStats::default(),
        status: TeamStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a test team with custom values
pub fn create_test_team_with(name: &str, club_id: Uuid, captain_id: Option<Uuid>) -> Team {
    Team {
        id: Uuid::new_v4(),
        name: name.to_string(),
        club_id,
        captain_id,
        logo: None,
        stats: TeamStats::default(),
        status: TeamStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a test team with specific stats
pub fn create_test_team_with_stats(
    name: &str,
    club_id: Uuid,
    matches: i32,
    wins: i32,
    draws: i32,
    losses: i32,
    points_for: i32,
    points_against: i32,
) -> Team {
    let diff = points_for - points_against;
    let points = wins * 3 + draws * 1; // Standard Palet Vendéen scoring
    let win_rate = if matches > 0 {
        wins as f64 / matches as f64
    } else {
        0.0
    };

    Team {
        id: Uuid::new_v4(),
        name: name.to_string(),
        club_id,
        captain_id: None,
        logo: None,
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
        status: TeamStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
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
        team1_score: Some(0),
        team2_score: Some(0),
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
        location: Some("Test Location".to_string()),
        duration: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        started_at: None,
        ended_at: None,
    }
}

/// Create a test match with custom values
pub fn create_test_match_with(
    competition_type: CompetitionType,
    team1_id: Uuid,
    team2_id: Uuid,
    journee: Option<i32>,
    phase: Option<String>,
    best_of: Option<i32>,
) -> Match {
    Match {
        id: Uuid::new_v4(),
        competition_id: Uuid::new_v4(),
        competition_type,
        journee,
        phase,
        round: None,
        team1_id,
        team2_id,
        team1_score: Some(0),
        team2_score: Some(0),
        winner: None,
        manches: None,
        best_of,
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
        location: Some("Test Location".to_string()),
        duration: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        started_at: None,
        ended_at: None,
    }
}

/// Create a test match with scores
pub fn create_test_match_with_scores(
    team1_id: Uuid,
    team2_id: Uuid,
    team1_score: i32,
    team2_score: i32,
    best_of: i32,
) -> Match {
    let status = if team1_score >= best_of || team2_score >= best_of {
        MatchStatus::Completed
    } else {
        MatchStatus::InProgress
    };

    let winner = if team1_score >= best_of {
        Some(MatchWinner::Team1)
    } else if team2_score >= best_of {
        Some(MatchWinner::Team2)
    } else {
        None
    };

    Match {
        id: Uuid::new_v4(),
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championnat,
        journee: Some(1),
        phase: None,
        round: None,
        team1_id,
        team2_id,
        team1_score: Some(team1_score),
        team2_score: Some(team2_score),
        winner,
        manches: None,
        best_of: Some(best_of),
        current_manche: Some(5),
        current_scores: Some(palet_shared::CurrentScores {
            team1: team1_score,
            team2: team2_score,
            manche: 5,
        }),
        status,
        verified: false,
        verified_by: None,
        verified_at: None,
        date: Utc::now(),
        time: None,
        location: Some("Test Location".to_string()),
        duration: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        started_at: if status == MatchStatus::InProgress || status == MatchStatus::Completed {
            Some(Utc::now())
        } else {
            None
        },
        ended_at: if status == MatchStatus::Completed {
            Some(Utc::now())
        } else {
            None
        },
    }
}

/// Create a test match with manches
pub fn create_test_match_with_manches(
    team1_id: Uuid,
    team2_id: Uuid,
    manches: Vec<MancheScore>,
    best_of: i32,
) -> Match {
    let team1_total: i32 = manches.iter().map(|m| m.team1).sum();
    let team2_total: i32 = manches.iter().map(|m| m.team2).sum();
    let current_manche = manches.len() as i32;

    let status = if team1_total >= best_of || team2_total >= best_of {
        MatchStatus::Completed
    } else {
        MatchStatus::InProgress
    };

    let winner = if team1_total >= best_of {
        Some(MatchWinner::Team1)
    } else if team2_total >= best_of {
        Some(MatchWinner::Team2)
    } else {
        None
    };

    Match {
        id: Uuid::new_v4(),
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championnat,
        journee: Some(1),
        phase: None,
        round: None,
        team1_id,
        team2_id,
        team1_score: Some(team1_total),
        team2_score: Some(team2_total),
        winner,
        manches: Some(manches),
        best_of: Some(best_of),
        current_manche: Some(current_manche),
        current_scores: Some(palet_shared::CurrentScores {
            team1: team1_total,
            team2: team2_total,
            manche: current_manche,
        }),
        status,
        verified: false,
        verified_by: None,
        verified_at: None,
        date: Utc::now(),
        time: None,
        location: Some("Test Location".to_string()),
        duration: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        started_at: Some(Utc::now()),
        ended_at: if status == MatchStatus::Completed {
            Some(Utc::now())
        } else {
            None
        },
    }
}

/// Create a test CreateMatchRequest
pub fn create_test_create_match_request(
    competition_type: CompetitionType,
    team1_id: Uuid,
    team2_id: Uuid,
) -> CreateMatchRequest {
    CreateMatchRequest {
        competition_id: Uuid::new_v4(),
        competition_type,
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

/// Create a test MancheScore
pub fn create_test_manche(number: i32, team1_score: i32, team2_score: i32) -> MancheScore {
    MancheScore {
        number,
        team1: team1_score,
        team2: team2_score,
        winner: if team1_score > team2_score {
            Some(MatchWinner::Team1)
        } else if team2_score > team1_score {
            Some(MatchWinner::Team2)
        } else {
            None
        },
    }
}

// ============================================
// CHAMPIONSHIP FIXTURES
// ============================================

/// Create a test championship with default values
pub fn create_test_championship() -> Championship {
    Championship {
        id: Uuid::new_v4(),
        name: "Test Championship".to_string(),
        code: Some("TC-2025".to_string()),
        season: "2025-2026".to_string(),
        description: Some("Test championship description".to_string()),
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
    season: &str,
    league: &str,
    divisions: Vec<String>,
    total_journees: i32,
    status: CompetitionStatus,
) -> Championship {
    Championship {
        id: Uuid::new_v4(),
        name: name.to_string(),
        code: Some(format!("{}-{}", name.chars().next().unwrap(), season).to_uppercase()),
        season: season.to_string(),
        description: Some(format!("Championnat {} - {}", name, season)),
        league: league.to_string(),
        divisions,
        total_journees,
        journee_duration: Some("1 journée".to_string()),
        classification_rules: palet_shared::ClassificationRules {
            primary: ClassificationPrimary::Points,
            secondary: ClassificationSecondary::Diff,
            tertiary: ClassificationTertiary::PointsFor,
        },
        status,
        current_journee: None,
        current_phase: None,
        organizer_id: Uuid::new_v4(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

/// Create a test CreateChampionshipRequest
pub fn create_test_create_championship_request(
    name: &str,
    season: &str,
    league: &str,
    divisions: Vec<String>,
    organizer_id: Uuid,
) -> CreateChampionshipRequest {
    CreateChampionshipRequest {
        name: name.to_string(),
        code: Some(format!("{}-{}", league, season).to_uppercase()),
        season: season.to_string(),
        description: Some(format!("Championnat {} - {}", league, season)),
        league: league.to_string(),
        divisions,
        total_journees: 10,
        journee_duration: Some("1 journée".to_string()),
        classification_rules: palet_shared::ClassificationRules {
            primary: ClassificationPrimary::Points,
            secondary: ClassificationSecondary::Diff,
            tertiary: ClassificationTertiary::PointsFor,
        },
        organizer_id,
    }
}

/// Create a test ChampionshipTeam
pub fn create_test_championship_team(
    position: i32,
    team_id: Uuid,
    matches: i32,
    wins: i32,
    draws: i32,
    losses: i32,
    points_for: i32,
    points_against: i32,
) -> ChampionshipTeam {
    let diff = points_for - points_against;
    let points = wins * 3 + draws * 1; // Standard Palet Vendéen scoring
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
        form: vec![MatchResult::Win, MatchResult::Win, MatchResult::Loss],
        status: TeamClassificationStatus::Safe,
    }
}

/// Create a test ChampionshipClassification
pub fn create_test_championship_classification(
    division: &str,
    journee: i32,
    teams: Vec<ChampionshipTeam>,
) -> ChampionshipClassification {
    ChampionshipClassification {
        division: division.to_string(),
        journee,
        updated_at: Utc::now(),
        teams,
    }
}

// ============================================
// TOURNAMENT FIXTURES
// ============================================

/// Create a test tournament with default values
pub fn create_test_tournament() -> Tournament {
    Tournament {
        id: Uuid::new_v4(),
        name: "Test Tournament".to_string(),
        code: Some("TT-2025".to_string()),
        competition_type: CompetitionType::Tournament,
        category: CompetitionCategory::Doublette,
        material: CompetitionMaterial::Fonte,
        season: "2025-2026".to_string(),
        description: Some("Test tournament description".to_string()),
        location: Some("Test Location".to_string()),
        divisions: vec!["Open".to_string()],
        format: CompetitionFormat::Knockout,
        registration_start: Some(Utc::now()),
        registration_end: Some(Utc::now() + chrono::Duration::days(7)),
        start_date: Some(Utc::now() + chrono::Duration::days(14)),
        end_date: Some(Utc::now() + chrono::Duration::days(15)),
        status: CompetitionStatus::Pending,
        current_phase: None,
        fee: Some(10.0),
        payment_required: true,
        payment_methods: vec!["Cash".to_string(), "Check".to_string()],
        rules: Some("Standard Palet Vendéen rules".to_string()),
        eligibility: TournamentEligibility {
            require_license: true,
            require_club: true,
            categories: vec![CompetitionCategory::Doublette],
            divisions: vec!["D1".to_string(), "D2".to_string()],
            max_players_per_club: Some(2),
            min_players: 2,
            max_players: 100,
        },
        max_teams: Some(32),
        registered_teams: 0,
        points_type: PointsType::P13,
        win_points: 3,
        draw_points: 1,
        loss_points: 0,
        auto_registration: false,
        manual_validation: true,
        is_public: true,
        is_archived: false,
        phases: vec![],
        organizer_id: Uuid::new_v4(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

// ============================================
// FEDERATION FIXTURES
// ============================================

/// Create a test federation
pub fn create_test_federation() -> Federation {
    Federation {
        id: Uuid::new_v4(),
        name: "Fédération Nationale du Sport en Milieu Rural".to_string(),
        slug: "fnsmr".to_string(),
        description: Some("Fédération nationale du palet en France".to_string()),
        created_at: Utc::now(),
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
        username: Some("testuser".to_string()),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        photo: None,
        bio: None,
        phone: None,
        birth_date: None,
        license_number: None,
        club_id: None,
        category: "Sénior".to_string(),
        dominant_hand: DominantHand::Right,
        play_style: None,
        role: UserRole::Player,
        permissions: vec![],
        status: UserStatus::Active,
        email_verified: false,
        preferences: palet_shared::UserPreferences {
            language: "fr".to_string(),
            timezone: "Europe/Paris".to_string(),
            distance_unit: palet_shared::DistanceUnit::Metric,
            notifications: palet_shared::NotificationsPreferences {
                email: palet_shared::EmailNotifications {
                    results: true,
                    news: true,
                    reminders: true,
                },
                push: palet_shared::PushNotifications {
                    results: true,
                    matches: true,
                },
            },
        },
        password_last_changed: None,
        mfa_enabled: false,
        last_login: None,
        social: SocialLinks::default(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

// ============================================
// BATCH FIXTURES
// ============================================

/// Create multiple test clubs
pub fn create_test_clubs(count: usize) -> Vec<Club> {
    (0..count)
        .map(|i| {
            let mut club = create_test_club();
            club.name = format!("Test Club {}", i + 1);
            club.code = format!("TC{:03}", i + 1);
            club
        })
        .collect()
}

/// Create multiple test players for a club
pub fn create_test_players_for_club(club_id: Uuid, count: usize) -> Vec<Player> {
    (0..count)
        .map(|i| {
            let mut player = create_test_player();
            player.club_id = club_id;
            player.first_name = format!("Player{}", i + 1);
            player.last_name = "Test".to_string();
            player.full_name = format!("Player{} Test", i + 1);
            player.email = format!("player{}@test.com", i + 1);
            player
        })
        .collect()
}

/// Create a complete test dataset (clubs, players, teams, matches, championship)
pub fn create_test_dataset() -> (
    Vec<Club>,
    Vec<Player>,
    Vec<Team>,
    Vec<Match>,
    Championship,
) {
    // Create clubs
    let clubs = create_test_clubs(2);
    let club1_id = clubs[0].id;
    let club2_id = clubs[1].id;

    // Create players
    let players = vec![
        create_test_player_with("John", "Doe", club1_id, "Sénior"),
        create_test_player_with("Jane", "Smith", club1_id, "Sénior"),
        create_test_player_with("Bob", "Johnson", club2_id, "Sénior"),
        create_test_player_with("Alice", "Williams", club2_id, "Sénior"),
    ];

    // Create teams
    let teams = vec![
        create_test_team_with("Team A", club1_id, Some(players[0].id)),
        create_test_team_with("Team B", club1_id, Some(players[1].id)),
        create_test_team_with("Team C", club2_id, Some(players[2].id)),
        create_test_team_with("Team D", club2_id, Some(players[3].id)),
    ];

    // Create matches
    let matches = vec![
        create_test_match_with_scores(
            teams[0].id,
            teams[1].id,
            13,
            8,
            13,
        ),
        create_test_match_with_scores(
            teams[2].id,
            teams[3].id,
            13,
            10,
            13,
        ),
    ];

    // Create championship
    let championship = create_test_championship_with(
        "Championnat de Vendée",
        "2025-2026",
        "Vendée",
        vec!["D1".to_string()],
        10,
        CompetitionStatus::InProgress,
    );

    (clubs, players, teams, matches, championship)
}
