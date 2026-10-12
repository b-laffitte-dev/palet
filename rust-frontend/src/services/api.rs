//! API service for Palet Vendéen Frontend
//!
//! Handles all HTTP requests to the backend API

use gloo::storage::{LocalStorage, SessionStorage};
use log::{debug, error, info};
use palet_shared::{
    ApiError, ApiResponse, Club, CreateClubRequest, Federation, HealthCheck, Match,
    PaginatedResponse, Player, Tournament, Championship, AuthToken, AuthState,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::window;

/// API service for making HTTP requests
#[derive(Clone)]
pub struct ApiService {
    base_url: String,
    token: Option<String>,
}

impl Default for ApiService {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiService {
    /// Create a new API service
    pub fn new() -> Self {
        let base_url = window()
            .unwrap()
            .location()
            .origin()
            .unwrap_or_else(|| "http://localhost:3000".to_string());

        Self {
            base_url,
            token: None,
        }
    }

    /// Create API service with custom base URL
    pub fn with_base_url(base_url: String) -> Self {
        Self {
            base_url,
            token: None,
        }
    }

    /// Set authentication token
    pub fn set_token(&mut self, token: Option<String>) {
        self.token = token;
    }

    /// Get authentication token
    pub fn get_token(&self) -> Option<String> {
        self.token.clone()
    }

    /// Get token from storage
    pub fn get_token_from_storage() -> Option<String> {
        let remember_me = LocalStorage::get("palet_remember_me")
            .map(|v: String| v == "true")
            .unwrap_or(false);

        let token_string = if remember_me {
            LocalStorage::get("palet_access_token")
        } else {
            SessionStorage::get("palet_access_token")
        };

        token_string.map(|v: String| v)
    }

    /// Store token in storage
    pub fn store_token(token: &AuthToken, remember_me: bool) {
        let token_json = serde_json::to_string(token).unwrap();
        
        if remember_me {
            LocalStorage::set("palet_access_token", &token_json).unwrap();
            LocalStorage::set("palet_remember_me", "true").unwrap();
        } else {
            SessionStorage::set("palet_access_token", &token_json).unwrap();
            LocalStorage::remove("palet_remember_me");
        }
    }

    /// Clear token from storage
    pub fn clear_token() {
        LocalStorage::remove("palet_access_token");
        SessionStorage::remove("palet_access_token");
        LocalStorage::remove("palet_remember_me");
    }

    /// Make a GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        self.request::<T, _>(path, reqwest::Method::GET, None, None).await
    }

    /// Make a POST request
    pub async fn post<T: DeserializeOwned, D: Serialize + std::fmt::Debug>(
        &self,
        path: &str,
        data: Option<D>,
    ) -> Result<T, ApiError> {
        self.request::<T, D>(path, reqwest::Method::POST, data, None).await
    }

    /// Make a PUT request
    pub async fn put<T: DeserializeOwned, D: Serialize + std::fmt::Debug>(
        &self,
        path: &str,
        data: Option<D>,
    ) -> Result<T, ApiError> {
        self.request::<T, D>(path, reqwest::Method::PUT, data, None).await
    }

    /// Make a PATCH request
    pub async fn patch<T: DeserializeOwned, D: Serialize + std::fmt::Debug>(
        &self,
        path: &str,
        data: Option<D>,
    ) -> Result<T, ApiError> {
        self.request::<T, D>(path, reqwest::Method::PATCH, data, None).await
    }

    /// Make a DELETE request
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        self.request::<T, _>(path, reqwest::Method::DELETE, None, None).await
    }

    /// Generic request method
    pub async fn request<T: DeserializeOwned, D: Serialize + std::fmt::Debug>(
        &self,
        path: &str,
        method: reqwest::Method,
        data: Option<D>,
        headers: Option<HashMap<String, String>>,
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", self.base_url, path);

        debug!("Making {} request to {}", method, url);

        let client = reqwest::Client::new();
        let mut request = client.request(method, &url);

        // Add authentication header if token exists
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        // Add custom headers
        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        // Add body if data exists
        if let Some(data) = data {
            request = request.json(&data);
        }

        // Send request
        let response = match request.send().await {
            Ok(resp) => resp,
            Err(e) => {
                error!("Request failed: {:?}", e);
                return Err(ApiError {
                    message: "Request failed".to_string(),
                    code: "NETWORK_ERROR".to_string(),
                    status_code: 0,
                    details: Some(json!({ "error": e.to_string() })),
                });
            }
        };

        let status = response.status();

        // Handle error responses
        if !status.is_success() {
            let error_body: serde_json::Value = match response.json().await {
                Ok(body) => body,
                Err(_) => json!({}),
            };

            let message = error_body
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error")
                .to_string();

            let code = error_body
                .get("code")
                .and_then(|v| v.as_str())
                .unwrap_or("UNKNOWN_ERROR")
                .to_string();

            error!("API error: {} - {}", status, message);

            return Err(ApiError {
                message,
                code,
                status_code: status.as_u16(),
                details: Some(error_body),
            });
        }

        // Parse successful response
        match response.json::<ApiResponse<T>>().await {
            Ok(api_response) => {
                if api_response.success {
                    Ok(api_response.data)
                } else {
                    let message = api_response
                        .message
                        .unwrap_or_else(|| "Request failed".to_string());
                    Err(ApiError {
                        message,
                        code: "API_ERROR".to_string(),
                        status_code: status.as_u16(),
                        details: None,
                    })
                }
            }
            Err(e) => {
                error!("Failed to parse response: {:?}", e);
                Err(ApiError {
                    message: "Failed to parse response".to_string(),
                    code: "PARSE_ERROR".to_string(),
                    status_code: status.as_u16(),
                    details: Some(json!({ "error": e.to_string() })),
                })
            }
        }
    }

    // ============================================
    // HEALTH CHECK ENDPOINTS
    // ============================================

    /// Check API health
    pub async fn health_check(&self) -> Result<HealthCheck, ApiError> {
        self.get::<HealthCheck>("/health").await
    }

    // ============================================
    // FEDERATION ENDPOINTS
    // ============================================

    /// Get all federations
    pub async fn get_federations(&self) -> Result<Vec<Federation>, ApiError> {
        self.get::<Vec<Federation>>("/federations").await
    }

    /// Get a federation by ID
    pub async fn get_federation(&self, id: &str) -> Result<Federation, ApiError> {
        self.get::<Federation>(&format!("/federations/{}", id)).await
    }

    // ============================================
    // CLUB ENDPOINTS
    // ============================================

    /// Get all clubs
    pub async fn get_clubs(&self) -> Result<PaginatedResponse<Club>, ApiError> {
        self.get::<PaginatedResponse<Club>>("/clubs").await
    }

    /// Get a club by ID
    pub async fn get_club(&self, id: &str) -> Result<Club, ApiError> {
        self.get::<Club>(&format!("/clubs/{}", id)).await
    }

    /// Create a new club
    pub async fn create_club(&self, request: CreateClubRequest) -> Result<Club, ApiError> {
        self.post::<Club, CreateClubRequest>("/clubs", Some(request)).await
    }

    /// Update a club
    pub async fn update_club(
        &self,
        id: &str,
        request: CreateClubRequest,
    ) -> Result<Club, ApiError> {
        self.put::<Club, CreateClubRequest>(&format!("/clubs/{}", id), Some(request)).await
    }

    /// Delete a club
    pub async fn delete_club(&self, id: &str) -> Result<(), ApiError> {
        self.delete::<()>(&format!("/clubs/{}", id)).await
    }

    // ============================================
    // PLAYER ENDPOINTS
    // ============================================

    /// Get all players
    pub async fn get_players(&self) -> Result<PaginatedResponse<Player>, ApiError> {
        self.get::<PaginatedResponse<Player>>("/players").await
    }

    /// Get a player by ID
    pub async fn get_player(&self, id: &str) -> Result<Player, ApiError> {
        self.get::<Player>(&format!("/players/{}", id)).await
    }

    /// Get players by club
    pub async fn get_players_by_club(&self, club_id: &str) -> Result<Vec<Player>, ApiError> {
        self.get::<Vec<Player>>(&format!("/clubs/{}/players", club_id)).await
    }

    // ============================================
    // TOURNAMENT ENDPOINTS
    // ============================================

    /// Get all tournaments
    pub async fn get_tournaments(&self) -> Result<PaginatedResponse<Tournament>, ApiError> {
        self.get::<PaginatedResponse<Tournament>>("/tournaments").await
    }

    /// Get a tournament by ID
    pub async fn get_tournament(&self, id: &str) -> Result<Tournament, ApiError> {
        self.get::<Tournament>(&format!("/tournaments/{}", id)).await
    }

    // ============================================
    // CHAMPIONSHIP ENDPOINTS
    // ============================================

    /// Get all championships
    pub async fn get_championships(&self) -> Result<PaginatedResponse<Championship>, ApiError> {
        self.get::<PaginatedResponse<Championship>>("/championships").await
    }

    /// Get a championship by ID
    pub async fn get_championship(&self, id: &str) -> Result<Championship, ApiError> {
        self.get::<Championship>(&format!("/championships/{}", id)).await
    }

    // ============================================
    // MATCH ENDPOINTS
    // ============================================

    /// Get all matches
    pub async fn get_matches(&self) -> Result<PaginatedResponse<Match>, ApiError> {
        self.get::<PaginatedResponse<Match>>("/matches").await
    }

    /// Get a match by ID
    pub async fn get_match(&self, id: &str) -> Result<Match, ApiError> {
        self.get::<Match>(&format!("/matches/{}", id)).await
    }

    // ============================================
    // AUTHENTICATION ENDPOINTS
    // ============================================

    /// Login
    pub async fn login(
        &self,
        email: String,
        password: String,
        remember_me: bool,
    ) -> Result<AuthToken, ApiError> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let body = json!({
            "email": email,
            "password": password,
            "remember_me": remember_me
        });

        let token: AuthToken = self
            .request::<AuthToken, _>("/auth/login", reqwest::Method::POST, Some(body), Some(headers))
            .await?;

        // Store token
        Self::store_token(&token, remember_me);
        self.token = Some(token.access_token.clone());

        Ok(token)
    }

    /// Register
    pub async fn register(
        &self,
        email: String,
        password: String,
        first_name: String,
        last_name: String,
    ) -> Result<AuthToken, ApiError> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let body = json!({
            "email": email,
            "password": password,
            "first_name": first_name,
            "last_name": last_name
        });

        let token: AuthToken = self
            .request::<AuthToken, _>("/auth/register", reqwest::Method::POST, Some(body), Some(headers))
            .await?;

        Ok(token)
    }

    /// Logout
    pub async fn logout(&mut self) -> Result<(), ApiError> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        self.request::<(), _>("/auth/logout", reqwest::Method::POST, None, Some(headers)).await?;

        // Clear token
        Self::clear_token();
        self.token = None;

        Ok(())
    }

    /// Get current user
    pub async fn get_current_user(&self) -> Result<Option<palet_shared::User>, ApiError> {
        self.get::<Option<palet_shared::User>>("/auth/me").await
    }
}

// ============================================
// HOOK FOR USING API SERVICE IN COMPONENTS
// ============================================

/// Hook to use API service in components
#[yew::function_component(UseApi)]
pub fn use_api() -> ApiService {
    use yew::prelude::*;
    
    // Get API service from context or create new one
    match use_context::<ApiService>() {
        Some(api) => api.clone(),
        None => ApiService::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_api_service_creation() {
        let api = ApiService::new();
        assert!(api.base_url.contains("localhost") || api.base_url.contains("127.0.0.1"));
    }

    #[wasm_bindgen_test]
    fn test_token_management() {
        let mut api = ApiService::new();
        
        // Initially no token
        assert!(api.get_token().is_none());
        
        // Set token
        api.set_token(Some("test-token".to_string()));
        assert_eq!(api.get_token(), Some("test-token".to_string()));
        
        // Clear token
        api.set_token(None);
        assert!(api.get_token().is_none());
    }
}
