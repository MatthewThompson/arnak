//! # arnak
//!
//! A Rust library for the [Board Game Geek XML API](https://boardgamegeek.com/wiki/page/BGG_XML_API2).
//!
//! ## Example:
//! ```rust
//! let api = BoardGameGeekApi::new();
//! let collection = api.collection().get_owned("blueearbgg").await?;
//!
//! for game in collection.games {
//!     match game.game_type {
//!         GameType::BoardGame => println!("{}", game.name),
//!         GameType::BoardGameExpansion => println!("{} [expansion]", game.name),
//!     }
//! }
//! ```
//!

mod api;
pub use api::*;

mod endpoints;
pub use endpoints::*;

mod error;
pub use error::*;

mod utils;
