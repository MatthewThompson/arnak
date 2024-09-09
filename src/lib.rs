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

#![deny(clippy::pedantic, clippy::cargo)]
#![allow(
    // Collection status would indeed be better as an enum, but the problem is that it is technically allowed for the
    // user to have a game as multiple statuses in their collection.
    clippy::struct_excessive_bools,
    // Too difficult with the custom deserialize functions for large types.
    // Those functions are mostly just long due to the large attribute key match block.
    clippy::too_many_lines,
    // Readability potentially improved in some cases.
    clippy::match_bool,
    clippy::match_same_arms,
    clippy::single_match_else,
    // Not needed since the modules are private and pub::use::* is used. So the user doesn't need to type the name twice.
    // Both of these could be reconsidered if the modules are made public instead, or all imports are listed.
    clippy::module_name_repetitions,
    clippy::wildcard_imports,
    // Misc noisy
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::unused_self,
)]
#![deny(missing_docs, unused_imports)]

mod api;
pub use api::*;

mod endpoints;
pub use endpoints::*;

mod error;
pub use error::*;

mod query_param;
pub(crate) use query_param::*;

mod deserialize;
