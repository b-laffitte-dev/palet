//! Génère un hash Argon2id pour un mot de passe (utile pour créer des comptes
//! de démo dans les seeds SQL) :
//! `cargo run -p palet-backend --example hash_password -- "MonMotDePasse!"`

use argon2::password_hash::{PasswordHasher, SaltString};
use argon2::Argon2;

fn main() {
    let password = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Palet2026!".to_string());
    let salt = SaltString::generate(&mut rand::rngs::OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("hash")
        .to_string();
    println!("{hash}");
}
