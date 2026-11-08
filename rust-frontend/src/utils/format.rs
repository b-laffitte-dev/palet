//! Format utilities for Palet Vendéen Frontend

use chrono::{DateTime, Datelike, Utc};

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

/// Current sports season, e.g. "2026-2027".
///
/// A palet season runs roughly July→June: from July onwards the season is
/// `{year}-{year+1}`, before that `{year-1}-{year}`.
pub fn current_season(now: &DateTime<Utc>) -> String {
    let year = now.year();
    if now.month() >= 7 {
        format!("{year}-{}", year + 1)
    } else {
        format!("{}-{}", year - 1, year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn season_starts_in_july() {
        let july = Utc.with_ymd_and_hms(2026, 7, 1, 12, 0, 0).unwrap();
        assert_eq!(current_season(&july), "2026-2027");
    }

    #[test]
    fn season_in_autumn_and_winter() {
        let november = Utc.with_ymd_and_hms(2026, 11, 8, 12, 0, 0).unwrap();
        assert_eq!(current_season(&november), "2026-2027");
        let january = Utc.with_ymd_and_hms(2027, 1, 15, 12, 0, 0).unwrap();
        assert_eq!(current_season(&january), "2026-2027");
    }

    #[test]
    fn season_before_july_belongs_to_previous_year() {
        let june = Utc.with_ymd_and_hms(2026, 6, 30, 12, 0, 0).unwrap();
        assert_eq!(current_season(&june), "2025-2026");
    }
}
