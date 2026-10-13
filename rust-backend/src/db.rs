//! Database module for Palet Vendéen Backend
//!
//! Handles all database operations using SQLx with PostgreSQL

use log::info;
use palet_shared::{generate_uuid, now_utc, Club, ClubStatus, CreateClubRequest, Federation};
use sqlx::{postgres::PgPool, Row};
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

    /// Initialize database schema (migrations)
    pub async fn initialize(&self) -> Result<(), sqlx::Error> {
        info!("Initializing database schema...");

        // Create palet_federations table (avoiding conflict with pg_catalog.federations)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS palet_federations (
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

        // Create palet_clubs table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS palet_clubs (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                federation_id UUID NOT NULL REFERENCES palet_federations(id),
                nom VARCHAR(255) NOT NULL,
                slug VARCHAR(255) NOT NULL UNIQUE,
                ville VARCHAR(100),
                cree_le TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        // Create palet_joueurs table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS palet_joueurs (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                club_id UUID NOT NULL REFERENCES palet_clubs(id),
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
            FROM palet_federations
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
            FROM palet_federations
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
    pub async fn create_federation(
        &self,
        name: String,
        slug: String,
        description: Option<String>,
    ) -> Result<Federation, sqlx::Error> {
        let id = generate_uuid();
        let created_at = now_utc();

        let row = sqlx::query(
            r#"
            INSERT INTO palet_federations (id, nom, slug, description, cree_le)
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
                c.cree_le as created_at
            FROM palet_clubs c
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
                social: palet_shared::SocialLinks {
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
                c.cree_le as created_at
            FROM palet_clubs c
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
            social: palet_shared::SocialLinks {
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

        let row = sqlx::query(
            r#"
            INSERT INTO palet_clubs (id, federation_id, nom, slug, ville, cree_le)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, nom as name, slug, ville as city, cree_le as created_at
            "#,
        )
        .bind(id)
        .bind(request.federation_id)
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
            address: None,
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
            social: palet_shared::SocialLinks {
                facebook: None,
                twitter: None,
                instagram: None,
            },
            created_at: row.get("created_at"),
            updated_at: row.get("created_at"),
        })
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
        self.initialize().await?;
        Ok(())
    }
}

// ============================================
// TEST UTILITIES
// ============================================

// pub mod mock;
