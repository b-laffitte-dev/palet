//! Tests de round-trip JSON (serde) pour les types partagés.
//!
//! Règle du skill rust-tdd : toujours tester la round-trip JSON des types
//! de `rust-shared` — un changement de `rename_all` ou d'un champ casserait
//! sinon silencieusement la communication backend ↔ frontend.

use palet_shared::{ApiResponse, Championship, Club, Match};

/// JSON → type → JSON doit être identique (round-trip stable).
fn assert_roundtrip<T>(json: serde_json::Value)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let value: T = serde_json::from_value(json.clone()).expect("decode");
    let back = serde_json::to_value(&value).expect("encode");
    assert_eq!(
        json,
        back,
        "round-trip JSON instable pour {}",
        std::any::type_name::<T>()
    );
}

#[test]
fn club_roundtrip_json() {
    assert_roundtrip::<Club>(serde_json::json!({
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
    }));
}

#[test]
fn match_roundtrip_json() {
    assert_roundtrip::<Match>(serde_json::json!({
        "id": "9b2c0f62-7c4e-4b1a-9f3a-2f0d1c8a5e11",
        "competition_id": "11111111-1111-1111-1111-111111111111",
        "competition_type": "championship",
        "journee": 12,
        "phase": "Finale",
        "round": null,
        "team1_id": "22222222-2222-2222-2222-222222222222",
        "team2_id": "33333333-3333-3333-3333-333333333333",
        "team1_score": 13,
        "team2_score": 9,
        "winner": "team1",
        "manches": [{"number": 1, "team1": 6, "team2": 4}],
        "best_of": 13,
        "current_manche": 1,
        "current_scores": {"team1": 13, "team2": 9, "manche": 1},
        "status": "completed",
        "verified": true,
        "verified_by": "44444444-4444-4444-4444-444444444444",
        "verified_at": "2025-06-14T17:00:00Z",
        "date": "2025-06-14T13:00:00Z",
        "time": "15h00",
        "location": "Terrain clos de La Roche-sur-Yon",
        "duration": null,
        "created_at": "2025-06-01T10:00:00Z",
        "updated_at": "2025-06-14T16:00:00Z",
        "started_at": "2025-06-14T15:00:00Z",
        "ended_at": "2025-06-14T17:00:00Z"
    }));
}

#[test]
fn championship_roundtrip_json() {
    assert_roundtrip::<Championship>(serde_json::json!({
        "id": "55555555-5555-5555-5555-555555555555",
        "name": "Championnat D1",
        "code": "D1-2025",
        "season": "2025-2026",
        "description": null,
        "league": "Vendée",
        "divisions": ["D1", "D2"],
        "total_journees": 14,
        "journee_duration": null,
        "classification_rules": {
            "primary": "points",
            "secondary": "diff",
            "tertiary": "points_for"
        },
        "status": "in_progress",
        "current_journee": 5,
        "current_phase": null,
        "organizer_id": "66666666-6666-6666-6666-666666666666",
        "created_at": "2025-09-01T10:00:00Z",
        "updated_at": "2025-11-01T10:00:00Z"
    }));
}

#[test]
fn api_response_roundtrip_json() {
    assert_roundtrip::<ApiResponse<Vec<String>>>(serde_json::json!({
        "data": ["a", "b"],
        "message": "ok",
        "success": true
    }));
}
