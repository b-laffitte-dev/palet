//! Génère un hash Argon2id pour un mot de passe (utile pour créer des comptes
//! de démo dans les seeds SQL) :
//! `cargo run -p palet-backend --example hash_password -- "MonMotDePasse!"`

use argon2::password_hash::phc::SaltString;
use argon2::password_hash::PasswordHasher;
use argon2::Argon2;

fn main() {
    let password = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Palet2026!".to_string());
    let salt = SaltString::generate();
    let hash = Argon2::default()
        .hash_password_with_salt(password.as_bytes(), salt.as_bytes())
        .expect("hash")
        .to_string();
    println!("{hash}");
}
