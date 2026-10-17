//! Database module for Palet Vendéen Backend
//!
//! Handles all database operations using SQLx with PostgreSQL

use log::info;
use palet_shared::{
    generate_uuid, now_utc, Championship, ChampionshipClassification, ChampionshipTeam,
    ClassificationPrimary, ClassificationRules, ClassificationSecondary, ClassificationTertiary,
    Club, ClubStatus, CompetitionCategory, CompetitionFormat, CompetitionMaterial,
    CompetitionStatus, CompetitionType, CreateChampionshipRequest, CreateClubRequest,
    CreateMatchRequest, Federation, MancheScore, Match, MatchStatus, MatchWinner, PointsType,
    TeamClassificationStatus, Tournament, TournamentMatch, TournamentPhase, TournamentPhaseType,
    UpdateChampionshipRequest, UpdateMatchRequest,
};
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

        // Create palet_equipes table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS palet_equipes (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                nom VARCHAR(255) NOT NULL,
                club_id UUID NOT NULL REFERENCES palet_clubs(id),
                capitaine_id UUID REFERENCES palet_joueurs(id),
                created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        // Create palet_championnats table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS palet_championnats (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                nom VARCHAR(255) NOT NULL,
                code VARCHAR(100),
                saison VARCHAR(100) NOT NULL,
                description TEXT,
                ligue VARCHAR(100) NOT NULL,
                divisions TEXT[],
                total_journees INTEGER NOT NULL,
                duree_journee VARCHAR(100),
                regles_classement JSONB NOT NULL,
                statut VARCHAR(50) NOT NULL DEFAULT 'pending',
                journee_actuelle INTEGER,
                phase_actuelle VARCHAR(100),
                organisateur_id UUID NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        // Create palet_championnat_classifications table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS palet_championnat_classifications (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                championnat_id UUID NOT NULL REFERENCES palet_championnats(id),
                division VARCHAR(100) NOT NULL,
                journee INTEGER NOT NULL,
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
                classements JSONB NOT NULL
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        // Create palet_matchs table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS palet_matchs (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                competition_id UUID NOT NULL,
                competition_type VARCHAR(50) NOT NULL,
                journee INTEGER,
                phase VARCHAR(100),
                tour VARCHAR(100),
                equipe1_id UUID NOT NULL,
                equipe2_id UUID NOT NULL,
                score_equipe1 INTEGER,
                score_equipe2 INTEGER,
                vainqueur VARCHAR(20),
                manches JSONB,
                meilleur_sur INTEGER DEFAULT 13,
                manche_actuelle INTEGER DEFAULT 0,
                scores_actuels JSONB,
                statut VARCHAR(50) NOT NULL DEFAULT 'scheduled',
                verifie BOOLEAN DEFAULT false,
                verifie_par UUID,
                verifie_le TIMESTAMP WITH TIME ZONE,
                date DATE NOT NULL,
                heure VARCHAR(20),
                lieu VARCHAR(255),
                duree VARCHAR(100),
                commence_a TIMESTAMP WITH TIME ZONE,
                termine_a TIMESTAMP WITH TIME ZONE,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
            )
        "#,
        )
        .execute(&self.pool)
        .await?;

        // Create palet_tournois table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS palet_tournois (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                nom VARCHAR(255) NOT NULL,
                code VARCHAR(100),
                type_competition VARCHAR(50) NOT NULL,
                categorie VARCHAR(50) NOT NULL,
                materiel VARCHAR(50) NOT NULL,
                saison VARCHAR(100) NOT NULL,
                description TEXT,
                lieu VARCHAR(255),
                divisions TEXT[],
                format VARCHAR(50) NOT NULL,
                date_inscription_debut TIMESTAMP WITH TIME ZONE,
                date_inscription_fin TIMESTAMP WITH TIME ZONE,
                date_debut TIMESTAMP WITH TIME ZONE,
                date_fin TIMESTAMP WITH TIME ZONE,
                statut VARCHAR(50) NOT NULL DEFAULT 'pending',
                phase_actuelle VARCHAR(100),
                frais DOUBLE PRECISION,
                paiement_requis BOOLEAN DEFAULT false,
                methodes_paiement TEXT[],
                regles TEXT,
                reglement JSONB,
                max_equipes INTEGER,
                equipes_inscrites INTEGER DEFAULT 0,
                points_type VARCHAR(20) DEFAULT 'P13',
                points_victoire INTEGER DEFAULT 3,
                points_nul INTEGER DEFAULT 1,
                points_defaite INTEGER DEFAULT 0,
                inscription_automatique BOOLEAN DEFAULT false,
                validation_manuelle BOOLEAN DEFAULT false,
                est_public BOOLEAN DEFAULT true,
                est_archive BOOLEAN DEFAULT false,
                organisateur_id UUID NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
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
    // MATCH OPERATIONS
    // ============================================

    /// Get all matches
    pub async fn get_matches(&self) -> Result<Vec<Match>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, competition_id, competition_type as "competition_type: CompetitionType",
                journee, phase, tour as round,
                equipe1_id as "team1_id: Uuid", equipe2_id as "team2_id: Uuid",
                score_equipe1 as team1_score, score_equipe2 as team2_score,
                vainqueur as "winner: MatchWinner",
                manches, meilleur_sur as best_of, manche_actuelle as current_manche,
                scores_actuels as "current_scores: palet_shared::CurrentScores",
                statut as "status: MatchStatus",
                verifie as verified, verifie_par as verified_by, verifie_le as verified_at,
                date, heure as time, lieu as location, duree as duration,
                commence_a as started_at, termine_a as ended_at,
                created_at, updated_at
            FROM palet_matchs
            ORDER BY date DESC, heure
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Match {
                id: row.get("id"),
                competition_id: row.get("competition_id"),
                competition_type: row.get("competition_type"),
                journee: row.get("journee"),
                phase: row.get("phase"),
                round: row.get("round"),
                team1_id: row.get("team1_id"),
                team2_id: row.get("team2_id"),
                team1_score: row.get("team1_score"),
                team2_score: row.get("team2_score"),
                winner: row.get("winner"),
                manches: row.get("manches"),
                best_of: row.get("best_of"),
                current_manche: row.get("current_manche"),
                current_scores: row.get("current_scores"),
                status: row.get("status"),
                verified: row.get("verified"),
                verified_by: row.get("verified_by"),
                verified_at: row.get("verified_at"),
                date: row.get("date"),
                time: row.get("time"),
                location: row.get("location"),
                duration: row.get("duration"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                started_at: row.get("started_at"),
                ended_at: row.get("ended_at"),
            })
            .collect())
    }

    /// Get matches by competition
    pub async fn get_matches_by_competition(
        &self,
        competition_id: Uuid,
    ) -> Result<Vec<Match>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, competition_id, competition_type as "competition_type: CompetitionType",
                journee, phase, tour as round,
                equipe1_id as "team1_id: Uuid", equipe2_id as "team2_id: Uuid",
                score_equipe1 as team1_score, score_equipe2 as team2_score,
                vainqueur as "winner: MatchWinner",
                manches, meilleur_sur as best_of, manche_actuelle as current_manche,
                scores_actuels as "current_scores: palet_shared::CurrentScores",
                statut as "status: MatchStatus",
                verifie as verified, verifie_par as verified_by, verifie_le as verified_at,
                date, heure as time, lieu as location, duree as duration,
                commence_a as started_at, termine_a as ended_at,
                created_at, updated_at
            FROM palet_matchs
            WHERE competition_id = $1
            ORDER BY date DESC, heure
            "#,
        )
        .bind(competition_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Match {
                id: row.get("id"),
                competition_id: row.get("competition_id"),
                competition_type: row.get("competition_type"),
                journee: row.get("journee"),
                phase: row.get("phase"),
                round: row.get("round"),
                team1_id: row.get("team1_id"),
                team2_id: row.get("team2_id"),
                team1_score: row.get("team1_score"),
                team2_score: row.get("team2_score"),
                winner: row.get("winner"),
                manches: row.get("manches"),
                best_of: row.get("best_of"),
                current_manche: row.get("current_manche"),
                current_scores: row.get("current_scores"),
                status: row.get("status"),
                verified: row.get("verified"),
                verified_by: row.get("verified_by"),
                verified_at: row.get("verified_at"),
                date: row.get("date"),
                time: row.get("time"),
                location: row.get("location"),
                duration: row.get("duration"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                started_at: row.get("started_at"),
                ended_at: row.get("ended_at"),
            })
            .collect())
    }

    /// Get a match by ID
    pub async fn get_match(&self, id: Uuid) -> Result<Option<Match>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT
                id, competition_id, competition_type as "competition_type: CompetitionType",
                journee, phase, tour as round,
                equipe1_id as "team1_id: Uuid", equipe2_id as "team2_id: Uuid",
                score_equipe1 as team1_score, score_equipe2 as team2_score,
                vainqueur as "winner: MatchWinner",
                manches, meilleur_sur as best_of, manche_actuelle as current_manche,
                scores_actuels as "current_scores: palet_shared::CurrentScores",
                statut as "status: MatchStatus",
                verifie as verified, verifie_par as verified_by, verifie_le as verified_at,
                date, heure as time, lieu as location, duree as duration,
                commence_a as started_at, termine_a as ended_at,
                created_at, updated_at
            FROM palet_matchs
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| Match {
            id: row.get("id"),
            competition_id: row.get("competition_id"),
            competition_type: row.get("competition_type"),
            journee: row.get("journee"),
            phase: row.get("phase"),
            round: row.get("round"),
            team1_id: row.get("team1_id"),
            team2_id: row.get("team2_id"),
            team1_score: row.get("team1_score"),
            team2_score: row.get("team2_score"),
            winner: row.get("winner"),
            manches: row.get("manches"),
            best_of: row.get("best_of"),
            current_manche: row.get("current_manche"),
            current_scores: row.get("current_scores"),
            status: row.get("status"),
            verified: row.get("verified"),
            verified_by: row.get("verified_by"),
            verified_at: row.get("verified_at"),
            date: row.get("date"),
            time: row.get("time"),
            location: row.get("location"),
            duration: row.get("duration"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            started_at: row.get("started_at"),
            ended_at: row.get("ended_at"),
        }))
    }

    /// Create a new match
    pub async fn create_match(&self, request: CreateMatchRequest) -> Result<Match, sqlx::Error> {
        let id = generate_uuid();
        let now = now_utc();

        let row = sqlx::query(
            r#"
            INSERT INTO palet_matchs (
                id, competition_id, competition_type, journee, phase, tour,
                equipe1_id, equipe2_id, date, heure, lieu,
                meilleur_sur, statut, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING
                id, competition_id, competition_type as "competition_type: CompetitionType",
                journee, phase, tour as round,
                equipe1_id as "team1_id: Uuid", equipe2_id as "team2_id: Uuid",
                score_equipe1 as team1_score, score_equipe2 as team2_score,
                vainqueur as "winner: MatchWinner",
                manches, meilleur_sur as best_of, manche_actuelle as current_manche,
                scores_actuels as "current_scores: palet_shared::CurrentScores",
                statut as "status: MatchStatus",
                verifie as verified, verifie_par as verified_by, verifie_le as verified_at,
                date, heure as time, lieu as location, duree as duration,
                commence_a as started_at, termine_a as ended_at,
                created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(request.competition_id)
        .bind(request.competition_type)
        .bind(request.journee)
        .bind(request.phase)
        .bind(request.round)
        .bind(request.team1_id)
        .bind(request.team2_id)
        .bind(request.date)
        .bind(request.time)
        .bind(request.location)
        .bind(request.best_of.unwrap_or(13))
        .bind(MatchStatus::Scheduled)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(Match {
            id: row.get("id"),
            competition_id: row.get("competition_id"),
            competition_type: row.get("competition_type"),
            journee: row.get("journee"),
            phase: row.get("phase"),
            round: row.get("round"),
            team1_id: row.get("team1_id"),
            team2_id: row.get("team2_id"),
            team1_score: row.get("team1_score"),
            team2_score: row.get("team2_score"),
            winner: row.get("winner"),
            manches: row.get("manches"),
            best_of: row.get("best_of"),
            current_manche: row.get("current_manche"),
            current_scores: row.get("current_scores"),
            status: row.get("status"),
            verified: row.get("verified"),
            verified_by: row.get("verified_by"),
            verified_at: row.get("verified_at"),
            date: row.get("date"),
            time: row.get("time"),
            location: row.get("location"),
            duration: row.get("duration"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            started_at: row.get("started_at"),
            ended_at: row.get("ended_at"),
        })
    }

    /// Update a match
    pub async fn update_match(
        &self,
        id: Uuid,
        request: UpdateMatchRequest,
    ) -> Result<Option<Match>, sqlx::Error> {
        let now = now_utc();

        let row = sqlx::query(
            r#"
            UPDATE palet_matchs SET
                score_equipe1 = COALESCE($1, score_equipe1),
                score_equipe2 = COALESCE($2, score_equipe2),
                statut = COALESCE($3::text, statut),
                vainqueur = COALESCE($4::text, vainqueur),
                verifie = COALESCE($5, verifie),
                verifie_par = COALESCE($6, verifie_par),
                verifie_le = COALESCE($7, verifie_le),
                date = COALESCE($8, date),
                heure = COALESCE($9, heure),
                lieu = COALESCE($10, lieu),
                duree = COALESCE($11, duree),
                manche_actuelle = COALESCE($12, manche_actuelle),
                scores_actuels = COALESCE($13, scores_actuels),
                manches = COALESCE($14, manches),
                updated_at = $15
            WHERE id = $16
            RETURNING
                id, competition_id, competition_type as "competition_type: CompetitionType",
                journee, phase, tour as round,
                equipe1_id as "team1_id: Uuid", equipe2_id as "team2_id: Uuid",
                score_equipe1 as team1_score, score_equipe2 as team2_score,
                vainqueur as "winner: MatchWinner",
                manches, meilleur_sur as best_of, manche_actuelle as current_manche,
                scores_actuels as "current_scores: palet_shared::CurrentScores",
                statut as "status: MatchStatus",
                verifie as verified, verifie_par as verified_by, verifie_le as verified_at,
                date, heure as time, lieu as location, duree as duration,
                commence_a as started_at, termine_a as ended_at,
                created_at, updated_at
            "#,
        )
        .bind(request.team1_score)
        .bind(request.team2_score)
        .bind(request.status.map(|s| s.to_string()))
        .bind(request.winner.map(|w| w.to_string()))
        .bind(request.verified)
        .bind(request.verified_by)
        .bind(request.verified_at)
        .bind(request.date)
        .bind(request.time)
        .bind(request.location)
        .bind(request.duration)
        .bind(request.current_manche)
        .bind(request.current_scores)
        .bind(request.manches)
        .bind(now)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| Match {
            id: row.get("id"),
            competition_id: row.get("competition_id"),
            competition_type: row.get("competition_type"),
            journee: row.get("journee"),
            phase: row.get("phase"),
            round: row.get("round"),
            team1_id: row.get("team1_id"),
            team2_id: row.get("team2_id"),
            team1_score: row.get("team1_score"),
            team2_score: row.get("team2_score"),
            winner: row.get("winner"),
            manches: row.get("manches"),
            best_of: row.get("best_of"),
            current_manche: row.get("current_manche"),
            current_scores: row.get("current_scores"),
            status: row.get("status"),
            verified: row.get("verified"),
            verified_by: row.get("verified_by"),
            verified_at: row.get("verified_at"),
            date: row.get("date"),
            time: row.get("time"),
            location: row.get("location"),
            duration: row.get("duration"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            started_at: row.get("started_at"),
            ended_at: row.get("ended_at"),
        }))
    }

    // ============================================
    // CHAMPIONSHIP OPERATIONS
    // ============================================

    /// Get all championships
    pub async fn get_championships(&self) -> Result<Vec<Championship>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT
                id, nom as name, code, saison as season, description,
                ligue as league, divisions, total_journees, duree_journee as journee_duration,
                regles_classement as "classification_rules: ClassificationRules",
                statut as "status: CompetitionStatus",
                journee_actuelle as current_journee, phase_actuelle as current_phase,
                organisateur_id as organizer_id,
                created_at, updated_at
            FROM palet_championnats
            ORDER BY saison DESC, ligue
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Championship {
                id: row.get("id"),
                name: row.get("name"),
                code: row.get("code"),
                season: row.get("season"),
                description: row.get("description"),
                league: row.get("league"),
                divisions: row.get("divisions"),
                total_journees: row.get("total_journees"),
                journee_duration: row.get("journee_duration"),
                classification_rules: row.get("classification_rules"),
                status: row.get("status"),
                current_journee: row.get("current_journee"),
                current_phase: row.get("current_phase"),
                organizer_id: row.get("organizer_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect())
    }

    /// Get a championship by ID
    pub async fn get_championship(&self, id: Uuid) -> Result<Option<Championship>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT
                id, nom as name, code, saison as season, description,
                ligue as league, divisions, total_journees, duree_journee as journee_duration,
                regles_classement as "classification_rules: ClassificationRules",
                statut as "status: CompetitionStatus",
                journee_actuelle as current_journee, phase_actuelle as current_phase,
                organisateur_id as organizer_id,
                created_at, updated_at
            FROM palet_championnats
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| Championship {
            id: row.get("id"),
            name: row.get("name"),
            code: row.get("code"),
            season: row.get("season"),
            description: row.get("description"),
            league: row.get("league"),
            divisions: row.get("divisions"),
            total_journees: row.get("total_journees"),
            journee_duration: row.get("journee_duration"),
            classification_rules: row.get("classification_rules"),
            status: row.get("status"),
            current_journee: row.get("current_journee"),
            current_phase: row.get("current_phase"),
            organizer_id: row.get("organizer_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }))
    }

    /// Create a new championship
    pub async fn create_championship(
        &self,
        request: CreateChampionshipRequest,
    ) -> Result<Championship, sqlx::Error> {
        let id = generate_uuid();
        let now = now_utc();

        let row = sqlx::query(
            r#"
            INSERT INTO palet_championnats (
                id, nom, code, saison, description, ligue, divisions,
                total_journees, duree_journee, regles_classement,
                statut, organisateur_id, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING
                id, nom as name, code, saison as season, description,
                ligue as league, divisions, total_journees, duree_journee as journee_duration,
                regles_classement as "classification_rules: ClassificationRules",
                statut as "status: CompetitionStatus",
                journee_actuelle as current_journee, phase_actuelle as current_phase,
                organisateur_id as organizer_id,
                created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(request.name)
        .bind(request.code)
        .bind(request.season)
        .bind(request.description)
        .bind(request.league)
        .bind(request.divisions)
        .bind(request.total_journees)
        .bind(request.journee_duration)
        .bind(request.classification_rules)
        .bind(CompetitionStatus::Pending)
        .bind(request.organizer_id)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(Championship {
            id: row.get("id"),
            name: row.get("name"),
            code: row.get("code"),
            season: row.get("season"),
            description: row.get("description"),
            league: row.get("league"),
            divisions: row.get("divisions"),
            total_journees: row.get("total_journees"),
            journee_duration: row.get("journee_duration"),
            classification_rules: row.get("classification_rules"),
            status: row.get("status"),
            current_journee: row.get("current_journee"),
            current_phase: row.get("current_phase"),
            organizer_id: row.get("organizer_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    /// Update a championship
    pub async fn update_championship(
        &self,
        id: Uuid,
        request: UpdateChampionshipRequest,
    ) -> Result<Option<Championship>, sqlx::Error> {
        let now = now_utc();

        let row = sqlx::query(
            r#"
            UPDATE palet_championnats SET
                nom = COALESCE($1, nom),
                code = COALESCE($2, code),
                saison = COALESCE($3, saison),
                description = COALESCE($4, description),
                ligue = COALESCE($5, ligue),
                divisions = COALESCE($6, divisions),
                total_journees = COALESCE($7, total_journees),
                duree_journee = COALESCE($8, duree_journee),
                regles_classement = COALESCE($9, regles_classement),
                statut = COALESCE($10::text, statut),
                journee_actuelle = COALESCE($11, journee_actuelle),
                phase_actuelle = COALESCE($12, phase_actuelle),
                updated_at = $13
            WHERE id = $14
            RETURNING
                id, nom as name, code, saison as season, description,
                ligue as league, divisions, total_journees, duree_journee as journee_duration,
                regles_classement as "classification_rules: ClassificationRules",
                statut as "status: CompetitionStatus",
                journee_actuelle as current_journee, phase_actuelle as current_phase,
                organisateur_id as organizer_id,
                created_at, updated_at
            "#,
        )
        .bind(request.name)
        .bind(request.code)
        .bind(request.season)
        .bind(request.description)
        .bind(request.league)
        .bind(request.divisions)
        .bind(request.total_journees)
        .bind(request.journee_duration)
        .bind(request.classification_rules)
        .bind(request.status.map(|s| s.to_string()))
        .bind(request.current_journee)
        .bind(request.current_phase)
        .bind(now)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| Championship {
            id: row.get("id"),
            name: row.get("name"),
            code: row.get("code"),
            season: row.get("season"),
            description: row.get("description"),
            league: row.get("league"),
            divisions: row.get("divisions"),
            total_journees: row.get("total_journees"),
            journee_duration: row.get("journee_duration"),
            classification_rules: row.get("classification_rules"),
            status: row.get("status"),
            current_journee: row.get("current_journee"),
            current_phase: row.get("current_phase"),
            organizer_id: row.get("organizer_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }))
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
