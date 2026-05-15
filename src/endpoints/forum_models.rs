use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::deserialize::deserialize_date_time_with_zone;

/// A forum containing metadata, as well as a single page of threads in the forum.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Forum {
    /// The unique identifier for this forum.
    #[serde(rename = "@id")]
    pub id: u64,
    /// The title of the forum, containing the topic that the threads should be related to.
    #[serde(rename = "@title")]
    pub title: String,
    /// The total number of threads in this forum.
    #[serde(rename = "@numthreads")]
    pub number_of_threads: u64,
    /// The total number of posts in all the threads in this forum.
    #[serde(rename = "@numposts")]
    pub number_of_posts: u64,
    /// Metadata for the threads in this forum
    #[serde(
        default = "Vec::new",
        deserialize_with = "deserialize_nested_threads_list"
    )]
    pub threads: Vec<ThreadDetails>,
    // Last post date excluded since it always returns Jan 1 1970 and not a real value.
    //#[serde(rename = "lastpostdate")]
    // pub last_post_date: DateTime<Utc>,
    // no posting excluded since it always returns false.
    // #[serde(rename = "noposting")]
    // pub no_posting: bool,
}

/// Metadata for a thread, posts in the thread can be queried from the threads endpoint.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ThreadDetails {
    /// The ID of the thread.
    #[serde(rename = "@id")]
    pub id: u64,
    /// The subject of the thread.
    #[serde(rename = "@subject")]
    pub subject: String,
    /// The user that created the thread.
    #[serde(rename = "@author")]
    pub author: String,
    /// The number of posts in the thread.
    #[serde(rename = "@numarticles")]
    pub number_of_articles: u64,
    /// The date that the thread was posted.
    #[serde(
        rename = "@postdate",
        deserialize_with = "deserialize_date_time_with_zone"
    )]
    pub post_date: DateTime<Utc>,
    /// The date that the last post in the thread was posted.
    #[serde(
        rename = "@lastpostdate",
        deserialize_with = "deserialize_date_time_with_zone"
    )]
    pub last_post_date: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize)]
struct ThreadsXml {
    #[serde(default = "Vec::new", rename = "thread")]
    threads: Vec<ThreadDetails>,
}

fn deserialize_nested_threads_list<'de, D>(deserializer: D) -> Result<Vec<ThreadDetails>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let threads_xml = ThreadsXml::deserialize(deserializer)?;
    Ok(threads_xml.threads)
}
