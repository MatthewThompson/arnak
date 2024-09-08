use core::fmt;

use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;

use super::{CollectionItemType, GameFamilyRank, GameVersion, VersionsXml};
use crate::utils::{
    date_deserializer, deserialize_1_0_bool, deserialize_minutes, XmlFloatValue, XmlIntValue,
};
use crate::XmlRanks;

/// A user's collection on boardgamegeek.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Collection<T> {
    /// List of games, expansions, and accessories in the user's collection. Each game
    /// is not necessarily owned but can be preowned, on the user's wishlist etc.
    ///
    /// Note that accessories and games can never be returned together in one collection,
    /// but games and game expansions can.
    #[serde(default = "Vec::new", rename = "item")]
    pub items: Vec<T>,
}

/// An item in a collection, in brief form. With the name, status, type,
/// and IDs, also a brief version of the game stats.
///
/// If requested and applicable, version information is also included,
/// this will be the same information as is included in the full version.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CollectionItemBrief {
    /// The ID of the item.
    #[serde(rename = "objectid")]
    pub id: u64,
    /// The collection ID of the object.
    #[serde(rename = "collid")]
    pub collection_id: u64,
    /// The type of collection item, which will either be boardgame, expansion, or accessory.
    #[serde(rename = "subtype")]
    pub item_type: CollectionItemType,
    /// The name of the item.
    pub name: String,
    /// Status of the item in this collection, such as own, preowned, wishlist.
    pub status: CollectionItemStatus,
    /// Game stats such as number of players.
    pub stats: CollectionItemStatsBrief,
    /// Information about this version of the game. Only included if version
    /// information is requested and also if the game is an alternate
    /// version of another game.
    #[serde(default, deserialize_with = "deserialize_version_list")]
    pub version: Option<GameVersion>,
}

/// A game, game expansion, or game accessory in a collection.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CollectionItem {
    /// The ID of the item.
    #[serde(rename = "objectid")]
    pub id: u64,
    /// The collection ID of the object.
    #[serde(rename = "collid")]
    pub collection_id: u64,
    /// The type of collection item, which will either be boardgame, expansion, or accessory.
    #[serde(rename = "subtype")]
    pub item_type: CollectionItemType,
    /// The name of the item.
    pub name: String,
    /// The year the item was first published.
    #[serde(rename = "yearpublished")]
    pub year_published: i64,
    /// A link to a jpg image for the item.
    pub image: String,
    /// A link to a jpg thumbnail image for the item.
    pub thumbnail: String,
    /// Status of the item in this collection, such as own, preowned, wishlist.
    pub status: CollectionItemStatus,
    /// The number of times the user has played the game.
    #[serde(rename = "numplays")]
    pub number_of_plays: u64,
    /// Game stats such as number of players.
    pub stats: CollectionItemStats,
    /// Information about this version of the game. Only included if version
    /// information is requested and also if the game is an alternate
    /// version of another game.
    #[serde(default, deserialize_with = "deserialize_version_list")]
    pub version: Option<GameVersion>,
}

/// The status of the item in the user's collection, such as preowned or
/// wishlist. Can be any or none of them.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CollectionItemStatus {
    /// User owns the item.
    #[serde(deserialize_with = "deserialize_1_0_bool")]
    pub own: bool,
    /// User has previously owned the item.
    #[serde(rename = "prevowned", deserialize_with = "deserialize_1_0_bool")]
    pub previously_owned: bool,
    /// User wants to trade away the item.
    #[serde(rename = "fortrade", deserialize_with = "deserialize_1_0_bool")]
    pub for_trade: bool,
    /// User wants to receive the item in a trade.
    #[serde(rename = "want", deserialize_with = "deserialize_1_0_bool")]
    pub want_in_trade: bool,
    /// User wants to play the item.
    #[serde(rename = "wanttoplay", deserialize_with = "deserialize_1_0_bool")]
    pub want_to_play: bool,
    /// User wants to buy the item.
    #[serde(rename = "wanttobuy", deserialize_with = "deserialize_1_0_bool")]
    pub want_to_buy: bool,
    /// User pre-ordered the item.
    #[serde(rename = "preordered", deserialize_with = "deserialize_1_0_bool")]
    pub pre_ordered: bool,
    /// User has the item on their wishlist.
    #[serde(deserialize_with = "deserialize_1_0_bool")]
    pub wishlist: bool,
    /// The priority of the wishlist.
    #[serde(default, rename = "wishlistpriority")]
    pub wishlist_priority: Option<WishlistPriority>,
    /// When the collection status was last modified.
    #[serde(rename = "lastmodified", with = "date_deserializer")]
    pub last_modified: DateTime<Utc>,
}

/// The status of the item in the user's collection, such as preowned or
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

pub(crate) fn deserialize_version_list<'de, D>(
    deserializer: D,
) -> Result<Option<GameVersion>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    // If the tag is missing the None case is already handled by the `#serde[(default)]` on
    // the type. If this deserialize returns an error it should be propagated.
    let mut versions: VersionsXml = serde::de::Deserialize::deserialize(deserializer)?;

    match versions.versions.len() {
        0 => Err(serde::de::Error::custom(format!(
            "empty version list found for game ID {}, expected \"1\"",
            versions.versions[0].id,
        ))),
        1 => Ok(Some(versions.versions.remove(0))),
        len => Err(serde::de::Error::custom(format!(
            "invalid number of versions found for game ID {}, expected \"1\", but got {}",
            versions.versions[0].id, len,
        ))),
    }
}

/// Stats of the game such as player count and duration. Can be omitted from the
/// response. More stats can be found from the specific game endpoint.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CollectionItemStatsBrief {
    /// Minimum players the game supports.
    #[serde(default, rename = "minplayers")]
    pub min_players: u32,
    /// Maximum players the game supports.
    #[serde(default, rename = "maxplayers")]
    pub max_players: u32,
    /// Minimum amount of time the game is suggested to take to play.
    #[serde(
        default,
        rename = "minplaytime",
        deserialize_with = "deserialize_minutes"
    )]
    pub min_playtime: Duration,
    /// Maximum amount of time the game is suggested to take to play.
    #[serde(
        default,
        rename = "maxplaytime",
        deserialize_with = "deserialize_minutes"
    )]
    pub max_playtime: Duration,
    /// The amount of time the game is suggested to take to play.
    #[serde(
        default,
        rename = "playingtime",
        deserialize_with = "deserialize_minutes"
    )]
    pub playing_time: Duration,
    /// The number of people that own this game.
    #[serde(rename = "numowned")]
    pub owned_by: u64,
    /// Information about the rating that this user, as well as all users, have
    /// given this game.
    pub rating: CollectionItemRatingBrief,
}

/// Stats of the game such as the player count and duration. Can be omitted from the
/// response. More stats can be found from the specific game endpoint.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CollectionItemStats {
    /// Minimum players the game supports.
    #[serde(default, rename = "minplayers")]
    pub min_players: u32,
    /// Maximum players the game supports.
    #[serde(default, rename = "maxplayers")]
    pub max_players: u32,
    /// Minimum amount of time the game is suggested to take to play.
    #[serde(
        default,
        rename = "minplaytime",
        deserialize_with = "deserialize_minutes"
    )]
    pub min_playtime: Duration,
    /// Maximum amount of time the game is suggested to take to play.
    #[serde(
        default,
        rename = "maxplaytime",
        deserialize_with = "deserialize_minutes"
    )]
    pub max_playtime: Duration,
    /// The amount of time the game is suggested to take to play.
    #[serde(
        default,
        rename = "playingtime",
        deserialize_with = "deserialize_minutes"
    )]
    pub playing_time: Duration,
    /// The number of people that own this game.
    #[serde(rename = "numowned")]
    pub owned_by: u64,
    /// Information about the rating that this user, as well as all users, have
    /// given this game.
    #[serde(rename = "rating")]
    pub rating: CollectionItemRating,
}

/// The 0-10 rating that the user gave to this item. Also includes the total
/// number of users that have rated it, as well as the averages.
#[derive(Clone, Debug, PartialEq)]
pub struct CollectionItemRatingBrief {
    /// The 0-10 rating that the user gave this item.
    pub user_rating: Option<f64>,
    /// The mean average rating for this item.
    pub average: f64,
    /// The bayesian average rating for this item.
    pub bayesian_average: f64,
}

impl<'de> Deserialize<'de> for CollectionItemRatingBrief {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Value,
            Average,
            BayesAverage,
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
                        Field::BayesAverage => {
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

/// The 0-10 rating that the user gave to this item. Also includes the total
/// number of users that have rated it, as well as the averages, and standard
/// deviation.
#[derive(Clone, Debug, PartialEq)]
pub struct CollectionItemRating {
    /// The 0-10 rating that the user gave this item.
    pub user_rating: Option<f64>,
    /// The total number of users who have given this item a rating.
    pub users_rated: u64,
    /// The mean average rating for this item.
    pub average: f64,
    /// The bayesian average rating for this item.
    pub bayesian_average: f64,
    /// The standard deviation of the average rating.
    pub standard_deviation: f64,
    // Kept private for now since the API always returns 0 for this seemingly.
    pub(crate) median: f64,
    /// The list of ranks the item is on the site within each of its item types.
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
            StdDev,
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
                        Field::StdDev => {
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
                            let ranks_xml: XmlRanks = map.next_value()?;
                            ranks = Some(ranks_xml.ranks);
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

/// Type of game family.
///
/// [`GameFamilyType::Subtype`] is used for the `boardgame` family that includes all games.
/// [`GameFamilyType::Family`] is used for everything else. Such as party games or strategy games.
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
/// either Ranked with a u64 for the rank, Or `NotRanked`.
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
