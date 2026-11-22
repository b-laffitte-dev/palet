//! Tests d'intégration HTTP Vague 3 : notifications, disponibilités,
//! recherche globale, statistiques (skill rust-tdd).
//!
//! Pool "lazy" (pas de base réelle) : les repositories sont remplacés par
//! des mocks en mémoire ; les méthodes non utilisées par le test sont
//! `unimplemented!()`.

use actix_web::{http::StatusCode, test, web, App};
use async_trait::async_trait;
use palet_backend::{
    config::AppConfig,
    db::{
        AvailabilityRepository, ClubRepository, CompetitionRepository, Database, DatabaseError,
        MatchRepository, NewsRepository, NotificationRepository, PlayerRepository, TeamRepository,
        UserRepository,
    },
    handlers::{availability_handler, notification_handler, search_handler, stats_handler},
    models::user::UserAccount,
    services::auth_service::{create_token, Claims, ACCESS_TOKEN_TYPE},
    AppState,
};
use palet_shared::{
    ApiResponse, AppNotification, AvailabilityStatus, AvailabilitySummary, Club, ClubStats,
    ClubStatus, CompetitionEvent, CompetitionType, CreateClubRequest, CreateCompetitionRequest,
    CreateNewsRequest, CreatePlayerRequest, Match, MatchAvailability, MatchStatus, MatchWinner,
    NewsArticle, Player, PlayerStats, Registration, SearchResults, SetAvailabilityRequest,
    SocialLinks, Team, TeamMember, TeamMemberRole, TeamMemberWithPlayer, TeamStats, TeamStatus,
    UpdateClubRequest, UpdateCompetitionRequest, UpdateNewsRequest, UserRole, UserStatus,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::{Arc, Mutex};
use time::OffsetDateTime;
use uuid::Uuid;

// ============================================
// FIXTURES
// ============================================

fn fixture_player(id: Uuid, club_id: Uuid, license: &str) -> Player {
    let now = OffsetDateTime::now_utc();
    Player {
        id,
        first_name: "Jean".to_string(),
        last_name: "Martin".to_string(),
        full_name: "Jean Martin".to_string(),
        alias: None,
        email: "jean@example.fr".to_string(),
        photo: None,
        status: palet_shared::PlayerStatus::Active,
        birth_date: now,
        age: 40,
        birth_place: None,
        nationality: "Française".to_string(),
        dominant_hand: palet_shared::DominantHand::Right,
        play_style: None,
        phone: None,
        address: None,
        city: None,
        postal_code: None,
        club_id,
        club_number: None,
        category: "senior".to_string(),
        license_number: license.to_string(),
        license_date: now,
        license_status: palet_shared::LicenseStatus::Valid,
        registration_date: now,
        created_at: now,
        updated_at: now,
    }
}

fn fixture_club(id: Uuid, name: &str) -> Club {
    let now = OffsetDateTime::now_utc();
    Club {
        id,
        name: name.to_string(),
        nickname: None,
        code: "LRY".to_string(),
        logo: None,
        address: None,
        city: "La Roche-sur-Yon".to_string(),
        postal_code: Some("85000".to_string()),
        league: "Vendée".to_string(),
        division: "D1".to_string(),
        status: ClubStatus::Active,
        players_count: 3,
        teams_count: 1,
        contact_email: None,
        contact_phone: None,
        website: None,
        social: SocialLinks::default(),
        created_at: now,
        updated_at: now,
    }
}

fn fixture_team(id: Uuid, club_id: Uuid, name: &str) -> Team {
    let now = OffsetDateTime::now_utc();
    Team {
        id,
        name: name.to_string(),
        club_id,
        captain_id: None,
        logo: None,
        stats: TeamStats::default(),
        status: TeamStatus::Active,
        created_at: now,
        updated_at: now,
    }
}

fn fixture_match(id: Uuid, t1: Uuid, t2: Uuid) -> Match {
    let now = OffsetDateTime::now_utc();
    Match {
        id,
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
        journee: Some(1),
        phase: None,
        round: None,
        team1_id: t1,
        team2_id: t2,
        team1_score: Some(13),
        team2_score: Some(9),
        winner: Some(MatchWinner::Team1),
        manches: None,
        best_of: Some(13),
        current_manche: None,
        current_scores: None,
        status: MatchStatus::Completed,
        verified: true,
        verified_by: None,
        verified_at: None,
        date: now,
        time: None,
        location: None,
        duration: None,
        created_at: now,
        updated_at: now,
        started_at: None,
        ended_at: None,
    }
}

fn fixture_user(
    id: Uuid,
    role: UserRole,
    club_id: Option<Uuid>,
    license: Option<&str>,
) -> UserAccount {
    let now = OffsetDateTime::now_utc();
    UserAccount {
        id,
        email: format!("{id}@example.fr"),
        username: None,
        password_hash: "x".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        role,
        club_id,
        license_number: license.map(|s| s.to_string()),
        category: "senior".to_string(),
        status: UserStatus::Active,
        email_verified: true,
        last_login: None,
        created_at: now,
        updated_at: now,
    }
}

// ============================================
// MOCKS
// ============================================

struct MockNotificationRepository {
    items: Mutex<Vec<AppNotification>>,
}

#[async_trait]
impl NotificationRepository for MockNotificationRepository {
    async fn create(
        &self,
        user_id: Uuid,
        kind: &str,
        title: &str,
        message: &str,
        link: Option<&str>,
    ) -> Result<AppNotification, DatabaseError> {
        let n = AppNotification {
            id: Uuid::new_v4(),
            user_id,
            kind: kind.to_string(),
            title: title.to_string(),
            message: message.to_string(),
            link: link.map(|s| s.to_string()),
            read: false,
            created_at: OffsetDateTime::now_utc(),
        };
        self.items.lock().unwrap().push(n.clone());
        Ok(n)
    }
    async fn list(
        &self,
        user_id: Uuid,
        unread_only: bool,
        _limit: i64,
    ) -> Result<Vec<AppNotification>, DatabaseError> {
        let mut v: Vec<_> = self
            .items
            .lock()
            .unwrap()
            .iter()
            .filter(|n| n.user_id == user_id && (!unread_only || !n.read))
            .cloned()
            .collect();
        v.sort_by_key(|n| std::cmp::Reverse(n.created_at));
        Ok(v)
    }
    async fn count_unread(&self, user_id: Uuid) -> Result<i64, DatabaseError> {
        Ok(self
            .items
            .lock()
            .unwrap()
            .iter()
            .filter(|n| n.user_id == user_id && !n.read)
            .count() as i64)
    }
    async fn mark_read(&self, id: Uuid, user_id: Uuid) -> Result<bool, DatabaseError> {
        let mut items = self.items.lock().unwrap();
        if let Some(n) = items
            .iter_mut()
            .find(|n| n.id == id && n.user_id == user_id && !n.read)
        {
            n.read = true;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    async fn mark_all_read(&self, user_id: Uuid) -> Result<i64, DatabaseError> {
        let mut items = self.items.lock().unwrap();
        let mut n = 0i64;
        for item in items.iter_mut().filter(|n| n.user_id == user_id && !n.read) {
            item.read = true;
            n += 1;
        }
        Ok(n)
    }
}

struct MockAvailabilityRepository {
    items: Mutex<Vec<MatchAvailability>>,
}

#[async_trait]
impl AvailabilityRepository for MockAvailabilityRepository {
    async fn upsert(
        &self,
        match_id: Uuid,
        player_id: Uuid,
        status: AvailabilityStatus,
        commentaire: Option<String>,
    ) -> Result<MatchAvailability, DatabaseError> {
        let mut items = self.items.lock().unwrap();
        let d = MatchAvailability {
            match_id,
            player_id,
            status,
            commentaire,
            updated_at: OffsetDateTime::now_utc(),
        };
        if let Some(existing) = items
            .iter_mut()
            .find(|x| x.match_id == match_id && x.player_id == player_id)
        {
            *existing = d.clone();
        } else {
            items.push(d.clone());
        }
        Ok(d)
    }
    async fn get_for_match(&self, match_id: Uuid) -> Result<Vec<MatchAvailability>, DatabaseError> {
        Ok(self
            .items
            .lock()
            .unwrap()
            .iter()
            .filter(|d| d.match_id == match_id)
            .cloned()
            .collect())
    }
    async fn get_for_player(
        &self,
        player_id: Uuid,
    ) -> Result<Vec<MatchAvailability>, DatabaseError> {
        Ok(self
            .items
            .lock()
            .unwrap()
            .iter()
            .filter(|d| d.player_id == player_id)
            .cloned()
            .collect())
    }
}

struct MockMatchRepository {
    matches: Mutex<Vec<Match>>,
}

#[async_trait]
impl MatchRepository for MockMatchRepository {
    async fn create_match(
        &self,
        _r: palet_shared::CreateMatchRequest,
    ) -> Result<Match, DatabaseError> {
        unimplemented!()
    }
    async fn get_match(&self, id: Uuid) -> Result<Option<Match>, DatabaseError> {
        Ok(self
            .matches
            .lock()
            .unwrap()
            .iter()
            .find(|m| m.id == id)
            .cloned())
    }
    async fn get_matches(&self) -> Result<Vec<Match>, DatabaseError> {
        Ok(self.matches.lock().unwrap().clone())
    }
    async fn get_matches_by_competition(&self, c: Uuid) -> Result<Vec<Match>, DatabaseError> {
        Ok(self
            .matches
            .lock()
            .unwrap()
            .iter()
            .filter(|m| m.competition_id == c)
            .cloned()
            .collect())
    }
    async fn get_matches_by_status(&self, _s: MatchStatus) -> Result<Vec<Match>, DatabaseError> {
        unimplemented!()
    }
    async fn update_match(
        &self,
        _id: Uuid,
        _r: palet_shared::UpdateMatchRequest,
    ) -> Result<Option<Match>, DatabaseError> {
        unimplemented!()
    }
    async fn delete_match(&self, _id: Uuid) -> Result<bool, DatabaseError> {
        unimplemented!()
    }
    async fn start_match(&self, _id: Uuid) -> Result<Match, DatabaseError> {
        unimplemented!()
    }
    async fn complete_match(&self, _id: Uuid, _w: MatchWinner) -> Result<Match, DatabaseError> {
        unimplemented!()
    }
    async fn add_manche(
        &self,
        _id: Uuid,
        _m: palet_shared::MancheScore,
    ) -> Result<Match, DatabaseError> {
        unimplemented!()
    }
    async fn update_manche(
        &self,
        _id: Uuid,
        _n: i32,
        _t1: i32,
        _t2: i32,
    ) -> Result<Match, DatabaseError> {
        unimplemented!()
    }
    async fn verify_match(&self, _id: Uuid, _v: Uuid) -> Result<Match, DatabaseError> {
        unimplemented!()
    }
    async fn unverify_match(&self, _id: Uuid) -> Result<Match, DatabaseError> {
        unimplemented!()
    }
    async fn get_matches_by_date_range(
        &self,
        _s: time::OffsetDateTime,
        _e: time::OffsetDateTime,
    ) -> Result<Vec<Match>, DatabaseError> {
        unimplemented!()
    }
    async fn get_matches_by_team(&self, team_id: Uuid) -> Result<Vec<Match>, DatabaseError> {
        Ok(self
            .matches
            .lock()
            .unwrap()
            .iter()
            .filter(|m| m.team1_id == team_id || m.team2_id == team_id)
            .cloned()
            .collect())
    }
    async fn get_matches_by_journee(&self, _j: i32, _c: Uuid) -> Result<Vec<Match>, DatabaseError> {
        unimplemented!()
    }
    async fn get_championship_matches(&self, _c: Uuid) -> Result<Vec<Match>, DatabaseError> {
        unimplemented!()
    }
    async fn get_completed_matches_by_championship(
        &self,
        _c: Uuid,
    ) -> Result<Vec<Match>, DatabaseError> {
        unimplemented!()
    }
}

struct MockPlayerRepository {
    players: Mutex<Vec<Player>>,
}

#[async_trait]
impl PlayerRepository for MockPlayerRepository {
    async fn create_player(&self, _r: CreatePlayerRequest) -> Result<Player, DatabaseError> {
        unimplemented!()
    }
    async fn get_player(&self, id: Uuid) -> Result<Option<Player>, DatabaseError> {
        Ok(self
            .players
            .lock()
            .unwrap()
            .iter()
            .find(|p| p.id == id)
            .cloned())
    }
    async fn get_players(&self) -> Result<Vec<Player>, DatabaseError> {
        Ok(self.players.lock().unwrap().clone())
    }
    async fn update_player(
        &self,
        _id: Uuid,
        _r: CreatePlayerRequest,
    ) -> Result<Option<Player>, DatabaseError> {
        unimplemented!()
    }
    async fn delete_player(&self, _id: Uuid) -> Result<bool, DatabaseError> {
        unimplemented!()
    }
    async fn get_players_by_club(&self, _c: Uuid) -> Result<Vec<Player>, DatabaseError> {
        unimplemented!()
    }
    async fn get_players_by_category(&self, _c: String) -> Result<Vec<Player>, DatabaseError> {
        unimplemented!()
    }
    async fn get_players_by_status(
        &self,
        _s: palet_shared::PlayerStatus,
    ) -> Result<Vec<Player>, DatabaseError> {
        unimplemented!()
    }
    async fn get_players_by_license_status(
        &self,
        _s: palet_shared::LicenseStatus,
    ) -> Result<Vec<Player>, DatabaseError> {
        unimplemented!()
    }
    async fn search_players(&self, _q: String) -> Result<Vec<Player>, DatabaseError> {
        unimplemented!()
    }
    async fn find_by_license(&self, license: &str) -> Result<Option<Player>, DatabaseError> {
        Ok(self
            .players
            .lock()
            .unwrap()
            .iter()
            .find(|p| p.license_number == license)
            .cloned())
    }
}

struct MockUserRepository {
    users: Mutex<Vec<UserAccount>>,
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn create_user(
        &self,
        _n: palet_backend::models::user::NewUser,
    ) -> Result<UserAccount, DatabaseError> {
        unimplemented!()
    }
    async fn find_by_email(&self, _e: &str) -> Result<Option<UserAccount>, DatabaseError> {
        unimplemented!()
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserAccount>, DatabaseError> {
        Ok(self
            .users
            .lock()
            .unwrap()
            .iter()
            .find(|u| u.id == id)
            .cloned())
    }
    async fn find_by_license(&self, license: &str) -> Result<Option<UserAccount>, DatabaseError> {
        Ok(self
            .users
            .lock()
            .unwrap()
            .iter()
            .find(|u| u.license_number.as_deref() == Some(license))
            .cloned())
    }
    async fn update_last_login(&self, _id: Uuid) -> Result<(), DatabaseError> {
        unimplemented!()
    }
    async fn create_refresh_token(
        &self,
        _u: Uuid,
        _t: String,
        _e: time::OffsetDateTime,
    ) -> Result<palet_backend::models::user::RefreshToken, DatabaseError> {
        unimplemented!()
    }
    async fn find_refresh_token(
        &self,
        _t: &str,
    ) -> Result<Option<palet_backend::models::user::RefreshToken>, DatabaseError> {
        unimplemented!()
    }
    async fn revoke_refresh_token(&self, _t: &str) -> Result<(), DatabaseError> {
        unimplemented!()
    }
    async fn revoke_all_user_tokens(&self, _id: Uuid) -> Result<(), DatabaseError> {
        unimplemented!()
    }
    async fn delete_expired_tokens(
        &self,
        _now: time::OffsetDateTime,
    ) -> Result<u64, DatabaseError> {
        unimplemented!()
    }
}

struct MockClubRepository {
    clubs: Mutex<Vec<Club>>,
}

#[async_trait]
impl ClubRepository for MockClubRepository {
    async fn create_club(&self, _r: CreateClubRequest) -> Result<Club, DatabaseError> {
        unimplemented!()
    }
    async fn get_club(&self, id: Uuid) -> Result<Option<Club>, DatabaseError> {
        Ok(self
            .clubs
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.id == id)
            .cloned())
    }
    async fn get_clubs(&self) -> Result<Vec<Club>, DatabaseError> {
        Ok(self.clubs.lock().unwrap().clone())
    }
    async fn update_club(
        &self,
        _id: Uuid,
        _r: UpdateClubRequest,
    ) -> Result<Option<Club>, DatabaseError> {
        unimplemented!()
    }
    async fn delete_club(&self, _id: Uuid) -> Result<bool, DatabaseError> {
        unimplemented!()
    }
    async fn get_clubs_by_league(&self, _l: String) -> Result<Vec<Club>, DatabaseError> {
        unimplemented!()
    }
    async fn get_clubs_by_division(&self, _d: String) -> Result<Vec<Club>, DatabaseError> {
        unimplemented!()
    }
    async fn get_clubs_by_status(&self, _s: ClubStatus) -> Result<Vec<Club>, DatabaseError> {
        unimplemented!()
    }
    async fn get_clubs_by_federation(&self, _f: Uuid) -> Result<Vec<Club>, DatabaseError> {
        unimplemented!()
    }
    async fn search_clubs(&self, _q: String) -> Result<Vec<Club>, DatabaseError> {
        unimplemented!()
    }
}

struct MockTeamRepository {
    teams: Mutex<Vec<Team>>,
    members: Mutex<Vec<(Uuid, Uuid)>>, // (team_id, player_id)
}

#[async_trait]
impl TeamRepository for MockTeamRepository {
    async fn create_team(
        &self,
        _n: String,
        _c: Uuid,
        _cap: Option<Uuid>,
    ) -> Result<Team, DatabaseError> {
        unimplemented!()
    }
    async fn get_team(&self, id: Uuid) -> Result<Option<Team>, DatabaseError> {
        Ok(self
            .teams
            .lock()
            .unwrap()
            .iter()
            .find(|t| t.id == id)
            .cloned())
    }
    async fn get_teams(&self) -> Result<Vec<Team>, DatabaseError> {
        Ok(self.teams.lock().unwrap().clone())
    }
    async fn update_team(
        &self,
        _id: Uuid,
        _n: Option<String>,
        _cap: Option<Uuid>,
        _s: Option<TeamStats>,
        _st: Option<TeamStatus>,
    ) -> Result<Option<Team>, DatabaseError> {
        unimplemented!()
    }
    async fn delete_team(&self, _id: Uuid) -> Result<bool, DatabaseError> {
        unimplemented!()
    }
    async fn get_teams_by_club(&self, club_id: Uuid) -> Result<Vec<Team>, DatabaseError> {
        Ok(self
            .teams
            .lock()
            .unwrap()
            .iter()
            .filter(|t| t.club_id == club_id)
            .cloned()
            .collect())
    }
    async fn get_teams_by_status(&self, _s: TeamStatus) -> Result<Vec<Team>, DatabaseError> {
        unimplemented!()
    }
    async fn get_teams_by_captain(&self, _c: Uuid) -> Result<Vec<Team>, DatabaseError> {
        unimplemented!()
    }
    async fn search_teams(&self, _q: String) -> Result<Vec<Team>, DatabaseError> {
        unimplemented!()
    }
    async fn add_player_to_team(
        &self,
        _t: Uuid,
        _p: Uuid,
        _r: TeamMemberRole,
        _pos: Option<String>,
    ) -> Result<TeamMember, DatabaseError> {
        unimplemented!()
    }
    async fn remove_player_from_team(&self, _t: Uuid, _p: Uuid) -> Result<bool, DatabaseError> {
        unimplemented!()
    }
    async fn get_team_members(&self, _t: Uuid) -> Result<Vec<TeamMemberWithPlayer>, DatabaseError> {
        unimplemented!()
    }
    async fn get_teams_of_player(&self, player_id: Uuid) -> Result<Vec<Team>, DatabaseError> {
        let teams = self.teams.lock().unwrap().clone();
        let members = self.members.lock().unwrap().clone();
        Ok(members
            .iter()
            .filter(|(_, p)| *p == player_id)
            .filter_map(|(t, _)| teams.iter().find(|t2| t2.id == *t).cloned())
            .collect())
    }
}

struct MockCompetitionRepository {
    items: Mutex<Vec<CompetitionEvent>>,
}

#[async_trait]
impl CompetitionRepository for MockCompetitionRepository {
    async fn create_competition(
        &self,
        _r: CreateCompetitionRequest,
    ) -> Result<CompetitionEvent, DatabaseError> {
        unimplemented!()
    }
    async fn get_competition(&self, id: Uuid) -> Result<Option<CompetitionEvent>, DatabaseError> {
        Ok(self
            .items
            .lock()
            .unwrap()
            .iter()
            .find(|c| c.id == id)
            .cloned())
    }
    async fn get_competitions(
        &self,
        _t: Option<CompetitionType>,
        _s: Option<palet_shared::CompetitionStatus>,
        _season: Option<String>,
    ) -> Result<Vec<CompetitionEvent>, DatabaseError> {
        Ok(self.items.lock().unwrap().clone())
    }
    async fn update_competition(
        &self,
        _id: Uuid,
        _r: UpdateCompetitionRequest,
    ) -> Result<Option<CompetitionEvent>, DatabaseError> {
        unimplemented!()
    }
    async fn delete_competition(&self, _id: Uuid) -> Result<bool, DatabaseError> {
        unimplemented!()
    }
    async fn register_team(
        &self,
        _c: Uuid,
        _t: Uuid,
        _u: Uuid,
    ) -> Result<Registration, DatabaseError> {
        unimplemented!()
    }
    async fn get_registrations(&self, _c: Uuid) -> Result<Vec<Registration>, DatabaseError> {
        unimplemented!()
    }
    async fn get_confirmed_team_ids(&self, _c: Uuid) -> Result<Vec<Uuid>, DatabaseError> {
        unimplemented!()
    }
    async fn confirm_registration(
        &self,
        _c: Uuid,
        _t: Uuid,
        _u: Uuid,
    ) -> Result<Option<Registration>, DatabaseError> {
        unimplemented!()
    }
    async fn count_registrations(&self, _c: Uuid) -> Result<i32, DatabaseError> {
        unimplemented!()
    }
}

struct MockNewsRepository;

#[async_trait]
impl NewsRepository for MockNewsRepository {
    async fn create_article(
        &self,
        _r: CreateNewsRequest,
        _a: Uuid,
    ) -> Result<NewsArticle, DatabaseError> {
        unimplemented!()
    }
    async fn get_article(&self, _id: Uuid) -> Result<Option<NewsArticle>, DatabaseError> {
        unimplemented!()
    }
    async fn get_article_by_slug(&self, _s: &str) -> Result<Option<NewsArticle>, DatabaseError> {
        unimplemented!()
    }
    async fn get_published_articles(
        &self,
        _c: Option<String>,
        _q: Option<String>,
    ) -> Result<Vec<NewsArticle>, DatabaseError> {
        Ok(Vec::new())
    }
    async fn get_all_articles(&self) -> Result<Vec<NewsArticle>, DatabaseError> {
        unimplemented!()
    }
    async fn update_article(
        &self,
        _id: Uuid,
        _r: UpdateNewsRequest,
    ) -> Result<Option<NewsArticle>, DatabaseError> {
        unimplemented!()
    }
    async fn delete_article(&self, _id: Uuid) -> Result<bool, DatabaseError> {
        unimplemented!()
    }
}

// ============================================
// APP DE TEST
// ============================================

struct TestRepos {
    notifications: Arc<MockNotificationRepository>,
    disponibilites: Arc<MockAvailabilityRepository>,
    matches: Arc<MockMatchRepository>,
    players: Arc<MockPlayerRepository>,
    users: Arc<MockUserRepository>,
    clubs: Arc<MockClubRepository>,
    teams: Arc<MockTeamRepository>,
    competitions: Arc<MockCompetitionRepository>,
}

fn test_state(repos: &TestRepos) -> web::Data<AppState> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_millis(500))
        .connect_lazy("postgres://palet:palet@localhost:5432/palet")
        .expect("pool lazy");
    let mut db = Database::new(pool);
    db.notifications = repos.notifications.clone();
    db.disponibilites = repos.disponibilites.clone();
    db.matches = repos.matches.clone();
    db.players = repos.players.clone();
    db.users = repos.users.clone();
    db.clubs = repos.clubs.clone();
    db.teams = repos.teams.clone();
    db.competitions = repos.competitions.clone();
    db.news = Arc::new(MockNewsRepository);
    web::Data::new(AppState {
        db: Arc::new(db),
        config: Arc::new(AppConfig::default()),
    })
}

macro_rules! init_app {
    ($repos:expr) => {
        test::init_service(
            App::new()
                .app_data(test_state(&$repos))
                .service(
                    web::scope("/notifications")
                        .route("", web::get().to(notification_handler::list_notifications))
                        .route("/count", web::get().to(notification_handler::unread_count))
                        .route("/{id}/lu", web::post().to(notification_handler::mark_read))
                        .route(
                            "/tout-lu",
                            web::post().to(notification_handler::mark_all_read),
                        ),
                )
                .service(
                    web::scope("/matchs")
                        .route(
                            "/{id}/dispos",
                            web::get().to(availability_handler::get_dispos),
                        )
                        .route(
                            "/{id}/dispo",
                            web::post().to(availability_handler::set_dispo),
                        ),
                )
                .service(web::scope("/recherche").route("", web::get().to(search_handler::search)))
                .service(
                    web::scope("/stats")
                        .route("/joueur/{id}", web::get().to(stats_handler::player_stats))
                        .route("/club/{id}", web::get().to(stats_handler::club_stats))
                        .route(
                            "/competition/{id}",
                            web::get().to(stats_handler::competition_stats),
                        ),
                ),
        )
        .await
    };
}

fn token_for(user: &UserAccount) -> String {
    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        role: user.role.clone(),
        club_id: user.club_id.map(|c| c.to_string()),
        token_type: ACCESS_TOKEN_TYPE.to_string(),
        iat: OffsetDateTime::now_utc().unix_timestamp(),
        exp: (OffsetDateTime::now_utc() + time::Duration::hours(1)).unix_timestamp(),
    };
    create_token(&AppConfig::default().jwt_secret, &claims).expect("token")
}

fn auth(user: &UserAccount) -> (actix_web::http::header::HeaderName, String) {
    (
        actix_web::http::header::AUTHORIZATION,
        format!("Bearer {}", token_for(user)),
    )
}

fn default_repos() -> TestRepos {
    TestRepos {
        notifications: Arc::new(MockNotificationRepository {
            items: Mutex::new(Vec::new()),
        }),
        disponibilites: Arc::new(MockAvailabilityRepository {
            items: Mutex::new(Vec::new()),
        }),
        matches: Arc::new(MockMatchRepository {
            matches: Mutex::new(Vec::new()),
        }),
        players: Arc::new(MockPlayerRepository {
            players: Mutex::new(Vec::new()),
        }),
        users: Arc::new(MockUserRepository {
            users: Mutex::new(Vec::new()),
        }),
        clubs: Arc::new(MockClubRepository {
            clubs: Mutex::new(Vec::new()),
        }),
        teams: Arc::new(MockTeamRepository {
            teams: Mutex::new(Vec::new()),
            members: Mutex::new(Vec::new()),
        }),
        competitions: Arc::new(MockCompetitionRepository {
            items: Mutex::new(Vec::new()),
        }),
    }
}

// ============================================
// TESTS — NOTIFICATIONS
// ============================================

#[actix_web::test]
async fn notifications_flow_list_count_read_all() {
    let repos = default_repos();
    let user = fixture_user(Uuid::new_v4(), UserRole::Player, None, Some("79-10000"));
    repos.users.lock_users_push(user.clone());
    repos
        .notifications
        .create(
            user.id,
            "match_verifie",
            "Match vérifié",
            "msg",
            Some("/arbitrage/x"),
        )
        .await
        .unwrap();
    repos
        .notifications
        .create(
            user.id,
            "inscription_confirmee",
            "Inscription confirmée",
            "msg2",
            None,
        )
        .await
        .unwrap();
    let app = init_app!(repos);
    let auth = auth(&user);

    // liste complète
    let req = test::TestRequest::get()
        .uri("/notifications")
        .insert_header(auth.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<Vec<AppNotification>> = test::read_body_json(resp).await;
    assert_eq!(body.data.len(), 2);

    // compteur non lues
    let req = test::TestRequest::get()
        .uri("/notifications/count")
        .insert_header(auth.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: ApiResponse<palet_shared::UnreadCount> = test::read_body_json(resp).await;
    assert_eq!(body.data.unread, 2);

    // marquer une comme lue
    let first_id = repos.notifications.items.lock().unwrap()[0].id;
    let req = test::TestRequest::post()
        .uri(&format!("/notifications/{first_id}/lu"))
        .insert_header(auth.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // unread = 1 puis tout-lu
    let req = test::TestRequest::post()
        .uri("/notifications/tout-lu")
        .insert_header(auth)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<i64> = test::read_body_json(resp).await;
    assert_eq!(body.data, 1);

    let req = test::TestRequest::get()
        .uri("/notifications/count")
        .insert_header((
            actix_web::http::header::AUTHORIZATION,
            format!("Bearer {}", token_for(&user)),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: ApiResponse<palet_shared::UnreadCount> = test::read_body_json(resp).await;
    assert_eq!(body.data.unread, 0);
}

#[actix_web::test]
async fn notifications_require_auth() {
    let repos = default_repos();
    let app = init_app!(repos);
    let req = test::TestRequest::get().uri("/notifications").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

// ============================================
// TESTS — DISPONIBILITÉS
// ============================================

#[actix_web::test]
async fn set_dispo_self_by_license_ok_and_summary() {
    let repos = default_repos();
    let club = Uuid::new_v4();
    let player = fixture_player(Uuid::new_v4(), club, "79-10000");
    let user = fixture_user(
        Uuid::new_v4(),
        UserRole::Player,
        Some(club),
        Some("79-10000"),
    );
    let m = fixture_match(Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
    repos.players.players.lock().unwrap().push(player.clone());
    repos.users.lock_users_push(user.clone());
    repos.matches.matches.lock().unwrap().push(m.clone());
    let app = init_app!(repos);
    let auth = auth(&user);

    // POST dispo pour soi-même (même licence)
    let req = test::TestRequest::post()
        .uri(&format!("/matchs/{}/dispo", m.id))
        .insert_header(auth.clone())
        .set_json(SetAvailabilityRequest {
            player_id: player.id,
            status: AvailabilityStatus::Disponible,
            commentaire: Some("OK pour samedi".to_string()),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<MatchAvailability> = test::read_body_json(resp).await;
    assert_eq!(body.data.status, AvailabilityStatus::Disponible);

    // GET dispos -> récap
    let req = test::TestRequest::get()
        .uri(&format!("/matchs/{}/dispos", m.id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<AvailabilitySummary> = test::read_body_json(resp).await;
    assert_eq!(body.data.total, 1);
    assert_eq!(body.data.disponibles, 1);
}

#[actix_web::test]
async fn set_dispo_other_player_forbidden() {
    let repos = default_repos();
    let club_a = Uuid::new_v4();
    let club_b = Uuid::new_v4();
    let other = fixture_player(Uuid::new_v4(), club_b, "79-99999");
    let user = fixture_user(
        Uuid::new_v4(),
        UserRole::Player,
        Some(club_a),
        Some("79-10000"),
    );
    let m = fixture_match(Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
    repos.players.players.lock().unwrap().push(other.clone());
    repos.users.lock_users_push(user.clone());
    repos.matches.matches.lock().unwrap().push(m.clone());
    let app = init_app!(repos);

    let req = test::TestRequest::post()
        .uri(&format!("/matchs/{}/dispo", m.id))
        .insert_header(auth(&user))
        .set_json(SetAvailabilityRequest {
            player_id: other.id,
            status: AvailabilityStatus::Indisponible,
            commentaire: None,
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_web::test]
async fn set_dispo_requires_auth_and_existing_match() {
    let repos = default_repos();
    let club = Uuid::new_v4();
    let player = fixture_player(Uuid::new_v4(), club, "79-10000");
    let m = fixture_match(Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
    repos.players.players.lock().unwrap().push(player.clone());
    repos.matches.matches.lock().unwrap().push(m.clone());
    let app = init_app!(repos);

    // sans token -> 401
    let req = test::TestRequest::post()
        .uri(&format!("/matchs/{}/dispo", m.id))
        .set_json(SetAvailabilityRequest {
            player_id: player.id,
            status: AvailabilityStatus::Disponible,
            commentaire: None,
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    // match inconnu -> 404 (avec token)
    let user = fixture_user(
        Uuid::new_v4(),
        UserRole::Player,
        Some(club),
        Some("79-10000"),
    );
    repos.users.lock_users_push(user.clone());
    let app = init_app!(repos);
    let req = test::TestRequest::post()
        .uri(&format!("/matchs/{}/dispo", Uuid::new_v4()))
        .insert_header(auth(&user))
        .set_json(SetAvailabilityRequest {
            player_id: player.id,
            status: AvailabilityStatus::Disponible,
            commentaire: None,
        })
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// ============================================
// TESTS — RECHERCHE
// ============================================

#[actix_web::test]
async fn search_finds_club_and_rejects_too_short_query() {
    let repos = default_repos();
    let club = fixture_club(Uuid::new_v4(), "La Roche-sur-Yon Palet Club");
    repos.clubs.clubs.lock().unwrap().push(club.clone());
    let app = init_app!(repos);

    let req = test::TestRequest::get()
        .uri("/recherche?q=roche")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<SearchResults> = test::read_body_json(resp).await;
    assert_eq!(body.data.clubs.len(), 1);
    assert_eq!(body.data.clubs[0].id, club.id);

    // requête trop courte -> résultats vides
    let req = test::TestRequest::get().uri("/recherche?q=x").to_request();
    let resp = test::call_service(&app, req).await;
    let body: ApiResponse<SearchResults> = test::read_body_json(resp).await;
    assert!(body.data.clubs.is_empty());
}

// ============================================
// TESTS — STATS
// ============================================

#[actix_web::test]
async fn player_stats_endpoint_computes_wins() {
    let repos = default_repos();
    let club = Uuid::new_v4();
    let team = fixture_team(Uuid::new_v4(), club, "LRPC 1");
    let player = fixture_player(Uuid::new_v4(), club, "79-10000");
    let m = fixture_match(Uuid::new_v4(), team.id, Uuid::new_v4()); // team gagne 13-9
    repos.teams.teams.lock().unwrap().push(team.clone());
    repos
        .teams
        .members
        .lock()
        .unwrap()
        .push((team.id, player.id));
    repos.players.players.lock().unwrap().push(player.clone());
    repos.matches.matches.lock().unwrap().push(m.clone());
    let app = init_app!(repos);

    let req = test::TestRequest::get()
        .uri(&format!("/stats/joueur/{}", player.id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<PlayerStats> = test::read_body_json(resp).await;
    assert_eq!(body.data.matches_played, 1);
    assert_eq!(body.data.wins, 1);
    assert_eq!(body.data.points_for, 13);
    assert!((body.data.win_rate - 1.0).abs() < 1e-9);

    // joueur inconnu -> 404
    let req = test::TestRequest::get()
        .uri(&format!("/stats/joueur/{}", Uuid::new_v4()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn club_stats_endpoint() {
    let repos = default_repos();
    let club = fixture_club(Uuid::new_v4(), "LRPC");
    let team = fixture_team(Uuid::new_v4(), club.id, "LRPC 1");
    let m = fixture_match(Uuid::new_v4(), team.id, Uuid::new_v4());
    repos.clubs.clubs.lock().unwrap().push(club.clone());
    repos.teams.teams.lock().unwrap().push(team.clone());
    repos.matches.matches.lock().unwrap().push(m);
    let app = init_app!(repos);

    let req = test::TestRequest::get()
        .uri(&format!("/stats/club/{}", club.id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<ClubStats> = test::read_body_json(resp).await;
    assert_eq!(body.data.teams_count, 1);
    assert_eq!(body.data.matches_played, 1);
    assert_eq!(body.data.wins, 1);
}

// petit helper pour pousser un user dans le mock
trait LockUsersPush {
    fn lock_users_push(&self, u: UserAccount);
}
impl LockUsersPush for Arc<MockUserRepository> {
    fn lock_users_push(&self, u: UserAccount) {
        self.users.lock().unwrap().push(u);
    }
}
