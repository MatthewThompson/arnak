use chrono::{DateTime, Duration, NaiveDate, Utc};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::ops::RangeInclusive;

use crate::api::BoardGameGeekApi;
use crate::utils::{
    date_deserializer, deserialize_1_0_bool, deserialize_game_ratings,
    deserialize_game_ratings_brief, deserialize_minutes, deserialize_rank_value_enum,
    deserialize_wishlist_priority,
};
use crate::Result;

pub trait CollectionItemType<'a>: DeserializeOwned {
    fn base_query(username: &'a str) -> BaseCollectionQuery<'a>;

    fn get_stats(&self) -> Option<CollectionItemStatsBrief>;
}

impl<'a> CollectionItemType<'a> for CollectionItemBrief {
    fn base_query(username: &'a str) -> BaseCollectionQuery<'a> {
        BaseCollectionQuery {
            username,
            brief: true,
        }
    }

    fn get_stats(&self) -> Option<CollectionItemStatsBrief> {
        self.stats.clone()
    }
}

impl<'a> CollectionItemType<'a> for CollectionItem {
    fn base_query(username: &'a str) -> BaseCollectionQuery<'a> {
        BaseCollectionQuery {
            username,
            brief: false,
        }
    }

    fn get_stats(&self) -> Option<CollectionItemStatsBrief> {
        self.stats.as_ref().map(|stats| CollectionItemStatsBrief {
            min_players: stats.min_players,
            max_players: stats.max_players,
            min_playtime: stats.min_playtime,
            max_playtime: stats.max_playtime,
            playing_time: stats.playing_time,
            owned_by: stats.owned_by,
            rating: CollectionItemRatingBrief {
                value: stats.rating.value,
                average: stats.rating.average,
                bayesian_average: stats.rating.bayesian_average,
            },
        })
    }
}

/// A user's collection on boardgamegeek.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Collection<T> {
    /// List of games and expansions in the user's collection. Each item
    /// is not necessarily owned but can be preowned, wishlisted etc.
    #[serde(rename = "$value")]
    pub items: Vec<T>,
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
    /// Game stats such as number of players, can sometimes be omitted from the result.
    pub stats: Option<CollectionItemStatsBrief>,
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
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, PartialOrd)]
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
pub struct CollectionItemStatsBrief {
    /// Minimum players the game supports.
    #[serde(rename = "minplayers")]
    pub min_players: u32,
    /// Maximum players the game supports.
    #[serde(rename = "maxplayers")]
    pub max_players: u32,
    /// Minimum amount of time the game is suggested to take to play.
    #[serde(rename = "minplaytime", deserialize_with = "deserialize_minutes")]
    pub min_playtime: Duration,
    /// Maximum amount of time the game is suggested to take to play.
    #[serde(rename = "maxplaytime", deserialize_with = "deserialize_minutes")]
    pub max_playtime: Duration,
    /// The amount of time the game is suggested to take to play.
    #[serde(rename = "playingtime", deserialize_with = "deserialize_minutes")]
    pub playing_time: Duration,
    /// The number of people that own this game.
    #[serde(rename = "numowned")]
    pub owned_by: u64,
    /// Information about the rating that this user, as well as all users, have given this game.
    #[serde(rename = "$value", deserialize_with = "deserialize_game_ratings_brief")]
    pub rating: CollectionItemRatingBrief,
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
    /// Minimum amount of time the game is suggested to take to play.
    #[serde(rename = "minplaytime", deserialize_with = "deserialize_minutes")]
    pub min_playtime: Duration,
    /// Maximum amount of time the game is suggested to take to play.
    #[serde(rename = "maxplaytime", deserialize_with = "deserialize_minutes")]
    pub max_playtime: Duration,
    /// The amount of time the game is suggested to take to play.
    #[serde(rename = "playingtime", deserialize_with = "deserialize_minutes")]
    pub playing_time: Duration,
    /// The number of people that own this game.
    #[serde(rename = "numowned")]
    pub owned_by: u64,
    /// Information about the rating that this user, as well as all users, have given this game.
    #[serde(rename = "$value", deserialize_with = "deserialize_game_ratings")]
    pub rating: CollectionItemRating,
}

/// The 0-10 rating that the user gave to this game. Also includes the total
/// number of users that have rated it, as well as the averages.
#[derive(Clone, Debug, PartialEq)]
pub struct CollectionItemRatingBrief {
    /// The 0-10 rating that the user gave this game.
    pub value: Option<f64>,
    /// The mean average rating for this game.
    pub average: f64,
    /// The bayesian average rating for this game.
    pub bayesian_average: f64,
}

/// The 0-10 rating that the user gave to this game. Also includes the total
/// number of users that have rated it, as well as the averages, and standard deviation.
#[derive(Clone, Debug, PartialEq)]
pub struct CollectionItemRating {
    /// The 0-10 rating that the user gave this game.
    pub value: Option<f64>,
    /// The total number of users who have given this game a rating.
    pub users_rated: u64,
    /// The mean average rating for this game.
    pub average: f64,
    /// The bayesian average rating for this game.
    pub bayesian_average: f64,
    /// The standard deviation of the average rating.
    pub standard_deviation: f64,
    // Kept private for now since the API always returns 0 for this seemingly.
    pub(crate) median: f64,
    /// The list of ranks the game is on the site within each of its game types.
    pub ranks: Vec<GameTypeRank>,
}

// Intermediary struct needed due to the way the XML is strcutured
// TODO move this into the deserialise file.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Ranks {
    #[serde(rename = "$value")]
    pub(crate) ranks: Vec<GameTypeRank>,
}

/// A struct containing the game's rank within a particular type of game.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GameTypeRank {
    // The type of game, TODO change to an enum.
    #[serde(rename = "type")]
    pub game_type: String,
    /// ID game type.
    pub id: u64,
    /// Name of the game type. "boardgame" used as the generic subtype that includes all baord games.
    pub name: String,
    /// User friendly name in the foramt "GENRE game rank" e.g. "Party Game Rank"
    #[serde(rename = "friendlyname")]
    pub friendly_name: String,
    /// The overall rank on the site within this type of game.
    #[serde(deserialize_with = "deserialize_rank_value_enum")]
    pub value: RankValue,
    // The score out of 10, as a bayseian average.
    #[serde(rename = "bayesaverage")]
    pub bayesian_average: f64,
}

/// A rank a particular board game has on the site, within a subtype. Can be either
/// Ranked with a u64 for the rank, Or NotRanked.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RankValue {
    Ranked(u64),
    NotRanked,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlIntValue {
    pub value: u64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlFloatValue {
    pub value: f64,
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
        match self.params.show_private {
            Some(true) => query_params.push(("showprivate", "1".to_string())),
            Some(false) => query_params.push(("showprivate", "0".to_string())),
            None => {}
        }
        if let Some(collection_id) = self.params.collection_id {
            query_params.push(("collid", collection_id.to_string()));
        }
        query_params
    }
}

/// Collection endpoint of the API. Used for returning user's collections
/// of games by their username. Filtering by [CollectionGameStatus], rating, recorded plays.
pub struct CollectionApi<'api, T: CollectionItemType<'api>> {
    pub(crate) api: &'api BoardGameGeekApi<'api>,
    endpoint: &'api str,
    type_marker: std::marker::PhantomData<T>,
}

impl<'api, T: CollectionItemType<'api> + 'api> CollectionApi<'api, T> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "collection",
            type_marker: std::marker::PhantomData,
        }
    }

    /// Get all items of all types in the user's collection.
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
        let mut collection = self
            .get_from_query(username, query_params.include_stats(true))
            .await?;

        collection.items.retain(|item| {
            item.get_stats().is_some_and(|s| {
                *player_counts.start() <= s.max_players && *player_counts.end() >= s.min_players
            })
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
        let mut collection = self
            .get_from_query(username, query_params.include_stats(true))
            .await?;

        collection.items.retain(|item| {
            item.get_stats()
                .is_some_and(|s| player_count <= s.max_players && player_count >= s.min_players)
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
    use super::*;
    use chrono::TimeZone;
    use mockito::Matcher;

    #[test]
    fn sort_wishlist_priority() {
        assert!(WishlistPriority::DontBuyThis < WishlistPriority::MustHave);
    }

    #[tokio::test]
    async fn get_owned_brief() {
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
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_brief_owned_single.xml")
                    .expect("failed to load test data"),
            )
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
                stats: None,
            },
            "returned collection game doesn't match expected",
        );
    }

    #[tokio::test]
    async fn get_owned_all() {
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
                Matcher::UrlEncoded("brief".into(), "0".into()),
                Matcher::UrlEncoded("own".into(), "1".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_multiple.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let collection = api.collection().get_owned("somename").await;
        mock.assert();

        println!("{:?}", collection);
        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.items.len(), 33);
    }

    #[tokio::test]
    async fn get_owned() {
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
                Matcher::UrlEncoded("brief".into(), "0".into()),
                Matcher::UrlEncoded("own".into(), "1".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_owned_single.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let collection = api.collection().get_owned("somename").await;
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
                stats: Some(CollectionItemStats {
                    min_players: 2,
                    max_players: 4,
                    min_playtime: Duration::minutes(30),
                    max_playtime: Duration::minutes(30),
                    playing_time: Duration::minutes(30),
                    owned_by: 36423,
                    rating: CollectionItemRating {
                        value: Some(3.0),
                        users_rated: 17063,
                        average: 6.27139,
                        bayesian_average: 6.08972,
                        standard_deviation: 1.45941,
                        median: 0.0,
                        ranks: vec![
                            GameTypeRank {
                                game_type: "subtype".into(),
                                id: 1,
                                name: "boardgame".into(),
                                friendly_name: "Board Game Rank".into(),
                                value: RankValue::Ranked(2486),
                                bayesian_average: 6.08972
                            },
                            GameTypeRank {
                                game_type: "family".into(),
                                id: 5499,
                                name: "familygames".into(),
                                friendly_name: "Family Game Rank".into(),
                                value: RankValue::Ranked(1006),
                                bayesian_average: 6.05246
                            },
                        ],
                    },
                }),
            },
            "returned collection game doesn't match expected",
        );
    }

    #[tokio::test]
    async fn get_wishlist() {
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
                Matcher::UrlEncoded("brief".into(), "0".into()),
                Matcher::UrlEncoded("wishlist".into(), "1".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/collection/collection_wishlist_single.xml")
                    .expect("failed to load test data"),
            )
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

    #[tokio::test]
    async fn get_from_query() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let api = BoardGameGeekApi {
            base_url: &url,
            client: reqwest::Client::new(),
        };
        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "someone".into()),
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
        mock.assert();
    }

    #[tokio::test]
    async fn get_by_player_counts() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let api = BoardGameGeekApi {
            base_url: &url,
            client: reqwest::Client::new(),
        };
        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "someone".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
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
        mock.assert();

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
                stats: Some(CollectionItemStats {
                    min_players: 3,
                    max_players: 16,
                    min_playtime: Duration::minutes(90),
                    max_playtime: Duration::minutes(90),
                    playing_time: Duration::minutes(90),
                    owned_by: 14400,
                    rating: CollectionItemRating {
                        value: Some(7.0),
                        users_rated: 8097,
                        average: 5.84098,
                        bayesian_average: 5.71005,
                        standard_deviation: 1.58457,
                        median: 0.0,
                        ranks: vec![
                            GameTypeRank {
                                game_type: "subtype".into(),
                                id: 1,
                                name: "boardgame".into(),
                                friendly_name: "Board Game Rank".into(),
                                value: RankValue::Ranked(5587),
                                bayesian_average: 5.71005,
                            },
                            GameTypeRank {
                                game_type: "family".into(),
                                id: 5498,
                                name: "partygames".into(),
                                friendly_name: "Party Game Rank".into(),
                                value: RankValue::Ranked(563),
                                bayesian_average: 5.65053,
                            }
                        ],
                    }
                }),
            },
            "returned collection game doesn't match expected",
        );

        // Looking for a game that supports any number of players between 1 and 16. All 37 games in the collection should be returned.
        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "someone".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
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
        mock.assert();

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.items.len(), 37);

        // Looking for a game that supports 17 players, not in the collection. Nothing should be returned.
        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "someone".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
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
        mock.assert();

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.items.len(), 0);
    }

    #[tokio::test]
    async fn get_by_player_count() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let api = BoardGameGeekApi {
            base_url: &url,
            client: reqwest::Client::new(),
        };
        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "someone".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
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
        mock.assert();

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
                stats: Some(CollectionItemStats {
                    min_players: 3,
                    max_players: 16,
                    min_playtime: Duration::minutes(90),
                    max_playtime: Duration::minutes(90),
                    playing_time: Duration::minutes(90),
                    owned_by: 14400,
                    rating: CollectionItemRating {
                        value: Some(7.0),
                        users_rated: 8097,
                        average: 5.84098,
                        bayesian_average: 5.71005,
                        standard_deviation: 1.58457,
                        median: 0.0,
                        ranks: vec![
                            GameTypeRank {
                                game_type: "subtype".into(),
                                id: 1,
                                name: "boardgame".into(),
                                friendly_name: "Board Game Rank".into(),
                                value: RankValue::Ranked(5587),
                                bayesian_average: 5.71005,
                            },
                            GameTypeRank {
                                game_type: "family".into(),
                                id: 5498,
                                name: "partygames".into(),
                                friendly_name: "Party Game Rank".into(),
                                value: RankValue::Ranked(563),
                                bayesian_average: 5.65053,
                            }
                        ],
                    }
                }),
            },
            "returned collection game doesn't match expected",
        );

        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "someone".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
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
        mock.assert();

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.items.len(), 30);
        for item in collection.items {
            assert!(
                item.stats.as_ref().unwrap().min_players <= 2
                    && item.stats.unwrap().max_players >= 2
            )
        }

        // Looking for a game that supports 17 players, not in the collection. Nothing should be returned.
        let mock = server
            .mock("GET", "/collection")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("username".into(), "someone".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
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
        mock.assert();

        assert!(collection.is_ok(), "error returned when okay expected");
        let collection = collection.unwrap();

        assert_eq!(collection.items.len(), 0);
    }
}
