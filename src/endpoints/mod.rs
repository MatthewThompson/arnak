//! Module for endpoints on the API.
//!
//! Each endpoint contains an API constructed from the base API,
//! models for the returned data, and a query builder that is used
//! internally for the exposed convenience functions as well as being
//! exposed so custom requests can be made.

pub(crate) mod models;
pub use models::*;

pub(crate) mod collection_models;
pub use collection_models::*;
pub(crate) mod collection;
pub use collection::*;

pub(crate) mod game_family_models;
pub use game_family_models::*;
pub(crate) mod game_family;
pub use game_family::*;

pub(crate) mod game_models;
pub use game_models::*;
pub(crate) mod game;
pub use game::*;

pub(crate) mod guild_models;
pub use guild_models::*;
pub(crate) mod guild;
pub use guild::*;

pub(crate) mod hot_list_models;
pub use hot_list_models::*;
pub(crate) mod hot_list;
pub use hot_list::*;

pub(crate) mod search_models;
pub use search_models::*;
pub(crate) mod search;
pub use search::*;
