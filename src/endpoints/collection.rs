use std::ops::RangeInclusive;

use chrono::NaiveDate;
use serde::de::DeserializeOwned;

use crate::api::BoardGameGeekApi;
use crate::{
    Collection, CollectionGame, CollectionGameBrief, CollectionGameRatingBrief,
    CollectionGameStatsBrief, GameType, Result, WishlistPriority,
};

/// Trait for a type that the collection endpoint can return. Allows us to get
/// values for the mandatory query params for the different types.
pub trait CollectionGameType<'a>: DeserializeOwned {
    /// Returns the values for the mandatory query params. This ensures that
    /// for the brief type, the `brief` query param is always set to true, and
    /// vice versa.
    fn base_query(username: &'a str) -> BaseCollectionQuery<'a>;

    /// Get the stats of the type, so post processing helper functions
    /// can be written.
    fn get_stats(&self) -> CollectionGameStatsBrief;
}

impl<'a> CollectionGameType<'a> for CollectionGameBrief {
    fn base_query(username: &'a str) -> BaseCollectionQuery<'a> {
        BaseCollectionQuery {
            username,
            brief: true,
        }
    }

    fn get_stats(&self) -> CollectionGameStatsBrief {
        self.stats.clone()
    }
}

impl<'a> CollectionGameType<'a> for CollectionGame {
    fn base_query(username: &'a str) -> BaseCollectionQuery<'a> {
        BaseCollectionQuery {
            username,
            brief: false,
        }
    }

    fn get_stats(&self) -> CollectionGameStatsBrief {
        CollectionGameStatsBrief {
            min_players: self.stats.min_players,
            max_players: self.stats.max_players,
            min_playtime: self.stats.min_playtime,
            max_playtime: self.stats.max_playtime,
            playing_time: self.stats.playing_time,
            owned_by: self.stats.owned_by,
            rating: CollectionGameRatingBrief {
                user_rating: self.stats.rating.user_rating,
                average: self.stats.rating.average,
                bayesian_average: self.stats.rating.bayesian_average,
            },
        }
    }
}

/// Required query paramters. Any type the collection query can implement
/// must be able to return a base query, so valid queries can be constructed
/// for both [CollectionGame] and [CollectionGameBrief].
#[derive(Clone, Debug)]
pub struct BaseCollectionQuery<'q> {
    pub(crate) username: &'q str,
    pub(crate) brief: bool,
}

/// All optional query parameters for making a request to the
/// collection endpoint.
///
/// By default if all status options are excluded then they are all returned.
/// If any are set to true then any excluded will not be returned.
#[derive(Clone, Debug, Default)]
pub struct CollectionQueryParams {
    /// Include only results for this game type.
    ///
    /// Note, if this is set to [GameType::BoardGame] then it will include both
    /// board games and expansions, but set the type of all of them to be
    /// [GameType::BoardGame] in the results. Explicitly exclude
    /// [GameType::BoardGameExpansion] to avoid this.
    game_type: Option<GameType>,
    /// Exclude results for this game type.
    exclude_game_type: Option<GameType>,
    /// Include games the user owns if true, exclude if false.
    include_owned: Option<bool>,
    /// Include games the user previously owned if true, exclude if false.
    include_previously_owned: Option<bool>,
    /// Include games the user wants to trade away if true, exclude if false.
    include_for_trade: Option<bool>,
    /// Include games the user wants in a trade if true, exclude if false.
    include_want_in_trade: Option<bool>,
    /// Include games the user wants to play if true, exclude if false.
    include_want_to_play: Option<bool>,
    /// Include games the user wants to buy if true, exclude if false.
    include_want_to_buy: Option<bool>,
    /// Include games the user has preordered if true, exclude if false.
    include_preordered: Option<bool>,
    /// Include games the user has on their wishlist if true, exclude if false.
    include_wishlist: Option<bool>,
    /// Only include games for this wishlist priority.
    wishlist_priority: Option<WishlistPriority>,
    /// Only include games modified since this date time.
    modified_since: Option<NaiveDate>,
    /// Include only games that have been rated by the user.
    include_rated_by_user: Option<bool>,
    /// Include only games that have been played by the user.
    include_played_by_user: Option<bool>,
    /// Include only games that have been commented on by the user.
    include_commented: Option<bool>,
    /// Include only games that have a comment in the `Has Parts` field.
    has_parts: Option<bool>,
    /// Include only games that have a comment in the `Want Parts` field.
    want_parts: Option<bool>,
    /// Include only games that the user has rated at least this value.
    min_rating: Option<f32>,
    /// Include only games that the user has rated at most this value.
    max_rating: Option<f32>,
    /// Include only games that have a Geek rating of at least this value.
    min_bgg_rating: Option<f32>,
    /// Include only games that have a Geek rating of at most this value.
    max_bgg_rating: Option<f32>,
    /// Include only games that the user has played at least this many times.
    min_plays: Option<u64>,
    /// Include only games that the user has played at most this many times.
    max_plays: Option<u64>,
    /// Show private collection info. Only works when viewing your own
    /// collection and you are logged in.
    show_private: Option<bool>,
    /// ID of a particular game in a collection.
    collection_id: Option<u64>,
}

impl CollectionQueryParams {
    /// Constructs a collection query with parameters set to None.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the game_type field, so that only that type of game will be
    /// returned.
    pub fn game_type(mut self, game_type: GameType) -> Self {
        self.game_type = Some(game_type);
        self
    }

    /// Set the exclude_game_type field, so that that type of game will be
    /// excluded from. the results.
    pub fn exclude_game_type(mut self, exclude_game_type: GameType) -> Self {
        self.exclude_game_type = Some(exclude_game_type);
        self
    }

    /// Sets the include_owned field. If true the result will include games that
    /// the user owns. Unless all status fields are kept at None, then they are
    /// all included.
    pub fn include_owned(mut self, include_owned: bool) -> Self {
        self.include_owned = Some(include_owned);
        self
    }

    /// Sets the include_previously_owned field. If true the result will include
    /// games that the user owns. Unless all status fields are kept at None,
    /// then they are all included.
    pub fn include_previously_owned(mut self, include_previously_owned: bool) -> Self {
        self.include_previously_owned = Some(include_previously_owned);
        self
    }

    /// Sets the include_for_trade field. If true the result will include games
    /// that the user wants to trade away. Unless all status fields are kept
    /// at None, then they are all included.
    pub fn include_for_trade(mut self, include_for_trade: bool) -> Self {
        self.include_for_trade = Some(include_for_trade);
        self
    }

    /// Sets the include_want_in_trade field. If true the result will include
    /// games that the user wants to receive in a trade. Unless all status
    /// fields are kept at None, then they are all included.
    pub fn include_want_in_trade(mut self, include_want_in_trade: bool) -> Self {
        self.include_want_in_trade = Some(include_want_in_trade);
        self
    }

    /// Sets the include_want_to_play field. If true the result will include
    /// games that the user wants to play. Unless all status fields are kept
    /// at None, then they are all included.
    pub fn include_want_to_play(mut self, include_want_to_play: bool) -> Self {
        self.include_want_to_play = Some(include_want_to_play);
        self
    }

    /// Sets the include_want_to_buy field. If true the result will include
    /// games that the user wants to buy. Unless all status fields are kept
    /// at None, then they are all included.
    pub fn include_want_to_buy(mut self, include_want_to_buy: bool) -> Self {
        self.include_want_to_buy = Some(include_want_to_buy);
        self
    }

    /// Sets the include_preordered field. If true the result will include games
    /// that the user wants to buy. Unless all status fields are kept at
    /// None, then they are all included.
    pub fn include_preordered(mut self, include_preordered: bool) -> Self {
        self.include_preordered = Some(include_preordered);
        self
    }

    /// Sets the include_wishlist field. If true the result will include the
    /// games that the user has on their wishlist. Unless all status fields
    /// are kept at None, then they are all included.
    pub fn include_wishlist(mut self, include_wishlist: bool) -> Self {
        self.include_wishlist = Some(include_wishlist);
        self
    }

    /// Sets the wishlist_priority field. If set then only results with that
    /// wishlist priority will be returned.
    pub fn wishlist_priority(mut self, wishlist_priority: WishlistPriority) -> Self {
        self.wishlist_priority = Some(wishlist_priority);
        self
    }

    /// Sets the modified_since field. If set then only results that have been
    /// modified since that datetime will be returned.
    pub fn modified_since(mut self, modified_since: NaiveDate) -> Self {
        self.modified_since = Some(modified_since);
        self
    }

    /// Sets the include_rated_by_user field. If set then only results that
    /// the user has rated will be returned.
    pub fn include_rated_by_user(mut self, include_rated_by_user: bool) -> Self {
        self.include_rated_by_user = Some(include_rated_by_user);
        self
    }

    /// Sets the include_played_by_user field. If set then only results that
    /// the user has marked as having played will be returned.
    pub fn include_played_by_user(mut self, include_played_by_user: bool) -> Self {
        self.include_played_by_user = Some(include_played_by_user);
        self
    }

    /// Sets the include_commented field. If set then only results that
    /// the user has commented on will be returned.
    pub fn include_commented(mut self, include_commented: bool) -> Self {
        self.include_commented = Some(include_commented);
        self
    }

    /// Sets the has_parts field. If set then only results that
    /// the user has commented on the `Has Parts` field.
    pub fn has_parts(mut self, has_parts: bool) -> Self {
        self.has_parts = Some(has_parts);
        self
    }

    /// Sets the want_parts field. If set then only results that
    /// the user has commented on the `Want Parts` field.
    pub fn want_parts(mut self, want_parts: bool) -> Self {
        self.want_parts = Some(want_parts);
        self
    }

    /// Sets the min_rating field. If set then only results that
    /// the user has rated equal or greater than that value will be returned.
    pub fn min_rating(mut self, min_rating: f32) -> Self {
        self.min_rating = Some(min_rating);
        self
    }

    /// Sets the max_rating field. If set then only results that
    /// the user has rated equal or less than that value will be returned.
    pub fn max_rating(mut self, max_rating: f32) -> Self {
        self.max_rating = Some(max_rating);
        self
    }

    /// Sets the min_bgg_rating field. If set then only results that
    /// have a "Geek rating" equal or greater than that value will be returned.
    pub fn min_bgg_rating(mut self, min_bgg_rating: f32) -> Self {
        self.min_bgg_rating = Some(min_bgg_rating);
        self
    }

    /// Sets the max_bgg_rating field. If set then only results that
    /// have a "Geek rating" equal or less than that value will be returned.
    pub fn max_bgg_rating(mut self, max_bgg_rating: f32) -> Self {
        self.max_bgg_rating = Some(max_bgg_rating);
        self
    }

    /// Sets the max_plays field. If set then only results that
    /// have this many recorded plays by the user or more will be returned.
    pub fn min_plays(mut self, min_plays: u64) -> Self {
        self.min_plays = Some(min_plays);
        self
    }

    /// Sets the max_plays field. If set then only results that
    /// have this many recorded plays by the user or less will be returned.
    pub fn max_plays(mut self, max_plays: u64) -> Self {
        self.max_plays = Some(max_plays);
        self
    }

    /// Sets the show_private field. If set then private information about
    /// the collection will be returned. Only works if the user is logged in
    /// and requesting their own collection.
    pub fn show_private(mut self, show_private: bool) -> Self {
        self.show_private = Some(show_private);
        self
    }

    /// Sets the collection_id field. If set then results will be filtered
    /// to get the game with the specific collection ID.
    pub fn collection_id(mut self, collection_id: u64) -> Self {
        self.collection_id = Some(collection_id);
        self
    }
}

/// Struct for building a query for the request to the collection endpoint.
#[derive(Clone, Debug)]
struct CollectionQueryBuilder<'q> {
    base: BaseCollectionQuery<'q>,
    params: CollectionQueryParams,
}

impl<'a> CollectionQueryBuilder<'a> {
    /// Constructs a new query builder from a base query, and the rest of the
    /// parameters.
    fn new(base: BaseCollectionQuery<'a>, params: CollectionQueryParams) -> Self {
        Self { base, params }
    }

    /// Converts the fields into a vector of (&str, &str) tuples that match
    /// the expected query parameter key value pairs.
    pub fn build(self) -> Vec<(&'a str, String)> {
        let mut query_params: Vec<_> = vec![];
        query_params.push(("username", self.base.username.to_string()));
        // The API is inconsistent with whether stats are returned or not when this is
        // omitted. Set it to always true to avoid any problems with this and
        // avoid the need for the type to be an optional.
        query_params.push(("stats", "1".to_string()));

        match self.base.brief {
            true => query_params.push(("brief", "1".to_string())),
            false => query_params.push(("brief", "0".to_string())),
        }
        match self.params.game_type {
            Some(GameType::BoardGame) => query_params.push(("subtype", "boardgame".to_string())),
            Some(GameType::BoardGameExpansion) => {
                query_params.push(("subtype", "boardgameexpansion".to_string()))
            },
            None => {},
        }
        match self.params.exclude_game_type {
            Some(GameType::BoardGame) => {
                query_params.push(("excludesubtype", "boardgame".to_string()))
            },
            Some(GameType::BoardGameExpansion) => {
                query_params.push(("excludesubtype", "boardgameexpansion".to_string()))
            },
            None => {},
        }
        match self.params.include_owned {
            Some(true) => query_params.push(("own", "1".to_string())),
            Some(false) => query_params.push(("own", "0".to_string())),
            None => {},
        }
        match self.params.include_previously_owned {
            Some(true) => query_params.push(("prevowned", "1".to_string())),
            Some(false) => query_params.push(("prevowned", "0".to_string())),
            None => {},
        }
        match self.params.include_for_trade {
            Some(true) => query_params.push(("trade", "1".to_string())),
            Some(false) => query_params.push(("trade", "0".to_string())),
            None => {},
        }
        match self.params.include_want_in_trade {
            Some(true) => query_params.push(("want", "1".to_string())),
            Some(false) => query_params.push(("want", "0".to_string())),
            None => {},
        }
        match self.params.include_want_to_play {
            Some(true) => query_params.push(("wanttoplay", "1".to_string())),
            Some(false) => query_params.push(("wanttoplay", "0".to_string())),
            None => {},
        }
        match self.params.include_want_to_buy {
            Some(true) => query_params.push(("wanttobuy", "1".to_string())),
            Some(false) => query_params.push(("wanttobuy", "0".to_string())),
            None => {},
        }
        match self.params.include_preordered {
            Some(true) => query_params.push(("preordered", "1".to_string())),
            Some(false) => query_params.push(("preordered", "0".to_string())),
            None => {},
        }
        match self.params.include_wishlist {
            Some(true) => query_params.push(("wishlist", "1".to_string())),
            Some(false) => query_params.push(("wishlist", "0".to_string())),
            None => {},
        }
        match self.params.wishlist_priority {
            Some(WishlistPriority::DontBuyThis) => {
                query_params.push(("wishlistpriority", "5".to_string()))
            },
            Some(WishlistPriority::ThinkingAboutIt) => {
                query_params.push(("wishlistpriority", "4".to_string()))
            },
            Some(WishlistPriority::LikeToHave) => {
                query_params.push(("wishlistpriority", "3".to_string()))
            },
            Some(WishlistPriority::LoveToHave) => {
                query_params.push(("wishlistpriority", "2".to_string()))
            },
            Some(WishlistPriority::MustHave) => {
                query_params.push(("wishlistpriority", "1".to_string()))
            },
            None => {},
        }
        if let Some(modified_since) = self.params.modified_since {
            query_params.push((
                "modifiedsince",
                modified_since.format("YY-MM-DD").to_string(),
            ));
        }
        match self.params.include_rated_by_user {
            Some(true) => query_params.push(("rated", "1".to_string())),
            Some(false) => query_params.push(("rated", "0".to_string())),
            None => {},
        }
        match self.params.include_played_by_user {
            Some(true) => query_params.push(("played", "1".to_string())),
            Some(false) => query_params.push(("played", "0".to_string())),
            None => {},
        }
        match self.params.include_commented {
            Some(true) => query_params.push(("comment", "1".to_string())),
            Some(false) => query_params.push(("comment", "0".to_string())),
            None => {},
        }
        match self.params.has_parts {
            Some(true) => query_params.push(("hasparts", "1".to_string())),
            Some(false) => query_params.push(("hasparts", "0".to_string())),
            None => {},
        }
        match self.params.want_parts {
            Some(true) => query_params.push(("wantparts", "1".to_string())),
            Some(false) => query_params.push(("wantparts", "0".to_string())),
            None => {},
        }
        if let Some(min_rating) = self.params.min_rating {
            query_params.push(("minrating", min_rating.to_string()));
        }
        if let Some(max_rating) = self.params.max_rating {
            query_params.push(("rating", max_rating.to_string()));
        }
        if let Some(min_bgg_rating) = self.params.min_bgg_rating {
            query_params.push(("minbggrating", min_bgg_rating.to_string()));
        }
        if let Some(max_bgg_rating) = self.params.max_bgg_rating {
            query_params.push(("bggrating", max_bgg_rating.to_string()));
        }
        if let Some(min_plays) = self.params.min_plays {
            query_params.push(("minplays", min_plays.to_string()));
        }
        if let Some(max_plays) = self.params.max_plays {
            query_params.push(("maxplays", max_plays.to_string()));
        }
        match self.params.show_private {
            Some(true) => query_params.push(("showprivate", "1".to_string())),
            Some(false) => query_params.push(("showprivate", "0".to_string())),
            None => {},
        }
        if let Some(collection_id) = self.params.collection_id {
            query_params.push(("collid", collection_id.to_string()));
        }
        query_params
    }
}

/// Collection endpoint of the API. Used for returning user's collections
/// of games by their username. Filtering by [CollectionGameStatus], rating,
/// recorded plays.
pub struct CollectionApi<'api, T: CollectionGameType<'api>> {
    pub(crate) api: &'api BoardGameGeekApi,
    endpoint: &'static str,
    type_marker: std::marker::PhantomData<T>,
}

impl<'api, T: CollectionGameType<'api> + 'api> CollectionApi<'api, T> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "collection",
            type_marker: std::marker::PhantomData,
        }
    }

    /// Get all games of all types in the user's collection.
    pub async fn get_all(&self, username: &'api str) -> Result<Collection<T>> {
        let query_params = CollectionQueryParams::default();
        self.get_from_query(username, query_params).await
    }

    /// Gets all the games that a given user owns.
    pub async fn get_owned(&self, username: &'api str) -> Result<Collection<T>> {
        let query_params = CollectionQueryParams::new().include_owned(true);
        self.get_from_query(username, query_params).await
    }

    /// Gets all the games that a given user has on their wishlist.
    pub async fn get_wishlist(&self, username: &'api str) -> Result<Collection<T>> {
        let query_params = CollectionQueryParams::new().include_wishlist(true);
        self.get_from_query(username, query_params).await
    }

    /// Gets all the games that support any player counts in a given range.
    /// The include_stats parameter is automatically set to true, as it is
    /// needed to filter the results.
    pub async fn get_by_player_counts(
        &self,
        username: &'api str,
        player_counts: RangeInclusive<u32>,
        query_params: CollectionQueryParams,
    ) -> Result<Collection<T>> {
        let mut collection = self.get_from_query(username, query_params).await?;

        collection.games.retain(|game| {
            let stats = game.get_stats();
            *player_counts.start() <= stats.max_players && *player_counts.end() >= stats.min_players
        });
        Ok(collection)
    }

    /// Gets all the games that support the given player count.
    /// The include_stats parameter is automatically set to true, as it is
    /// needed to filter the results.
    pub async fn get_by_player_count(
        &self,
        username: &'api str,
        player_count: u32,
        query_params: CollectionQueryParams,
    ) -> Result<Collection<T>> {
        let mut collection = self.get_from_query(username, query_params).await?;

        collection.games.retain(|game| {
            let stats = game.get_stats();
            player_count <= stats.max_players && player_count >= stats.min_players
        });
        Ok(collection)
    }

    /// Makes a request from a [CollectionQueryParams].
    pub async fn get_from_query(
        &self,
        username: &'api str,
        query_params: CollectionQueryParams,
    ) -> Result<Collection<T>> {
        let query = CollectionQueryBuilder::new(T::base_query(username), query_params);

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<Collection<T>>(request).await
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, TimeZone, Utc};
    use mockito::Matcher;

    use super::*;
    use crate::{
        CollectionGameRating, CollectionGameStats, CollectionGameStatus, GameFamilyRank,
        GameFamilyType, RankValue,
    };

    #[test]
    fn sort_wishlist_priority() {
        assert!(WishlistPriority::DontBuyThis < WishlistPriority::MustHave);
    }

    #[tokio::test]
    async fn get_owned_brief() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "somename".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
                Matcher::UrlEncoded("brief".into(), "1".into()),
                Matcher::UrlEncoded("own".into(), "1".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_brief_owned_single.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let collection = api.collection_brief().get_owned("somename").await;
        mock.assert_async().await;

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.games.len(), 1);
        assert_eq!(
            collection.games[0],
            CollectionGameBrief {
                id: 131835,
                collection_id: 118278872,
                game_type: GameType::BoardGame,
                name: "Boss Monster: The Dungeon Building Card Game".to_string(),
                status: CollectionGameStatus {
                    own: true,
                    previously_owned: false,
                    for_trade: false,
                    want_in_trade: false,
                    want_to_play: false,
                    want_to_buy: false,
                    wishlist: false,
                    wishlist_priority: None,
                    pre_ordered: false,
                    last_modified: Utc.with_ymd_and_hms(2024, 4, 13, 18, 29, 1).unwrap(),
                },
                stats: CollectionGameStatsBrief {
                    min_players: 2,
                    max_players: 4,
                    min_playtime: Duration::minutes(30),
                    max_playtime: Duration::minutes(30),
                    playing_time: Duration::minutes(30),
                    owned_by: 36423,
                    rating: CollectionGameRatingBrief {
                        user_rating: Some(3.0),
                        average: 6.27139,
                        bayesian_average: 6.08972,
                    },
                },
            },
            "returned collection game doesn't match expected",
        );
    }

    #[tokio::test]
    async fn get_owned_all() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "somename".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
                Matcher::UrlEncoded("brief".into(), "0".into()),
                Matcher::UrlEncoded("own".into(), "1".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_multiple.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let collection = api.collection().get_owned("somename").await;
        mock.assert_async().await;

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.games.len(), 39);
    }

    #[tokio::test]
    async fn get_owned() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "somename".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
                Matcher::UrlEncoded("brief".into(), "0".into()),
                Matcher::UrlEncoded("own".into(), "1".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_owned_single.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let collection = api.collection().get_owned("somename").await;
        mock.assert_async().await;

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.games.len(), 1);
        assert_eq!(
            collection.games[0],
            CollectionGame {
                id: 131835,
                collection_id: 118278872,
                game_type: GameType::BoardGame,
                name: "Boss Monster: The Dungeon Building Card Game".to_string(),
                year_published: 2013,
                image: "https://cf.geekdo-images.com/VBwaHyx-NWL3VLcCWKRA0w__original/img/izAmJ81QELl5DoK3y2bzJw55lhA=/0x0/filters:format(jpeg)/pic1732644.jpg".to_string(),
                thumbnail: "https://cf.geekdo-images.com/VBwaHyx-NWL3VLcCWKRA0w__thumb/img/wisLXxKXbo5-Ci-ZjEj8ryyoN2g=/fit-in/200x150/filters:strip_icc()/pic1732644.jpg".to_string(),
                status: CollectionGameStatus {
                    own: true,
                    previously_owned: false,
                    for_trade: false,
                    want_in_trade: false,
                    want_to_play: false,
                    want_to_buy: false,
                    wishlist: false,
                    wishlist_priority: None,
                    pre_ordered: false,
                    last_modified: Utc.with_ymd_and_hms(2024, 4, 13, 18, 29, 1).unwrap(),
                },
                number_of_plays: 2,
                stats: CollectionGameStats {
                    min_players: 2,
                    max_players: 4,
                    min_playtime: Duration::minutes(30),
                    max_playtime: Duration::minutes(30),
                    playing_time: Duration::minutes(30),
                    owned_by: 36423,
                    rating: CollectionGameRating {
                        user_rating: Some(3.0),
                        users_rated: 17063,
                        average: 6.27139,
                        bayesian_average: 6.08972,
                        standard_deviation: 1.45941,
                        median: 0.0,
                        ranks: vec![
                            GameFamilyRank {
                                game_family_type: GameFamilyType::Subtype,
                                id: 1,
                                name: "boardgame".into(),
                                friendly_name: "Board Game Rank".into(),
                                value: RankValue::Ranked(2486),
                                bayesian_average: 6.08972,
                            },
                            GameFamilyRank {
                                game_family_type: GameFamilyType::Family,
                                id: 5499,
                                name: "familygames".into(),
                                friendly_name: "Family Game Rank".into(),
                                value: RankValue::Ranked(1006),
                                bayesian_average: 6.05246,
                            },
                        ],
                    },
                },
            },
            "returned collection game doesn't match expected",
        );
    }

    #[tokio::test]
    async fn get_wishlist() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "somename".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
                Matcher::UrlEncoded("brief".into(), "0".into()),
                Matcher::UrlEncoded("wishlist".into(), "1".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_wishlist_single.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let collection = api.collection().get_wishlist("somename").await;
        mock.assert_async().await;

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.games.len(), 1);
        assert_eq!(
            collection.games[0],
            CollectionGame {
                id: 177736,
                collection_id: 118332974,
                game_type: GameType::BoardGame,
                name: "A Feast for Odin".to_string(),
                year_published: 2016,
                image: "https://domain/img.jpg".to_string(),
                thumbnail: "https://domain/thumbnail.jpg".to_string(),
                status: CollectionGameStatus {
                    own: false,
                    previously_owned: false,
                    for_trade: false,
                    want_in_trade: true,
                    want_to_play: false,
                    want_to_buy: false,
                    wishlist: true,
                    wishlist_priority: Some(WishlistPriority::LoveToHave),
                    pre_ordered: false,
                    last_modified: Utc.with_ymd_and_hms(2024, 4, 18, 19, 28, 17).unwrap(),
                },
                number_of_plays: 0,
                stats: CollectionGameStats {
                    min_players: 1,
                    max_players: 4,
                    min_playtime: Duration::minutes(30),
                    max_playtime: Duration::minutes(120),
                    playing_time: Duration::minutes(120),
                    owned_by: 37542,
                    rating: CollectionGameRating {
                        user_rating: None,
                        users_rated: 28890,
                        average: 8.17156,
                        bayesian_average: 7.94347,
                        standard_deviation: 1.37019,
                        median: 0.0,
                        ranks: vec![
                            GameFamilyRank {
                                game_family_type: GameFamilyType::Subtype,
                                id: 1,
                                name: "boardgame".into(),
                                friendly_name: "Board Game Rank".into(),
                                value: RankValue::Ranked(23),
                                bayesian_average: 7.94347,
                            },
                            GameFamilyRank {
                                game_family_type: GameFamilyType::Family,
                                id: 5497,
                                name: "strategygames".into(),
                                friendly_name: "Strategy Game Rank".into(),
                                value: RankValue::Ranked(19),
                                bayesian_average: 7.97338,
                            },
                        ],
                    },
                },
            },
            "returned collection game doesn't match expected",
        );
    }

    #[tokio::test]
    async fn get_from_query() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "someone".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
                Matcher::UrlEncoded("brief".into(), "0".into()),
                Matcher::UrlEncoded("hasparts".into(), "0".into()),
                Matcher::UrlEncoded("own".into(), "1".into()),
                Matcher::UrlEncoded("minplays".into(), "14".into()),
                Matcher::UrlEncoded("wishlist".into(), "1".into()),
                Matcher::UrlEncoded("wishlistpriority".into(), "5".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_owned_single.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let query = CollectionQueryParams::new()
            .has_parts(false)
            .include_owned(true)
            .min_plays(14)
            .include_wishlist(true)
            .wishlist_priority(WishlistPriority::DontBuyThis);

        let _ = api.collection().get_from_query("someone", query).await;
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn get_by_player_counts() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "someone".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
                Matcher::UrlEncoded("brief".into(), "0".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_owned_with_stats.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let collection = api
            .collection()
            .get_by_player_counts("someone", 16..=17, CollectionQueryParams::new())
            .await;
        mock.assert_async().await;

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.games.len(), 1);
        assert_eq!(
            collection.games[0],
            CollectionGame {
                id: 2281,
                collection_id: 118280658,
                game_type: GameType::BoardGame,
                name: "Pictionary".to_string(),
                year_published: 1985,
                image: "https://cf.geekdo-images.com/YfUxodD7JSqYitxvjXB69Q__original/img/YRJAlLzkxMuJHVPsdnBLNFpoODA=/0x0/filters:format(png)/pic5147022.png".to_string(),
                thumbnail: "https://cf.geekdo-images.com/YfUxodD7JSqYitxvjXB69Q__thumb/img/7ls1a8ak5oT7BaKM-rVHpOVrP14=/fit-in/200x150/filters:strip_icc()/pic5147022.png".to_string(),
                status: CollectionGameStatus {
                    own: true,
                    previously_owned: false,
                    for_trade: false,
                    want_in_trade: false,
                    want_to_play: false,
                    want_to_buy: false,
                    wishlist: false,
                    wishlist_priority: None,
                    pre_ordered: false,
                    last_modified: Utc.with_ymd_and_hms(2024, 4, 14, 9, 47, 38).unwrap(),
                },
                number_of_plays: 0,
                stats: CollectionGameStats {
                    min_players: 3,
                    max_players: 16,
                    min_playtime: Duration::minutes(90),
                    max_playtime: Duration::minutes(90),
                    playing_time: Duration::minutes(90),
                    owned_by: 14400,
                    rating: CollectionGameRating {
                        user_rating: Some(7.0),
                        users_rated: 8097,
                        average: 5.84098,
                        bayesian_average: 5.71005,
                        standard_deviation: 1.58457,
                        median: 0.0,
                        ranks: vec![
                            GameFamilyRank {
                                game_family_type: GameFamilyType::Subtype,
                                id: 1,
                                name: "boardgame".into(),
                                friendly_name: "Board Game Rank".into(),
                                value: RankValue::Ranked(5587),
                                bayesian_average: 5.71005,
                            },
                            GameFamilyRank {
                                game_family_type: GameFamilyType::Family,
                                id: 5498,
                                name: "partygames".into(),
                                friendly_name: "Party Game Rank".into(),
                                value: RankValue::Ranked(563),
                                bayesian_average: 5.65053,
                            }
                        ],
                    }
                },
            },
            "returned collection game doesn't match expected",
        );

        // Looking for a game that supports any number of players between 1 and 16. All
        // 37 games in the collection should be returned.
        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "someone".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
                Matcher::UrlEncoded("brief".into(), "0".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_owned_with_stats.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let collection = api
            .collection()
            .get_by_player_counts("someone", 1..=16, CollectionQueryParams::new())
            .await;
        mock.assert_async().await;

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.games.len(), 37);

        // Looking for a game that supports 17 players, not in the collection. Nothing
        // should be returned.
        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "someone".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
                Matcher::UrlEncoded("brief".into(), "0".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_owned_with_stats.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let collection = api
            .collection()
            .get_by_player_counts("someone", 17..=17, CollectionQueryParams::new())
            .await;
        mock.assert_async().await;

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.games.len(), 0);
    }

    #[tokio::test]
    async fn get_by_player_count() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "someone".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
                Matcher::UrlEncoded("brief".into(), "0".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_owned_with_stats.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let collection = api
            .collection()
            .get_by_player_count("someone", 16, CollectionQueryParams::new())
            .await;
        mock.assert_async().await;

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.games.len(), 1);
        assert_eq!(
            collection.games[0],
            CollectionGame {
                id: 2281,
                collection_id: 118280658,
                game_type: GameType::BoardGame,
                name: "Pictionary".to_string(),
                year_published: 1985,
                image: "https://cf.geekdo-images.com/YfUxodD7JSqYitxvjXB69Q__original/img/YRJAlLzkxMuJHVPsdnBLNFpoODA=/0x0/filters:format(png)/pic5147022.png".to_string(),
                thumbnail: "https://cf.geekdo-images.com/YfUxodD7JSqYitxvjXB69Q__thumb/img/7ls1a8ak5oT7BaKM-rVHpOVrP14=/fit-in/200x150/filters:strip_icc()/pic5147022.png".to_string(),
                status: CollectionGameStatus {
                    own: true,
                    previously_owned: false,
                    for_trade: false,
                    want_in_trade: false,
                    want_to_play: false,
                    want_to_buy: false,
                    wishlist: false,
                    wishlist_priority: None,
                    pre_ordered: false,
                    last_modified: Utc.with_ymd_and_hms(2024, 4, 14, 9, 47, 38).unwrap(),
                },
                number_of_plays: 0,
                stats: CollectionGameStats {
                    min_players: 3,
                    max_players: 16,
                    min_playtime: Duration::minutes(90),
                    max_playtime: Duration::minutes(90),
                    playing_time: Duration::minutes(90),
                    owned_by: 14400,
                    rating: CollectionGameRating {
                        user_rating: Some(7.0),
                        users_rated: 8097,
                        average: 5.84098,
                        bayesian_average: 5.71005,
                        standard_deviation: 1.58457,
                        median: 0.0,
                        ranks: vec![
                            GameFamilyRank {
                                game_family_type: GameFamilyType::Subtype,
                                id: 1,
                                name: "boardgame".into(),
                                friendly_name: "Board Game Rank".into(),
                                value: RankValue::Ranked(5587),
                                bayesian_average: 5.71005,
                            },
                            GameFamilyRank {
                                game_family_type: GameFamilyType::Family,
                                id: 5498,
                                name: "partygames".into(),
                                friendly_name: "Party Game Rank".into(),
                                value: RankValue::Ranked(563),
                                bayesian_average: 5.65053,
                            }
                        ],
                    }
                },
            },
            "returned collection game doesn't match expected",
        );

        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "someone".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
                Matcher::UrlEncoded("brief".into(), "0".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_owned_with_stats.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let collection = api
            .collection()
            .get_by_player_count("someone", 2, CollectionQueryParams::new())
            .await;
        mock.assert_async().await;

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.games.len(), 30);
        for game in collection.games {
            assert!(game.stats.min_players <= 2 && game.stats.max_players >= 2)
        }

        // Looking for a game that supports 17 players, not in the collection. Nothing
        // should be returned.
        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "someone".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
                Matcher::UrlEncoded("brief".into(), "0".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_owned_with_stats.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let collection = api
            .collection()
            .get_by_player_count("someone", 17, CollectionQueryParams::new())
            .await;
        mock.assert_async().await;

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.games.len(), 0);
    }
}
