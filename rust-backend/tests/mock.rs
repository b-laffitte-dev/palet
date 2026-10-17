//! Mock Database Implementation
//!
//! This module provides a mock implementation of the database for testing
//! without requiring a real PostgreSQL connection.

use chrono::Utc;
use palet_shared::{Club, ClubStatus, CreateClubRequest, Federation, SocialLinks};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;
use uuid::Uuid;

/// Mock database state for testing
#[derive(Debug, Default, Clone)]
pub struct MockDatabaseState {
    pub federations: HashMap<Uuid, Federation>,
    pub clubs: HashMap<Uuid, Club>,
    pub is_connected: bool,
    pub schema_initialized: bool,
}

/// Mock database implementation that doesn't require a real PostgreSQL connection
#[derive(Debug, Clone, Default)]
pub struct MockDatabase {
    state: Arc<RwLock<MockDatabaseState>>,
}

impl MockDatabase {
    /// Create a new mock database
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(MockDatabaseState {
                federations: HashMap::new(),
                clubs: HashMap::new(),
                is_connected: true,
                schema_initialized: false,
            })),
        }
    }

    /// Set connection status
    pub fn set_connected(&self, connected: bool) {
        let mut state = self.state.write().unwrap();
        state.is_connected = connected;
    }

    // ============================================
    // DATABASE OPERATIONS
    // ============================================

    /// Check if the database connection is alive
    pub async fn check_connection(&self) -> bool {
        let state = self.state.read().unwrap();
        state.is_connected
    }

    /// Initialize database schema
    pub async fn initialize(&self) -> Result<(), MockDatabaseError> {
        let mut state = self.state.write().unwrap();
        if !state.is_connected {
            return Err(MockDatabaseError::ConnectionFailed);
        }
        state.schema_initialized = true;
        Ok(())
    }

    /// Run database migrations
    pub async fn run_migrations(&self) -> Result<(), MockDatabaseError> {
        let state = self.state.read().unwrap();
        if !state.is_connected {
            return Err(MockDatabaseError::ConnectionFailed);
        }
        if !state.schema_initialized {
            return Err(MockDatabaseError::SchemaInitializationFailed);
        }
        Ok(())
    }

    // ============================================
    // FEDERATION OPERATIONS
    // ============================================

    /// Get all federations
    pub async fn get_federations(&self) -> Result<Vec<Federation>, MockDatabaseError> {
        let state = self.state.read().unwrap();
        if !state.is_connected {
            return Err(MockDatabaseError::ConnectionFailed);
        }
        Ok(state.federations.values().cloned().collect())
    }

    /// Get a federation by ID
    pub async fn get_federation(&self, id: Uuid) -> Result<Option<Federation>, MockDatabaseError> {
        let state = self.state.read().unwrap();
        if !state.is_connected {
            return Err(MockDatabaseError::ConnectionFailed);
        }
        Ok(state.federations.get(&id).cloned())
    }

    /// Create a new federation
    pub async fn create_federation(
        &self,
        name: String,
        slug: String,
        description: Option<String>,
    ) -> Result<Federation, MockDatabaseError> {
        let mut state = self.state.write().unwrap();
        if !state.is_connected {
            return Err(MockDatabaseError::ConnectionFailed);
        }

        let id = Uuid::new_v4();
        let federation = Federation {
            id,
            name,
            slug,
            description,
            created_at: Utc::now(),
        };

        state.federations.insert(id, federation.clone());
        Ok(federation)
    }

    // ============================================
    // CLUB OPERATIONS
    // ============================================

    /// Get all clubs
    pub async fn get_clubs(&self) -> Result<Vec<Club>, MockDatabaseError> {
        let state = self.state.read().unwrap();
        if !state.is_connected {
            return Err(MockDatabaseError::ConnectionFailed);
        }
        Ok(state.clubs.values().cloned().collect())
    }

    /// Get a club by ID
    pub async fn get_club(&self, id: Uuid) -> Result<Option<Club>, MockDatabaseError> {
        let state = self.state.read().unwrap();
        if !state.is_connected {
            return Err(MockDatabaseError::ConnectionFailed);
        }
        Ok(state.clubs.get(&id).cloned())
    }

    /// Create a new club
    pub async fn create_club(&self, request: CreateClubRequest) -> Result<Club, MockDatabaseError> {
        let mut state = self.state.write().unwrap();
        if !state.is_connected {
            return Err(MockDatabaseError::ConnectionFailed);
        }

        // Verify federation exists
        if !state.federations.contains_key(&request.federation_id) {
            return Err(MockDatabaseError::FederationRequired);
        }

        let id = Uuid::new_v4();
        let now = Utc::now();

        let club = Club {
            id,
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
            social: SocialLinks {
                facebook: None,
                twitter: None,
                instagram: None,
            },
            created_at: now,
            updated_at: now,
        };

        state.clubs.insert(id, club.clone());
        Ok(club)
    }
}

/// Error type for mock database operations
#[derive(Debug, Error)]
pub enum MockDatabaseError {
    #[error("Connection failed")]
    ConnectionFailed,
    #[error("Schema initialization failed")]
    SchemaInitializationFailed,
    #[error("Federation required")]
    FederationRequired,
}
