//! Player Repository - Database operations for players
//!
//! This module provides the database operations for managing players.

use async_trait::async_trait;
use chrono::Utc;
use palet_shared::{CreatePlayerRequest, DominantHand, LicenseStatus, Player, PlayerStatus};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DatabaseError;

// ============================================
// PLAYER REPOSITORY TRAIT (Interface)
// ============================================

/// Trait defining the interface for player repository operations
#[async_trait]
pub trait PlayerRepository {
    // CRUD Operations
    async fn create_player(&self, request: CreatePlayerRequest) -> Result<Player, DatabaseError>;
    async fn get_player(&self, player_id: Uuid) -> Result<Option<Player>, DatabaseError>;
    async fn get_players(&self) -> Result<Vec<Player>, DatabaseError>;
    async fn update_player(&self, player_id: Uuid, request: CreatePlayerRequest) -> Result<Option<Player>, DatabaseError>;
    async fn delete_player(&self, player_id: Uuid) -> Result<bool, DatabaseError>;

    // Query Operations
    async fn get_players_by_club(&self, club_id: Uuid) -> Result<Vec<Player>, DatabaseError>;
    async fn get_players_by_category(&self, category: String) -> Result<Vec<Player>, DatabaseError>;
    async fn get_players_by_status(&self, status: PlayerStatus) -> Result<Vec<Player>, DatabaseError>;
    async fn get_players_by_license_status(
        &self,
        status: LicenseStatus,
    ) -> Result<Vec<Player>, DatabaseError>;

    // Search Operations
    async fn search_players(&self, query: String) -> Result<Vec<Player>, DatabaseError>;
}

// ============================================
// POSTGRES PLAYER REPOSITORY (Implementation)
// ============================================

/// PostgreSQL implementation of PlayerRepository
#[derive(Clone)]
pub struct PostgresPlayerRepository {
    pool: Arc<PgPool>,
}

impl PostgresPlayerRepository {
    /// Create a new PostgresPlayerRepository
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    // Helper function to convert database row to Player
    async fn row_to_player(row: sqlx::postgres::PgRow) -> Result<Player, sqlx::Error> {
        let id: Uuid = row.get::<_, Uuid>("id");
        let first_name: String = row.get::<_, String>("first_name");
        let last_name: String = row.get::<_, String>("last_name");
        let full_name = format!("{} {}", first_name, last_name);
        let alias: Option<String> = row.get::<_, Option<String>>("alias");
        let email: String = row.get::<_, String>("email");
        let photo: Option<String> = row.get::<_, Option<String>>("photo");
        let status: PlayerStatus = row.get::<_, PlayerStatus>("status");
        let birth_date: chrono::DateTime<Utc> = row.get::<_, chrono::DateTime<Utc>>("birth_date");
        let age: i32 = row.get::<_, i32>("age");
        let birth_place: Option<String> = row.get::<_, Option<String>>("birth_place");
        let nationality: String = row.get::<_, String>("nationality");
        let dominant_hand: DominantHand = row.get::<_, DominantHand>("dominant_hand");
        let play_style: Option<String> = row.get::<_, Option<String>>("play_style");
        let phone: Option<String> = row.get::<_, Option<String>>("phone");
        let address: Option<String> = row.get::<_, Option<String>>("address");
        let city: Option<String> = row.get::<_, String>("city");
        let postal_code: Option<String> = row.get::<_, Option<String>>("postal_code");
        let club_id: Uuid = row.get::<_, Uuid>("club_id");
        let club_number: Option<String> = row.get::<_, Option<String>>("club_number");
        let category: String = row.get::<_, String>("category");
        let license_number: String = row.get::<_, String>("license_number");
        let license_date: chrono::DateTime<Utc> = row.get::<_, chrono::DateTime<Utc>>("license_date");
        let license_status: LicenseStatus = row.get::<_, LicenseStatus>("license_status");
        let registration_date: chrono::DateTime<Utc> = row.get::<_, chrono::DateTime<Utc>>("registration_date");
        let created_at: chrono::DateTime<Utc> = row.get::<_, chrono::DateTime<Utc>>("created_at");
        let updated_at: chrono::DateTime<Utc> = row.get::<_, chrono::DateTime<Utc>>("updated_at");

        Ok(Player {
            id,
            first_name,
            last_name,
            full_name,
            alias,
            email,
            photo,
            status,
            birth_date,
            age,
            birth_place,
            nationality,
            dominant_hand,
            play_style,
            phone,
            address,
            city,
            postal_code,
            club_id,
            club_number,
            category,
            license_number,
            license_date,
            license_status,
            registration_date,
            created_at,
            updated_at,
        })
    }
}

#[async_trait]
impl PlayerRepository for PostgresPlayerRepository {
    async fn create_player(&self, request: CreatePlayerRequest) -> Result<Player, DatabaseError> {
        let age = Utc::now().signed_duration_since(request.birth_date).num_days() / 365;

        let player = Player {
            id: Uuid::new_v4(),
            first_name: request.first_name,
            last_name: request.last_name,
            full_name: format!("{} {}", request.first_name, request.last_name),
            alias: None,
            email: request.email,
            photo: None,
            status: PlayerStatus::Active,
            birth_date: request.birth_date,
            age,
            birth_place: None,
            nationality: request.nationality,
            dominant_hand: request.dominant_hand,
            play_style: None,
            phone: None,
            address: None,
            city: None,
            postal_code: None,
            club_id: request.club_id,
            club_number: None,
            category: request.category,
            license_number: request.license_number.unwrap_or_default(),
            license_date: request.license_date.unwrap_or(Utc::now()),
            license_status: LicenseStatus::Valid,
            registration_date: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Insert into database
        let query = r#"
            INSERT INTO players (
                id, first_name, last_name, full_name, alias, email, photo, status,
                birth_date, age, birth_place, nationality, dominant_hand, play_style,
                phone, address, city, postal_code, club_id, club_number, category,
                license_number, license_date, license_status, registration_date,
                created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8,
                $9, $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20, $21,
                $22, $23, $24, $25, $26, $27
            )
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(player.id)
            .bind(&player.first_name)
            .bind(&player.last_name)
            .bind(&player.full_name)
            .bind(player.alias)
            .bind(&player.email)
            .bind(player.photo)
            .bind(player.status)
            .bind(player.birth_date)
            .bind(player.age)
            .bind(player.birth_place)
            .bind(&player.nationality)
            .bind(player.dominant_hand)
            .bind(player.play_style)
            .bind(player.phone)
            .bind(player.address)
            .bind(player.city)
            .bind(player.postal_code)
            .bind(player.club_id)
            .bind(player.club_number)
            .bind(&player.category)
            .bind(&player.license_number)
            .bind(player.license_date)
            .bind(player.license_status)
            .bind(player.registration_date)
            .bind(player.created_at)
            .bind(player.updated_at)
            .fetch_one(&*self.pool)
            .await?;

        Self::row_to_player(row).await.map_err(DatabaseError::from)
    }

    async fn get_player(&self, player_id: Uuid) -> Result<Option<Player>, DatabaseError> {
        let query = "SELECT * FROM players WHERE id = $1";
        let row = sqlx::query(query)
            .bind(player_id)
            .fetch_optional(&*self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_player(row).await?)),
            None => Ok(None),
        }
    }

    async fn get_players(&self) -> Result<Vec<Player>, DatabaseError> {
        let query = "SELECT * FROM players ORDER BY last_name, first_name";
        let rows = sqlx::query(query).fetch_all(&*self.pool).await?;

        let mut players = Vec::new();
        for row in rows {
            players.push(Self::row_to_player(row).await?);
        }
        Ok(players)
    }

    async fn update_player(
        &self,
        player_id: Uuid,
        request: CreatePlayerRequest,
    ) -> Result<Option<Player>, DatabaseError> {
        // Check if player exists
        let existing = self.get_player(player_id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        let mut player = existing.unwrap();

        // Update fields from request
        player.first_name = request.first_name;
        player.last_name = request.last_name;
        player.full_name = format!("{} {}", request.first_name, request.last_name);
        player.email = request.email;
        player.nationality = request.nationality;
        player.dominant_hand = request.dominant_hand;
        player.category = request.category;
        player.club_id = request.club_id;

        if let Some(license_number) = request.license_number {
            player.license_number = license_number;
        }
        if let Some(license_date) = request.license_date {
            player.license_date = license_date;
        }

        player.updated_at = Utc::now();

        // Update in database
        let query = r#"
            UPDATE players SET
                first_name = $1,
                last_name = $2,
                full_name = $3,
                email = $4,
                nationality = $5,
                dominant_hand = $6,
                category = $7,
                club_id = $8,
                license_number = COALESCE($9, license_number),
                license_date = COALESCE($10, license_date),
                updated_at = $11
            WHERE id = $12
            RETURNING *
        "#;

        let row = sqlx::query(query)
            .bind(&player.first_name)
            .bind(&player.last_name)
            .bind(&player.full_name)
            .bind(&player.email)
            .bind(&player.nationality)
            .bind(player.dominant_hand)
            .bind(&player.category)
            .bind(player.club_id)
            .bind(player.license_number)
            .bind(player.license_date)
            .bind(player.updated_at)
            .bind(player_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(Some(Self::row_to_player(row).await?))
    }

    async fn delete_player(&self, player_id: Uuid) -> Result<bool, DatabaseError> {
        let query = "DELETE FROM players WHERE id = $1";
        let result = sqlx::query(query)
            .bind(player_id)
            .execute(&*self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_players_by_club(&self, club_id: Uuid) -> Result<Vec<Player>, DatabaseError> {
        let query = "SELECT * FROM players WHERE club_id = $1 ORDER BY last_name, first_name";
        let rows = sqlx::query(query)
            .bind(club_id)
            .fetch_all(&*self.pool)
            .await?;

        let mut players = Vec::new();
        for row in rows {
            players.push(Self::row_to_player(row).await?);
        }
        Ok(players)
    }

    async fn get_players_by_category(&self, category: String) -> Result<Vec<Player>, DatabaseError> {
        let query = "SELECT * FROM players WHERE category = $1 ORDER BY last_name, first_name";
        let rows = sqlx::query(query)
            .bind(category)
            .fetch_all(&*self.pool)
            .await?;

        let mut players = Vec::new();
        for row in rows {
            players.push(Self::row_to_player(row).await?);
        }
        Ok(players)
    }

    async fn get_players_by_status(&self, status: PlayerStatus) -> Result<Vec<Player>, DatabaseError> {
        let query = "SELECT * FROM players WHERE status = $1 ORDER BY last_name, first_name";
        let rows = sqlx::query(query)
            .bind(status)
            .fetch_all(&*self.pool)
            .await?;

        let mut players = Vec::new();
        for row in rows {
            players.push(Self::row_to_player(row).await?);
        }
        Ok(players)
    }

    async fn get_players_by_license_status(
        &self,
        status: LicenseStatus,
    ) -> Result<Vec<Player>, DatabaseError> {
        let query = "SELECT * FROM players WHERE license_status = $1 ORDER BY last_name, first_name";
        let rows = sqlx::query(query)
            .bind(status)
            .fetch_all(&*self.pool)
            .await?;

        let mut players = Vec::new();
        for row in rows {
            players.push(Self::row_to_player(row).await?);
        }
        Ok(players)
    }

    async fn search_players(&self, query: String) -> Result<Vec<Player>, DatabaseError> {
        let search_pattern = format!("%{}%", query);
        let sql_query = r#"
            SELECT * FROM players 
            WHERE first_name ILIKE $1 OR last_name ILIKE $1 OR full_name ILIKE $1 OR email ILIKE $1
            ORDER BY last_name, first_name
        "#;
        let rows = sqlx::query(sql_query)
            .bind(search_pattern)
            .fetch_all(&*self.pool)
            .await?;

        let mut players = Vec::new();
        for row in rows {
            players.push(Self::row_to_player(row).await?);
        }
        Ok(players)
    }
}

// ============================================
// MOCK PLAYER REPOSITORY (For Testing)
// ============================================

/// Mock implementation of PlayerRepository for testing
#[derive(Clone, Default)]
pub struct MockPlayerRepository {
    pub players: Vec<Player>,
    pub should_fail: bool,
}

impl MockPlayerRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_players(players: Vec<Player>) -> Self {
        Self {
            players,
            should_fail: false,
        }
    }

    pub fn set_fail(&mut self, fail: bool) {
        self.should_fail = fail;
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn find_player_mut(&mut self, player_id: Uuid) -> Option<&mut Player> {
        self.players.iter_mut().find(|p| p.id == player_id)
    }
}

#[async_trait]
impl PlayerRepository for MockPlayerRepository {
    async fn create_player(&self, request: CreatePlayerRequest) -> Result<Player, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        let age = Utc::now().signed_duration_since(request.birth_date).num_days() / 365;

        let player = Player {
            id: Uuid::new_v4(),
            first_name: request.first_name,
            last_name: request.last_name,
            full_name: format!("{} {}", request.first_name, request.last_name),
            alias: None,
            email: request.email,
            photo: None,
            status: PlayerStatus::Active,
            birth_date: request.birth_date,
            age,
            birth_place: None,
            nationality: request.nationality,
            dominant_hand: request.dominant_hand,
            play_style: None,
            phone: None,
            address: None,
            city: None,
            postal_code: None,
            club_id: request.club_id,
            club_number: None,
            category: request.category,
            license_number: request.license_number.unwrap_or_default(),
            license_date: request.license_date.unwrap_or(Utc::now()),
            license_status: LicenseStatus::Valid,
            registration_date: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(player)
    }

    async fn get_player(&self, player_id: Uuid) -> Result<Option<Player>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self.players.iter().find(|p| p.id == player_id).cloned())
    }

    async fn get_players(&self) -> Result<Vec<Player>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self.players.clone())
    }

    async fn update_player(
        &self,
        player_id: Uuid,
        request: CreatePlayerRequest,
    ) -> Result<Option<Player>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        if let Some(player) = self.players.iter_mut().find(|p| p.id == player_id) {
            player.first_name = request.first_name;
            player.last_name = request.last_name;
            player.full_name = format!("{} {}", request.first_name, request.last_name);
            player.email = request.email;
            player.nationality = request.nationality;
            player.dominant_hand = request.dominant_hand;
            player.category = request.category;
            player.club_id = request.club_id;

            if let Some(license_number) = request.license_number {
                player.license_number = license_number;
            }
            if let Some(license_date) = request.license_date {
                player.license_date = license_date;
            }

            player.updated_at = Utc::now();

            Ok(Some(player.clone()))
        } else {
            Ok(None)
        }
    }

    async fn delete_player(&self, player_id: Uuid) -> Result<bool, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        let index = self.players.iter().position(|p| p.id == player_id);
        Ok(index.is_some())
    }

    async fn get_players_by_club(&self, club_id: Uuid) -> Result<Vec<Player>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .players
            .iter()
            .filter(|p| p.club_id == club_id)
            .cloned()
            .collect())
    }

    async fn get_players_by_category(&self, category: String) -> Result<Vec<Player>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .players
            .iter()
            .filter(|p| p.category == category)
            .cloned()
            .collect())
    }

    async fn get_players_by_status(&self, status: PlayerStatus) -> Result<Vec<Player>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .players
            .iter()
            .filter(|p| p.status == status)
            .cloned()
            .collect())
    }

    async fn get_players_by_license_status(
        &self,
        status: LicenseStatus,
    ) -> Result<Vec<Player>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        Ok(self
            .players
            .iter()
            .filter(|p| p.license_status == status)
            .cloned()
            .collect())
    }

    async fn search_players(&self, query: String) -> Result<Vec<Player>, DatabaseError> {
        if self.should_fail {
            return Err(DatabaseError::ConnectionError(
                "Mock connection failed".to_string(),
            ));
        }

        let query_lower = query.to_lowercase();
        Ok(self
            .players
            .iter()
            .filter(|p| {
                p.first_name.to_lowercase().contains(&query_lower)
                    || p.last_name.to_lowercase().contains(&query_lower)
                    || p.full_name.to_lowercase().contains(&query_lower)
                    || p.email.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect())
    }
}
