use chrono::{DateTime, NaiveDate, Utc};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::future::Future;

use crate::api::BoardGameGeekApi;
use crate::utils::{date_deserializer, deserialize_1_0_bool, deserialize_wishlist_priority};
use crate::Result;

pub trait CollectionType<'q>: DeserializeOwned {
    fn base_query(username: &'q str) -> BaseCollectionQuery<'q>;
}

impl<'q> CollectionType<'q> for CollectionBrief {
    fn base_query(username: &'q str) -> BaseCollectionQuery<'q> {
        BaseCollectionQuery {
            username,
            brief: true,
        }
    }
}
impl<'q> CollectionType<'q> for Collection {
    fn base_query(username: &'q str) -> BaseCollectionQuery<'q> {
        BaseCollectionQuery {
            username,
            brief: false,
        }
    }
}

/// A user's collection on boardgamegeek, with only the name and statuses returned.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CollectionBrief {
    /// List of games and expansions in the user's collection. Each item
    /// is not necessarily owned but can be preowned, wishlisted etc.
    #[serde(rename = "$value")]
    pub items: Vec<CollectionItemBrief>,
}

/// An item in a collection, in brief form. With only the name, status, type, and IDs.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CollectionItemBrief {
    /// The ID of the game.
    #[serde(rename = "objectid")]
    pub id: u64,
    /// The collection ID of the object.
    #[serde(rename = "collid")]
    pub collection_id: u64,
    /// The type of game, which will either be boardgame or expansion.
    #[serde(rename = "subtype")]
    pub item_type: ItemType,
    /// The name of the game.
    pub name: String,
    /// Status of the game in this collection, such as own, preowned, wishlist.
    pub status: CollectionItemStatus,
}

/// A user's collection on boardgamegeek.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Collection {
    /// List of games and expansions in the user's collection. Each item
    /// is not necessarily owned but can be preowned, wishlisted etc.
    #[serde(rename = "$value")]
    pub items: Vec<CollectionItem>,
}

/// A game or game expansion in a collection.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CollectionItem {
    /// The ID of the game.
    #[serde(rename = "objectid")]
    pub id: u64,
    /// The collection ID of the object.
    #[serde(rename = "collid")]
    pub collection_id: u64,
    /// The type of game, which will either be boardgame or expansion.
    #[serde(rename = "subtype")]
    pub item_type: ItemType,
    /// The name of the game.
    pub name: String,
    /// The year the game was first published.
    #[serde(rename = "yearpublished")]
    pub year_published: i64,
    /// A link to a jpg image for the game.
    pub image: String,
    /// A link to a jpg thumbnail image for the game.
    pub thumbnail: String,
    /// Status of the game in this collection, such as own, preowned, wishlist.
    pub status: CollectionItemStatus,
    /// The number of times the user has played the game.
    #[serde(rename = "numplays")]
    pub number_of_plays: u64,
    /// Game stats such as number of players, can sometimes be omitted from the result.
    pub stats: Option<CollectionItemStats>,
}

/// The type of game, board game or expansion.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum ItemType {
    /// A board game, or expansion.
    ///
    /// Due to the way the API works, this type can include expansions too.
    /// If a request is made for just board games, or the game type is not
    /// filtered, then both items with a type of [GameType::BoardGame] and
    /// those with a type of [GameType::BoardGameExpansion] will be returned,
    /// and they will ALL have the type of [GameType::BoardGame]. However when
    /// requesting just expansions, the returned items will correctly have the
    /// type [GameType::BoardGameExpansion].
    ///
    /// A workaround to this can be to make 2 requests, one to include
    /// [GameType::BoardGame] and exclude [GameType::BoardGameExpansion],
    /// followed by another to just include [GameType::BoardGameExpansion].
    #[serde(rename = "boardgame")]
    BoardGame,
    /// A board game expansion.
    #[serde(rename = "boardgameexpansion")]
    BoardGameExpansion,
}

/// The status of the game in the user's collection, such as preowned or wishlist.
/// Can be any or none of them.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CollectionItemStatus {
    /// User owns the game.
    #[serde(deserialize_with = "deserialize_1_0_bool")]
    pub own: bool,
    /// User has previously owned the game.
    #[serde(rename = "prevowned", deserialize_with = "deserialize_1_0_bool")]
    pub previously_owned: bool,
    /// User wants to trade away the game.
    #[serde(rename = "fortrade", deserialize_with = "deserialize_1_0_bool")]
    pub for_trade: bool,
    /// User wants to receive the game in a trade.
    #[serde(rename = "want", deserialize_with = "deserialize_1_0_bool")]
    pub want_in_trade: bool,
    /// User wants to play the game.
    #[serde(rename = "wanttoplay", deserialize_with = "deserialize_1_0_bool")]
    pub want_to_play: bool,
    /// User wants to buy the game.
    #[serde(rename = "wanttobuy", deserialize_with = "deserialize_1_0_bool")]
    pub want_to_buy: bool,
    /// User pre-ordered the game.
    #[serde(rename = "preordered", deserialize_with = "deserialize_1_0_bool")]
    pub pre_ordered: bool,
    /// User has the game on their wishlist.
    #[serde(deserialize_with = "deserialize_1_0_bool")]
    pub wishlist: bool,
    /// The priority of the wishlist.
    #[serde(
        default,
        rename = "wishlistpriority",
        deserialize_with = "deserialize_wishlist_priority"
    )]
    pub wishlist_priority: Option<WishlistPriority>,
    /// When the collection status was last modified
    #[serde(rename = "lastmodified", with = "date_deserializer")]
    pub last_modified: DateTime<Utc>,
}

/// The status of the game in the user's collection, such as preowned or wishlist.
/// Can be any or none of them.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd)]
pub enum WishlistPriority {
    /// Lowest priority.
    DontBuyThis,
    /// Thinking about buying it.
    ThinkingAboutIt,
    /// The default value, would like to have it.
    LikeToHave,
    /// Would love to have it.
    LoveToHave,
    /// Highest wishlist priority, a must have.
    MustHave,
}

/// Stats of the game such as playercount and duration. Can be omitted from the response.
/// More stats can be found from the specific game endpoint.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CollectionItemStats {
    /// Minimum players the game supports.
    #[serde(rename = "minplayers")]
    pub min_players: u32,
    /// Maximum players the game supports.
    #[serde(rename = "maxplayers")]
    pub max_players: u32,
}

/// Required query paramters. Any type the collection query can implement
/// must be able to return a base query, so valid queries can be constructed
/// for both [Collection] and [CollectionBrief].
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
    /// Include only results for this item type.
    ///
    /// Note, if this is set to [ItemType::BoardGame] then it will include both
    /// board games and expansions, but set the type of all of them to be
    /// [ItemType::BoardGame] in the results. Explicitly exclude expansions
    /// to avoid this.
    item_type: Option<ItemType>,
    /// Exclude results for this item type.
    exclude_item_type: Option<ItemType>,
    /// Include items the user owns if true, exclude if false.
    include_owned: Option<bool>,
    /// Include items the user previously owned if true, exclude if false.
    include_previously_owned: Option<bool>,
    /// Include items the user wants to trade away if true, exclude if false.
    include_for_trade: Option<bool>,
    /// Include items the user wants in a trade if true, exclude if false.
    include_want_in_trade: Option<bool>,
    /// Include items the user wants to play if true, exclude if false.
    include_want_to_play: Option<bool>,
    /// Include items the user wants to buy if true, exclude if false.
    include_want_to_buy: Option<bool>,
    /// Include items the user has preordered if true, exclude if false.
    include_preordered: Option<bool>,
    /// Include items the user has on their wishlist if true, exclude if false.
    include_wishlist: Option<bool>,
    /// Only include items for this wishlist priority.
    wishlist_priority: Option<WishlistPriority>,
    /// Only include items modified since this date time.
    modified_since: Option<NaiveDate>,
    // Include extra stats about the game if true, false otherwise. By default they are included.
    include_stats: Option<bool>,
    /// Include only items that have been rated by the user.
    include_rated_by_user: Option<bool>,
    /// Include only items that have been played by the user.
    include_played_by_user: Option<bool>,
    /// Include only items that have been commented on by the user.
    include_commented: Option<bool>,
    /// Include only items that have a comment in the `Has Parts` field.
    has_parts: Option<bool>,
    /// Include only items that have a comment in the `Want Parts` field.
    want_parts: Option<bool>,
    /// Include only items that the user has rated at least this value.
    min_rating: Option<f32>,
    /// Include only items that the user has rated at most this value.
    max_rating: Option<f32>,
    /// Include only items that have a Geek rating of at least this value.
    min_bgg_rating: Option<f32>,
    /// Include only items that have a Geek rating of at most this value.
    max_bgg_rating: Option<f32>,
    /// Include only items that the user has played at least this many times.
    min_plays: Option<u64>,
    /// Include only items that the user has played at most this many times.
    max_plays: Option<u64>,
    /// Show private collection info. Only works when viewing your own collection and you are logged in.
    show_private: Option<bool>,
    /// ID of a particular item in a collection.
    collection_id: Option<u64>,
}

impl CollectionQueryParams {
    /// Constructs a new query builder from a base query, and the rest of the parameters.
    fn new() -> Self {
        Self::default()
    }

    /// Sets the item_type field, so that only that type of item will be returned.
    pub fn item_type(mut self, item_type: ItemType) -> Self {
        self.item_type = Some(item_type);
        self
    }

    /// Set the exclude_item_type field, so that that type of item will be excluded from.
    /// the results.
    pub fn exclude_item_type(mut self, exclude_item_type: ItemType) -> Self {
        self.exclude_item_type = Some(exclude_item_type);
        self
    }

    /// Sets the include_owned field. If true the result will include items that
    /// the user owns. Unless all status fields are kept at None, then they are all included.
    pub fn include_owned(mut self, include_owned: bool) -> Self {
        self.include_owned = Some(include_owned);
        self
    }

    /// Sets the include_previously_owned field. If true the result will include items that
    /// the user owns. Unless all status fields are kept at None, then they are all included.
    pub fn include_previously_owned(mut self, include_previously_owned: bool) -> Self {
        self.include_previously_owned = Some(include_previously_owned);
        self
    }

    /// Sets the include_for_trade field. If true the result will include items that
    /// the user wants to trade away. Unless all status fields are kept at None,
    /// then they are all included.
    pub fn include_for_trade(mut self, include_for_trade: bool) -> Self {
        self.include_for_trade = Some(include_for_trade);
        self
    }

    /// Sets the include_want_in_trade field. If true the result will include items that
    /// the user wants to receive in a trade. Unless all status fields are kept at None,
    /// then they are all included.
    pub fn include_want_in_trade(mut self, include_want_in_trade: bool) -> Self {
        self.include_want_in_trade = Some(include_want_in_trade);
        self
    }

    /// Sets the include_want_to_play field. If true the result will include items that
    /// the user wants to play. Unless all status fields are kept at None,
    /// then they are all included.
    pub fn include_want_to_play(mut self, include_want_to_play: bool) -> Self {
        self.include_want_to_play = Some(include_want_to_play);
        self
    }

    /// Sets the include_want_to_buy field. If true the result will include items that
    /// the user wants to buy. Unless all status fields are kept at None,
    /// then they are all included.
    pub fn include_want_to_buy(mut self, include_want_to_buy: bool) -> Self {
        self.include_want_to_buy = Some(include_want_to_buy);
        self
    }

    /// Sets the include_preordered field. If true the result will include items that
    /// the user wants to buy. Unless all status fields are kept at None,
    /// then they are all included.
    pub fn include_preordered(mut self, include_preordered: bool) -> Self {
        self.include_preordered = Some(include_preordered);
        self
    }

    /// Sets the include_wishlist field. If true the result will include the items
    /// that the user has on their wishlist. Unless all status fields are kept at None, then they are all included.
    pub fn include_wishlist(mut self, include_wishlist: bool) -> Self {
        self.include_wishlist = Some(include_wishlist);
        self
    }

    /// Sets the wishlist_priority field. If set then only results with that wishlist
    /// priority will be returned.
    pub fn wishlist_priority(mut self, wishlist_priority: WishlistPriority) -> Self {
        self.wishlist_priority = Some(wishlist_priority);
        self
    }

    /// Sets the modified_since field. If set then only results that have been modified
    /// since that datetime will be returned.
    pub fn modified_since(mut self, modified_since: NaiveDate) -> Self {
        self.modified_since = Some(modified_since);
        self
    }

    /// Sets the include_stats field. If false the stats are omitted.
    /// Since the default behaviour is inconsistent. Keeping this at None will
    /// be treated as true at build time.
    pub fn include_stats(mut self, include_stats: bool) -> Self {
        self.include_stats = Some(include_stats);
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
    /// to get the item with the specific collection ID.
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
    /// Constructs a new query builder from a base query, and the rest of the parameters.
    fn new(base: BaseCollectionQuery<'a>, params: CollectionQueryParams) -> Self {
        Self { base, params }
    }

    /// Converts the fields into a vector of (&str, &str) tuples that match
    /// the expected query parameter key value pairs.
    pub fn build(self) -> Vec<(&'a str, String)> {
        let mut query_params: Vec<_> = vec![];
        query_params.push(("username", self.base.username.to_string()));

        match self.base.brief {
            true => query_params.push(("brief", "1".to_string())),
            false => query_params.push(("brief", "0".to_string())),
        }
        match self.params.item_type {
            Some(ItemType::BoardGame) => query_params.push(("subtype", "boardgame".to_string())),
            Some(ItemType::BoardGameExpansion) => {
                query_params.push(("subtype", "boardgameexpansion".to_string()))
            }
            None => {}
        }
        match self.params.exclude_item_type {
            Some(ItemType::BoardGame) => {
                query_params.push(("excludesubtype", "boardgame".to_string()))
            }
            Some(ItemType::BoardGameExpansion) => {
                query_params.push(("excludesubtype", "boardgameexpansion".to_string()))
            }
            None => {}
        }
        match self.params.include_owned {
            Some(true) => query_params.push(("own", "1".to_string())),
            Some(false) => query_params.push(("own", "0".to_string())),
            None => {}
        }
        match self.params.include_previously_owned {
            Some(true) => query_params.push(("prevowned", "1".to_string())),
            Some(false) => query_params.push(("prevowned", "0".to_string())),
            None => {}
        }
        match self.params.include_for_trade {
            Some(true) => query_params.push(("trade", "1".to_string())),
            Some(false) => query_params.push(("trade", "0".to_string())),
            None => {}
        }
        match self.params.include_want_in_trade {
            Some(true) => query_params.push(("want", "1".to_string())),
            Some(false) => query_params.push(("want", "0".to_string())),
            None => {}
        }
        match self.params.include_want_to_play {
            Some(true) => query_params.push(("wanttoplay", "1".to_string())),
            Some(false) => query_params.push(("wanttoplay", "0".to_string())),
            None => {}
        }
        match self.params.include_want_to_buy {
            Some(true) => query_params.push(("wanttobuy", "1".to_string())),
            Some(false) => query_params.push(("wanttobuy", "0".to_string())),
            None => {}
        }
        match self.params.include_preordered {
            Some(true) => query_params.push(("preordered", "1".to_string())),
            Some(false) => query_params.push(("preordered", "0".to_string())),
            None => {}
        }
        match self.params.include_wishlist {
            Some(true) => query_params.push(("wishlist", "1".to_string())),
            Some(false) => query_params.push(("wishlist", "0".to_string())),
            None => {}
        }
        match self.params.wishlist_priority {
            Some(WishlistPriority::DontBuyThis) => {
                query_params.push(("wishlistpriority", "5".to_string()))
            }
            Some(WishlistPriority::ThinkingAboutIt) => {
                query_params.push(("wishlistpriority", "4".to_string()))
            }
            Some(WishlistPriority::LikeToHave) => {
                query_params.push(("wishlistpriority", "3".to_string()))
            }
            Some(WishlistPriority::LoveToHave) => {
                query_params.push(("wishlistpriority", "2".to_string()))
            }
            Some(WishlistPriority::MustHave) => {
                query_params.push(("wishlistpriority", "1".to_string()))
            }
            None => {}
        }
        if let Some(modified_since) = self.params.modified_since {
            query_params.push((
                "modifiedsince",
                modified_since.format("YY-MM-DD").to_string(),
            ));
        }
        match self.params.include_stats {
            Some(true) => query_params.push(("stats", "1".to_string())),
            Some(false) => query_params.push(("stats", "0".to_string())),
            // When omitted, the API has inconsistent behaviour, and will include the stats usually
            // but not when specific game types are requested, so we set it to true for consistency.
            None => query_params.push(("stats", "1".to_string())),
        }
        match self.params.include_rated_by_user {
            Some(true) => query_params.push(("rated", "1".to_string())),
            Some(false) => query_params.push(("rated", "0".to_string())),
            None => {}
        }
        match self.params.include_played_by_user {
            Some(true) => query_params.push(("played", "1".to_string())),
            Some(false) => query_params.push(("played", "0".to_string())),
            None => {}
        }
        match self.params.include_commented {
            Some(true) => query_params.push(("comment", "1".to_string())),
            Some(false) => query_params.push(("comment", "0".to_string())),
            None => {}
        }
        match self.params.has_parts {
            Some(true) => query_params.push(("hasparts", "1".to_string())),
            Some(false) => query_params.push(("hasparts", "0".to_string())),
            None => {}
        }
        match self.params.want_parts {
            Some(true) => query_params.push(("wantparts", "1".to_string())),
            Some(false) => query_params.push(("wantparts", "0".to_string())),
            None => {}
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
        query_params
    }
}

/// Collection endpoint of the API. Used for returning user's collections
/// of games by their username. Filtering by [CollectionGameStatus], rating, recorded plays.
pub struct CollectionApi<'api, T: CollectionType<'api>> {
    pub(crate) api: &'api BoardGameGeekApi<'api>,
    endpoint: &'api str,
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

    /// Get all items of all types in the user's collection.
    pub fn get_all(&self, username: &'api str) -> impl Future<Output = Result<T>> + 'api {
        let query =
            CollectionQueryBuilder::new(T::base_query(username), CollectionQueryParams::new());

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<T>(request)
    }

    /// Gets all the games that a given user owns.
    pub fn get_owned(&self, username: &'api str) -> impl Future<Output = Result<T>> + 'api {
        let query_params = CollectionQueryParams::new().include_owned(true);
        let query = CollectionQueryBuilder::new(T::base_query(username), query_params);

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<T>(request)
    }

    /// Gets all the games that a given user has on their wishlist.
    pub fn get_wishlist(&self, username: &'api str) -> impl Future<Output = Result<T>> + 'api {
        let query_params = CollectionQueryParams::new().include_wishlist(true);
        let query = CollectionQueryBuilder::new(T::base_query(username), query_params);

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<T>(request)
    }

    /// Makes a request from a [CollectionQueryParams].
    pub fn get_from_query(
        &self,
        username: &'api str,
        query_params: CollectionQueryParams,
    ) -> impl Future<Output = Result<T>> + 'api {
        let query = CollectionQueryBuilder::new(T::base_query(username), query_params);

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<T>(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use mockito::Matcher;

    #[tokio::test]
    async fn test_get_owned_brief() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let api = BoardGameGeekApi {
            base_url: &url,
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "somename".into()),
                Matcher::UrlEncoded("own".into(), "1".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
              ]))
            .with_status(200)
            .with_body(r#"
<items>
    <item objecttype="thing" objectid="131835" subtype="boardgame" collid="118278872">
        <name sortindex="1">Boss Monster: The Dungeon Building Card Game</name>
        <status own="1" prevowned="0" fortrade="0" want="0" wanttoplay="0" wanttobuy="0" wishlist="0" preordered="0" lastmodified="2024-04-13 18:29:01"/>
    </item>
</items>
            "#)
            .create_async()
            .await;

        let collection = api.collection_brief().get_owned("somename").await;
        println!("{collection:?}");
        mock.assert();

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
            },
            "returned collection game doesn't match expected",
        );
    }

    #[tokio::test]
    async fn test_get_owned() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let api = BoardGameGeekApi {
            base_url: &url,
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "somename".into()),
                Matcher::UrlEncoded("own".into(), "1".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
              ]))
            .with_status(200)
            .with_body(r#"
<items>
    <item objecttype="thing" objectid="131835" subtype="boardgame" collid="118278872">
        <name sortindex="1">Boss Monster: The Dungeon Building Card Game</name>
        <yearpublished>2013</yearpublished>
        <image>
            https://domain/img.jpg
        </image>
        <thumbnail>
            https://domain/thumbnail.jpg
        </thumbnail>
        <status own="1" prevowned="0" fortrade="0" want="0" wanttoplay="0" wanttobuy="0" wishlist="0" preordered="0" lastmodified="2024-04-13 18:29:01"/>
        <numplays>2</numplays>
    </item>
</items>
            "#)
            .create_async()
            .await;

        let collection = api.collection().get_owned("somename").await;
        println!("{collection:?}");
        mock.assert();

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
                image: "https://domain/img.jpg".to_string(),
                thumbnail: "https://domain/thumbnail.jpg".to_string(),
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
                stats: None,
            },
            "returned collection game doesn't match expected",
        );
    }

    #[tokio::test]
    async fn test_get_wishlist() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let api = BoardGameGeekApi {
            base_url: &url,
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "somename".into()),
                Matcher::UrlEncoded("wishlist".into(), "1".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
              ]))
            .with_status(200)
            .with_body(r#"
<items>
    <item objecttype="thing" objectid="177736" subtype="boardgame" collid="118332974">
        <name sortindex="3">A Feast for Odin</name>
        <yearpublished>2016</yearpublished>
        <image>
            https://domain/img.jpg
        </image>
        <thumbnail>
            https://domain/thumbnail.jpg
        </thumbnail>
        <status own="0" prevowned="0" fortrade="0" want="1" wanttoplay="0" wanttobuy="0" wishlist="1" wishlistpriority="2" preordered="0" lastmodified="2024-04-18 19:28:17"/>
        <numplays>0</numplays>
    </item>
</items>
            "#)
            .create_async()
            .await;

        let collection = api.collection().get_wishlist("somename").await;
        println!("{collection:?}");
        mock.assert();

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
                stats: None,
            },
            "returned collection game doesn't match expected",
        );
    }
}
