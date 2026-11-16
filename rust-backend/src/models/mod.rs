//! Data models for Palet Vendéen Backend

// Models are defined in the shared crate
// This module re-exports them for convenience

pub mod user;

pub use palet_shared::*;
pub use user::{NewUser, RefreshToken, UserAccount};
