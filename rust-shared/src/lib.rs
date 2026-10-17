//! Shared types and utilities for Palet Vendéen

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Import and re-export competition types
pub mod competition_types;
pub use competition_types::*;

// Import and re-export match types
pub mod match_types;
pub use match_types::*;

// Import and re-export championship types
pub mod championship_types;
pub use championship_types::*;

// ============================================
// COMMON TYPES
// ============================================

/// Trait for identifiable entities
pub trait Identifiable {
    fn id(&self) -> &Uuid;
}

/// Trait for timestamped entities
pub trait Timestamped {
    fn created_at(&self) -> &DateTime<Utc>;
    fn updated_at(&self) -> &DateTime<Utc>;
}

// ============================================
// USER & AUTHENTICATION TYPES
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Player,
    ClubAdmin,
    CommissionAdmin,
    SuperAdmin,
    Arbitre,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: Option<String>,
    pub first_name: String,
    pub last_name: String,

    // Profile
    pub photo: Option<String>,
    pub bio: Option<String>,
    pub phone: Option<String>,
    pub birth_date: Option<DateTime<Utc>>,

    // Palet Info
    pub license_number: Option<String>,
    pub club_id: Option<Uuid>,
    pub category: String,
    pub dominant_hand: DominantHand,
    pub play_style: Option<String>,

    // Account
    pub role: UserRole,
    pub permissions: Vec<Permission>,
    pub status: UserStatus,
    pub email_verified: bool,

    // Preferences
    pub preferences: UserPreferences,

    // Security
    pub password_last_changed: Option<DateTime<Utc>>,
    pub mfa_enabled: bool,
    pub last_login: Option<DateTime<Utc>>,

    // Social
    pub social: SocialLinks,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Identifiable for User {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Timestamped for User {
    fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DominantHand {
    Right,
    Left,
    Ambidextrous,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    Banned,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UserPreferences {
    pub language: String,
    pub timezone: String,
    pub distance_unit: DistanceUnit,
    pub notifications: NotificationsPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DistanceUnit {
    Metric,
    Imperial,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct NotificationsPreferences {
    pub email: EmailNotifications,
    pub push: PushNotifications,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct EmailNotifications {
    pub results: bool,
    pub news: bool,
    pub reminders: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PushNotifications {
    pub results: bool,
    pub matches: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub category: PermissionCategory,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PermissionCategory {
    Competition,
    Player,
    Club,
    System,
    Report,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AuthToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64, // seconds
    pub token_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AuthState {
    pub user: Option<User>,
    pub token: Option<AuthToken>,
    pub is_authenticated: bool,
    pub is_loading: bool,
    pub error: Option<String>,
    pub requires_2fa: bool,
    pub two_factor_pending: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SocialLinks {
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub instagram: Option<String>,
}

// ============================================
// CLUB TYPES
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Club {
    pub id: Uuid,
    pub name: String,
    pub nickname: Option<String>,
    pub code: String,
    pub logo: Option<String>,

    // Location
    pub address: Option<String>,
    pub city: String,
    pub postal_code: Option<String>,

    // Affiliation
    pub league: String,
    pub division: String,
    pub status: ClubStatus,

    // Stats
    pub players_count: i32,
    pub teams_count: i32,

    // Contact
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub website: Option<String>,

    // Social
    pub social: SocialLinks,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Identifiable for Club {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Timestamped for Club {
    fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ClubStatus {
    Active,
    Inactive,
    Suspended,
}

// ============================================
// FEDERATION TYPES
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Federation {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Identifiable for Federation {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

// ============================================
// PLAYER TYPES
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Player {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub alias: Option<String>,
    pub email: String,
    pub photo: Option<String>,
    pub status: PlayerStatus,

    // Personal info
    pub birth_date: DateTime<Utc>,
    pub age: i32,
    pub birth_place: Option<String>,
    pub nationality: String,
    pub dominant_hand: DominantHand,
    pub play_style: Option<String>,

    // Contact info
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,

    // Club info
    pub club_id: Uuid,
    pub club_number: Option<String>,
    pub category: String,

    // License info
    pub license_number: String,
    pub license_date: DateTime<Utc>,
    pub license_status: LicenseStatus,
    pub registration_date: DateTime<Utc>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Identifiable for Player {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Timestamped for Player {
    fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PlayerStatus {
    Active,
    Inactive,
    Suspended,
    Banned,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum LicenseStatus {
    Valid,
    Expired,
    Pending,
}

// ============================================
// TEAM TYPES
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub club_id: Uuid,
    pub captain_id: Option<Uuid>,
    pub logo: Option<String>,

    // Stats
    pub stats: TeamStats,

    // Status
    pub status: TeamStatus,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Identifiable for Team {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Timestamped for Team {
    fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TeamStats {
    pub matches: i32,
    pub wins: i32,
    pub draws: i32,
    pub losses: i32,
    pub points_for: i32,
    pub points_against: i32,
    pub diff: i32,
    pub points: i32,
    pub win_rate: f64,
}

impl TeamStats {
    pub fn new() -> Self {
        TeamStats {
            matches: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points_for: 0,
            points_against: 0,
            diff: 0,
            points: 0,
            win_rate: 0.0,
        }
    }

    pub fn update_from_match(&mut self, points_for: i32, points_against: i32, result: MatchResult) {
        self.matches += 1;
        self.points_for += points_for;
        self.points_against += points_against;
        self.diff = self.points_for - self.points_against;

        match result {
            MatchResult::Win => {
                self.wins += 1;
                self.points += 2;
            }
            MatchResult::Draw => {
                self.draws += 1;
                self.points += 1;
            }
            MatchResult::Loss => {
                self.losses += 1;
            }
        }

        if self.matches > 0 {
            self.win_rate = self.wins as f64 / self.matches as f64;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TeamStatus {
    Active,
    Inactive,
    Disbanded,
}

// ============================================
// COMPETITION TYPES
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompetitionType {
    Championship,
    Tournament,
    Cup,
    Friendly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompetitionCategory {
    Individual,
    Doublette,
    Triplette,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompetitionMaterial {
    Fonte,
    Laiton,
    Bois,
    Mixte,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompetitionStatus {
    Pending,
    Registration,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Competition {
    pub id: Uuid,
    pub name: String,
    pub code: Option<String>,
    pub competition_type: CompetitionType,
    pub category: CompetitionCategory,
    pub material: CompetitionMaterial,
    pub season: String,

    // Details
    pub description: Option<String>,
    pub location: Option<String>,

    // Structure
    pub divisions: Vec<String>,
    pub format: CompetitionFormat,

    // Dates
    pub registration_start: Option<DateTime<Utc>>,
    pub registration_end: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,

    // Status
    pub status: CompetitionStatus,
    pub current_journee: Option<i32>,
    pub total_journees: Option<i32>,
    pub current_phase: Option<String>,

    // Participants
    pub max_teams: Option<i32>,
    pub registered_teams: i32,

    // Scoring
    pub points_type: PointsType,
    pub win_points: i32,
    pub draw_points: i32,
    pub loss_points: i32,

    // Settings
    pub auto_registration: bool,
    pub manual_validation: bool,
    pub is_public: bool,
    pub is_archived: bool,

    // Organizer
    pub organizer_id: Uuid,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Identifiable for Competition {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Timestamped for Competition {
    fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CompetitionFormat {
    RoundRobin,
    League,
    Groups,
    Knockout,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PointsType {
    P13,
    P15,
}

// ============================================
// CHAMPIONSHIP TYPES
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Championship {
    pub id: Uuid,
    pub name: String,
    pub code: Option<String>,
    pub season: String,
    pub description: Option<String>,
    pub league: String,
    pub divisions: Vec<String>,
    pub total_journees: i32,
    pub journee_duration: Option<String>,
    pub classification_rules: ClassificationRules,
    pub status: CompetitionStatus,
    pub current_journee: Option<i32>,
    pub current_phase: Option<String>,
    pub organizer_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ClassificationRules {
    pub primary: ClassificationPrimary,
    pub secondary: ClassificationSecondary,
    pub tertiary: ClassificationTertiary,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ClassificationPrimary {
    Points,
    WinRate,
    Diff,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ClassificationSecondary {
    Diff,
    PointsFor,
    HeadToHead,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ClassificationTertiary {
    PointsFor,
    PointsAgainst,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ChampionshipClassification {
    pub division: String,
    pub journee: i32,
    pub updated_at: DateTime<Utc>,
    pub teams: Vec<ChampionshipTeam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ChampionshipTeam {
    pub position: i32,
    pub team_id: Uuid,
    pub stats: TeamStats,
    pub form: Vec<MatchResult>,
    pub status: TeamClassificationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MatchResult {
    Win,
    Draw,
    Loss,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TeamClassificationStatus {
    Qualified,
    Barrage,
    Relegated,
    Safe,
}

// ============================================
// TOURNAMENT TYPES
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Tournament {
    pub id: Uuid,
    pub name: String,
    pub code: Option<String>,
    pub competition_type: CompetitionType,
    pub category: CompetitionCategory,
    pub material: CompetitionMaterial,
    pub season: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub divisions: Vec<String>,
    pub format: CompetitionFormat,

    // Dates
    pub registration_start: Option<DateTime<Utc>>,
    pub registration_end: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,

    // Status
    pub status: CompetitionStatus,
    pub current_phase: Option<String>,

    // Registration
    pub fee: Option<f64>,
    pub payment_required: bool,
    pub payment_methods: Vec<String>,

    // Rules
    pub rules: Option<String>,
    pub eligibility: TournamentEligibility,

    // Participants
    pub max_teams: Option<i32>,
    pub registered_teams: i32,

    // Scoring
    pub points_type: PointsType,
    pub win_points: i32,
    pub draw_points: i32,
    pub loss_points: i32,

    // Settings
    pub auto_registration: bool,
    pub manual_validation: bool,
    pub is_public: bool,
    pub is_archived: bool,

    // Organizer
    pub organizer_id: Uuid,

    // Phases
    pub phases: Vec<TournamentPhase>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TournamentPhase {
    pub id: Uuid,
    pub name: String,
    pub phase_type: TournamentPhaseType,
    pub order: i32,
    pub matches: Vec<TournamentMatch>,
    pub groups: Option<Vec<TournamentGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum TournamentPhaseType {
    Group,
    Knockout,
    Final,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TournamentGroup {
    pub id: Uuid,
    pub name: String,
    pub team_ids: Vec<Uuid>,
    pub matches: Vec<TournamentMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TournamentMatch {
    pub id: Uuid,
    pub team1_id: Option<Uuid>,
    pub team2_id: Option<Uuid>,
    pub team1_score: Option<i32>,
    pub team2_score: Option<i32>,
    pub winner_id: Option<Uuid>,
    pub status: MatchStatus,
    pub date: Option<DateTime<Utc>>,
    pub time: Option<String>,
    pub location: Option<String>,
    pub round: String,
    pub manche: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TournamentEligibility {
    pub require_license: bool,
    pub require_club: bool,
    pub categories: Vec<CompetitionCategory>,
    pub divisions: Vec<String>,
    pub max_players_per_club: Option<i32>,
    pub min_players: i32,
    pub max_players: i32,
}

// ============================================
// MATCH TYPES
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MatchStatus {
    Scheduled,
    InProgress,
    Completed,
    Postponed,
    Cancelled,
    Disputed,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Match {
    pub id: Uuid,
    pub competition_id: Uuid,
    pub competition_type: CompetitionType,
    pub journee: Option<i32>,
    pub phase: Option<String>,
    pub round: Option<String>,

    // Teams
    pub team1_id: Uuid,
    pub team2_id: Uuid,

    // Scores
    pub team1_score: Option<i32>,
    pub team2_score: Option<i32>,
    pub winner: Option<MatchWinner>,

    // Manche scores
    pub manches: Option<Vec<MancheScore>>,
    pub best_of: Option<i32>,
    pub current_manche: Option<i32>,
    pub current_scores: Option<CurrentScores>,

    // Status
    pub status: MatchStatus,
    pub verified: bool,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,

    // Details
    pub date: DateTime<Utc>,
    pub time: Option<String>,
    pub location: Option<String>,
    pub duration: Option<String>,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
}

impl Identifiable for Match {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Timestamped for Match {
    fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MatchWinner {
    Team1,
    Team2,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct MancheScore {
    pub number: i32,
    pub team1: i32,
    pub team2: i32,
    pub winner: Option<MatchWinner>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CurrentScores {
    pub team1: i32,
    pub team2: i32,
    pub manche: i32,
}

// ============================================
// API RESPONSE TYPES
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ApiResponse<T> {
    pub data: T,
    pub message: Option<String>,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ApiError {
    pub message: String,
    pub code: String,
    pub status_code: u16,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Pagination {
    pub current: i64,
    pub total: i64,
    pub page_size: i64,
    pub total_items: i64,
    pub has_next: bool,
    pub has_previous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct HealthCheck {
    pub statut: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
    pub database: DatabaseStatus,
    pub cache: CacheStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseStatus {
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CacheStatus {
    Connected,
    Disconnected,
}

// ============================================
// REQUEST/RESPONSE DTOs
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateClubRequest {
    pub name: String,
    pub nickname: Option<String>,
    pub code: String,
    pub city: String,
    pub postal_code: Option<String>,
    pub league: String,
    pub division: String,
    pub federation_id: Uuid,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateClubRequest {
    pub name: Option<String>,
    pub nickname: Option<String>,
    pub code: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub league: Option<String>,
    pub division: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreatePlayerRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub birth_date: DateTime<Utc>,
    pub nationality: String,
    pub dominant_hand: DominantHand,
    pub category: String,
    pub club_id: Uuid,
    pub license_number: Option<String>,
    pub license_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateTournamentRequest {
    pub name: String,
    pub code: Option<String>,
    pub competition_type: CompetitionType,
    pub category: CompetitionCategory,
    pub material: CompetitionMaterial,
    pub season: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub divisions: Vec<String>,
    pub format: CompetitionFormat,
    pub registration_start: Option<DateTime<Utc>>,
    pub registration_end: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub fee: Option<f64>,
    pub payment_required: bool,
    pub payment_methods: Vec<String>,
    pub rules: Option<String>,
    pub max_teams: Option<i32>,
    pub auto_registration: bool,
    pub manual_validation: bool,
    pub is_public: bool,
    pub organizer_id: Uuid,
}

// ============================================
// UTILITY FUNCTIONS
// ============================================

/// Generate a new UUID v4
pub fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}

/// Get current UTC timestamp
pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

/// Format date for display
pub fn format_date(date: &DateTime<Utc>) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// Format date and time for display
pub fn format_datetime(date: &DateTime<Utc>) -> String {
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}
