use core::fmt;

use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;

use crate::utils::{
    date_deserializer, deserialize_1_0_bool, deserialize_minutes, XmlFloatValue, XmlIntValue,
};

/// A user's collection on boardgamegeek.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Collection<T> {
    /// List of games and expansions in the user's collection. Each item
    /// is not necessarily owned but can be preowned, wishlisted etc.
    #[serde(rename = "$value")]
    pub games: Vec<T>,
}

/// An item in a collection, in brief form. With only the name, status, type,
/// and IDs.
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
    pub item_type: GameType,
    /// The name of the game.
    pub name: String,
    /// Status of the game in this collection, such as own, preowned, wishlist.
    pub status: CollectionItemStatus,
    /// Game stats such as number of players.
    pub stats: CollectionItemStatsBrief,
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
    pub item_type: GameType,
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
    /// Game stats such as number of players.
    pub stats: CollectionItemStats,
}

/// The type of game, board game or expansion.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum GameType {
    /// A board game, or expansion.
    ///
    /// Due to the way the API works, this type can include expansions too.
    /// If a request is made for just board games, or the game type is not
    /// filtered, then both games with a type of [GameType::BoardGame] and
    /// those with a type of [GameType::BoardGameExpansion] will be returned,
    /// and they will ALL have the type of [GameType::BoardGame]. However when
    /// requesting just expansions, the returned games will correctly have the
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

/// The status of the game in the user's collection, such as preowned or
/// wishlist. Can be any or none of them.
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
    #[serde(default, rename = "wishlistpriority")]
    pub wishlist_priority: Option<WishlistPriority>,
    /// When the collection status was last modified
    #[serde(rename = "lastmodified", with = "date_deserializer")]
    pub last_modified: DateTime<Utc>,
}

/// The status of the game in the user's collection, such as preowned or
/// wishlist. Can be any or none of them.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
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

impl<'de> Deserialize<'de> for WishlistPriority {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = serde::de::Deserialize::deserialize(deserializer)?;

        match s.as_str() {
            "5" => Ok(WishlistPriority::DontBuyThis),
            "4" => Ok(WishlistPriority::ThinkingAboutIt),
            "3" => Ok(WishlistPriority::LikeToHave),
            "2" => Ok(WishlistPriority::LoveToHave),
            "1" => Ok(WishlistPriority::MustHave),
            s => Err(serde::de::Error::custom(format!(
                "invalid value for wishlist priority, expected \"1\" - \"5\" but got {s}"
            ))),
        }
    }
}

/// Stats of the game such as playercount and duration. Can be omitted from the
/// response. More stats can be found from the specific game endpoint.
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
    /// Information about the rating that this user, as well as all users, have
    /// given this game.
    #[serde(rename = "$value")]
    pub rating: CollectionItemRatingBrief,
}

/// Stats of the game such as playercount and duration. Can be omitted from the
/// response. More stats can be found from the specific game endpoint.
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
    /// Information about the rating that this user, as well as all users, have
    /// given this game.
    #[serde(rename = "$value")]
    pub rating: CollectionItemRating,
}

/// The 0-10 rating that the user gave to this game. Also includes the total
/// number of users that have rated it, as well as the averages.
#[derive(Clone, Debug, PartialEq)]
pub struct CollectionItemRatingBrief {
    /// The 0-10 rating that the user gave this game.
    pub user_rating: Option<f64>,
    /// The mean average rating for this game.
    pub average: f64,
    /// The bayesian average rating for this game.
    pub bayesian_average: f64,
}

impl<'de> Deserialize<'de> for CollectionItemRatingBrief {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Value,
            Average,
            Bayesaverage,
        }

        struct CollectionItemRatingBriefVisitor;

        impl<'de> serde::de::Visitor<'de> for CollectionItemRatingBriefVisitor {
            type Value = CollectionItemRatingBrief;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string containing the XML for a user's rating of a board game, which includes the average rating on the site and the number of ratings.")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut user_rating = None;
                let mut average = None;
                let mut bayesian_average = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Value => {
                            if user_rating.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            let user_rating_str: String = map.next_value()?;
                            user_rating = match user_rating_str.as_str() {
                                "N/A" => Some(None),
                                other => Some(Some(other.parse::<f64>().map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "failed to parse value as N/A or float: {e}"
                                    ))
                                })?)),
                            }
                        },
                        Field::Average => {
                            if average.is_some() {
                                return Err(serde::de::Error::duplicate_field("average"));
                            }
                            let average_xml_tag: XmlFloatValue = map.next_value()?;
                            average = Some(average_xml_tag.value);
                        },
                        Field::Bayesaverage => {
                            if bayesian_average.is_some() {
                                return Err(serde::de::Error::duplicate_field("bayesaverage"));
                            }
                            let bayesian_average_xml_tag: XmlFloatValue = map.next_value()?;
                            bayesian_average = Some(bayesian_average_xml_tag.value);
                        },
                    }
                }
                let user_rating =
                    user_rating.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                let average = average.ok_or_else(|| serde::de::Error::missing_field("average"))?;
                let bayesian_average = bayesian_average
                    .ok_or_else(|| serde::de::Error::missing_field("bayesaverage"))?;
                Ok(Self::Value {
                    user_rating,
                    average,
                    bayesian_average,
                })
            }
        }
        deserializer.deserialize_any(CollectionItemRatingBriefVisitor)
    }
}

/// The 0-10 rating that the user gave to this game. Also includes the total
/// number of users that have rated it, as well as the averages, and standard
/// deviation.
#[derive(Clone, Debug, PartialEq)]
pub struct CollectionItemRating {
    /// The 0-10 rating that the user gave this game.
    pub user_rating: Option<f64>,
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
    pub ranks: Vec<GameFamilyRank>,
}

impl<'de> Deserialize<'de> for CollectionItemRating {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Value,
            UsersRated,
            Average,
            Bayesaverage,
            Stddev,
            Median,
            Ranks,
        }

        struct CollectionItemRatingVisitor;

        impl<'de> serde::de::Visitor<'de> for CollectionItemRatingVisitor {
            type Value = CollectionItemRating;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string containing the XML for a user's rating of a board game, which includes the average rating on the site and the number of ratings.")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut user_rating = None;
                let mut users_rated = None;
                let mut average = None;
                let mut bayesian_average = None;
                let mut standard_deviation = None;
                let mut median = None;
                let mut ranks = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Value => {
                            if user_rating.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            let user_rating_str: String = map.next_value()?;
                            user_rating = match user_rating_str.as_str() {
                                "N/A" => Some(None),
                                other => Some(Some(other.parse::<f64>().map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "failed to parse value as N/A or float: {e}"
                                    ))
                                })?)),
                            }
                        },
                        Field::UsersRated => {
                            if users_rated.is_some() {
                                return Err(serde::de::Error::duplicate_field("usersrated"));
                            }
                            let users_rated_xml_tag: XmlIntValue = map.next_value()?;
                            users_rated = Some(users_rated_xml_tag.value);
                        },
                        Field::Average => {
                            if average.is_some() {
                                return Err(serde::de::Error::duplicate_field("average"));
                            }
                            let average_xml_tag: XmlFloatValue = map.next_value()?;
                            average = Some(average_xml_tag.value);
                        },
                        Field::Bayesaverage => {
                            if bayesian_average.is_some() {
                                return Err(serde::de::Error::duplicate_field("bayesaverage"));
                            }
                            let bayesian_average_xml_tag: XmlFloatValue = map.next_value()?;
                            bayesian_average = Some(bayesian_average_xml_tag.value);
                        },
                        Field::Stddev => {
                            if standard_deviation.is_some() {
                                return Err(serde::de::Error::duplicate_field("stddev"));
                            }
                            let standard_deviation_xml_tag: XmlFloatValue = map.next_value()?;
                            standard_deviation = Some(standard_deviation_xml_tag.value);
                        },
                        Field::Median => {
                            if median.is_some() {
                                return Err(serde::de::Error::duplicate_field("median"));
                            }
                            let median_xml_tag: XmlFloatValue = map.next_value()?;
                            median = Some(median_xml_tag.value);
                        },
                        Field::Ranks => {
                            if ranks.is_some() {
                                return Err(serde::de::Error::duplicate_field("ranks"));
                            }
                            // An extra layer of indirection is needed due to the way the XML is
                            // structured, but should be removed for the final
                            // structure.
                            let ranks_struct: Ranks = map.next_value()?;
                            ranks = Some(ranks_struct.ranks);
                        },
                    }
                }
                let user_rating =
                    user_rating.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                let users_rated =
                    users_rated.ok_or_else(|| serde::de::Error::missing_field("usersrated"))?;
                let average = average.ok_or_else(|| serde::de::Error::missing_field("average"))?;
                let bayesian_average = bayesian_average
                    .ok_or_else(|| serde::de::Error::missing_field("bayesaverage"))?;
                let standard_deviation =
                    standard_deviation.ok_or_else(|| serde::de::Error::missing_field("stddev"))?;
                let median = median.ok_or_else(|| serde::de::Error::missing_field("median"))?;
                let ranks = ranks.ok_or_else(|| serde::de::Error::missing_field("ranks"))?;
                Ok(Self::Value {
                    user_rating,
                    users_rated,
                    average,
                    bayesian_average,
                    standard_deviation,
                    median,
                    ranks,
                })
            }
        }
        deserializer.deserialize_any(CollectionItemRatingVisitor)
    }
}

// Intermediary struct needed due to the way the XML is strcutured
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct Ranks {
    #[serde(rename = "$value")]
    pub(crate) ranks: Vec<GameFamilyRank>,
}

/// A struct containing the game's rank within a particular type of game.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GameFamilyRank {
    /// The type of this group of games. Can be `subtype` for rank within all
    /// board games. Or it can be `family` if it is the rank within a family of games
    /// such as party games or strategy games.
    #[serde(rename = "type")]
    pub game_family_type: GameFamilyType,
    /// ID of the game family.
    pub id: u64,
    /// Name of the game type. "boardgame" used as the generic subtype that
    /// includes all board games.
    pub name: String,
    /// User friendly name in the foramt "GENRE game rank" e.g. "Party Game
    /// Rank"
    #[serde(rename = "friendlyname")]
    pub friendly_name: String,
    /// The overall rank on the site within this type of game.
    pub value: RankValue,
    /// The score out of 10, as a bayseian average.
    ///
    /// This is what boardgamegeek calls a Geek Rating. It is the average rating
    /// that the users have given it along with a few thousand 5.5 ratings added
    /// in too.
    #[serde(rename = "bayesaverage")]
    pub bayesian_average: f64,
}

/// Type of game family,  [GameFamilyType::Subtype] is used for the `boardgame` family that includes
/// all games. [GameFamilyType::Family] is used for everything else. Such as party games
/// or strategy games.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum GameFamilyType {
    /// Used only for the generic `boardgame` family that includes all games.
    #[serde(rename = "subtype")]
    Subtype,
    /// Used for all families of games such as party games and strategy games.
    #[serde(rename = "family")]
    Family,
}

/// A rank a particular board game has on the site, within a subtype. Can be
/// either Ranked with a u64 for the rank, Or NotRanked.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RankValue {
    /// The rank of a game within a particular family of games, or all games. Where
    /// 1 means that it has the highest overall rank of every game in that category.
    Ranked(u64),
    /// The game does not have a rank in a given category, possibly due to not having
    /// enough ratings.
    NotRanked,
}

impl<'de> Deserialize<'de> for RankValue {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s: String = serde::de::Deserialize::deserialize(deserializer)?;
        if s == "Not Ranked" {
            return Ok(RankValue::NotRanked);
        }

        let rank: Result<u64, _> = s.parse();
        match rank {
            Ok(value) => Ok(RankValue::Ranked(value)),
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &["u64", "Not Ranked"],
            )),
        }
    }
}
