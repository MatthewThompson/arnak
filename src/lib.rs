//! # arnak
//!
//! A Rust library for the [Board Game Geek XML API](https://boardgamegeek.com/wiki/page/BGG_XML_API2).
//!
//! ## Example:
//! ```rust
//! use arnak::BoardGameGeekApi;
//!
//! // Enter tokio async runtime.
//! let rt = tokio::runtime::Runtime::new().unwrap();
//! rt.block_on(async {
//!     let api = BoardGameGeekApi::new();
//!     let collection = api.collection().get_owned("bluebearbgg").await;
//!
//!     match collection {
//!         Ok(collection) => println!("bluebearbgg owns {} games.", collection.items.len()),
//!         Err(e) => println!("Error: {e}"),
//!     }
//! })
//! ```

#![deny(missing_docs)]

mod api;
pub use api::*;

mod endpoints;
pub use endpoints::*;

mod error;
pub use error::*;

mod query_param;
pub(crate) use query_param::*;

mod utils;
