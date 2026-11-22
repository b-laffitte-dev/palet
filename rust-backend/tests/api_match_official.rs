//! Tests d'intégration HTTP du flow arbitre : start → manches → complete →
//! verify/unverify sur /matchs (skill rust-tdd).
//!
//! Pool "lazy" (pas de base réelle requise) : le repository `matches` est
//! remplacé par un mock en mémoire qui reproduit la logique métier
//! (statuts, scores cumulés, vérification réservée aux matchs terminés).

use actix_web::{http::StatusCode, test, web, App};
use async_trait::async_trait;
use palet_backend::{
    config::AppConfig,
    db::{Database, DatabaseError, MatchRepository},
    handlers::match_handler,
    services::auth_service::{create_token, Claims, ACCESS_TOKEN_TYPE},
    AppState,
};
use palet_shared::{
    ApiResponse, CompetitionType, CreateMatchRequest, MancheScore, Match, MatchStatus, MatchWinner,
    UpdateMatchRequest,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::{Arc, Mutex};
use time::OffsetDateTime;
use uuid::Uuid;

// ============================================
// MOCK MATCH REPOSITORY (logique en mémoire)
// ============================================

struct MockMatchRepository {
    matches: Mutex<Vec<Match>>,
}

impl MockMatchRepository {
    fn new() -> Self {
        Self {
            matches: Mutex::new(Vec::new()),
        }
    }

    fn push(&self, m: Match) {
        self.matches.lock().unwrap().push(m);
    }
}

fn sample_match(status: MatchStatus) -> Match {
    let now = OffsetDateTime::now_utc();
    Match {
        id: Uuid::new_v4(),
        competition_id: Uuid::new_v4(),
        competition_type: CompetitionType::Championship,
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
        current_manche: None,
        current_scores: None,
        status,
        verified: false,
        verified_by: None,
        verified_at: None,
        date: now,
        time: Some("15h00".to_string()),
        location: Some("Terrain clos".to_string()),
        duration: None,
        created_at: now,
        updated_at: now,
        started_at: None,
        ended_at: None,
    }
}

#[async_trait]
impl MatchRepository for MockMatchRepository {
    async fn create_match(&self, request: CreateMatchRequest) -> Result<Match, DatabaseError> {
        let now = OffsetDateTime::now_utc();
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
        id: Uuid,
        request: UpdateMatchRequest,
    ) -> Result<Option<Match>, DatabaseError> {
        let mut ms = self.matches.lock().unwrap();
        let Some(m) = ms.iter_mut().find(|m| m.id == id) else {
            return Ok(None);
        };
        if let Some(s) = request.team1_score {
            m.team1_score = Some(s);
        }
        if let Some(s) = request.team2_score {
            m.team2_score = Some(s);
        }
        if let Some(s) = request.status {
            m.status = s;
        }
        if let Some(w) = request.winner {
            m.winner = Some(w);
        }
        if let Some(v) = request.verified {
            m.verified = v;
        }
        Ok(Some(m.clone()))
    }

    async fn delete_match(&self, id: Uuid) -> Result<bool, DatabaseError> {
        let mut ms = self.matches.lock().unwrap();
        let before = ms.len();
        ms.retain(|m| m.id != id);
        Ok(ms.len() != before)
    }

    async fn start_match(&self, id: Uuid) -> Result<Match, DatabaseError> {
        let mut ms = self.matches.lock().unwrap();
        let m = ms
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or_else(|| DatabaseError::NotFound("Match introuvable".to_string()))?;
        if m.status != MatchStatus::Scheduled {
            return Err(DatabaseError::ValidationError(
                "Seul un match programmé peut être démarré".to_string(),
            ));
        }
        m.status = MatchStatus::InProgress;
        m.started_at = Some(OffsetDateTime::now_utc());
        Ok(m.clone())
    }

    async fn complete_match(&self, id: Uuid, winner: MatchWinner) -> Result<Match, DatabaseError> {
        let mut ms = self.matches.lock().unwrap();
        let m = ms
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or_else(|| DatabaseError::NotFound("Match introuvable".to_string()))?;
        if m.status != MatchStatus::InProgress {
            return Err(DatabaseError::ValidationError(
                "Seul un match en cours peut être terminé".to_string(),
            ));
        }
        m.status = MatchStatus::Completed;
        m.winner = Some(winner);
        m.ended_at = Some(OffsetDateTime::now_utc());
        Ok(m.clone())
    }

    async fn add_manche(&self, id: Uuid, manche: MancheScore) -> Result<Match, DatabaseError> {
        let mut ms = self.matches.lock().unwrap();
        let m = ms
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or_else(|| DatabaseError::NotFound("Match introuvable".to_string()))?;
        if m.status != MatchStatus::InProgress {
            return Err(DatabaseError::ValidationError(
                "Manche impossible hors match en cours".to_string(),
            ));
        }
        let num = manche.number;
        let mut manches = m.manches.clone().unwrap_or_default();
        if let Some(existing) = manches.iter_mut().find(|x| x.number == num) {
            *existing = manche;
        } else {
            manches.push(manche);
        }
        manches.sort_by_key(|x| x.number);
        let t1: i32 = manches.iter().map(|x| x.team1).sum();
        let t2: i32 = manches.iter().map(|x| x.team2).sum();
        m.manches = Some(manches);
        m.current_manche = Some(num);
        m.current_scores = Some(palet_shared::CurrentScores {
            team1: t1,
            team2: t2,
            manche: num,
        });
        m.team1_score = Some(t1);
        m.team2_score = Some(t2);
        Ok(m.clone())
    }

    async fn update_manche(
        &self,
        id: Uuid,
        manche_number: i32,
        team1_score: i32,
        team2_score: i32,
    ) -> Result<Match, DatabaseError> {
        let mut ms = self.matches.lock().unwrap();
        let m = ms
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or_else(|| DatabaseError::NotFound("Match introuvable".to_string()))?;
        let mut manches = m.manches.clone().unwrap_or_default();
        let Some(x) = manches.iter_mut().find(|x| x.number == manche_number) else {
            return Err(DatabaseError::NotFound("Manche introuvable".to_string()));
        };
        x.team1 = team1_score;
        x.team2 = team2_score;
        let t1: i32 = manches.iter().map(|x| x.team1).sum();
        let t2: i32 = manches.iter().map(|x| x.team2).sum();
        m.manches = Some(manches);
        m.team1_score = Some(t1);
        m.team2_score = Some(t2);
        Ok(m.clone())
    }

    async fn verify_match(&self, id: Uuid, verifier_id: Uuid) -> Result<Match, DatabaseError> {
        let mut ms = self.matches.lock().unwrap();
        let m = ms
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or_else(|| DatabaseError::NotFound("Match introuvable".to_string()))?;
        if m.status != MatchStatus::Completed {
            // même comportement que le SQL (WHERE status = 'completed' → RowNotFound)
            return Err(DatabaseError::NotFound(
                "Match introuvable ou non terminé".to_string(),
            ));
        }
        m.verified = true;
        m.verified_by = Some(verifier_id);
        m.verified_at = Some(OffsetDateTime::now_utc());
        Ok(m.clone())
    }

    async fn unverify_match(&self, id: Uuid) -> Result<Match, DatabaseError> {
        let mut ms = self.matches.lock().unwrap();
        let m = ms
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or_else(|| DatabaseError::NotFound("Match introuvable".to_string()))?;
        m.verified = false;
        m.verified_by = None;
        m.verified_at = None;
        Ok(m.clone())
    }

    async fn get_matches_by_date_range(
        &self,
        start: time::OffsetDateTime,
        end: time::OffsetDateTime,
    ) -> Result<Vec<Match>, DatabaseError> {
        Ok(self
            .matches
            .lock()
            .unwrap()
            .iter()
            .filter(|m| m.date >= start && m.date <= end)
            .cloned()
            .collect())
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

    async fn get_matches_by_journee(
        &self,
        journee: i32,
        competition_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError> {
        Ok(self
            .matches
            .lock()
            .unwrap()
            .iter()
            .filter(|m| m.journee == Some(journee) && m.competition_id == competition_id)
            .cloned()
            .collect())
    }

    async fn get_championship_matches(
        &self,
        championship_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError> {
        self.get_matches_by_competition(championship_id).await
    }

    async fn get_completed_matches_by_championship(
        &self,
        championship_id: Uuid,
    ) -> Result<Vec<Match>, DatabaseError> {
        Ok(self
            .matches
            .lock()
            .unwrap()
            .iter()
            .filter(|m| m.competition_id == championship_id && m.status == MatchStatus::Completed)
            .cloned()
            .collect())
    }
}

// ============================================
// APP DE TEST
// ============================================

fn test_state(matches_repo: Arc<MockMatchRepository>) -> web::Data<AppState> {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_millis(500))
        .connect_lazy("postgres://palet:palet@localhost:5432/palet")
        .expect("pool lazy");
    let mut db = Database::new(pool);
    db.matches = matches_repo;
    web::Data::new(AppState {
        db: Arc::new(db),
        config: Arc::new(AppConfig::default()),
    })
}

macro_rules! init_app {
    ($repo:expr) => {
        test::init_service(
            App::new()
                .app_data(test_state($repo))
                .service(
                    web::scope("/matchs")
                        .route("", web::get().to(match_handler::get_matches))
                        .route("/{id}", web::get().to(match_handler::get_match))
                        .route("/{id}/start", web::post().to(match_handler::start_match))
                        .route("/{id}/manche", web::post().to(match_handler::add_manche))
                        .route(
                            "/{id}/complete",
                            web::post().to(match_handler::complete_match),
                        )
                        .route("/{id}/verify", web::post().to(match_handler::verify_match))
                        .route(
                            "/{id}/unverify",
                            web::post().to(match_handler::unverify_match),
                        ),
                )
                .service(web::scope("/matchs/status").route(
                    "/{status}",
                    web::get().to(match_handler::get_matches_by_status),
                )),
        )
        .await
    };
}

fn claims_for(role: UserRole, sub: Uuid) -> Claims {
    Claims {
        sub: sub.to_string(),
        email: "officiel@example.fr".to_string(),
        role,
        club_id: None,
        token_type: ACCESS_TOKEN_TYPE.to_string(),
        iat: OffsetDateTime::now_utc().unix_timestamp(),
        exp: (OffsetDateTime::now_utc() + time::Duration::hours(1)).unix_timestamp(),
    }
}

fn token_for(role: UserRole, sub: Uuid) -> String {
    create_token(&AppConfig::default().jwt_secret, &claims_for(role, sub)).expect("token")
}

fn auth_header(role: UserRole, sub: Uuid) -> (actix_web::http::header::HeaderName, String) {
    (
        actix_web::http::header::AUTHORIZATION,
        format!("Bearer {}", token_for(role, sub)),
    )
}

use palet_shared::UserRole;

// ============================================
// TESTS
// ============================================

#[actix_web::test]
async fn verify_requires_authentication() {
    let repo = Arc::new(MockMatchRepository::new());
    let m = sample_match(MatchStatus::Completed);
    let id = m.id;
    repo.push(m);
    let app = init_app!(repo);

    let req = test::TestRequest::post()
        .uri(&format!("/matchs/{id}/verify"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn verify_forbidden_for_player_role() {
    let repo = Arc::new(MockMatchRepository::new());
    let m = sample_match(MatchStatus::Completed);
    let id = m.id;
    repo.push(m);
    let app = init_app!(repo);

    let req = test::TestRequest::post()
        .uri(&format!("/matchs/{id}/verify"))
        .insert_header(auth_header(UserRole::Player, Uuid::new_v4()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_web::test]
async fn official_runs_full_match_flow_then_verifies() {
    let repo = Arc::new(MockMatchRepository::new());
    let m = sample_match(MatchStatus::Scheduled);
    let id = m.id;
    repo.push(m);
    let app = init_app!(repo);
    let referee = Uuid::new_v4();
    let auth = auth_header(UserRole::Arbitre, referee);

    // start
    let req = test::TestRequest::post()
        .uri(&format!("/matchs/{id}/start"))
        .insert_header(auth.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<Match> = test::read_body_json(resp).await;
    assert_eq!(body.data.status, MatchStatus::InProgress);

    // deux manches
    for (t1, t2) in [(6, 4), (7, 6)] {
        let req = test::TestRequest::post()
            .uri(&format!("/matchs/{id}/manche"))
            .insert_header(auth.clone())
            .set_json(serde_json::json!({
                "team1_score": t1,
                "team2_score": t2,
                "current_manche": if t1 == 6 { 0 } else { 1 },
            }))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
    let req = test::TestRequest::get()
        .uri(&format!("/matchs/{id}"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: ApiResponse<Match> = test::read_body_json(resp).await;
    assert_eq!(body.data.team1_score, Some(13));
    assert_eq!(body.data.team2_score, Some(10));
    assert_eq!(body.data.manches.as_ref().map(Vec::len), Some(2));

    // complete
    let req = test::TestRequest::post()
        .uri(&format!("/matchs/{id}/complete?winner=team1"))
        .insert_header(auth.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<Match> = test::read_body_json(resp).await;
    assert_eq!(body.data.status, MatchStatus::Completed);
    assert_eq!(body.data.winner, Some(MatchWinner::Team1));

    // verify : recorded avec l'id de l'arbitre
    let req = test::TestRequest::post()
        .uri(&format!("/matchs/{id}/verify"))
        .insert_header(auth.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<Match> = test::read_body_json(resp).await;
    assert!(body.data.verified);
    assert_eq!(body.data.verified_by, Some(referee));
    assert!(body.data.verified_at.is_some());

    // unverify
    let req = test::TestRequest::post()
        .uri(&format!("/matchs/{id}/unverify"))
        .insert_header(auth)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: ApiResponse<Match> = test::read_body_json(resp).await;
    assert!(!body.data.verified);
    assert_eq!(body.data.verified_by, None);
}

#[actix_web::test]
async fn verify_rejects_match_not_completed() {
    let repo = Arc::new(MockMatchRepository::new());
    let m = sample_match(MatchStatus::Scheduled);
    let id = m.id;
    repo.push(m);
    let app = init_app!(repo);

    let req = test::TestRequest::post()
        .uri(&format!("/matchs/{id}/verify"))
        .insert_header(auth_header(UserRole::CommissionAdmin, Uuid::new_v4()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn start_rejects_match_already_in_progress() {
    let repo = Arc::new(MockMatchRepository::new());
    let m = sample_match(MatchStatus::InProgress);
    let id = m.id;
    repo.push(m);
    let app = init_app!(repo);

    let req = test::TestRequest::post()
        .uri(&format!("/matchs/{id}/start"))
        .insert_header(auth_header(UserRole::Arbitre, Uuid::new_v4()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
