//! Pages module for Palet Vendéen Frontend

pub mod home;
pub mod clubs;
pub mod club_detail;
pub mod players;
pub mod player_detail;
pub mod tournaments;
pub mod tournament_detail;
pub mod championships;
pub mod championship_detail;
pub mod federation;
pub mod actualites;
pub mod connexion;
pub mod inscription;
pub mod not_found;

pub use home::Home;
pub use clubs::Clubs;
pub use club_detail::ClubDetail;
pub use players::Joueurs;
pub use player_detail::PlayerDetail;
pub use tournaments::Tournois;
pub use tournament_detail::TournamentDetail;
pub use championships::Championnats;
pub use championship_detail::ChampionnatDetail;
pub use federation::Federation;
pub use actualites::Actualites;
pub use connexion::Connexion;
pub use inscription::Inscription;
pub use not_found::NotFound;
