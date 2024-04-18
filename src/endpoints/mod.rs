//! Module for endpoints on the API.
//!
//! Each endpoint contains an API constructed from the base API,
//! models for the returned data, and a query builder that is used
//! internally for the exposed convenience functions as well as being
//! exposed so custom requests can be made.

pub(crate) mod collection;
pub use collection::*;
