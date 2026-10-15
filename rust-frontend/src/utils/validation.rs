//! Validation utilities for Palet Vendéen Frontend

/// Validate an email address
pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

/// Validate a phone number (French format)
pub fn validate_phone(phone: &str) -> bool {
    phone.len() >= 10
        && phone
            .chars()
            .all(|c| c.is_ascii_digit() || c == '+' || c == ' ')
}

/// Validate a license number
pub fn validate_license(license: &str) -> bool {
    license.len() >= 6 && license.len() <= 10 && license.chars().all(|c| c.is_ascii_alphanumeric())
}

/// Validate a postal code (French format)
pub fn validate_postal_code(code: &str) -> bool {
    code.len() == 5 && code.chars().all(|c| c.is_ascii_digit())
}
