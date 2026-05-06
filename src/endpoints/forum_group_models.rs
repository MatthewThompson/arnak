use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::deserialize::deserialize_maybe_date_time_with_zone;
use crate::ItemDomain;

/// All forums specific to a certain game or game family.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ForumGroup {
    /// The ID of the game or game family that these forums pertain to.
    #[serde(rename = "id")]
    pub domain_id: u64,
    /// The type of domain, whether it is for a game or a game family.
    #[serde(rename = "type")]
    pub forum_domain: ItemDomain,
    /// The forums in the group.
    #[serde(rename = "forum")]
    pub forums: Vec<ForumDetails>,
}

/// The details for a particular forum. Threads in the forum can be fetched via the forum endpoint
/// using the ID of the forum.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ForumDetails {
    /// The unique identifier for this forum.
    pub id: u64,
    /// The title of the forum.
    pub title: String,
    /// A brief description on what the forum is used for.
    pub description: String,
    /// True if posting in this forum is not allowed.
    #[serde(rename = "noposting")]
    pub no_posting: bool,
    /// The total number of threads in this forum.
    #[serde(rename = "numthreads")]
    pub number_of_threads: u64,
    /// The total number of posts in this forum.
    #[serde(rename = "numposts")]
    pub number_of_posts: u64,
    /// The date and time of the last post in this forum, or none if there are no posts yet.
    #[serde(
        rename = "lastpostdate",
        deserialize_with = "deserialize_maybe_date_time_with_zone"
    )]
    pub last_post_date: Option<DateTime<Utc>>,
}
