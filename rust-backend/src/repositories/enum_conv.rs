//! Helpers to convert between shared enums and their PostgreSQL text representation.

use crate::db::DatabaseError;

pub fn enum_to_db_string<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default()
}

pub fn enum_from_db_string<T: serde::de::DeserializeOwned>(
    value: &str,
) -> Result<T, DatabaseError> {
    serde_json::from_value(serde_json::Value::String(value.to_string()))
        .map_err(|e| DatabaseError::ProtocolError(format!("invalid enum value '{}': {}", value, e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use palet_shared::MatchStatus;

    #[test]
    fn enum_to_db_string_ecrit_enum_sans_guillemets() {
        // Les colonnes TEXT et les littéraux SQL du repo utilisent la valeur
        // nue ('in_progress'), pas la string JSON quotée ('"in_progress"').
        assert_eq!(enum_to_db_string(&MatchStatus::InProgress), "in_progress");
    }

    #[test]
    fn round_trip_enum_via_db_string() {
        let s = enum_to_db_string(&MatchStatus::Completed);
        let back: MatchStatus = enum_from_db_string(&s).expect("deserialize");
        assert_eq!(back, MatchStatus::Completed);
    }
}
