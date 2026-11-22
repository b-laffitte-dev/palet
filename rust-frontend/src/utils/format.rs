//! Format utilities for Palet Vendéen Frontend

use time::macros::format_description;
use time::OffsetDateTime;

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

/// Current sports season, e.g. "2026-2027".
///
/// A palet season runs roughly July→June: from July onwards the season is
/// `{year}-{year+1}`, before that `{year-1}-{year}`.
pub fn current_season(now: &OffsetDateTime) -> String {
    let year = now.year();
    if u8::from(now.month()) >= 7 {
        format!("{year}-{}", year + 1)
    } else {
        format!("{}-{}", year - 1, year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn year_of_renvoie_l_annee() {
        let date = datetime!(2026-11-22 12:00:00 UTC);
        assert_eq!(year_of(&date), 2026);
    }

    #[test]
    fn season_starts_in_july() {
        let july = datetime!(2026-07-01 12:00:00 UTC);
        assert_eq!(current_season(&july), "2026-2027");
    }

    #[test]
    fn season_in_autumn_and_winter() {
        let november = datetime!(2026-11-08 12:00:00 UTC);
        assert_eq!(current_season(&november), "2026-2027");
        let january = datetime!(2027-01-15 12:00:00 UTC);
        assert_eq!(current_season(&january), "2026-2027");
    }

    #[test]
    fn season_before_july_belongs_to_previous_year() {
        let june = datetime!(2026-06-30 12:00:00 UTC);
        assert_eq!(current_season(&june), "2025-2026");
    }
}
