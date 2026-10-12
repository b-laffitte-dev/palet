//! Database module for Palet Vendéen Backend
//!
//! Handles all database operations using SQLx with PostgreSQL

use chrono::{DateTime, Utc};
use log::{debug, error, info, warn};
use palet_shared::{
    Club, CreateClubRequest, Federation, Match, Player, Tournament, Championship,
    CompetitionCategory, CompetitionFormat, CompetitionMaterial, CompetitionStatus,
    CompetitionType, ClubStatus, DominantHand, LicenseStatus, MatchStatus,
    PlayerStatus, Team, TeamStatus, User, UserRole, UserStatus,
    ClassificationPrimary, ClassificationSecondary, ClassificationTertiary,
    DistanceUnit, EmailNotifications, MatchResult, MatchWinner, 
    NotificationsPreferences, PermissionCategory, PointsType, PushNotifications,
    SocialLinks, TeamClassificationStatus, TournamentPhaseType,
    generate_uuid, now_utc,
};
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow}, 
    FromRow, PgConnection, Row,
};
use std::str::FromStr;
use uuid::Uuid;

/// Database connection wrapper
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create a new Database instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get a connection from the pool
    pub async fn get_connection(&self) -> Result<PgConnection, sqlx::Error> {
        self.pool.acquire().await
    }

    /// Initialize database schema (migrations)
    pub async fn initialize(&self) -> Result<(), sqlx::Error> {
        info!("Initializing database schema...");

        // Create federations table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS federations (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                nom VARCHAR(255) NOT NULL,
                slug VARCHAR(255) NOT NULL UNIQUE,
                description TEXT,
                cree_le TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        // Create clubs table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS clubs (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                federation_id UUID NOT NULL REFERENCES federations(id),
                nom VARCHAR(255) NOT NULL,
                slug VARCHAR(255) NOT NULL UNIQUE,
                ville VARCHAR(100),
                cree_le TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        // Create joueurs table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS joueurs (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                club_id UUID NOT NULL REFERENCES clubs(id),
                nom VARCHAR(100) NOT NULL,
                prenom VARCHAR(100) NOT NULL,
                licence VARCHAR(50) UNIQUE,
                cree_le TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        info!("Database schema initialized successfully");
        Ok(())
    }

    // ============================================
    // FEDERATION OPERATIONS
    // ============================================

    /// Get all federations
    pub async fn get_federations(&self) -> Result<Vec<Federation>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, nom as name, slug, description, cree_le as created_at
            FROM federations
            ORDER BY nom
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Federation {
                id: row.get("id"),
                name: row.get("name"),
                slug: row.get("slug"),
                description: row.get("description"),
                created_at: row.get("created_at"),
            })
            .collect())
    }

    /// Get a federation by ID
    pub async fn get_federation(&self, id: Uuid) -> Result<Option<Federation>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, nom as name, slug, description, cree_le as created_at
            FROM federations
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| Federation {
            id: row.get("id"),
            name: row.get("name"),
            slug: row.get("slug"),
            description: row.get("description"),
            created_at: row.get("created_at"),
        }))
    }

    /// Create a new federation
    pub async fn create_federation(&self, name: String, slug: String, description: Option<String>) -> Result<Federation, sqlx::Error> {
        let id = generate_uuid();
        let created_at = now_utc();

        let row = sqlx::query(
            r#"
            INSERT INTO federations (id, nom, slug, description, cree_le)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, nom as name, slug, description, cree_le as created_at
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(slug)
        .bind(description)
        .bind(created_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(Federation {
            id: row.get("id"),
            name: row.get("name"),
            slug: row.get("slug"),
            description: row.get("description"),
            created_at: row.get("created_at"),
        })
    }

    // ============================================
    // CLUB OPERATIONS
    // ============================================

    /// Get all clubs
    pub async fn get_clubs(&self) -> Result<Vec<Club>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT 
                c.id,
                c.nom as name,
                c.slug,
                c.ville as city,
                c.cree_le as created_at,
                f.id as federation_id,
                f.nom as federation_name,
                f.slug as federation_slug
            FROM clubs c
            JOIN federations f ON c.federation_id = f.id
            ORDER BY c.nom
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Club {
                id: row.get("id"),
                name: row.get("name"),
                nickname: None,
                code: row.get("slug"),
                logo: None,
                address: None,
                city: row.get("city"),
                postal_code: None,
                league: "Deux-Sèvres".to_string(),
                division: "D1".to_string(),
                status: ClubStatus::Active,
                players_count: 0,
                teams_count: 0,
                contact_email: None,
                contact_phone: None,
                website: None,
                social: SocialLinks {
                    facebook: None,
                    twitter: None,
                    instagram: None,
                },
                created_at: row.get("created_at"),
                updated_at: row.get("created_at"),
            })
            .collect())
    }

    /// Get a club by ID
    pub async fn get_club(&self, id: Uuid) -> Result<Option<Club>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT 
                c.id,
                c.nom as name,
                c.slug,
                c.ville as city,
                c.cree_le as created_at,
                f.id as federation_id,
                f.nom as federation_name
            FROM clubs c
            JOIN federations f ON c.federation_id = f.id
            WHERE c.id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| Club {
            id: row.get("id"),
            name: row.get("name"),
            nickname: None,
            code: row.get("slug"),
            logo: None,
            address: None,
            city: row.get("city"),
            postal_code: None,
            league: "Deux-Sèvres".to_string(),
            division: "D1".to_string(),
            status: ClubStatus::Active,
            players_count: 0,
            teams_count: 0,
            contact_email: None,
            contact_phone: None,
            website: None,
            social: SocialLinks {
                facebook: None,
                twitter: None,
                instagram: None,
            },
            created_at: row.get("created_at"),
            updated_at: row.get("created_at"),
        }))
    }

    /// Create a new club
    pub async fn create_club(&self, request: CreateClubRequest) -> Result<Club, sqlx::Error> {
        let id = generate_uuid();
        let created_at = now_utc();

        // Get federation ID from slug or use default
        let federation_id = self
            .get_federation_by_slug(&request.league)
            .await?
            .map(|f| f.id)
            .unwrap_or_else(|| {
                // Create default federation if not exists
                Uuid::nil()
            });

        let row = sqlx::query(
            r#"
            INSERT INTO clubs (id, federation_id, nom, slug, ville, cree_le)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, nom as name, slug, ville as city, cree_le as created_at
            "#,
        )
        .bind(id)
        .bind(federation_id)
        .bind(request.name)
        .bind(request.code)
        .bind(request.city)
        .bind(created_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(Club {
            id: row.get("id"),
            name: row.get("name"),
            nickname: request.nickname,
            code: row.get("slug"),
            logo: None,
            address: request.address,
            city: row.get("city"),
            postal_code: request.postal_code,
            league: request.league,
            division: request.division,
            status: ClubStatus::Active,
            players_count: 0,
            teams_count: 0,
            contact_email: request.contact_email,
            contact_phone: request.contact_phone,
            website: request.website,
            social: SocialLinks {
                facebook: None,
                twitter: None,
                instagram: None,
            },
            created_at: row.get("created_at"),
            updated_at: row.get("created_at"),
        })
    }

    /// Get federation by slug
    pub async fn get_federation_by_slug(&self, slug: &str) -> Result<Option<Federation>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, nom as name, slug, description, cree_le as created_at
            FROM federations
            WHERE slug = $1
            "#,
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| Federation {
            id: row.get("id"),
            name: row.get("name"),
            slug: row.get("slug"),
            description: row.get("description"),
            created_at: row.get("created_at"),
        }))
    }

    // ============================================
    // PLAYER OPERATIONS
    // ============================================

    /// Get all players
    pub async fn get_players(&self) -> Result<Vec<Player>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT 
                j.id,
                j.nom as last_name,
                j.prenom as first_name,
                j.licence,
                j.cree_le as created_at,
                c.id as club_id,
                c.nom as club_name
            FROM joueurs j
            JOIN clubs c ON j.club_id = c.id
            ORDER BY j.nom, j.prenom
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Player {
                id: row.get("id"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                full_name: format!("{} {}", row.get::<String, _>("first_name"), row.get::<String, _>("last_name")),
                alias: None,
                email: "".to_string(),
                photo: None,
                status: PlayerStatus::Active,
                birth_date: now_utc(),
                age: 0,
                birth_place: None,
                nationality: "FR".to_string(),
                dominant_hand: DominantHand::Right,
                play_style: None,
                phone: None,
                address: None,
                city: None,
                postal_code: None,
                club_id: row.get("club_id"),
                club_number: None,
                category: "Senior".to_string(),
                license_number: row.get("licence"),
                license_date: now_utc(),
                license_status: LicenseStatus::Valid,
                registration_date: now_utc(),
                created_at: row.get("created_at"),
                updated_at: row.get("created_at"),
            })
            .collect())
    }

    /// Get a player by ID
    pub async fn get_player(&self, id: Uuid) -> Result<Option<Player>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT 
                j.id,
                j.nom as last_name,
                j.prenom as first_name,
                j.licence,
                j.cree_le as created_at,
                c.id as club_id,
                c.nom as club_name
            FROM joueurs j
            JOIN clubs c ON j.club_id = c.id
            WHERE j.id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| Player {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            full_name: format!("{} {}", row.get::<String, _>("first_name"), row.get::<String, _>("last_name")),
            alias: None,
            email: "".to_string(),
            photo: None,
            status: PlayerStatus::Active,
            birth_date: now_utc(),
            age: 0,
            birth_place: None,
            nationality: "FR".to_string(),
            dominant_hand: DominantHand::Right,
            play_style: None,
            phone: None,
            address: None,
            city: None,
            postal_code: None,
            club_id: row.get("club_id"),
            club_number: None,
            category: "Senior".to_string(),
            license_number: row.get("licence"),
            license_date: now_utc(),
            license_status: LicenseStatus::Valid,
            registration_date: now_utc(),
            created_at: row.get("created_at"),
            updated_at: row.get("created_at"),
        }))
    }

    /// Get players by club
    pub async fn get_players_by_club(&self, club_id: Uuid) -> Result<Vec<Player>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT 
                j.id,
                j.nom as last_name,
                j.prenom as first_name,
                j.licence,
                j.cree_le as created_at
            FROM joueurs j
            WHERE j.club_id = $1
            ORDER BY j.nom, j.prenom
            "#,
        )
        .bind(club_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Player {
                id: row.get("id"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                full_name: format!("{} {}", row.get::<String, _>("first_name"), row.get::<String, _>("last_name")),
                alias: None,
                email: "".to_string(),
                photo: None,
                status: PlayerStatus::Active,
                birth_date: now_utc(),
                age: 0,
                birth_place: None,
                nationality: "FR".to_string(),
                dominant_hand: DominantHand::Right,
                play_style: None,
                phone: None,
                address: None,
                city: None,
                postal_code: None,
                club_id,
                club_number: None,
                category: "Senior".to_string(),
                license_number: row.get("licence"),
                license_date: now_utc(),
                license_status: LicenseStatus::Valid,
                registration_date: now_utc(),
                created_at: row.get("created_at"),
                updated_at: row.get("created_at"),
            })
            .collect())
    }

    // ============================================
    // TOURNAMENT OPERATIONS
    // ============================================

    /// Get all tournaments
    pub async fn get_tournaments(&self) -> Result<Vec<Tournament>, sqlx::Error> {
        // For now, return empty vector as tournaments aren't in the original schema
        Ok(Vec::new())
    }

    // ============================================
    // MATCH OPERATIONS
    // ============================================

    /// Get all matches
    pub async fn get_matches(&self) -> Result<Vec<Match>, sqlx::Error> {
        // For now, return empty vector as matches aren't in the original schema
        Ok(Vec::new())
    }

    // ============================================
    // CHAMPIONSHIP OPERATIONS
    // ============================================

    /// Get all championships
    pub async fn get_championships(&self) -> Result<Vec<Championship>, sqlx::Error> {
        // For now, return empty vector as championships aren't in the original schema
        Ok(Vec::new())
    }

    // ============================================
    // UTILITY FUNCTIONS
    // ============================================

    /// Check database connection
    pub async fn check_connection(&self) -> bool {
        sqlx::query("SELECT 1")
            .fetch_optional(&self.pool)
            .await
            .is_ok()
    }

    /// Run migrations (placeholder - actual migrations would use a migration tool)
    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        info!("Running database migrations...");
        // In a real implementation, this would run SQL migrations
        // For now, just ensure tables exist
        self.initialize().await?;
        Ok(())
    }
}

// ============================================
// CONVERSION TRAITS
// ============================================

/// Convert from SQLx row to Club
impl FromRow<'_, PgRow> for Club {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Club {
            id: row.get("id"),
            name: row.get("name"),
            nickname: row.get("nickname"),
            code: row.get("code"),
            logo: row.get("logo"),
            address: row.get("address"),
            city: row.get("city"),
            postal_code: row.get("postal_code"),
            league: row.get("league"),
            division: row.get("division"),
            status: row.get("status"),
            players_count: row.get("players_count"),
            teams_count: row.get("teams_count"),
            contact_email: row.get("contact_email"),
            contact_phone: row.get("contact_phone"),
            website: row.get("website"),
            social: row.get("social"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

/// Convert from SQLx row to Player
impl FromRow<'_, PgRow> for Player {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Player {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            full_name: row.get("full_name"),
            alias: row.get("alias"),
            email: row.get("email"),
            photo: row.get("photo"),
            status: row.get("status"),
            birth_date: row.get("birth_date"),
            age: row.get("age"),
            birth_place: row.get("birth_place"),
            nationality: row.get("nationality"),
            dominant_hand: row.get("dominant_hand"),
            play_style: row.get("play_style"),
            phone: row.get("phone"),
            address: row.get("address"),
            city: row.get("city"),
            postal_code: row.get("postal_code"),
            club_id: row.get("club_id"),
            club_number: row.get("club_number"),
            category: row.get("category"),
            license_number: row.get("license_number"),
            license_date: row.get("license_date"),
            license_status: row.get("license_status"),
            registration_date: row.get("registration_date"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

// ============================================
// TESTS
// ============================================

#[cfg(test)]
pub mod tests {
    use super::*;
    use sqlx::PgPool;

    pub async fn create_test_db() -> Database {
        let db_url = "postgres://palet:palet@localhost:5432/palet_test";
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
            .expect("Failed to connect to test database");

        Database::new(pool)
    }

    #[tokio::test]
    async fn test_database_initialization() {
        let db = create_test_db().await;
        let result = db.initialize().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_federation_operations() {
        let db = create_test_db().await;
        db.initialize().await.expect("Failed to initialize");

        // Create federation
        let fed = db.create_federation(
            "Fédération Vendéenne".to_string(),
            "fv".to_string(),
            Some("Fédération de Palet Vendéen".to_string()),
        )
        .await
        .expect("Failed to create federation");

        assert_eq!(fed.name, "Fédération Vendéenne");
        assert_eq!(fed.slug, "fv");

        // Get federation
        let retrieved = db.get_federation(fed.id)
            .await
            .expect("Failed to get federation")
            .expect("Federation not found");

        assert_eq!(retrieved.id, fed.id);
    }
}
