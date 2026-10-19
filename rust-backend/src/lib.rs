//! Palet Venden Backend Library
//!
//! This crate provides the backend functionality for the Palet Venden application.
//! It includes database connectivity, handlers, and configuration.

// Re-export all public modules for use in tests and other crates
pub mod config;
pub mod db;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod services;

// Re-export the main types for convenience
pub use config::AppConfig;
pub use db::Database;

// AppState needs to be defined here for library use
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub config: Arc<config::AppConfig>,
}
