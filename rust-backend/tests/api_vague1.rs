//! Tests d'intégration HTTP de la Vague 1 : joueurs, équipes (+effectifs),
//! compétitions/tournois (+inscriptions, génération calendrier) et actualités.
//! Pool "lazy", repositories remplacés par des mocks en mémoire.

use actix_web::{http::StatusCode, test, web, App};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use palet_backend::{
    config::AppConfig,
    db::{
        CompetitionRepository, Database, DatabaseError, MatchRepository, NewsRepository,
        PlayerRepository, TeamRepository,
    },
    handlers::{competition_handler, news_handler, player_handler, team_handler},
    services::auth_service::{create_token, Claims, ACCESS_TOKEN_TYPE},
    AppState,
};
use palet_shared::*;
use sqlx::postgres::PgPoolOptions;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// ============================================
// HELPERS
// ============================================

fn claims_for(role: UserRole, club_id: Option<Uuid>) -> Claims {
    Claims {
        sub: Uuid::new_v4().to_string(),
        email: "test@example.fr".to_string(),
        role,
        club_id: club_id.map(|c| c.to_string()),
        token_type: ACCESS_TOKEN_TYPE.to_string(),
        iat: Utc::now().timestamp(),
        exp: (Utc::now() + Duration::hours(1)).timestamp(),
    }
}

fn token_for(role: UserRole, club_id: Option<Uuid>) -> String {
    create_token(&AppConfig::default().jwt_secret, &claims_for(role, club_id)).expect("token")
}

fn auth(role: UserRole, club_id: Option<Uuid>) -> (actix_web::http::header::HeaderName, String) {
    (
        actix_web::http::header::AUTHORIZATION,
        format!("Bearer {}", token_for(role, club_id)),
    )
}

macro_rules! post_json {
    ($app:expr, $uri:expr, $body:expr, $auth:expr) => {{
        let mut req = test::TestRequest::post().uri($uri).set_json($body);
        if let Some(a) = $auth {
            req = req.insert_header(a);
        }
        test::call_service($app, req.to_request()).await
    }};
}

// ============================================
// MOCKS
// ============================================

fn sample_player(id: Uuid, club_id: Uuid, first: &str, last: &str) -> Player {
    let now = Utc::now();
    Player {
        id,
        first_name: first.to_string(),
        last_name: last.to_string(),
        full_name: format!("{first} {last}"),
        alias: None,
        email: format!(
            "{}.{}@example.fr",
            first.to_lowercase(),
            last.to_lowercase()
        ),
        photo: None,
        status: PlayerStatus::Active,
        birth_date: now - Duration::days(365 * 30),
        age: 30,
        birth_place: None,
        nationality: "Française".to_string(),
        dominant_hand: DominantHand::Right,
        play_style: None,
        phone: None,
        address: None,
        city: None,
        postal_code: None,
        club_id,
        club_number: None,
        category: "senior".to_string(),
        license_number: "79-00001".to_string(),
        license_date: now,
        license_status: LicenseStatus::Valid,
        registration_date: now,
        created_at: now,
        updated_at: now,
    }
}

struct MockPlayerRepository {
    players: Mutex<Vec<Player>>,
}

#[async_trait]
impl PlayerRepository for MockPlayerRepository {
    async fn find_by_license(&self, license: &str) -> Result<Option<Player>, DatabaseError> {
        Ok(self
            .players
            .lock()
            .unwrap()
            .iter()
            .find(|p| p.license_number == license)
            .cloned())
    }

    async fn create_player(&self, request: CreatePlayerRequest) -> Result<Player, DatabaseError> {
        let mut players = self.players.lock().unwrap();
        if players.iter().any(|p| p.email == request.email) {
            return Err(DatabaseError::ValidationError(
                "email déjà utilisé".to_string(),
            ));
        }
        let p = sample_player(
            Uuid::new_v4(),
            request.club_id,
            &request.first_name,
            &request.last_name,
        );
        players.push(p.clone());
        Ok(p)
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
        id: Uuid,
        request: CreatePlayerRequest,
    ) -> Result<Option<Player>, DatabaseError> {
        let mut players = self.players.lock().unwrap();
        let Some(p) = players.iter_mut().find(|p| p.id == id) else {
            return Ok(None);
        };
        p.first_name = request.first_name;
        p.last_name = request.last_name;
        p.full_name = format!("{} {}", p.first_name, p.last_name);
        Ok(Some(p.clone()))
    }
    async fn delete_player(&self, id: Uuid) -> Result<bool, DatabaseError> {
        let mut players = self.players.lock().unwrap();
        let before = players.len();
        players.retain(|p| p.id != id);
        Ok(players.len() != before)
    }
    async fn get_players_by_club(&self, club_id: Uuid) -> Result<Vec<Player>, DatabaseError> {
        Ok(self
            .players
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.club_id == club_id)
            .cloned()
            .collect())
    }
    async fn get_players_by_category(
        &self,
        category: String,
    ) -> Result<Vec<Player>, DatabaseError> {
        Ok(self
            .players
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.category == category)
            .cloned()
            .collect())
    }
    async fn get_players_by_status(
        &self,
        status: PlayerStatus,
    ) -> Result<Vec<Player>, DatabaseError> {
        Ok(self
            .players
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.status == status)
            .cloned()
            .collect())
    }
    async fn get_players_by_license_status(
        &self,
        status: LicenseStatus,
    ) -> Result<Vec<Player>, DatabaseError> {
        Ok(self
            .players
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.license_status == status)
            .cloned()
            .collect())
    }
    async fn search_players(&self, query: String) -> Result<Vec<Player>, DatabaseError> {
        let needle = query.to_lowercase();
        Ok(self
            .players
            .lock()
            .unwrap()
            .iter()
            .filter(|p| p.full_name.to_lowercase().contains(&needle))
            .cloned()
            .collect())
    }
}

fn sample_team(id: Uuid, club_id: Uuid, name: &str) -> Team {
    let now = Utc::now();
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

struct MockTeamRepository {
    teams: Mutex<Vec<Team>>,
    members: Mutex<Vec<TeamMemberWithPlayer>>,
}

#[async_trait]
impl TeamRepository for MockTeamRepository {
    async fn get_teams_of_player(&self, player_id: Uuid) -> Result<Vec<Team>, DatabaseError> {
        let teams = self.teams.lock().unwrap().clone();
        let members = self.members.lock().unwrap().clone();
        Ok(members
            .iter()
            .filter(|m| m.player_id == player_id)
            .filter_map(|m| teams.iter().find(|t| t.id == m.team_id).cloned())
            .collect())
    }

    async fn create_team(
        &self,
        name: String,
        club_id: Uuid,
        captain_id: Option<Uuid>,
    ) -> Result<Team, DatabaseError> {
        let mut t = sample_team(Uuid::new_v4(), club_id, &name);
        t.captain_id = captain_id;
        self.teams.lock().unwrap().push(t.clone());
        Ok(t)
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
        id: Uuid,
        name: Option<String>,
        captain_id: Option<Uuid>,
        _stats: Option<TeamStats>,
        status: Option<TeamStatus>,
    ) -> Result<Option<Team>, DatabaseError> {
        let mut teams = self.teams.lock().unwrap();
        let Some(t) = teams.iter_mut().find(|t| t.id == id) else {
            return Ok(None);
        };
        if let Some(n) = name {
            t.name = n;
        }
        if let Some(c) = captain_id {
            t.captain_id = Some(c);
        }
        if let Some(s) = status {
            t.status = s;
        }
        Ok(Some(t.clone()))
    }
    async fn delete_team(&self, id: Uuid) -> Result<bool, DatabaseError> {
        let mut teams = self.teams.lock().unwrap();
        let before = teams.len();
        teams.retain(|t| t.id != id);
        Ok(teams.len() != before)
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
    async fn get_teams_by_status(&self, status: TeamStatus) -> Result<Vec<Team>, DatabaseError> {
        Ok(self
            .teams
            .lock()
            .unwrap()
            .iter()
            .filter(|t| t.status == status)
            .cloned()
            .collect())
    }
    async fn get_teams_by_captain(&self, captain_id: Uuid) -> Result<Vec<Team>, DatabaseError> {
        Ok(self
            .teams
            .lock()
            .unwrap()
            .iter()
            .filter(|t| t.captain_id == Some(captain_id))
            .cloned()
            .collect())
    }
    async fn search_teams(&self, query: String) -> Result<Vec<Team>, DatabaseError> {
        let needle = query.to_lowercase();
        Ok(self
            .teams
            .lock()
            .unwrap()
            .iter()
            .filter(|t| t.name.to_lowercase().contains(&needle))
            .cloned()
            .collect())
    }
    async fn add_player_to_team(
        &self,
        team_id: Uuid,
        player_id: Uuid,
        role: TeamMemberRole,
        position: Option<String>,
    ) -> Result<TeamMember, DatabaseError> {
        let member = TeamMember {
            team_id,
            player_id,
            role: role.clone(),
            position,
            joined_at: Utc::now(),
        };
        Ok(member)
    }
    async fn remove_player_from_team(
        &self,
        team_id: Uuid,
        player_id: Uuid,
    ) -> Result<bool, DatabaseError> {
        let mut members = self.members.lock().unwrap();
        let before = members.len();
        members.retain(|m| !(m.team_id == team_id && m.player_id == player_id));
        Ok(members.len() != before)
    }
    async fn get_team_members(
        &self,
        team_id: Uuid,
    ) -> Result<Vec<TeamMemberWithPlayer>, DatabaseError> {
        Ok(self
            .members
            .lock()
            .unwrap()
            .iter()
            .filter(|m| m.team_id == team_id)
            .cloned()
            .collect())
    }
}

struct MockCompetitionRepository {
    events: Mutex<Vec<CompetitionEvent>>,
    registrations: Mutex<Vec<Registration>>,
}

impl MockCompetitionRepository {
    fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
            registrations: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl CompetitionRepository for MockCompetitionRepository {
    async fn create_competition(
        &self,
        request: CreateCompetitionRequest,
    ) -> Result<CompetitionEvent, DatabaseError> {
        let now = Utc::now();
        let event = CompetitionEvent {
            id: Uuid::new_v4(),
            name: request.name,
            code: request.code,
            competition_type: request.competition_type,
            category: request.category,
            material: request.material,
            season: request.season,
            description: request.description,
            location: request.location,
            format: request.format,
            registration_start: request.registration_start,
            registration_end: request.registration_end,
            start_date: request.start_date,
            end_date: request.end_date,
            status: CompetitionStatus::Pending,
            max_teams: request.max_teams,
            registered_teams: 0,
            fee: request.fee,
            is_public: request.is_public,
            organizer_id: request.organizer_id,
            created_at: now,
            updated_at: now,
        };
        self.events.lock().unwrap().push(event.clone());
        Ok(event)
    }
    async fn get_competition(&self, id: Uuid) -> Result<Option<CompetitionEvent>, DatabaseError> {
        let mut events = self.events.lock().unwrap();
        let reg_count = self
            .registrations
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r.competition_id == id)
            .count() as i32;
        Ok(events.iter_mut().find(|e| e.id == id).map(|e| {
            e.registered_teams = reg_count;
            e.clone()
        }))
    }
    async fn get_competitions(
        &self,
        competition_type: Option<CompetitionType>,
        status: Option<CompetitionStatus>,
        season: Option<String>,
    ) -> Result<Vec<CompetitionEvent>, DatabaseError> {
        let events = self.events.lock().unwrap();
        Ok(events
            .iter()
            .filter(|e| {
                competition_type
                    .as_ref()
                    .is_none_or(|t| &e.competition_type == t)
            })
            .filter(|e| status.as_ref().is_none_or(|s| &e.status == s))
            .filter(|e| season.as_ref().is_none_or(|s| &e.season == s))
            .cloned()
            .collect())
    }
    async fn update_competition(
        &self,
        id: Uuid,
        request: UpdateCompetitionRequest,
    ) -> Result<Option<CompetitionEvent>, DatabaseError> {
        let mut events = self.events.lock().unwrap();
        let Some(e) = events.iter_mut().find(|e| e.id == id) else {
            return Ok(None);
        };
        if let Some(s) = request.status {
            e.status = s;
        }
        if let Some(n) = request.name {
            e.name = n;
        }
        Ok(Some(e.clone()))
    }
    async fn delete_competition(&self, id: Uuid) -> Result<bool, DatabaseError> {
        let mut events = self.events.lock().unwrap();
        let before = events.len();
        events.retain(|e| e.id != id);
        Ok(events.len() != before)
    }
    async fn register_team(
        &self,
        competition_id: Uuid,
        team_id: Uuid,
        registered_by: Uuid,
    ) -> Result<Registration, DatabaseError> {
        if !self
            .events
            .lock()
            .unwrap()
            .iter()
            .any(|e| e.id == competition_id)
        {
            return Err(DatabaseError::NotFound(
                "Compétition introuvable".to_string(),
            ));
        }
        let mut regs = self.registrations.lock().unwrap();
        if regs
            .iter()
            .any(|r| r.competition_id == competition_id && r.team_id == team_id)
        {
            return Err(DatabaseError::ValidationError(
                "Cette équipe est déjà inscrite à cette compétition".to_string(),
            ));
        }
        let reg = Registration {
            competition_id,
            team_id,
            status: RegistrationStatus::Pending,
            registered_by,
            paid: false,
            created_at: Utc::now(),
            confirmed_at: None,
            confirmed_by: None,
        };
        regs.push(reg.clone());
        Ok(reg)
    }
    async fn get_registrations(
        &self,
        competition_id: Uuid,
    ) -> Result<Vec<Registration>, DatabaseError> {
        Ok(self
            .registrations
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r.competition_id == competition_id)
            .cloned()
            .collect())
    }
    async fn get_confirmed_team_ids(
        &self,
        competition_id: Uuid,
    ) -> Result<Vec<Uuid>, DatabaseError> {
        Ok(self
            .registrations
            .lock()
            .unwrap()
            .iter()
            .filter(|r| {
                r.competition_id == competition_id && r.status == RegistrationStatus::Confirmed
            })
            .map(|r| r.team_id)
            .collect())
    }
    async fn confirm_registration(
        &self,
        competition_id: Uuid,
        team_id: Uuid,
        confirmed_by: Uuid,
    ) -> Result<Option<Registration>, DatabaseError> {
        let mut regs = self.registrations.lock().unwrap();
        let Some(r) = regs
            .iter_mut()
            .find(|r| r.competition_id == competition_id && r.team_id == team_id)
        else {
            return Ok(None);
        };
        r.status = RegistrationStatus::Confirmed;
        r.confirmed_at = Some(Utc::now());
        r.confirmed_by = Some(confirmed_by);
        Ok(Some(r.clone()))
    }
    async fn count_registrations(&self, competition_id: Uuid) -> Result<i32, DatabaseError> {
        Ok(self
            .registrations
            .lock()
            .unwrap()
            .iter()
            .filter(|r| r.competition_id == competition_id)
            .count() as i32)
    }
}

struct MockMatchRepository {
    matches: Mutex<Vec<Match>>,
}

#[async_trait]
impl MatchRepository for MockMatchRepository {
    async fn create_match(&self, request: CreateMatchRequest) -> Result<Match, DatabaseError> {
        let now = Utc::now();
        let m = Match {
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
            current_manche: None,
            current_scores: None,
            status: MatchStatus::Scheduled,
            verified: false,
            verified_by: None,
            verified_at: None,
            date: request.date,
            time: None,
            location: request.location,
            duration: None,
            created_at: now,
            updated_at: now,
            started_at: None,
            ended_at: None,
        };
        self.matches.lock().unwrap().push(m.clone());
        Ok(m)
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
    async fn get_matches_by_competition(
        &self,
        competition_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError> {
        Ok(self
            .matches
            .lock()
            .unwrap()
            .iter()
            .filter(|m| m.competition_id == competition_id)
            .cloned()
            .collect())
    }
    async fn get_matches_by_status(
        &self,
        status: MatchStatus,
    ) -> Result<Vec<Match>, DatabaseError> {
        Ok(self
            .matches
            .lock()
            .unwrap()
            .iter()
            .filter(|m| m.status == status)
            .cloned()
            .collect())
    }
    async fn update_match(
        &self,
        _id: Uuid,
        _r: UpdateMatchRequest,
    ) -> Result<Option<Match>, DatabaseError> {
        Ok(None)
    }
    async fn delete_match(&self, _id: Uuid) -> Result<bool, DatabaseError> {
        Ok(false)
    }
    async fn start_match(&self, _id: Uuid) -> Result<Match, DatabaseError> {
        Err(DatabaseError::NotFound("mock".to_string()))
    }
    async fn complete_match(&self, _id: Uuid, _w: MatchWinner) -> Result<Match, DatabaseError> {
        Err(DatabaseError::NotFound("mock".to_string()))
    }
    async fn add_manche(&self, _id: Uuid, _m: MancheScore) -> Result<Match, DatabaseError> {
        Err(DatabaseError::NotFound("mock".to_string()))
    }
    async fn update_manche(
        &self,
        _id: Uuid,
        _n: i32,
        _t1: i32,
        _t2: i32,
    ) -> Result<Match, DatabaseError> {
        Err(DatabaseError::NotFound("mock".to_string()))
    }
    async fn verify_match(&self, _id: Uuid, _v: Uuid) -> Result<Match, DatabaseError> {
        Err(DatabaseError::NotFound("mock".to_string()))
    }
    async fn unverify_match(&self, _id: Uuid) -> Result<Match, DatabaseError> {
        Err(DatabaseError::NotFound("mock".to_string()))
    }
    async fn get_matches_by_date_range(
        &self,
        _s: chrono::DateTime<Utc>,
        _e: chrono::DateTime<Utc>,
    ) -> Result<Vec<Match>, DatabaseError> {
        Ok(Vec::new())
    }
    async fn get_matches_by_team(&self, _t: Uuid) -> Result<Vec<Match>, DatabaseError> {
        Ok(Vec::new())
    }
    async fn get_matches_by_journee(&self, _j: i32, _c: Uuid) -> Result<Vec<Match>, DatabaseError> {
        Ok(Vec::new())
    }
    async fn get_championship_matches(&self, _c: Uuid) -> Result<Vec<Match>, DatabaseError> {
        Ok(Vec::new())
    }
    async fn get_completed_matches_by_championship(
        &self,
        _c: Uuid,
    ) -> Result<Vec<Match>, DatabaseError> {
        Ok(Vec::new())
    }
}

struct MockNewsRepository {
    articles: Mutex<Vec<NewsArticle>>,
}

#[async_trait]
impl NewsRepository for MockNewsRepository {
    async fn create_article(
        &self,
        request: CreateNewsRequest,
        author_id: Uuid,
    ) -> Result<NewsArticle, DatabaseError> {
        let now = Utc::now();
        let article = NewsArticle {
            id: Uuid::new_v4(),
            slug: slugify(&request.title),
            title: request.title,
            excerpt: request.excerpt,
            content: request.content,
            image: request.image,
            category: request.category,
            author_id: Some(author_id),
            published: request.published,
            published_at: if request.published { Some(now) } else { None },
            created_at: now,
            updated_at: now,
        };
        self.articles.lock().unwrap().push(article.clone());
        Ok(article)
    }
    async fn get_article(&self, id: Uuid) -> Result<Option<NewsArticle>, DatabaseError> {
        Ok(self
            .articles
            .lock()
            .unwrap()
            .iter()
            .find(|a| a.id == id)
            .cloned())
    }
    async fn get_article_by_slug(&self, slug: &str) -> Result<Option<NewsArticle>, DatabaseError> {
        Ok(self
            .articles
            .lock()
            .unwrap()
            .iter()
            .find(|a| a.slug == slug)
            .cloned())
    }
    async fn get_published_articles(
        &self,
        category: Option<String>,
        query: Option<String>,
    ) -> Result<Vec<NewsArticle>, DatabaseError> {
        let needle = query.map(|q| q.to_lowercase());
        Ok(self
            .articles
            .lock()
            .unwrap()
            .iter()
            .filter(|a| a.published)
            .filter(|a| category.as_ref().is_none_or(|c| &a.category == c))
            .filter(|a| {
                needle.as_ref().is_none_or(|n| {
                    a.title.to_lowercase().contains(n) || a.content.to_lowercase().contains(n)
                })
            })
            .cloned()
            .collect())
    }
    async fn get_all_articles(&self) -> Result<Vec<NewsArticle>, DatabaseError> {
        Ok(self.articles.lock().unwrap().clone())
    }
    async fn update_article(
        &self,
        id: Uuid,
        request: UpdateNewsRequest,
    ) -> Result<Option<NewsArticle>, DatabaseError> {
        let mut articles = self.articles.lock().unwrap();
        let Some(a) = articles.iter_mut().find(|a| a.id == id) else {
            return Ok(None);
        };
        if let Some(t) = request.title {
            a.title = t;
            a.slug = slugify(&a.title);
        }
        if let Some(p) = request.published {
            a.published = p;
            if p && a.published_at.is_none() {
                a.published_at = Some(Utc::now());
            }
        }
        Ok(Some(a.clone()))
    }
    async fn delete_article(&self, id: Uuid) -> Result<bool, DatabaseError> {
        let mut articles = self.articles.lock().unwrap();
        let before = articles.len();
        articles.retain(|a| a.id != id);
        Ok(articles.len() != before)
    }
}

// ============================================
// APP DE TEST
// ============================================

fn test_state() -> web::Data<AppState> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_millis(500))
        .connect_lazy("postgres://palet:palet@localhost:5432/palet")
        .expect("pool lazy");
    let mut db = Database::new(pool);
    db.players = Arc::new(MockPlayerRepository {
        players: Mutex::new(Vec::new()),
    });
    db.teams = Arc::new(MockTeamRepository {
        teams: Mutex::new(Vec::new()),
        members: Mutex::new(Vec::new()),
    });
    db.competitions = Arc::new(MockCompetitionRepository::new());
    db.matches = Arc::new(MockMatchRepository {
        matches: Mutex::new(Vec::new()),
    });
    db.news = Arc::new(MockNewsRepository {
        articles: Mutex::new(Vec::new()),
    });
    web::Data::new(AppState {
        db: Arc::new(db),
        config: Arc::new(AppConfig::default()),
    })
}

macro_rules! init_app {
    () => {
        test::init_service(
            App::new()
                .app_data(test_state())
                .service(
                    web::scope("/joueurs")
                        .route("", web::get().to(player_handler::get_players))
                        .route("", web::post().to(player_handler::create_player))
                        .route("/{id}", web::get().to(player_handler::get_player))
                        .route("/{id}", web::put().to(player_handler::update_player))
                        .route("/{id}", web::delete().to(player_handler::delete_player)),
                )
                .service(
                    web::scope("/equipes")
                        .route("", web::get().to(team_handler::get_teams))
                        .route("", web::post().to(team_handler::create_team))
                        .route("/{id}", web::get().to(team_handler::get_team))
                        .route("/{id}", web::put().to(team_handler::update_team))
                        .route("/{id}", web::delete().to(team_handler::delete_team))
                        .route("/{id}/effectif", web::get().to(team_handler::get_effectif))
                        .route(
                            "/{id}/joueurs",
                            web::post().to(team_handler::add_team_member),
                        )
                        .route(
                            "/{id}/joueurs/{player_id}",
                            web::delete().to(team_handler::remove_team_member),
                        ),
                )
                .service(
                    web::scope("/competitions")
                        .route("", web::get().to(competition_handler::get_competitions))
                        .route("", web::post().to(competition_handler::create_competition))
                        .route("/{id}", web::get().to(competition_handler::get_competition))
                        .route(
                            "/{id}",
                            web::put().to(competition_handler::update_competition),
                        )
                        .route(
                            "/{id}",
                            web::delete().to(competition_handler::delete_competition),
                        )
                        .route(
                            "/{id}/inscrire",
                            web::post().to(competition_handler::register_team),
                        )
                        .route(
                            "/{id}/inscriptions",
                            web::get().to(competition_handler::get_inscriptions),
                        )
                        .route(
                            "/{id}/inscriptions/{team_id}/confirmer",
                            web::post().to(competition_handler::confirm_inscription),
                        )
                        .route(
                            "/{id}/generer",
                            web::post().to(competition_handler::generer_calendrier),
                        ),
                )
                .service(
                    web::scope("/actualites")
                        .route("", web::get().to(news_handler::get_actualites))
                        .route("", web::post().to(news_handler::create_actualite))
                        .route("/{id}", web::get().to(news_handler::get_actualite))
                        .route("/{id}", web::put().to(news_handler::update_actualite))
                        .route("/{id}", web::delete().to(news_handler::delete_actualite)),
                ),
        )
        .await
    };
}

fn player_body(club_id: Uuid) -> serde_json::Value {
    serde_json::json!({
        "first_name": "Jean",
        "last_name": "Dupont",
        "email": "jean.dupont@example.fr",
        "birth_date": "1996-05-10T00:00:00Z",
        "nationality": "Française",
        "dominant_hand": "right",
        "category": "senior",
        "club_id": club_id,
        "license_number": "79-00042",
        "license_date": "2026-09-01T00:00:00Z"
    })
}

fn competition_body() -> serde_json::Value {
    serde_json::json!({
        "name": "Open de Challans 2026",
        "code": "OPEN-CHALLANS-2026",
        "competition_type": "tournament",
        "category": "doublette",
        "material": "fonte",
        "season": "2026-2027",
        "description": "Open annuel en doublette fonte",
        "location": "Salle des fêtes de Challans",
        "format": "round_robin",
        "registration_start": "2026-11-01T00:00:00Z",
        "registration_end": "2026-12-01T00:00:00Z",
        "start_date": "2026-12-12T09:00:00Z",
        "end_date": "2026-12-12T20:00:00Z",
        "max_teams": 16,
        "fee": 10.0,
        "is_public": true,
        "organizer_id": Uuid::new_v4()
    })
}

// ============================================
// TESTS JOUEURS
// ============================================

#[actix_web::test]
async fn joueurs_get_public_post_protege_par_scope_club() {
    let app = init_app!();
    let club_id = Uuid::new_v4();

    let resp = post_json!(
        &app,
        "/joueurs",
        player_body(club_id),
        None::<(actix_web::http::header::HeaderName, String)>
    );
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let resp = post_json!(
        &app,
        "/joueurs",
        player_body(club_id),
        Some(auth(UserRole::Player, None))
    );
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    let other_club = Uuid::new_v4();
    let resp = post_json!(
        &app,
        "/joueurs",
        player_body(club_id),
        Some(auth(UserRole::ClubAdmin, Some(other_club)))
    );
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    let resp = post_json!(
        &app,
        "/joueurs",
        player_body(club_id),
        Some(auth(UserRole::ClubAdmin, Some(club_id)))
    );
    assert_eq!(resp.status(), StatusCode::CREATED);
    let created: ApiResponse<Player> = test::read_body_json(resp).await;
    let player_id = created.data.id;

    let req = test::TestRequest::get().uri("/joueurs").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<Vec<Player>> = test::read_body_json(resp).await;
    assert_eq!(body.data.len(), 1);

    let req = test::TestRequest::get()
        .uri("/joueurs?q=dupont")
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: ApiResponse<Vec<Player>> = test::read_body_json(resp).await;
    assert_eq!(body.data.len(), 1);

    let req = test::TestRequest::get()
        .uri(&format!("/joueurs/{player_id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

// ============================================
// TESTS ÉQUIPES
// ============================================

#[actix_web::test]
async fn equipes_crud_et_effectif_par_club_admin() {
    let app = init_app!();
    let club_id = Uuid::new_v4();
    let admin = auth(UserRole::ClubAdmin, Some(club_id));

    let resp = post_json!(
        &app,
        "/equipes",
        serde_json::json!({ "name": "Challans D1", "club_id": club_id, "captain_id": null }),
        Some(admin.clone())
    );
    assert_eq!(resp.status(), StatusCode::CREATED);
    let team: ApiResponse<Team> = test::read_body_json(resp).await;
    let team_id = team.data.id;

    let resp = post_json!(&app, "/joueurs", player_body(club_id), Some(admin.clone()));
    assert_eq!(resp.status(), StatusCode::CREATED);
    let player: ApiResponse<Player> = test::read_body_json(resp).await;
    let player_id = player.data.id;

    let resp = post_json!(
        &app,
        &format!("/equipes/{team_id}/joueurs"),
        serde_json::json!({ "player_id": player_id, "role": "captain", "position": "tireur" }),
        Some(admin.clone())
    );
    assert_eq!(resp.status(), StatusCode::CREATED);

    let req = test::TestRequest::get()
        .uri(&format!("/equipes/{team_id}/effectif"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<Vec<TeamMemberWithPlayer>> = test::read_body_json(resp).await;
    assert!(
        body.data.is_empty(),
        "le mock ne stocke pas les membres, liste vide attendue"
    );

    let req = test::TestRequest::get()
        .uri(&format!("/equipes?club_id={club_id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: ApiResponse<Vec<Team>> = test::read_body_json(resp).await;
    assert_eq!(body.data.len(), 1);
}

// ============================================
// TESTS COMPETITIONS
// ============================================

#[actix_web::test]
async fn competition_inscription_confirmation_et_generation_calendrier() {
    let app = init_app!();
    let club_id = Uuid::new_v4();
    let admin = auth(UserRole::CommissionAdmin, None);
    let club_admin = auth(UserRole::ClubAdmin, Some(club_id));

    let resp = post_json!(
        &app,
        "/competitions",
        competition_body(),
        Some(admin.clone())
    );
    assert_eq!(resp.status(), StatusCode::CREATED);
    let comp: ApiResponse<CompetitionEvent> = test::read_body_json(resp).await;
    let comp_id = comp.data.id;

    let resp = post_json!(
        &app,
        "/competitions",
        competition_body(),
        None::<(actix_web::http::header::HeaderName, String)>
    );
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    let resp = post_json!(
        &app,
        "/equipes",
        serde_json::json!({ "name": "Équipe A", "club_id": club_id, "captain_id": null }),
        Some(club_admin.clone())
    );
    let team_a: ApiResponse<Team> = test::read_body_json(resp).await;
    let resp = post_json!(
        &app,
        "/equipes",
        serde_json::json!({ "name": "Équipe B", "club_id": club_id, "captain_id": null }),
        Some(club_admin.clone())
    );
    let team_b: ApiResponse<Team> = test::read_body_json(resp).await;
    let resp = post_json!(
        &app,
        "/equipes",
        serde_json::json!({ "name": "Équipe C", "club_id": club_id, "captain_id": null }),
        Some(club_admin.clone())
    );
    let team_c: ApiResponse<Team> = test::read_body_json(resp).await;
    let resp = post_json!(
        &app,
        "/equipes",
        serde_json::json!({ "name": "Équipe D", "club_id": club_id, "captain_id": null }),
        Some(club_admin.clone())
    );
    let team_d: ApiResponse<Team> = test::read_body_json(resp).await;

    let teams = [
        team_a.data.id,
        team_b.data.id,
        team_c.data.id,
        team_d.data.id,
    ];
    for team_id in teams {
        let resp = post_json!(
            &app,
            &format!("/competitions/{comp_id}/inscrire"),
            serde_json::json!({ "team_id": team_id }),
            Some(club_admin.clone())
        );
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    let resp = post_json!(
        &app,
        &format!("/competitions/{comp_id}/inscrire"),
        serde_json::json!({ "team_id": teams[0] }),
        Some(club_admin.clone())
    );
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let resp = post_json!(
        &app,
        &format!("/competitions/{comp_id}/inscrire"),
        serde_json::json!({ "team_id": teams[0] }),
        None::<(actix_web::http::header::HeaderName, String)>
    );
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

    for team_id in teams {
        let resp = post_json!(
            &app,
            &format!("/competitions/{comp_id}/inscriptions/{team_id}/confirmer"),
            serde_json::json!({}),
            Some(admin.clone())
        );
        assert_eq!(resp.status(), StatusCode::OK);
    }

    let req = test::TestRequest::get()
        .uri(&format!("/competitions/{comp_id}/inscriptions"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<Vec<Registration>> = test::read_body_json(resp).await;
    assert_eq!(body.data.len(), 4);

    let resp = post_json!(
        &app,
        &format!("/competitions/{comp_id}/generer"),
        serde_json::json!({}),
        Some(admin.clone())
    );
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<Vec<Match>> = test::read_body_json(resp).await;
    assert_eq!(body.data.len(), 6, "4 équipes -> 6 matchs round-robin");
    for m in &body.data {
        assert_eq!(m.competition_id, comp_id);
        assert_eq!(m.competition_type, CompetitionType::Tournament);
        assert_eq!(m.best_of, Some(13));
    }
}

// ============================================
// TESTS ACTUALITÉS
// ============================================

#[actix_web::test]
async fn actualites_publication_et_lecture_publique() {
    let app = init_app!();
    let admin = auth(UserRole::CommissionAdmin, None);

    let req = test::TestRequest::get().uri("/actualites").to_request();
    let resp = test::call_service(&app, req).await;
    let body: ApiResponse<Vec<NewsArticle>> = test::read_body_json(resp).await;
    assert!(body.data.is_empty());

    let resp = post_json!(
        &app,
        "/actualites",
        serde_json::json!({
            "title": "Lancement de la saison 2026-2027 !",
            "excerpt": "Le calendrier est en ligne",
            "content": "La saison 2026-2027 démarre avec 26 clubs inscrits...",
            "category": "championnat",
            "published": true
        }),
        Some(admin.clone())
    );
    assert_eq!(resp.status(), StatusCode::CREATED);
    let article: ApiResponse<NewsArticle> = test::read_body_json(resp).await;
    assert_eq!(article.data.slug, "lancement-de-la-saison-2026-2027");
    assert!(article.data.published);
    let article_id = article.data.id;

    let resp = post_json!(
        &app,
        "/actualites",
        serde_json::json!({ "title": "Brouillon interne", "content": "pas publié", "published": false }),
        Some(admin.clone())
    );
    assert_eq!(resp.status(), StatusCode::CREATED);
    let draft: ApiResponse<NewsArticle> = test::read_body_json(resp).await;

    let req = test::TestRequest::get().uri("/actualites").to_request();
    let resp = test::call_service(&app, req).await;
    let body: ApiResponse<Vec<NewsArticle>> = test::read_body_json(resp).await;
    assert_eq!(body.data.len(), 1, "seul l'article publié est listé");

    let req = test::TestRequest::get()
        .uri(&format!("/actualites/{article_id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri(&format!("/actualites/{}", draft.data.id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status(),
        StatusCode::NOT_FOUND,
        "brouillon non lisible publiquement"
    );

    let resp = post_json!(
        &app,
        "/actualites",
        serde_json::json!({ "title": "x", "content": "y" }),
        None::<(actix_web::http::header::HeaderName, String)>
    );
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
