//! Home page sections, matching `docs/maquettes/homepage.png`.
//!
//! These components are pure presentation: they receive the view models
//! built in [`model`] from the real API data.

pub mod hero;
pub mod home_footer;
pub mod live_scores;
pub mod model;
pub mod site_header;
pub mod spaces;
pub mod standings;
pub mod top_bar;
pub mod top_players;
pub mod upcoming;

pub use hero::HeroSection;
pub use home_footer::HomeFooter;
pub use live_scores::LiveScores;
pub use model::HomeData;
pub use site_header::SiteHeader;
pub use spaces::Spaces;
pub use standings::Standings;
pub use top_bar::TopBar;
pub use top_players::TopPlayers;
pub use upcoming::Upcoming;
