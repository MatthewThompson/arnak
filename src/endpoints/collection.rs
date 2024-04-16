use serde::{Deserialize, Serialize};
use std::future::Future;

use crate::api::BoardGameGeekAPI;
use crate::utils::deserialize_1_0_bool;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Collection {
    #[serde(rename = "$value")]
    pub games: Vec<CollectionGame>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CollectionGame {
    /// The ID of the game.
    #[serde(rename = "objectid")]
    pub id: u64,
    /// The the object type, which will either be boardgame or expansion.
    #[serde(rename = "subtype")]
    pub game_type: GameType,
    // TODO make sure this actually works because this is technically a list
    // of different languages, tagged with primary/alternate
    // Possibly make a list of LanguageName structs and then manually set
    // the name value to the primary afterwards
    pub name: String,
    /// The year the game was first published.
    #[serde(rename = "yearpublished")]
    pub year_published: i64,
    /// Status of the game in this collection, such as own, preowned, wishlist...
    pub status: CollectionGameStatus,
    /// Game stats such as number of players, can sometimes be omitted from the result.
    pub stats: Option<CollectionGameStats>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum GameType {
    #[serde(rename = "boardgame")]
    BoardGame,
    #[serde(rename = "boardgameexpansion")]
    BoardGameExpansion,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CollectionGameStatus {
    #[serde(deserialize_with = "deserialize_1_0_bool")]
    pub own: bool,
    #[serde(deserialize_with = "deserialize_1_0_bool")]
    pub wishlist: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CollectionGameStats {
    #[serde(rename = "minplayers")]
    pub min_players: u32,
    #[serde(rename = "maxplayers")]
    pub max_players: u32,
}

pub struct CollectionApi<'api> {
    pub(crate) api: &'api BoardGameGeekAPI,
    endpoint: &'api str,
}

pub struct CollectionQuery<'q> {
    username: &'q str,
    include_owned: Option<bool>,
    include_wishlist: Option<bool>,
    include_stats: Option<bool>,
}

impl<'a> CollectionQuery<'a> {
    pub fn new(username: &'a str) -> Self {
        Self {
            username,
            include_owned: None,
            include_wishlist: None,
            include_stats: None,
        }
    }

    pub fn owned(mut self, include_owned: bool) -> Self {
        self.include_owned = Some(include_owned);
        self
    }

    pub fn wishlist(mut self, include_wishlist: bool) -> Self {
        self.include_wishlist = Some(include_wishlist);
        self
    }

    pub fn stats(mut self, include_stats: bool) -> Self {
        self.include_stats = Some(include_stats);
        self
    }

    pub fn build(self) -> Vec<(&'a str, &'a str)> {
        let mut query_params: Vec<_> = vec![];
        query_params.push(("username", self.username));

        match self.include_owned {
            Some(true) => query_params.push(("own", "1")),
            Some(false) => query_params.push(("own", "0")),
            None => {}
        }
        match self.include_wishlist {
            Some(true) => query_params.push(("wishlist", "1")),
            Some(false) => query_params.push(("wishlist", "0")),
            None => {}
        }
        match self.include_stats {
            Some(true) => query_params.push(("stats", "1")),
            Some(false) => query_params.push(("stats", "0")),
            // When omitted, the API has inconsistent behaviour, and will include the stats usually
            // but not when specific game types are requested, so we set it to true for consistency.
            None => query_params.push(("stats", "1")),
        }
        query_params
    }
}

impl<'api> CollectionApi<'api> {
    pub fn new(api: &'api BoardGameGeekAPI) -> Self {
        Self {
            api,
            endpoint: "collection",
        }
    }

    pub fn get_owned(
        &self,
        username: &str,
    ) -> impl Future<Output = Result<Collection, reqwest::Error>> + 'api {
        let query = CollectionQuery::new(username).owned(true);
        let request = self.api.build_request(self.endpoint, &query.build());
        let future = self.api.execute_request::<Collection>(request);
        future
    }

    pub fn get_wishlist(
        &self,
        username: &str,
    ) -> impl Future<Output = Result<Collection, reqwest::Error>> + 'api {
        let query = CollectionQuery::new(username).wishlist(true);
        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<Collection>(request)
    }

    pub fn get_from_query(
        &self,
        query: CollectionQuery,
    ) -> impl Future<Output = Result<Collection, reqwest::Error>> + 'api {
        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<Collection>(request)
    }
}
