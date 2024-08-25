use std::ops::RangeInclusive;

use chrono::NaiveDate;
use serde::de::DeserializeOwned;

use crate::api::BoardGameGeekApi;
use crate::{
    Collection, CollectionItem, CollectionItemBrief, CollectionItemRatingBrief,
    CollectionItemStatsBrief, ItemType, Result, WishlistPriority,
};

/// Trait for a type that the collection endpoint can return. Allows us to get
/// values for the mandatory query params for the different types.
pub trait CollectionType<'a>: DeserializeOwned {
    /// Returns the values for the mandatory query params. This ensures that
    /// for the brief type, the `brief` query param is always set to true, and
    /// vice versa.
    fn base_query(username: &'a str) -> BaseCollectionQuery<'a>;

    /// Get the stats of the type, so post processing helper functions
    /// can be written.
    fn get_stats(&self) -> CollectionItemStatsBrief;
}

impl<'a> CollectionType<'a> for CollectionItemBrief {
    fn base_query(username: &'a str) -> BaseCollectionQuery<'a> {
        BaseCollectionQuery {
            username,
            brief: true,
        }
    }

    fn get_stats(&self) -> CollectionItemStatsBrief {
        self.stats.clone()
    }
}

impl<'a> CollectionType<'a> for CollectionItem {
    fn base_query(username: &'a str) -> BaseCollectionQuery<'a> {
        BaseCollectionQuery {
            username,
            brief: false,
        }
    }

    fn get_stats(&self) -> CollectionItemStatsBrief {
        CollectionItemStatsBrief {
            min_players: self.stats.min_players,
            max_players: self.stats.max_players,
            min_playtime: self.stats.min_playtime,
            max_playtime: self.stats.max_playtime,
            playing_time: self.stats.playing_time,
            owned_by: self.stats.owned_by,
            rating: CollectionItemRatingBrief {
                user_rating: self.stats.rating.user_rating,
                average: self.stats.rating.average,
                bayesian_average: self.stats.rating.bayesian_average,
            },
        }
    }
}

/// Required query parameters. Any type the collection query can implement
/// must be able to return a base query, so valid queries can be constructed
/// for both [CollectionItem] and [CollectionItemBrief].
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
    /// Filter collection to only return items of particular IDs.
    item_ids: Vec<u64>,
    /// Include only results for this item type.
    ///
    /// Note, if this is set to [ItemType::BoardGame] then it will include both
    /// board games and expansions, but set the type of all of them to be
    /// [ItemType::BoardGame] in the results. Explicitly exclude
    /// [ItemType::BoardGameExpansion] to avoid this.
    item_type: Option<ItemType>,
    /// Exclude results for this item type.
    exclude_item_type: Option<ItemType>,
    /// Include the version information for this item, if applicable.
    include_version_info: Option<bool>,
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
    /// Include games the user has pre-ordered if true, exclude if false.
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

    /// Adds an ID to the list of item IDs to retrieve.
    pub fn item_id(mut self, id: u64) -> Self {
        self.item_ids.push(id);
        self
    }

    /// Adds a list of IDs to the list of item IDs to retrieve.
    pub fn item_ids(mut self, ids: Vec<u64>) -> Self {
        self.item_ids.extend(ids);
        self
    }

    /// Sets the item_type field, so that only that type of game will be
    /// returned.
    pub fn item_type(mut self, item_type: ItemType) -> Self {
        self.item_type = Some(item_type);
        self
    }

    /// Set the exclude_item_type field, so that that type of game will be
    /// excluded from. the results.
    pub fn exclude_item_type(mut self, exclude_item_type: ItemType) -> Self {
        self.exclude_item_type = Some(exclude_item_type);
        self
    }

    /// Sets the include_version_info field. If true the result will include version
    /// info for the games that have it. For most games this will be empty still,
    /// but it will be set for games which are an alternative or translated
    /// version of an existing game.
    pub fn include_version_info(mut self, include_version_info: bool) -> Self {
        self.include_version_info = Some(include_version_info);
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
    /// modified since that date and time will be returned.
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

    // NOTE currently private until logging in can be done via the API.
    /// Sets the show_private field. If set then private information about
    /// the collection will be returned. Only works if the user is logged in
    /// and requesting their own collection.
    #[allow(dead_code)]
    fn show_private(mut self, show_private: bool) -> Self {
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
    // Constructs a new query builder from a base query, and the rest of the
    // parameters.
    fn new(base: BaseCollectionQuery<'a>, params: CollectionQueryParams) -> Self {
        Self { base, params }
    }

    // Converts the list of parameters into a vector of
    // key value pairs that reqwest can use as HTTP query parameters.
    fn build(self) -> Vec<(&'a str, String)> {
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

        let id_list_string = self
            .params
            .item_ids
            .iter()
            .map(u64::to_string)
            .collect::<Vec<String>>()
            .join(",");
        query_params.push(("id", id_list_string));

        match self.params.item_type {
            Some(ItemType::BoardGame) => query_params.push(("subtype", "boardgame".to_string())),
            Some(ItemType::BoardGameExpansion) => {
                query_params.push(("subtype", "boardgameexpansion".to_string()))
            },
            Some(ItemType::BoardGameAccessory) => {
                query_params.push(("subtype", "boardgameaccessory".to_string()))
            },
            None => {},
        }
        match self.params.exclude_item_type {
            Some(ItemType::BoardGame) => {
                query_params.push(("excludesubtype", "boardgame".to_string()))
            },
            Some(ItemType::BoardGameExpansion) => {
                query_params.push(("excludesubtype", "boardgameexpansion".to_string()))
            },
            Some(ItemType::BoardGameAccessory) => {
                query_params.push(("excludesubtype", "boardgameaccessory".to_string()))
            },
            None => {},
        }
        match self.params.include_version_info {
            Some(true) => query_params.push(("version", "1".to_string())),
            Some(false) => query_params.push(("version", "0".to_string())),
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
                modified_since.format("%y-%m-%d").to_string(),
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
/// of games by their username. Filtering by [crate::CollectionItemStatus], rating,
/// recorded plays.
pub struct CollectionApi<'api, T: CollectionType<'api>> {
    pub(crate) api: &'api BoardGameGeekApi,
    endpoint: &'static str,
    type_marker: std::marker::PhantomData<T>,
}

impl<'api, T: CollectionType<'api> + 'api> CollectionApi<'api, T> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "collection",
            type_marker: std::marker::PhantomData,
        }
    }

    /// Makes a request to a given user's collection with no additional parameters set.
    /// This will default to including board games and board game expansions, but the
    /// [ItemType] will be set to [ItemType::BoardGame] for all results. This is a
    /// "feature" of the underlying API.
    pub async fn get_all_games(&self, username: &'api str) -> Result<Collection<T>> {
        let query_params = CollectionQueryParams::new();
        self.get_from_query(username, query_params).await
    }

    /// Get the user's board game accessory collection. Filtering by any additional
    /// query parameters provided. No board games will be returned in the collection
    /// alongside the accessories.
    pub async fn get_accessory_collection(
        &self,
        username: &'api str,
        query_params: CollectionQueryParams,
    ) -> Result<Collection<T>> {
        self.get_from_query(
            username,
            query_params.item_type(ItemType::BoardGameAccessory),
        )
        .await
    }

    /// Gets all the items in a collection that the given user owns.
    pub async fn get_owned(&self, username: &'api str) -> Result<Collection<T>> {
        let query_params = CollectionQueryParams::new().include_owned(true);
        self.get_from_query(username, query_params).await
    }

    /// Gets all the items in a collection that the given user has on their wishlist.
    pub async fn get_wishlist(&self, username: &'api str) -> Result<Collection<T>> {
        let query_params = CollectionQueryParams::new().include_wishlist(true);
        self.get_from_query(username, query_params).await
    }

    /// Gets all the games that support any player counts in a given range.
    ///
    /// Note that the minimum and maximum player count fields are not included for
    /// [ItemType::BoardGameAccessory], and will be defaulted to 0 in the result.
    pub async fn get_by_player_counts(
        &self,
        username: &'api str,
        player_counts: RangeInclusive<u32>,
        query_params: CollectionQueryParams,
    ) -> Result<Collection<T>> {
        let mut collection = self.get_from_query(username, query_params).await?;

        collection.items.retain(|items| {
            let stats = items.get_stats();
            *player_counts.start() <= stats.max_players && *player_counts.end() >= stats.min_players
        });
        Ok(collection)
    }

    /// Gets all the games that support the given player count.
    ///
    /// Note that the minimum and maximum player count fields are not included for
    /// [ItemType::BoardGameAccessory], and will be defaulted to 0 in the result.
    pub async fn get_by_player_count(
        &self,
        username: &'api str,
        player_count: u32,
        query_params: CollectionQueryParams,
    ) -> Result<Collection<T>> {
        let mut collection = self.get_from_query(username, query_params).await?;

        collection.items.retain(|items| {
            let stats = items.get_stats();
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
        CollectionItemRating, CollectionItemStats, CollectionItemStatus, Dimensions, Game,
        GameArtist, GameFamilyRank, GameFamilyType, GamePublisher, GameVersion, ItemType, Language,
        RankValue,
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

        assert_eq!(collection.items.len(), 1);
        assert_eq!(
            collection.items[0],
            CollectionItemBrief {
                id: 131835,
                collection_id: 118278872,
                item_type: ItemType::BoardGame,
                name: "Boss Monster: The Dungeon Building Card Game".to_string(),
                status: CollectionItemStatus {
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
                stats: CollectionItemStatsBrief {
                    min_players: 2,
                    max_players: 4,
                    min_playtime: Duration::minutes(30),
                    max_playtime: Duration::minutes(30),
                    playing_time: Duration::minutes(30),
                    owned_by: 36423,
                    rating: CollectionItemRatingBrief {
                        user_rating: Some(3.0),
                        average: 6.27139,
                        bayesian_average: 6.08972,
                    },
                },
                version: None,
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

        assert_eq!(collection.items.len(), 39);
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

        assert_eq!(collection.items.len(), 1);
        assert_eq!(
            collection.items[0],
            CollectionItem {
                id: 131835,
                collection_id: 118278872,
                item_type: ItemType::BoardGame,
                name: "Boss Monster: The Dungeon Building Card Game".to_string(),
                year_published: 2013,
                image: "https://cf.geekdo-images.com/VBwaHyx-NWL3VLcCWKRA0w__original/img/izAmJ81QELl5DoK3y2bzJw55lhA=/0x0/filters:format(jpeg)/pic1732644.jpg".to_string(),
                thumbnail: "https://cf.geekdo-images.com/VBwaHyx-NWL3VLcCWKRA0w__thumb/img/wisLXxKXbo5-Ci-ZjEj8ryyoN2g=/fit-in/200x150/filters:strip_icc()/pic1732644.jpg".to_string(),
                status: CollectionItemStatus {
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
                stats: CollectionItemStats {
                    min_players: 2,
                    max_players: 4,
                    min_playtime: Duration::minutes(30),
                    max_playtime: Duration::minutes(30),
                    playing_time: Duration::minutes(30),
                    owned_by: 36423,
                    rating: CollectionItemRating {
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
                version: None,
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

        assert_eq!(collection.items.len(), 1);
        assert_eq!(
            collection.items[0],
            CollectionItem {
                id: 177736,
                collection_id: 118332974,
                item_type: ItemType::BoardGame,
                name: "A Feast for Odin".to_string(),
                year_published: 2016,
                image: "https://domain/img.jpg".to_string(),
                thumbnail: "https://domain/thumbnail.jpg".to_string(),
                status: CollectionItemStatus {
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
                stats: CollectionItemStats {
                    min_players: 1,
                    max_players: 4,
                    min_playtime: Duration::minutes(30),
                    max_playtime: Duration::minutes(120),
                    playing_time: Duration::minutes(120),
                    owned_by: 37542,
                    rating: CollectionItemRating {
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
                version: None,
            },
            "returned collection game doesn't match expected",
        );
    }

    #[tokio::test]
    async fn get_version() {
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
                Matcher::UrlEncoded("version".into(), "1".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_brief_with_version.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let query = CollectionQueryParams::new().include_version_info(true);

        let collection = api
            .collection_brief()
            .get_from_query("somename", query)
            .await;
        mock.assert_async().await;

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.items.len(), 3);
        assert_eq!(
            collection.items[0],
            CollectionItemBrief {
                id: 356510,
                collection_id: 118278786,
                item_type: ItemType::BoardGame,
                name: "Spirit Island: Feather & Flame".to_string(),
                status: CollectionItemStatus {
                    own: true,
                    previously_owned: false,
                    for_trade: false,
                    want_in_trade: false,
                    want_to_play: false,
                    want_to_buy: false,
                    wishlist: false,
                    wishlist_priority: None,
                    pre_ordered: false,
                    last_modified: Utc.with_ymd_and_hms(2024, 4, 13, 17, 56, 44).unwrap(),
                },
                stats: CollectionItemStatsBrief {
                    min_players: 1,
                    max_players: 4,
                    min_playtime: Duration::minutes(90),
                    max_playtime: Duration::minutes(120),
                    playing_time: Duration::minutes(120),
                    owned_by: 5071,
                    rating: CollectionItemRatingBrief {
                        user_rating: None,
                        average: 8.98038,
                        bayesian_average: 6.55157,
                    },
                },
                version: None,
            },
            "returned collection game with no version doesn't match expected",
        );
        assert_eq!(
            collection.items[1],
            CollectionItemBrief {
                id: 13,
                collection_id: 122520827,
                item_type: ItemType::BoardGame,
                name: "Колонизаторы".to_string(),
                status: CollectionItemStatus {
                    own: false,
                    previously_owned: false,
                    for_trade: false,
                    want_in_trade: false,
                    want_to_play: false,
                    want_to_buy: false,
                    wishlist: true,
                    wishlist_priority: Some(WishlistPriority::LikeToHave),
                    pre_ordered: false,
                    last_modified: Utc.with_ymd_and_hms(2024, 8, 24, 8, 21, 52).unwrap(),
                },
                stats: CollectionItemStatsBrief {
                    min_players: 3,
                    max_players: 4,
                    min_playtime: Duration::minutes(60),
                    max_playtime: Duration::minutes(120),
                    playing_time: Duration::minutes(120),
                    owned_by: 210387,
                    rating: CollectionItemRatingBrief {
                        user_rating: None,
                        average: 7.09836,
                        bayesian_average: 6.91963,
                    },
                },
                version: Some(GameVersion {
                    id: 712636,
                    name: "Russian edition 2024".into(),
                    alternate_names: vec![],
                    year_published: 2024,
                    image: "https://cf.geekdo-images.com/IfUVNbebRWbtQ_SlGxG6ZQ__original/img/DNNED1WasGboaJH8OWMoGm6Zg1k=/0x0/filters:format(jpeg)/pic8177684.jpg".into(),
                    thumbnail: "https://cf.geekdo-images.com/IfUVNbebRWbtQ_SlGxG6ZQ__thumb/img/E4EX1sbROpEXxte6IMx7cRgSL2E=/fit-in/200x150/filters:strip_icc()/pic8177684.jpg".into(),
                    original_game: Game {
                        id: 13,
                        name: "CATAN".into(),
                    },
                    publishers: vec![GamePublisher {
                        id: 18852,
                        name: "Hobby World".into(),
                    }],
                    artists: vec![GameArtist {
                        id: 11825,
                        name: "Michael Menzel".into(),
                    }],
                    languages: vec![Language {
                        id: 2202,
                        name: "Russian".into(),
                    }],
                    dimensions: Some(Dimensions {
                        width: 11.7323,
                        length: 11.7323,
                        depth: 2.79528,
                    }),
                    weight: Some(2.7205),
                    product_code: Some("915853".into()),
                }),
            },
            "returned collection game with version doesn't match expected",
        );
        assert_eq!(
            collection.items[2],
            CollectionItemBrief {
                id: 352515,
                collection_id: 118278970,
                item_type: ItemType::BoardGame,
                name: "ナナ".to_string(),
                status: CollectionItemStatus {
                    own: true,
                    previously_owned: false,
                    for_trade: false,
                    want_in_trade: false,
                    want_to_play: false,
                    want_to_buy: false,
                    wishlist: false,
                    wishlist_priority: None,
                    pre_ordered: false,
                    last_modified: Utc.with_ymd_and_hms(2024, 4, 14, 9, 47, 13).unwrap(),
                },
                stats: CollectionItemStatsBrief {
                    min_players: 3,
                    max_players: 6,
                    min_playtime: Duration::minutes(15),
                    max_playtime: Duration::minutes(15),
                    playing_time: Duration::minutes(15),
                    owned_by: 8301,
                    rating: CollectionItemRatingBrief {
                        user_rating: Some(5.0),
                        average: 7.31486,
                        bayesian_average: 6.64921,
                    },
                },
                version: Some(GameVersion {
                    id: 590616,
                    name: "English/Japanese edition".into(),
                    alternate_names: vec![],
                    year_published: 2021,
                    image: "https://cf.geekdo-images.com/rt5qzjbrXq7PgI9IaRekNA__original/img/rh02vSdTIvg-oPr4ymQETCLUEjU=/0x0/filters:format(jpeg)/pic7227031.jpg".into(),
                    thumbnail: "https://cf.geekdo-images.com/rt5qzjbrXq7PgI9IaRekNA__thumb/img/HGNVOyEKBxZVl0Ry7YDwIcQ5vVc=/fit-in/200x150/filters:strip_icc()/pic7227031.jpg".into(),
                    original_game: Game {
                        id: 352515,
                        name: "Trio".into(),
                    },
                    publishers: vec![GamePublisher {
                        id: 50472,
                        name: "Mob+ (Mob Plus)".into(),
                    }],
                    artists: vec![GameArtist {
                        id: 108040,
                        name: "別府さい (Sai Beppu)".into(),
                    }],
                    languages: vec![
                        Language {
                            id: 2184,
                            name: "English".into(),
                        },
                        Language {
                            id: 2194,
                            name: "Japanese".into(),
                        },
                    ],
                    dimensions: None,
                    weight: None,
                    product_code: None,
                }),
            },
            "returned collection game with version doesn't match expected",
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
                // This user's collection
                Matcher::UrlEncoded("username".into(), "someone".into()),
                // Which info to include for each item
                Matcher::UrlEncoded("stats".into(), "1".into()),
                Matcher::UrlEncoded("brief".into(), "0".into()),
                Matcher::UrlEncoded("version".into(), "1".into()),
                // Status filtering
                Matcher::UrlEncoded("own".into(), "1".into()),
                Matcher::UrlEncoded("trade".into(), "0".into()),
                Matcher::UrlEncoded("want".into(), "1".into()),
                Matcher::UrlEncoded("wishlist".into(), "1".into()),
                Matcher::UrlEncoded("wishlistpriority".into(), "5".into()),
                Matcher::UrlEncoded("preordered".into(), "1".into()),
                Matcher::UrlEncoded("wanttoplay".into(), "0".into()),
                Matcher::UrlEncoded("wanttobuy".into(), "0".into()),
                Matcher::UrlEncoded("prevowned".into(), "1".into()),
                // Filtering
                Matcher::UrlEncoded("subtype".into(), "boardgameexpansion".into()),
                Matcher::UrlEncoded("excludesubtype".into(), "boardgame".into()),
                Matcher::UrlEncoded("id".into(), "13,3000,1".into()),
                Matcher::UrlEncoded("rated".into(), "1".into()),
                Matcher::UrlEncoded("played".into(), "1".into()),
                Matcher::UrlEncoded("comment".into(), "1".into()),
                Matcher::UrlEncoded("hasparts".into(), "0".into()),
                Matcher::UrlEncoded("wantparts".into(), "1".into()),
                Matcher::UrlEncoded("minrating".into(), "3.5".into()),
                Matcher::UrlEncoded("rating".into(), "9".into()),
                Matcher::UrlEncoded("minbggrating".into(), "5.5".into()),
                Matcher::UrlEncoded("bggrating".into(), "8.7".into()),
                Matcher::UrlEncoded("minplays".into(), "2".into()),
                Matcher::UrlEncoded("maxplays".into(), "450".into()),
                Matcher::UrlEncoded("showprivate".into(), "1".into()),
                Matcher::UrlEncoded("collid".into(), "345".into()),
                Matcher::UrlEncoded("modifiedsince".into(), "24-05-17".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_owned_single.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let query = CollectionQueryParams::new()
            .include_version_info(true)
            .include_owned(true)
            .include_for_trade(false)
            .include_want_in_trade(true)
            .include_wishlist(true)
            .wishlist_priority(WishlistPriority::DontBuyThis)
            .include_preordered(true)
            .include_want_to_play(false)
            .include_want_to_buy(false)
            .include_previously_owned(true)
            .item_type(ItemType::BoardGameExpansion)
            .exclude_item_type(ItemType::BoardGame)
            .item_id(13)
            .item_ids(vec![3000, 1])
            .include_rated_by_user(true)
            .include_played_by_user(true)
            .include_commented(true)
            .has_parts(false)
            .want_parts(true)
            .min_rating(3.5)
            .max_rating(9.0)
            .min_bgg_rating(5.5)
            .max_bgg_rating(8.7)
            .min_plays(2)
            .max_plays(450)
            .show_private(true)
            .collection_id(345)
            .modified_since(
                Utc.with_ymd_and_hms(2024, 5, 17, 0, 0, 0)
                    .unwrap()
                    .naive_utc()
                    .date(),
            );

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

        assert_eq!(collection.items.len(), 1);
        assert_eq!(
            collection.items[0],
            CollectionItem {
                id: 2281,
                collection_id: 118280658,
                item_type: ItemType::BoardGame,
                name: "Pictionary".to_string(),
                year_published: 1985,
                image: "https://cf.geekdo-images.com/YfUxodD7JSqYitxvjXB69Q__original/img/YRJAlLzkxMuJHVPsdnBLNFpoODA=/0x0/filters:format(png)/pic5147022.png".to_string(),
                thumbnail: "https://cf.geekdo-images.com/YfUxodD7JSqYitxvjXB69Q__thumb/img/7ls1a8ak5oT7BaKM-rVHpOVrP14=/fit-in/200x150/filters:strip_icc()/pic5147022.png".to_string(),
                status: CollectionItemStatus {
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
                stats: CollectionItemStats {
                    min_players: 3,
                    max_players: 16,
                    min_playtime: Duration::minutes(90),
                    max_playtime: Duration::minutes(90),
                    playing_time: Duration::minutes(90),
                    owned_by: 14400,
                    rating: CollectionItemRating {
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
                version: None,
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

        assert_eq!(collection.items.len(), 37);

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

        assert_eq!(collection.items.len(), 0);
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

        assert_eq!(collection.items.len(), 1);
        assert_eq!(
            collection.items[0],
            CollectionItem {
                id: 2281,
                collection_id: 118280658,
                item_type: ItemType::BoardGame,
                name: "Pictionary".to_string(),
                year_published: 1985,
                image: "https://cf.geekdo-images.com/YfUxodD7JSqYitxvjXB69Q__original/img/YRJAlLzkxMuJHVPsdnBLNFpoODA=/0x0/filters:format(png)/pic5147022.png".to_string(),
                thumbnail: "https://cf.geekdo-images.com/YfUxodD7JSqYitxvjXB69Q__thumb/img/7ls1a8ak5oT7BaKM-rVHpOVrP14=/fit-in/200x150/filters:strip_icc()/pic5147022.png".to_string(),
                status: CollectionItemStatus {
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
                stats: CollectionItemStats {
                    min_players: 3,
                    max_players: 16,
                    min_playtime: Duration::minutes(90),
                    max_playtime: Duration::minutes(90),
                    playing_time: Duration::minutes(90),
                    owned_by: 14400,
                    rating: CollectionItemRating {
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
                version: None,
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

        assert_eq!(collection.items.len(), 30);
        for game in collection.items {
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

        assert_eq!(collection.items.len(), 0);
    }

    #[tokio::test]
    async fn get_accessory_collection() {
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
                Matcher::UrlEncoded("subtype".into(), "boardgameaccessory".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_accessories.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let collection = api
            .collection()
            .get_accessory_collection("somename", CollectionQueryParams::new())
            .await;
        mock.assert_async().await;

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.items.len(), 2);
        assert_eq!(
            collection.items[0],
            CollectionItem {
                id: 142974,
                collection_id: 122439219,
                item_type: ItemType::BoardGameAccessory,
                name: "12 Realms: Buildings Pack".to_string(),
                year_published: 2013,
                image: "https://cf.geekdo-images.com/5fKeQe2FG2FR1W3maIj1Gw__original/img/vWskbRA9FmoLrFghnPi_5RGVMec=/0x0/filters:format(jpeg)/pic2522878.jpg".to_string(),
                thumbnail: "https://cf.geekdo-images.com/5fKeQe2FG2FR1W3maIj1Gw__thumb/img/Fu2YriGdZVDzf8sJyvnWADloJPU=/fit-in/200x150/filters:strip_icc()/pic2522878.jpg".to_string(),
                status: CollectionItemStatus {
                    own: true,
                    previously_owned: false,
                    for_trade: false,
                    want_in_trade: false,
                    want_to_play: false,
                    want_to_buy: false,
                    wishlist: false,
                    wishlist_priority: None,
                    pre_ordered: false,
                    last_modified: Utc.with_ymd_and_hms(2024, 8, 21, 16, 47, 5).unwrap(),
                },
                number_of_plays: 0,
                stats: CollectionItemStats {
                    min_players: 0,
                    max_players: 0,
                    min_playtime: Duration::minutes(0),
                    max_playtime: Duration::minutes(0),
                    playing_time: Duration::minutes(0),
                    owned_by: 281,
                    rating: CollectionItemRating {
                        user_rating: None,
                        users_rated: 38,
                        average: 6.51053,
                        bayesian_average: 6.10014,
                        standard_deviation: 1.89983,
                        median: 0.0,
                        ranks: vec![
                            GameFamilyRank {
                                game_family_type: GameFamilyType::Subtype,
                                id: 62,
                                name: "boardgameaccessory".into(),
                                friendly_name: "Accessory Rank".into(),
                                value: RankValue::Ranked(749),
                                bayesian_average: 6.10014,
                            },
                        ],
                    },
                },
                version: None,
            },
            "returned collection game doesn't match expected",
        );
        assert_eq!(
            collection.items[1],
            CollectionItem {
                id: 22510,
                collection_id: 122524875,
                item_type: ItemType::BoardGameAccessory,
                name: "Wings of War: Miniatures".to_string(),
                year_published: 2007,
                image: "https://cf.geekdo-images.com/qGV1v8Ye0FKTxZNCF1ZINw__original/img/49pxPDdA4CHNFZOMQM1UTM8FNL4=/0x0/filters:format(jpeg)/pic830522.jpg".to_string(),
                thumbnail: "https://cf.geekdo-images.com/qGV1v8Ye0FKTxZNCF1ZINw__thumb/img/vgAzbZuLXNSawia3yp4BAPT_2is=/fit-in/200x150/filters:strip_icc()/pic830522.jpg".to_string(),
                status: CollectionItemStatus {
                    own: false,
                    previously_owned: false,
                    for_trade: false,
                    want_in_trade: false,
                    want_to_play: true,
                    want_to_buy: false,
                    wishlist: false,
                    wishlist_priority: None,
                    pre_ordered: false,
                    last_modified: Utc.with_ymd_and_hms(2024, 8, 24, 11, 9, 37).unwrap(),
                },
                number_of_plays: 0,
                stats: CollectionItemStats {
                    min_players: 0,
                    max_players: 0,
                    min_playtime: Duration::minutes(0),
                    max_playtime: Duration::minutes(0),
                    playing_time: Duration::minutes(0),
                    owned_by: 1618,
                    rating: CollectionItemRating {
                        user_rating: None,
                        users_rated: 893,
                        average: 7.87269,
                        bayesian_average: 7.55507,
                        standard_deviation: 1.30371,
                        median: 0.0,
                        ranks: vec![
                            GameFamilyRank {
                                game_family_type: GameFamilyType::Subtype,
                                id: 62,
                                name: "boardgameaccessory".into(),
                                friendly_name: "Accessory Rank".into(),
                                value: RankValue::Ranked(22),
                                bayesian_average: 7.55507,
                            },
                        ],
                    },
                },
                version: None,
            },
            "returned collection game doesn't match expected",
        );
    }
}
