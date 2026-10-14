//! Palet Vendéen Mobile - Tauri Implementation
//!
//! Library crate for mobile application

pub mod android;
pub mod ios;

// Re-export main types
pub use android::*;
pub use ios::*;
