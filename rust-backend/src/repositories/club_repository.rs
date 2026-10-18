//! Club Repository - Database operations for clubs
//!
//! This module provides the database operations for managing clubs.

use async_trait::async_trait;
use chrono::Utc;
use palet_shared::{Club, ClubStatus, CreateClubRequest, UpdateClubRequest};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DatabaseError;

// ============================================
// CLUB REPOSITORY TRAIT (Interface)
// ============================================

/// Trait defining the interface for club repository operations
#[async_trait]
pub trait ClubRepository {
    // CRUD Operations
    async fn create_club(&self, request: CreateClubRequest) -> Result<Club, DatabaseError>;
    async fn get_club(&self, club_id: Uuid) -> Result<Option<Club>, DatabaseError>;
    async fn get_clubs(&self) -> Result<Vec<Club>, DatabaseError>;
    async fn update_club(
        &self,
        club_id: Uuid,
        request: UpdateClubRequest,
    ) -> Result<Option<Club>, DatabaseError>;
    async fn delete_club(&self, club_id: Uuid) -> Result<bool, DatabaseError>;

    // Query Operations
    async fn get_clubs_by_league(&self, league: String) -> Result<Vec<Club>, DatabaseError>;
    async fn get_clubs_by_division(
        &self,
        division: String,
    ) -> Result<Vec<Club>, DatabaseError>;
    async fn get_clubs_by_status(&self, status: ClubStatus) -> Result<Vec<Club>, DatabaseError>;
    async fn get_clubs_by_federation(
        &self,
        federation_id: Uuid,
    ) -> Result<Vec<Club>, DatabaseError>;

    // Search Operations
    async fn search_clubs(&self, query: String) -> Result<Vec<Club>, DatabaseError>;
}

// ============================================
// POSTGRES CLUB REPOSITORY (Implementation)
// ============================================

/// PostgreSQL implementation of ClubRepository
#[derive(Clone)]
pub struct PostgresClubRepository {
    pool: Arc<PgPool>,
}

impl PostgresClubRepository {
    /// Create a new PostgresClubRepository
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    // Helper function to convert database row to Club
    async fn row_to_club(row: sqlx::postgres::PgRow) -> Result<Club, sqlx::Error> {
        let id: Uuid = row.get::<_, Uuid>("id");
        let name: String = row.get::<_, String>("name");
        let nickname: Option<String> = row.get::<_, Option<String>>("nickname");
        let code: String = row.get::<_, String>("code");
        let logo: Option<String> = row.get::<_, Option<String>>("logo");
        let address: Option<String> = row.get::<_, Option<String>>("address");
        let city: String = row.get::<_, String>("city");
        let postal_code: Option<String> = row.get::<_, Option<String>>("postal_code");
        let league: String = row.get::<_, String>("league");
        let division: String = row.get::<_, String>("division");
        let status: ClubStatus = row.get::<_, ClubStatus>("status");
        let players_count: i32 = row.get::<_, i32>("players_count");
        let teams_count: i32 = row.get::<_, i32>("teams_count");
        let contact_email: Option<String> = row.get::<_, Option<String>>("contact_email");
        let contact_phone: Option<String> = row.get::<_, Option<String>>("contact_phone");
        let website: Option<String> = row.get::<_, Option<String>>("website");

        // Parse social links
        let social_json: serde_json::Value = row.get::<_, serde_json::Value>("social");
        let social: palet_shared::SocialLinks = serde_json::from_value(social_json).unwrap_or_default();

        let created_at: chrono::DateTime<Utc> = row.get::<_, chrono::DateTime<Utc>>("created_at");
        let updated_at: chrono::DateTime<Utc> = row.get::<_, chrono::DateTime<Utc>>("updated_at");

        Ok(Club {
            id,
            name,
            nickname,
            code,
            logo,
            address,
            city,
            postal_code,
            league,
            division,
            status,
            players_count,
            teams_count,
            contact_email,
            contact_phone,
            website,
            social,
            created_at,
            updated_at,
        })
    }
}

#[async_trait]
impl ClubRepository for PostgresClubRepository {
    async fn create_club(&self, request: CreateClubRequest) -> Result<Club, DatabaseError> {
        let club = Club {
            id: Uuid::new_v4(),
            name: request.name,
            nickname: request.nickname,
            code: request.code,
            logo: None,
            address: None,
            city: request.city,
            postal_code: request.postal_code,
            league: request.league,
            division: request.division,
            status: ClubStatus::Active,
            players_count: 0,
            teams_count: 0,
            contact_email: request.contact_email,
            contact_phone: request.contact_phone,
            website: request.website,
            social: palet_shared::SocialLinks::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Insert into database
        let query = r#"
            INSERT INTO clubs (
                id, name, nickname, code, logo, address, city, postal_code,
                league, division, status, players_count, teams_count,
                contact_email, contact_phone, website, social, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8,
                $9, $10, $11, $12, $13,
                $14, $15, $16, $17, $18, $19
            )
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(club.id)
            .bind(&club.name)
            .bind(club.nickname)
            .bind(&club.code)
            .bind(club.logo)
            .bind(club.address)
            .bind(&club.city)
            .bind(club.postal_code)
            .bind(&club.league)
            .bind(&club.division)
            .bind(club.status)
            .bind(club.players_count)
            .bind(club.teams_count)
            .bind(club.contact_email)
            .bind(club.contact_phone)
            .bind(club.website)
            .bind(sqlx::types::Json(club.social))
            .bind(club.created_at)
            .bind(club.updated_at)
            .fetch_one(&*self.pool)
            .await?;

        Self::row_to_club(row).await.map_err(DatabaseError::from)
    }

    async fn get_club(&self, club_id: Uuid) -> Result<Option<Club>, DatabaseError> {
        let query = "SELECT * FROM clubs WHERE id = $1";
        let row = sqlx::query(query)
            .bind(club_id)
            .fetch_optional(&*self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_club(row).await?)),
            None => Ok(None),
        }
    }

    async fn get_clubs(&self) -> Result<Vec<Club>, DatabaseError> {
        let query = "SELECT * FROM clubs ORDER BY name";
        let rows = sqlx::query(query).fetch_all(&*self.pool).await?;

        let mut clubs = Vec::new();
        for row in rows {
            clubs.push(Self::row_to_club(row).await?);
        }
        Ok(clubs)
    }

    async fn update_club(
        &self,
        club_id: Uuid,
        request: UpdateClubRequest,
    ) -> Result<Option<Club>, DatabaseError> {
        // Check if club exists
        let existing = self.get_club(club_id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        let mut club = existing.unwrap();

        // Update fields from request
        if let Some(name) = request.name {
            club.name = name;
        }
        if let Some(nickname) = request.nickname {
            club.nickname = Some(nickname);
        }
        if let Some(code) = request.code {
            club.code = code;
        }
        if let Some(city) = request.city {
            club.city = city;
        }
        if let Some(postal_code) = request.postal_code {
            club.postal_code = Some(postal_code);
        }
        if let Some(league) = request.league {
            club.league = league;
        }
        if let Some(division) = request.division {
            club.division = division;
        }
        if let Some(contact_email) = request.contact_email {
            club.contact_email = Some(contact_email);
        }
        if let Some(contact_phone) = request.contact_phone {
            club.contact_phone = Some(contact_phone);
        }
        if let Some(website) = request.website {
            club.website = Some(website);
        }

        club.updated_at = Utc::now();

        // Update in database
        let query = r#"
            UPDATE clubs SET
                name = COALESCE($1, name),
                nickname = COALESCE($2, nickname),
                code = COALESCE($3, code),
                city = COALESCE($4, city),
                postal_code = COALESCE($5, postal_code),
                league = COALESCE($6, league),
                division = COALESCE($7, division),
                contact_email = COALESCE($8, contact_email),
                contact_phone = COALESCE($9, contact_phone),
                website = COALESCE($10, website),
                updated_at = $11
            WHERE id = $12
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(club.name)
            .bind(club.nickname)
            .bind(club.code)
            .bind(club.city)
            .bind(club.postal_code)
            .bind(club.league)
            .bind(club.division)
            .bind(club.contact_email)
            .bind(club.contact_phone)
            .bind(club.website)
            .bind(club.updated_at)
            .bind(club_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(Some(Self::row_to_club(row).await?))
    }

    async fn delete_club(&self, club_id: Uuid) -> Result<bool, DatabaseError> {
        let query = "DELETE FROM clubs WHERE id = $1";
        let result = sqlx::query(query)
            .bind(club_id)
            .execute(&*self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_clubs_by_league(&self, league: String) -> Result<Vec<Club>, DatabaseError> {
        let query = "SELECT * FROM clubs WHERE league = $1 ORDER BY name";
        let rows = sqlx::query(query)
            .bind(league)
            .fetch_all(&*self.pool)
            .await?;

        let mut clubs = Vec::new();
        for row in rows {
            clubs.push(Self::row_to_club(row).await?);
        }
        Ok(clubs)
    }

    async fn get_clubs_by_division(&self, division: String) -> Result<Vec<Club>, DatabaseError> {
        let query = "SELECT * FROM clubs WHERE division = $1 ORDER BY name";
        let rows = sqlx::query(query)
            .bind(division)
            .fetch_all(&*self.pool)
            .await?;

        let mut clubs = Vec::new();
        for row in rows {
            clubs.push(Self::row_to_club(row).await?);
        }
        Ok(clubs)
    }

    async fn get_clubs_by_status(&self, status: ClubStatus) -> Result<Vec<Club>, DatabaseError> {
        let query = "SELECT * FROM clubs WHERE status = $1 ORDER BY name";
        let rows = sqlx::query(query)
            .bind(status)
            .fetch_all(&*self.pool)
            .await?;

        let mut clubs = Vec::new();
        for row in rows {
            clubs.push(Self::row_to_club(row).await?);
        }
        Ok(clubs)
    }

    async fn get_clubs_by_federation(
        &self,
        _federation_id: Uuid,
    ) -> Result<Vec<Club>, DatabaseError> {
        // For now, return all clubs (federation filtering not implemented)
        self.get_clubs().await
    }

    async fn search_clubs(&self, query: String) -> Result<Vec<Club>, DatabaseError> {
        let search_pattern = format!("%{}%", query);
        let sql_query = r#"
            SELECT * FROM clubs 
            WHERE name ILIKE $1 OR code ILIKE $1 OR city ILIKE $1
            ORDER BY name
        "#;
        let rows = sqlx::query(sql_query)
            .bind(search_pattern)
            .fetch_all(&*self.pool)
            .await?;

        let mut clubs = Vec::new();
        for row in rows {
            clubs.push(Self::row_to_club(row).await?);
        }
        Ok(clubs)
    }
}

// ============================================
// MOCK CLUB REPOSITORY (For Testing)
// ============================================

/// Mock implementation of ClubRepository for testing
#[derive(Clone, Default)]
pub struct MockClubRepository {
    pub clubs: Vec<Club>,
    pub should_fail: bool,
}

impl MockClubRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_clubs(clubs: Vec<Club>) -> Self {
        Self {
            clubs,
            should_fail: false,
        }
    }

    pub fn set_fail(&mut self, fail: bool) {
        self.should_fail = fail;
    }

    pub fn add_club(&mut self, club: Club) {
        self.clubs.push(club);
    }

    pub fn find_club_mut(&mut self, club_id: Uuid) -> Option<&mut Club> {
        self.clubs.iter_mut().find(|c| c.id == club_id)
    }
}

#[async_trait]
impl ClubRepository for MockClubRepository {
    async fn create_club(&self, request: CreateClubRequest) -> Result<Club, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        let club = Club {
            id: Uuid::new_v4(),
            name: request.name,
            nickname: request.nickname,
            code: request.code,
            logo: None,
            address: None,
            city: request.city,
            postal_code: request.postal_code,
            league: request.league,
            division: request.division,
            status: ClubStatus::Active,
            players_count: 0,
            teams_count: 0,
            contact_email: request.contact_email,
            contact_phone: request.contact_phone,
            website: request.website,
            social: palet_shared::SocialLinks::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(club)
    }

    async fn get_club(&self, club_id: Uuid) -> Result<Option<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self.clubs.iter().find(|c| c.id == club_id).cloned())
    }

    async fn get_clubs(&self) -> Result<Vec<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self.clubs.clone())
    }

    async fn update_club(
        &self,
        club_id: Uuid,
        request: UpdateClubRequest,
    ) -> Result<Option<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        if let Some(club) = self.clubs.iter_mut().find(|c| c.id == club_id) {
            if let Some(name) = request.name {
                club.name = name;
            }
            if let Some(nickname) = request.nickname {
                club.nickname = Some(nickname);
            }
            if let Some(code) = request.code {
                club.code = code;
            }
            if let Some(city) = request.city {
                club.city = city;
            }
            if let Some(postal_code) = request.postal_code {
                club.postal_code = Some(postal_code);
            }
            if let Some(league) = request.league {
                club.league = league;
            }
            if let Some(division) = request.division {
                club.division = division;
            }
            if let Some(contact_email) = request.contact_email {
                club.contact_email = Some(contact_email);
            }
            if let Some(contact_phone) = request.contact_phone {
                club.contact_phone = Some(contact_phone);
            }
            if let Some(website) = request.website {
                club.website = Some(website);
            }

            club.updated_at = Utc::now();

            Ok(Some(club.clone()))
        } else {
            Ok(None)
        }
    }

    async fn delete_club(&self, club_id: Uuid) -> Result<bool, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        let index = self.clubs.iter().position(|c| c.id == club_id);
        Ok(index.is_some())
    }

    async fn get_clubs_by_league(&self, league: String) -> Result<Vec<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .clubs
            .iter()
            .filter(|c| c.league == league)
            .cloned()
            .collect())
    }

    async fn get_clubs_by_division(&self, division: String) -> Result<Vec<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .clubs
            .iter()
            .filter(|c| c.division == division)
            .cloned()
            .collect())
    }

    async fn get_clubs_by_status(&self, status: ClubStatus) -> Result<Vec<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .clubs
            .iter()
            .filter(|c| c.status == status)
            .cloned()
            .collect())
    }

    async fn get_clubs_by_federation(
        &self,
        _federation_id: Uuid,
    ) -> Result<Vec<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self.clubs.clone())
    }

    async fn search_clubs(&self, query: String) -> Result<Vec<Club>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        let query_lower = query.to_lowercase();
        Ok(self
            .clubs
            .iter()
            .filter(|c| {
                c.name.to_lowercase().contains(&query_lower)
                    || c.code.to_lowercase().contains(&query_lower)
                    || c.city.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect())
    }
}
