//! Pages module for Palet Vendéen Frontend

pub mod actualites;
pub mod championship_detail;
pub mod championships;
pub mod club_detail;
pub mod clubs;
pub mod connexion;
pub mod federation;
pub mod home;
pub mod inscription;
pub mod not_found;
pub mod player_detail;
pub mod players;
pub mod tournament_detail;
pub mod tournaments;

pub use actualites::Actualites;
pub use championship_detail::ChampionnatDetail;
pub use championships::Championnats;
pub use club_detail::ClubDetail;
pub use clubs::Clubs;
pub use connexion::Connexion;
pub use federation::Federation;
pub use home::Home;
pub use inscription::Inscription;
pub use not_found::NotFound;
pub use player_detail::PlayerDetail;
pub use players::Joueurs;
pub use tournament_detail::TournamentDetail;
pub use tournaments::Tournois;
