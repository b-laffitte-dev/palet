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
