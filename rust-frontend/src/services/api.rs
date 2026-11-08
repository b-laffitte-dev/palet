//! API service for Palet Vendéen Frontend
//!
//! Thin HTTP client over the Actix backend (`rust-backend`). Every public
//! method maps to an existing route, wrapped in `ApiResponse<T>`.
//!
//! Base URL resolution:
//! - `dx serve` dev server (port 8081): the API is not proxied, so we call
//!   `http://localhost:3000` directly (the backend allows any origin).
//! - production (nginx): `/api/` is proxied to the backend, so a relative
//!   `/api` base URL is used.
//!
//! `with_base_url` still allows overriding this explicitly.

use palet_shared::{
    ApiResponse, Championship, ChampionshipClassification, Club, CreateDivisionRequest, Division,
    Match, PaginatedResponse,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

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
    /// Create a new API service with automatic base URL detection
    pub fn new() -> Self {
        Self {
            base_url: Self::detect_base_url(),
        }
    }

    /// Create a new API service with custom base URL
    pub fn with_base_url(base_url: String) -> Self {
        Self { base_url }
    }

    /// Dev server (`dx serve`, port 8081) has no reverse proxy: hit the
    /// backend directly. Everywhere else nginx proxies `/api/`.
    fn detect_base_url() -> String {
        if let Some(window) = web_sys::window() {
            if let Ok(port) = window.location().port() {
                if port == "8081" {
                    return "http://localhost:3000".to_string();
                }
            }
        }
        "/api".to_string()
    }

    /// Perform a GET request and deserialize the JSON body
    async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T, String> {
        self.request_json("GET", path, None::<&serde_json::Value>)
            .await
    }

    /// Perform a POST request with a JSON body and deserialize the JSON response
    async fn post_json<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &impl Serialize,
    ) -> Result<T, String> {
        self.request_json("POST", path, Some(body)).await
    }

    /// Perform a DELETE request and deserialize the JSON response
    async fn delete_json<T: DeserializeOwned>(&self, path: &str) -> Result<T, String> {
        self.request_json("DELETE", path, None::<&serde_json::Value>)
            .await
    }

    /// Shared HTTP helper: fetch `path` with `method` (and optional JSON body),
    /// then deserialize the JSON body. Error responses include the body text.
    async fn request_json<T: DeserializeOwned>(
        &self,
        method: &str,
        path: &str,
        body: Option<&impl Serialize>,
    ) -> Result<T, String> {
        let url = format!("{}{}", self.base_url, path);
        let window = web_sys::window().ok_or_else(|| "no window available".to_string())?;

        let init = web_sys::RequestInit::new();
        init.set_method(method);
        if let Some(body) = body {
            let json = serde_json::to_string(body)
                .map_err(|e| format!("encoding body for {url} failed: {e}"))?;
            init.set_body(&JsValue::from_str(&json));
            let headers = web_sys::Headers::new().map_err(|e| format!("headers: {e:?}"))?;
            headers
                .set("Content-Type", "application/json")
                .map_err(|e| format!("content-type header: {e:?}"))?;
            init.set_headers(headers.as_ref());
        }

        let response_value = JsFuture::from(window.fetch_with_str_and_init(&url, &init))
            .await
            .map_err(|e| format!("request to {url} failed: {e:?}"))?;
        let response: web_sys::Response = response_value
            .dyn_into()
            .map_err(|_| format!("response of {url} is not a fetch Response"))?;

        let text_value = JsFuture::from(response.text().map_err(|e| format!("{e:?}"))?)
            .await
            .map_err(|e| format!("reading body of {url} failed: {e:?}"))?;
        let body = text_value
            .as_string()
            .ok_or_else(|| format!("body of {url} is not a string"))?;

        if !response.ok() {
            return Err(format!("HTTP {} for {url}: {body}", response.status()));
        }
        serde_json::from_str(&body).map_err(|e| format!("decoding {url} failed: {e}"))
    }

    /// Get all clubs — GET /clubs
    pub async fn get_clubs(&self) -> Result<Vec<Club>, String> {
        let response: ApiResponse<PaginatedResponse<Club>> = self.get_json("/clubs").await?;
        Ok(response.data.data)
    }

    /// Get a specific club by ID — GET /clubs/{id}
    pub async fn get_club(&self, id: String) -> Result<Club, String> {
        let response: ApiResponse<Club> = self.get_json(&format!("/clubs/{id}")).await?;
        Ok(response.data)
    }

    /// Get all matches — GET /matchs
    pub async fn get_matches(&self) -> Result<Vec<Match>, String> {
        let response: ApiResponse<PaginatedResponse<Match>> = self.get_json("/matchs").await?;
        Ok(response.data.data)
    }

    /// Get matches filtered by status — GET /matchs/status/{status}
    ///
    /// `status` is one of: scheduled, in_progress, completed, postponed,
    /// cancelled, disputed.
    pub async fn get_matches_by_status(&self, status: &str) -> Result<Vec<Match>, String> {
        let response: ApiResponse<Vec<Match>> =
            self.get_json(&format!("/matchs/status/{status}")).await?;
        Ok(response.data)
    }

    /// Get all championships — GET /championnats
    pub async fn get_championships(&self) -> Result<Vec<Championship>, String> {
        let response: ApiResponse<PaginatedResponse<Championship>> =
            self.get_json("/championnats").await?;
        Ok(response.data.data)
    }

    /// Get the classification of a championship — GET /championnats/{id}/classification
    ///
    /// Returns one `ChampionshipClassification` per division.
    pub async fn get_championship_classification(
        &self,
        championship_id: uuid::Uuid,
    ) -> Result<Vec<ChampionshipClassification>, String> {
        let response: ApiResponse<Vec<ChampionshipClassification>> = self
            .get_json(&format!("/championnats/{championship_id}/classification"))
            .await?;
        Ok(response.data)
    }

    /// Get championships of a league (département) — GET /championnats/league/{league}
    pub async fn get_championships_by_league(
        &self,
        league: &str,
    ) -> Result<Vec<Championship>, String> {
        let response: ApiResponse<Vec<Championship>> = self
            .get_json(&format!("/championnats/league/{}", url_encode(league)))
            .await?;
        Ok(response.data)
    }

    /// Get divisions, optionally filtered by league — GET /divisions?league={league}
    ///
    /// Divisions are dynamic: they are created/deleted from the federal
    /// space and listed here ordered by position (tab order on the home page).
    pub async fn get_divisions(&self, league: Option<&str>) -> Result<Vec<Division>, String> {
        let path = match league {
            Some(league) => format!("/divisions?league={}", url_encode(league)),
            None => "/divisions".to_string(),
        };
        let response: ApiResponse<Vec<Division>> = self.get_json(&path).await?;
        Ok(response.data)
    }

    /// Create a division — POST /divisions (espace fédéral)
    pub async fn create_division(
        &self,
        request: &CreateDivisionRequest,
    ) -> Result<Division, String> {
        let response: ApiResponse<Division> = self.post_json("/divisions", request).await?;
        Ok(response.data)
    }

    /// Delete a division — DELETE /divisions/{id} (espace fédéral)
    pub async fn delete_division(&self, id: uuid::Uuid) -> Result<bool, String> {
        let response: ApiResponse<bool> = self.delete_json(&format!("/divisions/{id}")).await?;
        Ok(response.data)
    }
}

/// Percent-encode a path/query segment (UTF-8), e.g. "Deux-Sèvres".
///
/// Pure Rust so it is unit-testable outside the browser (unlike
/// `js_sys::encode_uri_component`).
fn url_encode(segment: &str) -> String {
    let mut out = String::with_capacity(segment.len());
    for b in segment.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use palet_shared::MatchStatus;

    /// Sample shaped exactly like `GET /matchs/status/in_progress`
    /// (Actix handler wraps everything in `ApiResponse`).
    #[test]
    fn parses_match_list_response() {
        let body = r#"{
            "data": [
                {
                    "id": "9b2c0f62-7c4e-4b1a-9f3a-2f0d1c8a5e11",
                    "competition_id": "11111111-1111-1111-1111-111111111111",
                    "competition_type": "championship",
                    "journee": 12,
                    "phase": "Finale",
                    "round": null,
                    "team1_id": "22222222-2222-2222-2222-222222222222",
                    "team2_id": "33333333-3333-3333-3333-333333333333",
                    "team1_score": 72,
                    "team2_score": 68,
                    "winner": null,
                    "manches": null,
                    "best_of": 13,
                    "current_manche": 24,
                    "current_scores": {"team1": 72, "team2": 68, "manche": 24},
                    "status": "in_progress",
                    "verified": false,
                    "verified_by": null,
                    "verified_at": null,
                    "date": "2025-06-14T13:00:00Z",
                    "time": "15h00",
                    "location": "Terrain clos de La Roche-sur-Yon",
                    "duration": null,
                    "created_at": "2025-06-01T10:00:00Z",
                    "updated_at": "2025-06-14T16:00:00Z",
                    "started_at": "2025-06-14T15:00:00Z",
                    "ended_at": null
                }
            ],
            "message": "Matchs retrieved successfully",
            "success": true
        }"#;

        let parsed: ApiResponse<Vec<Match>> = serde_json::from_str(body).expect("decode");
        assert!(parsed.success);
        assert_eq!(parsed.data.len(), 1);
        let m = &parsed.data[0];
        assert_eq!(m.status, MatchStatus::InProgress);
        assert_eq!(m.current_scores.as_ref().unwrap().team1, 72);
        assert_eq!(m.journee, Some(12));
    }

    /// Sample shaped like `GET /clubs` (`ApiResponse<PaginatedResponse<Club>>`).
    #[test]
    fn parses_paginated_club_response() {
        let body = r#"{
            "data": {
                "data": [
                    {
                        "id": "22222222-2222-2222-2222-222222222222",
                        "name": "La Roche-sur-Yon Palet Club",
                        "nickname": null,
                        "code": "LRY",
                        "logo": null,
                        "address": null,
                        "city": "La Roche-sur-Yon",
                        "postal_code": "85000",
                        "league": "Vendée",
                        "division": "D1",
                        "status": "active",
                        "players_count": 42,
                        "teams_count": 3,
                        "contact_email": null,
                        "contact_phone": null,
                        "website": null,
                        "social": {"facebook": null, "twitter": null, "instagram": null},
                        "created_at": "2024-09-01T10:00:00Z",
                        "updated_at": "2025-01-01T10:00:00Z"
                    }
                ],
                "pagination": {
                    "current": 1, "total": 1, "page_size": 1,
                    "total_items": 1, "has_next": false, "has_previous": false
                }
            },
            "message": "Clubs retrieved successfully",
            "success": true
        }"#;

        let parsed: ApiResponse<PaginatedResponse<Club>> =
            serde_json::from_str(body).expect("decode");
        assert_eq!(parsed.data.data.len(), 1);
        assert_eq!(parsed.data.data[0].city, "La Roche-sur-Yon");
    }

    /// Sample shaped exactly like `GET /divisions?league=Deux-Sèvres`.
    #[test]
    fn parses_division_list_response() {
        let body = r#"{
            "data": [
                {
                    "id": "185420b2-3a6a-4d2a-acea-53f98733eee6",
                    "name": "D1",
                    "league": "Deux-Sèvres",
                    "season": "2026-2027",
                    "position": 1,
                    "created_at": "2026-11-08T14:14:57.919667Z",
                    "updated_at": "2026-11-08T14:14:57.919667Z"
                },
                {
                    "id": "ff889101-9233-4f52-a032-ba486da2d62e",
                    "name": "D2",
                    "league": "Deux-Sèvres",
                    "season": "2026-2027",
                    "position": 2,
                    "created_at": "2026-11-08T14:14:57.943486Z",
                    "updated_at": "2026-11-08T14:14:57.943486Z"
                }
            ],
            "message": "Divisions retrieved successfully",
            "success": true
        }"#;

        let parsed: ApiResponse<Vec<Division>> = serde_json::from_str(body).expect("decode");
        assert!(parsed.success);
        assert_eq!(parsed.data.len(), 2);
        assert_eq!(parsed.data[0].name, "D1");
        assert_eq!(parsed.data[1].position, 2);
        assert_eq!(parsed.data[0].league, "Deux-Sèvres");
    }

    #[test]
    fn url_encode_percent_encodes_non_ascii() {
        assert_eq!(url_encode("Deux-Sèvres"), "Deux-S%C3%A8vres");
        assert_eq!(url_encode("D1"), "D1");
    }
}
