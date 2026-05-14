use chrono::{DateTime, Utc};
use serde::Deserialize;

/// A thread in a forum, contains a subject and posts made to the thread.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Thread {
    /// ID of the thread.
    pub id: u64,
    /// The number of articles in the thread.
    #[serde(rename = "numarticles")]
    pub number_of_articles: u64,
    /// Link to the thread in boardgamegeek.com.
    pub link: String,
    /// Topic for the thread.
    pub subject: String,
    /// The ordered posts in the thread. Depending on what was requested, this may not include all
    /// posts, or start at the start.
    #[serde(
        default = "Vec::new",
        rename = "articles",
        deserialize_with = "deserialize_nested_thread_posts_list"
    )]
    pub posts: Vec<ThreadPost>,
}

/// A post on a thread.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ThreadPost {
    /// ID of the post.
    pub id: u64,
    /// The username of the user who posted.
    pub username: String,
    /// Link to the post in boardgamegeek.com.
    pub link: String,
    /// The date the post was posted.
    #[serde(rename = "postdate")]
    pub post_date: DateTime<Utc>,
    /// The date of the most recent edit, same as post date if no edits..
    #[serde(rename = "editdate")]
    pub edit_date: DateTime<Utc>,
    /// The number of times this post has been edited
    #[serde(rename = "numedits")]
    pub number_of_edits: u64,
    /// The subject, which is often the same as the thread subject for the first post, and "Re:
    /// that subject" for the rest.
    pub subject: String,
    /// The content of the post.
    pub body: String,
}

#[derive(Clone, Debug, Deserialize)]
struct ThreadPostsXml {
    #[serde(default = "Vec::new", rename = "article")]
    posts: Vec<ThreadPost>,
}

fn deserialize_nested_thread_posts_list<'de, D>(
    deserializer: D,
) -> Result<Vec<ThreadPost>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let posts_xml = ThreadPostsXml::deserialize(deserializer)?;
    Ok(posts_xml.posts)
}
