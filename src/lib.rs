//! # arnak
//!
//! A Rust library for the [Board Game Geek XML API](https://boardgamegeek.com/wiki/page/BGG_XML_API2).
//!
//! ## Example:
//! ```rust
//! use arnak::{
//!     BoardGameGeekApi,
//!     ItemType,
//! };
//!
//! // Enter tokio async runtime.
//! let rt = tokio::runtime::Runtime::new().unwrap();
//! rt.block_on(async {
//!     let api = BoardGameGeekApi::new();
//!     let collection = api.collection().get_owned("bluebearbgg").await.expect("Failed to get owned games.");
//!
//!     for game in collection.items {
//!         match game.item_type {
//!             ItemType::BoardGame => println!("{}", game.name),
//!             ItemType::BoardGameExpansion => println!("{} [expansion]", game.name),
//!         }
//!     }
//! })
//! ```

mod api;
pub use api::*;

mod endpoints;
pub use endpoints::*;

mod error;
pub use error::*;

mod utils;
