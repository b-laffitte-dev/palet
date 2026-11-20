//! Common components for Palet Vendéen Frontend

pub mod badge;
pub mod button;
pub mod card;
pub mod logo;
pub mod match_table;

pub use button::{Button, ButtonSize, ButtonVariant};
pub use logo::Logo;
pub use match_table::{team_name, MatchTable};
