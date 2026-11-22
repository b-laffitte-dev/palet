//! Format utilities for Palet Vendéen Frontend

use time::OffsetDateTime;
use time::macros::format_description;

/// Format a date for display
pub fn format_date(date: &OffsetDateTime) -> String {
    date.format(&format_description!("[year]-[month]-[day]"))
        .unwrap_or_default()
}

/// Format a date and time for display
pub fn format_datetime(date: &OffsetDateTime) -> String {
    date.format(&format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second]"
    ))
    .unwrap_or_default()
}

/// Format a date for French locale
pub fn format_date_fr(date: &OffsetDateTime) -> String {
    date.format(&format_description!("[day]/[month]/[year]"))
        .unwrap_or_default()
}

/// Année d'une date (p. ex. pour le copyright du footer).
pub fn year_of(now: &OffsetDateTime) -> i32 {
    now.year()
}
