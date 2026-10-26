//! API service for Palet Vendéen Frontend

use palet_shared::{Championship, Club, Federation, Player, Tournament};

/// API Service for making HTTP requests to the backend
#[derive(Clone)]
pub struct ApiService {
    base_url: String,
}

impl Default for ApiService {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiService {
    /// Create a new API service
    pub fn new() -> Self {
        Self {
            base_url: "http://localhost:3000".to_string(),
        }
    }

    /// Create a new API service with custom base URL
    pub fn with_base_url(base_url: String) -> Self {
        Self { base_url }
    }

    /// Get all clubs
    pub async fn get_clubs(&self) -> Result<Vec<Club>, String> {
        // In a real implementation, use reqwest or web-sys
        Ok(vec![])
    }

    /// Get a specific club by ID
    pub async fn get_club(&self, _id: String) -> Result<Club, String> {
        Ok(Club {
            id: uuid::Uuid::new_v4(),
            name: "Test Club".to_string(),
            nickname: None,
            code: "TC001".to_string(),
            logo: None,
            address: None,
            city: "Test City".to_string(),
            postal_code: None,
            league: "Deux-Sèvres".to_string(),
            division: "D1".to_string(),
            status: palet_shared::ClubStatus::Active,
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
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    /// Get all tournaments
    pub async fn get_tournaments(&self) -> Result<Vec<Tournament>, String> {
        Ok(vec![])
    }

    /// Get all championships
    pub async fn get_championships(&self) -> Result<Vec<Championship>, String> {
        Ok(vec![])
    }

    /// Get all players
    pub async fn get_players(&self) -> Result<Vec<Player>, String> {
        Ok(vec![])
    }

    /// Get a specific player by ID
    pub async fn get_player(&self, _id: String) -> Result<Player, String> {
        Err("Not implemented".to_string())
    }

    /// Get federations
    pub async fn get_federations(&self) -> Result<Vec<Federation>, String> {
        Ok(vec![])
    }
}
