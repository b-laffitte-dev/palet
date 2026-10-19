//! Format utilities for Palet Vendéen Frontend

use chrono::{DateTime, Utc};

/// Format a date for display
pub fn format_date(date: &DateTime<Utc>) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// Format a date and time for display
pub fn format_datetime(date: &DateTime<Utc>) -> String {
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Format a date for French locale
pub fn format_date_fr(date: &DateTime<Utc>) -> String {
    date.format("%d/%m/%Y").to_string()
}
