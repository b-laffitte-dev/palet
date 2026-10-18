//! TDD Harness for Palet Vendéen
//!
//! This module provides a comprehensive TDD harness for testing Palet Vendéen functionality.
//! It includes test utilities, fixtures, mocks, and assertions for matches and championships.

pub mod assertions;
pub mod fixtures;
pub mod mocks;
pub mod utils;

// Re-export commonly used items
pub use assertions::*;
pub use fixtures::*;
pub use mocks::*;
pub use utils::*;
