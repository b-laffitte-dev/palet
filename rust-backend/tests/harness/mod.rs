//! TDD Harness for Palet Vendéen
//!
//! This module provides a comprehensive TDD harness for testing Palet Vendéen functionality.
//! It includes test utilities, fixtures, and assertions for matches and championships.

pub mod fixtures;
pub mod assertions;
pub mod utils;

// Re-export commonly used items
pub use fixtures::*;
pub use assertions::*;
pub use utils::*;
