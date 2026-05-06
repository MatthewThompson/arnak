//! Module for endpoints on the API.
//!
//! Each endpoint contains an API constructed from the base API,
//! models for the returned data, and a query builder that is used
//! internally for the exposed convenience functions as well as being
//! exposed so custom requests can be made.

pub(crate) mod models;
pub use models::*;

pub(crate) mod accessory_models;
pub use accessory_models::*;
pub(crate) mod accessory;
pub use accessory::*;

pub(crate) mod collection_models;
pub use collection_models::*;
pub(crate) mod collection;
pub use collection::*;

pub(crate) mod forum_group_models;
pub use forum_group_models::*;
pub(crate) mod forum_group;
pub use forum_group::*;

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

pub(crate) mod plays_models;
pub use plays_models::*;
pub(crate) mod plays;
pub use plays::*;

pub(crate) mod search_models;
pub use search_models::*;
pub(crate) mod search;
pub use search::*;

pub(crate) mod user_models;
pub use user_models::*;
pub(crate) mod user;
pub use user::*;
