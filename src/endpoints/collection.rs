use serde::Deserialize;
use std::future::Future;

use crate::api::BoardGameGeekApi;
use crate::utils::deserialize_1_0_bool;
use crate::Result;

/// A user's collection on boardgamegeek.
#[derive(Clone, Debug, Deserialize)]
pub struct Collection {
    /// List of games and expansions in the user's collection. Each item
    /// is not necessarily owned but can be preowned, wishlisted etc.
    #[serde(rename = "$value")]
    pub games: Vec<CollectionGame>,
}

/// A game or game expansion in a collection.
#[derive(Clone, Debug, Deserialize)]
pub struct CollectionGame {
    /// The ID of the game.
    #[serde(rename = "objectid")]
    pub id: u64,
    /// The type of game, which will either be boardgame or expansion.
    #[serde(rename = "subtype")]
    pub game_type: GameType,
    /// The name of the game.
    pub name: String,
    /// The year the game was first published.
    #[serde(rename = "yearpublished")]
    pub year_published: i64,
    /// Status of the game in this collection, such as own, preowned, wishlist.
    pub status: CollectionGameStatus,
    /// Game stats such as number of players, can sometimes be omitted from the result.
    pub stats: Option<CollectionGameStats>,
}

/// The type of game, board game or expansion.
#[derive(Clone, Debug, Deserialize)]
pub enum GameType {
    /// A board game.
    #[serde(rename = "boardgame")]
    BoardGame,
    /// A board game expansion.
    #[serde(rename = "boardgameexpansion")]
    BoardGameExpansion,
}

/// The status of the game in the user's collection, such as preowned or wishlist.
/// Can be any or none of them.
#[derive(Clone, Debug, Deserialize)]
pub struct CollectionGameStatus {
    /// User owns the game.
    #[serde(deserialize_with = "deserialize_1_0_bool")]
    pub own: bool,
    /// User has the game on their wishlist.
    #[serde(deserialize_with = "deserialize_1_0_bool")]
    pub wishlist: bool,
}

/// Stats of the game such as playercount and duration. Can be omitted from the response.
/// More stats can be found from the specific game endpoint.
#[derive(Clone, Debug, Deserialize)]
pub struct CollectionGameStats {
    /// Minimum players the game supports.
    #[serde(rename = "minplayers")]
    pub min_players: u32,
    /// Maximum players the game supports.
    #[serde(rename = "maxplayers")]
    pub max_players: u32,
}

/// Struct for building a query for the request to the collection endpoint.
pub struct CollectionQueryBuilder<'q> {
    username: &'q str,
    include_owned: Option<bool>,
    include_wishlist: Option<bool>,
    include_stats: Option<bool>,
}

impl<'a> CollectionQueryBuilder<'a> {
    /// Constructs a new query builder from a name, which is a required parmeter.
    /// Sets all other fields to None.
    pub fn new(username: &'a str) -> Self {
        Self {
            username,
            include_owned: None,
            include_wishlist: None,
            include_stats: None,
        }
    }

    /// Sets the include_owned field. If true the result will include items that
    /// the user owns. Unless all status fields are kept at None, then they are all included.
    pub fn owned(mut self, include_owned: bool) -> Self {
        self.include_owned = Some(include_owned);
        self
    }

    /// Sets the include_wishlist field. If true the result will include the items
    /// that the user has on their wishlist. Unless all status fields are kept at None, then they are all included.
    pub fn wishlist(mut self, include_wishlist: bool) -> Self {
        self.include_wishlist = Some(include_wishlist);
        self
    }

    /// Sets the include_stats field. If false the stats are omitted.
    /// Since the default behaviour is inconsistent. Keeping this at None will
    /// be treated as true at build time.
    pub fn stats(mut self, include_stats: bool) -> Self {
        self.include_stats = Some(include_stats);
        self
    }

    /// Converts the fields into a vector of (&str, &str) tuples that match
    /// the expected query parameter key value pairs.
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

/// Collection endpoint of the API. Used for returning user's collections
/// of games by their username. Filtering by [CollectionGameStatus], rating, recorded plays.
pub struct CollectionApi<'api> {
    pub(crate) api: &'api BoardGameGeekApi<'api>,
    endpoint: &'api str,
}

impl<'api> CollectionApi<'api> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "collection",
        }
    }

    /// Gets all the games that a given user owns.
    pub fn get_owned(&self, username: &str) -> impl Future<Output = Result<Collection>> + 'api {
        let query = CollectionQueryBuilder::new(username).owned(true);
        let request = self.api.build_request(self.endpoint, &query.build());
        let future = self.api.execute_request::<Collection>(request);
        future
    }

    /// Gets all the games that a given user has on their wishlist.
    pub fn get_wishlist(&self, username: &str) -> impl Future<Output = Result<Collection>> + 'api {
        let query = CollectionQueryBuilder::new(username).wishlist(true);
        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<Collection>(request)
    }

    /// Makes a request from a [CollectionQueryBuilder].
    pub fn get_from_query(
        &self,
        query: CollectionQueryBuilder,
    ) -> impl Future<Output = Result<Collection>> + 'api {
        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<Collection>(request)
    }
}
