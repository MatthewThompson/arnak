use core::fmt;

use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;

use super::{
    Game, GameAccessory, GameArtist, GameCategory, GameCompilation, GameDesigner, GameFamilyName,
    GameImplementation, GameMechanic, GamePublisher, GameType, GameVersion, ItemFamilyRank, User,
};
use crate::deserialize::{
    date_time_with_zone_from_string, xml_ranks_to_ranks, XmlDateTimeValue, XmlFloatValue,
    XmlIntValue, XmlLink, XmlName, XmlRanks, XmlSignedValue, XmlStringValue,
};
use crate::{NameType, VersionsXml};

// A struct containing the list of requested games with the full details.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Games {
    // List of games.
    #[serde(default, rename = "item")]
    pub(crate) games: Vec<GameDetails>,
}

/// A game, or expansion, with full details.
///
/// A game returned from the game endpoint which includes all details about a game.
/// This includes the name, description, image and thumbnail. As well as lists of all
/// the alternate names, expansions, artists and publishers.
///
/// Some information, such as version info, comments, and marketplace data is only
/// optionally included if requested.
#[derive(Clone, Debug, PartialEq)]
pub struct GameDetails {
    /// The ID of the game.
    pub id: u64,
    /// The type of the game, whether it is an expansion or not.
    pub game_type: GameType,
    /// The name of the game.
    pub name: String,
    /// A list of alternate names for the game, usually translations of the primary name.
    pub alternate_names: Vec<String>,
    /// A brief description of the game.
    pub description: String,
    /// A link to a jpg image for the game.
    pub image: String,
    /// A link to a jpg thumbnail image for the game.
    pub thumbnail: String,
    /// The year the game was first published.
    pub year_published: i64,
    /// The minimum number of players the game supports.
    pub min_players: u64,
    /// The maximum number of players the game supports.
    pub max_players: u64,
    /// The suggested number of players as selected by voters on the site.
    ///
    /// Poll results for whether each number of players is recommended, not recommended,
    /// or best. Includes options outside of the suggested minimum and maximum player counts.
    pub suggested_player_count: SuggestedPlayerCountPoll,
    /// The amount of time the game is suggested to take to play.
    pub playing_time: Duration,
    /// Minimum amount of time the game is suggested to take to play.
    pub min_play_time: Duration,
    /// Maximum amount of time the game is suggested to take to play.
    pub max_play_time: Duration,
    /// The minimum suggested age suitable for playing this game.
    pub min_age: u64,
    /// The suggested number minimum suitable age for playing this game.
    ///
    /// Poll results for at which age is a suitable minimum age for playing this game.
    pub suggested_player_age: SuggestedPlayerAgePoll,
    /// The suggested dependence on knowing the game's language in order to be able to play it.
    ///
    /// Poll results, with five options, for whether the game can be played without knowing the
    /// language, if it does not have much in game text. Through being completely unplayable
    /// due to extensive text in the game.
    pub suggested_language_dependence: LanguageDependencePoll,
    // Categories and mechanics have IDs too, but I don't think it would be beneficial to include
    // them over just the names.
    /// A list of category names that this game belongs to.
    pub categories: Vec<GameCategory>,
    /// A list of mechanic names that this game belongs to.
    pub mechanics: Vec<GameMechanic>,
    /// A list of game families names that this game belongs to.
    pub game_families: Vec<GameFamilyName>,
    /// A list of expansions to this game.
    pub expansions: Vec<Game>,
    /// A list of games that this game is an expansion for.
    pub expansion_for: Vec<Game>,
    /// A list of accessories specific to this game.
    pub accessories: Vec<GameAccessory>,
    /// A list of compilations for this game.
    pub compilations: Vec<GameCompilation>,
    /// A list of reimplementations of this game.
    pub reimplementations: Vec<GameImplementation>,
    /// The designer of this game.
    pub designers: Vec<GameDesigner>,
    /// A list of artists for this game.
    pub artists: Vec<GameArtist>,
    /// The list of publishers for this game.
    pub publishers: Vec<GamePublisher>,
    /// Rating statistics for this game.
    ///
    /// Includes the game user average rating, as well as the rank and rating
    /// within different families.
    ///
    /// Also includes the number of users on the site who own the game, as well
    /// as have it as other collection statuses.
    pub stats: GameStats,
    /// Information for the various versions of the game.
    pub versions: Vec<GameVersion>,
    /// User uploaded videos that relate to this game.
    pub videos: Vec<Video>,
    /// Information of where to buy the game and for how much.
    pub marketplace_listings: Vec<MarketplaceListing>,
    /// List of comments and ratings users have given to the game.
    ///
    /// Each comment may have a rating but no comment, a comment but no rating, or both. However
    /// the underlying API will only return all comments, whether or not they have ratings,
    /// with the `include_comments` query parameter. Or all ratings, whether or not they have
    /// comments. However the same tag is used to return them so there is no way to return all
    /// comments and ratings together.
    ///
    /// Page number and page size can be controlled via query parameters.
    pub rating_comments: Option<RatingCommentPage>,
}

/// Various statistics for the game, including the number of users who own the game as
/// well as the ratings.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GameStats {
    /// The number of users who have rated this game.
    pub users_rated: u64,
    /// The average rating from users, from 0-10.
    pub average_rating: f64,
    /// The score out of 10, as a bayesian average.
    ///
    /// This is what boardgamegeek calls a Geek Rating. It is the average rating
    /// that the users have given it along with a few thousand 5.5 ratings added
    /// in too.
    pub bayesian_average: f64,
    /// The standard deviation of the ratings.
    pub standard_deviation: f64,
    // Private because it's always 0.
    pub(crate) median: f64,
    /// The rank of this game amongst all games.
    pub rank: ItemFamilyRank,
    /// The list of ranks the game is on the site within various game families, such as family or
    /// strategy games.
    pub sub_family_ranks: Vec<ItemFamilyRank>,
    /// The number of users who own this game.
    pub users_owned: u64,
    /// The number of users who are trading away this game.
    pub users_trading: u64,
    /// The number of users who want to get this game in a trade.
    pub users_want_in_trade: u64,
    /// The number of users who have this game on their wishlist.
    pub users_wishlisted: u64,
    /// The number of comments made by users on this game.
    pub number_of_comments: u64,
    /// The total number of weight ratings given to the game by users.
    pub number_of_weights: u64,
    /// The average weight score given to the game.
    ///
    /// The weight is a score of complexity from 0-5, where the most complex games will have a
    /// weight of closer to 5.
    pub weight_rating: f64,
}

// Structure of the stats tag in the returned XML
#[derive(Debug, Deserialize)]
struct XmlGameStats {
    ratings: StatsRatings,
}

// Structure of the ratings tag in the returned XML
#[derive(Debug, Deserialize)]
struct StatsRatings {
    usersrated: XmlIntValue,
    average: XmlFloatValue,
    bayesaverage: XmlFloatValue,
    stddev: XmlFloatValue,
    median: XmlFloatValue,
    ranks: XmlRanks,
    owned: XmlIntValue,
    trading: XmlIntValue,
    wishing: XmlIntValue,
    wanting: XmlIntValue,
    numcomments: XmlIntValue,
    numweights: XmlIntValue,
    averageweight: XmlFloatValue,
}

/// A user answered poll for how many players this game is best suited for.
///
/// Options typically include all player counts from 1 (even if the minimum player count is more
/// than 1) all the way to the max player count and then an option for playing with more than the
/// max.
///
/// For each player count users can vote on whether the option is not recommended, recommended, or
/// best.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SuggestedPlayerCountPoll {
    /// User friendly name for the poll.
    pub title: String,
    /// Total number of users who voted on this poll.
    pub total_voters: u64,
    /// Results for this poll, contains a separate vote for each player count option, including an
    /// option for the max player count or above.
    pub results: Vec<SuggestedPlayerCount>,
}

/// A suggested player count, along with community votes as to whether it is recommended or not.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SuggestedPlayerCount {
    /// The number of players this vote result is for.
    pub player_count: PlayerCount,
    /// How many users voted this player count as the best option.
    pub best_votes: u64,
    /// How many users voted this player count as recommended.
    pub recommended_votes: u64,
    /// How many users voted this player count as not recommended.
    pub not_recommended_votes: u64,
}

impl TryFrom<Poll> for SuggestedPlayerCountPoll {
    // String as the error type, because this will be wrapped in a serde deserialize error during
    // the deserialise step.
    type Error = String;

    fn try_from(poll: Poll) -> Result<Self, Self::Error> {
        Ok(SuggestedPlayerCountPoll {
            title: poll.title,
            total_voters: poll.total_voters,
            results: poll
                .results
                .into_iter()
                .map(SuggestedPlayerCount::try_from)
                .collect::<Result<Vec<SuggestedPlayerCount>, Self::Error>>()?,
        })
    }
}

impl TryFrom<PollResults> for SuggestedPlayerCount {
    // String as the error type, because this will be wrapped in a serde deserialize error during
    // the deserialise step.
    type Error = String;

    fn try_from(results: PollResults) -> Result<Self, Self::Error> {
        let mut best_votes = None;
        let mut recommended_votes = None;
        let mut not_recommended_votes = None;
        for vote_result in results.results {
            match vote_result.value.as_str() {
                "Best" => {
                    best_votes = Some(vote_result.number_of_votes);
                },
                "Recommended" => {
                    recommended_votes = Some(vote_result.number_of_votes);
                },
                "Not Recommended" => {
                    not_recommended_votes = Some(vote_result.number_of_votes);
                },
                unexpected => {
                    return Err(format!("unexpected player count vote option: {unexpected}",));
                },
            }
        }
        Ok(SuggestedPlayerCount {
            player_count: results
                .number_of_players
                .ok_or("player count was `None` when `Some` was expected")?,
            best_votes: best_votes.ok_or("value `Best` missing for player count vote")?,
            recommended_votes: recommended_votes
                .ok_or("value `Recommended` missing for player count vote")?,
            not_recommended_votes: not_recommended_votes
                .ok_or("value `Not Recommended` missing for player count vote")?,
        })
    }
}

/// A number of players for the purpose of voting on what the best player count for a game may be.
///
/// Can be either an exact number of players or a number or above. Voting options typically contain
/// from 1 all the way to the max player count, and then an option for max player count or above.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerCount {
    /// An exact number of players.
    Players(u64),
    /// This number of players or above.
    PlayersOrAbove(u64),
}

impl<'de> Deserialize<'de> for PlayerCount {
    fn deserialize<D>(deserializer: D) -> Result<PlayerCount, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s: String = serde::de::Deserialize::deserialize(deserializer)?;
        if s.is_empty() {
            return Err(serde::de::Error::custom(
                "expected player count but got empty string",
            ));
        }
        match s.chars().last() {
            Some('+') => {
                let players = s.replace('+', "").parse::<u64>().map_err(|e| {
                    serde::de::Error::custom(format!("unable to parse player count to u64: {e}"))
                })?;
                Ok(PlayerCount::PlayersOrAbove(players))
            },
            Some(_) => {
                let players = s.parse::<u64>().map_err(|e| {
                    serde::de::Error::custom(format!("unable to parse player count to u64: {e}"))
                })?;
                Ok(PlayerCount::Players(players))
            },
            None => Err(serde::de::Error::custom(
                "expected player count but got empty string",
            )),
        }
    }
}

/// A user answered poll for the minimum player age this game is best suited for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SuggestedPlayerAgePoll {
    /// User friendly name for the poll.
    pub title: String,
    /// Total number of users who voted on this poll.
    pub total_voters: u64,
    /// Results for this poll, contains a separate vote for each age from 2, going up in 2s from 6.
    /// And an option for 21 and up.
    pub results: Vec<SuggestedPlayerAge>,
}

/// A suggested minimum player age, along with how many users voted for this age.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SuggestedPlayerAge {
    /// The number of players this vote result is for.
    pub player_age: PlayerAge,
    /// How many users voted this player age as the most suitable minimum age to play this game.
    pub votes: u64,
}

/// A minimum suitable age for playing a game.
///
/// Can be either an exact number age or a number or above.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerAge {
    /// An exact player age.
    Age(u64),
    /// This age or above.
    AgeOrAbove(u64),
}

impl TryFrom<Poll> for SuggestedPlayerAgePoll {
    // String as the error type, because this will be wrapped in a serde deserialize error during
    // the deserialise step.
    type Error = String;

    fn try_from(mut poll: Poll) -> Result<Self, Self::Error> {
        if poll.results.len() != 1 {
            return Err(format!(
                "expected 1 set of results but got {}",
                poll.results.len(),
            ));
        }
        let results = poll.results.remove(0).results;
        Ok(SuggestedPlayerAgePoll {
            title: poll.title,
            total_voters: poll.total_voters,
            results: results
                .into_iter()
                .map(|result| {
                    Ok(SuggestedPlayerAge {
                        player_age: result.value.try_into()?,
                        votes: result.number_of_votes,
                    })
                })
                .collect::<Result<Vec<SuggestedPlayerAge>, Self::Error>>()?,
        })
    }
}

impl TryFrom<String> for PlayerAge {
    // String as the error type, because this will be wrapped in a serde deserialize error during
    // the deserialise step.
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.strip_suffix(" and up") {
            Some(age_string) => {
                let player_age = age_string
                    .parse::<u64>()
                    .map_err(|e| format!("unable to parse player age to u64: {e}"))?;
                Ok(PlayerAge::AgeOrAbove(player_age))
            },
            None => {
                let player_age = s
                    .parse::<u64>()
                    .map_err(|e| format!("unable to parse player count to u64: {e}"))?;
                Ok(PlayerAge::Age(player_age))
            },
        }
    }
}

/// A user answered poll for how playable the game would be, should the player not speak the
/// language.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanguageDependencePoll {
    /// User friendly name for the poll.
    pub title: String,
    /// Total number of users who voted on this poll.
    pub total_voters: u64,
    /// Results for this poll, contains 5 levels of severity ranging from no necessary in game text
    /// to being unplayable in another language.
    pub results: Vec<LanguageDependence>,
}

/// A suggested minimum player age, along with how many users voted for this age.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanguageDependence {
    /// Level of dependence where the higher the value the more dependent the game is on knowing
    /// the language.
    pub level: u64,
    /// Description of how dependent the game is on the language used.
    pub dependence: String,
    /// How many users voted that the game has this level of language dependence.
    pub votes: u64,
}

impl TryFrom<Poll> for LanguageDependencePoll {
    // String as the error type, because this will be wrapped in a serde deserialize error during
    // the deserialise step.
    type Error = String;

    fn try_from(mut poll: Poll) -> Result<Self, Self::Error> {
        if poll.results.len() != 1 {
            return Err(format!(
                "expected 1 set of results but got {}",
                poll.results.len()
            ));
        }
        let results = poll.results.remove(0).results;
        Ok(LanguageDependencePoll {
            title: poll.title,
            total_voters: poll.total_voters,
            results: results
                .into_iter()
                .map(|result| {
                    Ok(LanguageDependence {
                        level: result.level.ok_or("missing language dependence level")?,
                        dependence: result.value,
                        votes: result.number_of_votes,
                    })
                })
                .collect::<Result<Vec<LanguageDependence>, Self::Error>>()?,
        })
    }
}

// A poll for users to vote on things such as the best player age and player count for the game.
#[derive(Clone, Debug, Deserialize, PartialEq)]
struct Poll {
    // Fixed slug name for the poll.
    name: String,
    // Pretty formatted title for the poll.
    title: String,
    // The total number of users who have voted on this poll.
    #[serde(rename = "totalvotes")]
    total_voters: u64,
    // List of results.
    results: Vec<PollResults>,
}

// A results for a poll.
#[derive(Clone, Debug, Deserialize, PartialEq)]
struct PollResults {
    // Only included in the player count vote, the number of players this vote result is for.
    #[serde(default, rename = "numplayers")]
    number_of_players: Option<PlayerCount>,
    // List of results.
    #[serde(rename = "result")]
    results: Vec<PollResult>,
}

// A result for a poll.
#[derive(Clone, Debug, Deserialize, PartialEq)]
struct PollResult {
    // Only included for language dependence poll, level of dependence where the higher the value
    // the more dependent the game is on knowing the language.
    #[serde(default)]
    level: Option<u64>,
    // Name of the vote option.
    value: String,
    // How many people voted for it.
    #[serde(rename = "numvotes")]
    number_of_votes: u64,
}

// A list of videos. Define the type in xml that can be deserialised, but pull out the nested
// list in the game details deserialise implementation.
#[derive(Clone, Debug, Deserialize, PartialEq)]
struct VideosXml {
    // List of videos, each in an XML tag called `video`.
    #[serde(rename = "video")]
    videos: Vec<Video>,
}

/// A video relating to a game.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Video {
    /// ID of the video.
    pub id: u64,
    /// The title of the video.
    pub title: String,
    /// Type of video, if it is a play-through or rules teach for example.
    pub category: VideoCategory,
    /// Language of the video.
    pub language: String,
    /// Youtube link to the video.
    pub link: String,
    /// Name and ID of the user who uploaded this video.
    pub uploader: User,
    /// The date and time the video was posted.
    pub post_date: DateTime<Utc>,
}

/// Type of video for a video related to a particular game.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VideoCategory {
    /// A video review of a game.
    Review,
    /// A play-through session of a game.
    Session,
    /// Strategy or how to play videos.
    Instructional,
    /// Interviews with people related to the game.
    Interview,
    /// Game unboxing videos.
    Unboxing,
    /// Miscellaneous funny videos about the game.
    Humor,
    /// Videos relating to the game that do not fir in any other category.
    Other,
}

impl<'de> Deserialize<'de> for Video {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Id,
            Title,
            Category,
            Language,
            Link,
            Username,
            UserId,
            PostDate,
        }

        struct VideoVisitor;

        impl<'de> serde::de::Visitor<'de> for VideoVisitor {
            type Value = Video;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an XML object for a video returned inside the videos tag in the response to the `thing` endpoint from boardgamegeek")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut title = None;
                let mut category = None;
                let mut language = None;
                let mut link = None;
                let mut username = None;
                let mut user_id = None;
                let mut post_date = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        },
                        Field::Title => {
                            if title.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title = Some(map.next_value()?);
                        },
                        Field::Category => {
                            if category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            category = Some(map.next_value()?);
                        },
                        Field::Language => {
                            if language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            language = Some(map.next_value()?);
                        },
                        Field::Link => {
                            if link.is_some() {
                                return Err(serde::de::Error::duplicate_field("link"));
                            }
                            link = Some(map.next_value()?);
                        },
                        Field::Username => {
                            if username.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username = Some(map.next_value()?);
                        },
                        Field::UserId => {
                            if user_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("userid"));
                            }
                            user_id = Some(map.next_value()?);
                        },
                        Field::PostDate => {
                            if post_date.is_some() {
                                return Err(serde::de::Error::duplicate_field("postdate"));
                            }
                            let date_string: String = map.next_value()?;
                            let parsed = date_time_with_zone_from_string(&date_string)
                                .map_err(serde::de::Error::custom)?;
                            post_date = Some(parsed);
                        },
                    }
                }
                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let title = title.ok_or_else(|| serde::de::Error::missing_field("title"))?;
                let category =
                    category.ok_or_else(|| serde::de::Error::missing_field("category"))?;
                let language =
                    language.ok_or_else(|| serde::de::Error::missing_field("language"))?;
                let link = link.ok_or_else(|| serde::de::Error::missing_field("link"))?;
                let username =
                    username.ok_or_else(|| serde::de::Error::missing_field("username"))?;
                let user_id = user_id.ok_or_else(|| serde::de::Error::missing_field("userid"))?;
                let post_date =
                    post_date.ok_or_else(|| serde::de::Error::missing_field("postdate"))?;

                Ok(Self::Value {
                    id,
                    title,
                    category,
                    language,
                    link,
                    uploader: User { user_id, username },
                    post_date,
                })
            }
        }
        deserializer.deserialize_any(VideoVisitor)
    }
}

// A list of marketplace listings. Define the type in xml that can be deserialised, but pull out the
// nested list in the game details deserialise implementation
#[derive(Clone, Debug, Deserialize, PartialEq)]
struct MarketplaceListingsXml {
    // List of listings, each in an XML tag called `listing`
    #[serde(rename = "listing")]
    listings: Vec<MarketplaceListing>,
}

/// A game sale listing, for people selling games on the site.
#[derive(Clone, Debug, PartialEq)]
pub struct MarketplaceListing {
    /// The date and time when this listing was listed.
    pub list_date: DateTime<Utc>,
    /// Price of the game.
    pub price: Price,
    /// The condition of the game, if it is new or used and what quality it is in if used.
    pub condition: GameCondition,
    /// Any custom notes about the game for sale.
    pub notes: String,
    /// Link to buy the game on the site.
    pub link: String,
}

/// The price of a game in a marketplace listing.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Price {
    /// The name of the currency for this price value.
    pub currency: String,
    /// The amount the game costs, as a string so the consumer can decide
    /// to convert to float, or a decimal, or an integer for the dollar/euro/gbp and another
    /// integer for the cents/pence or keep as a string depending on use case.
    pub value: String,
}

// XML representation of the market place listing link
#[derive(Debug, Deserialize)]
struct XmlMarketplaceLink {
    // Link to this listing on the site.
    href: String,
    // Fixed at `marketlisting` so we don't include it in the proper type.
    #[allow(dead_code)]
    title: String,
}

/// The condition of a game for sale.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum GameCondition {
    /// Condition good enough to play, but no better.
    Acceptable,
    /// Game is in good condition.
    Good,
    /// Game is in very good condition.
    VeryGood,
    /// Not a new, unused game, but the condition is as such.
    LikeNew,
    /// A new game, unused.
    New,
}

// XML representation of the market place listing condition
#[derive(Debug, Deserialize)]
pub(crate) struct XmlGameCondition {
    pub(crate) value: GameCondition,
}

impl<'de> Deserialize<'de> for MarketplaceListing {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            ListDate,
            Price,
            Condition,
            Notes,
            Link,
        }

        struct MarketplaceListingVisitor;

        impl<'de> serde::de::Visitor<'de> for MarketplaceListingVisitor {
            type Value = MarketplaceListing;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an XML object for a marketplace listing returned inside the marketplacelistings tag in the response to the `thing` endpoint from boardgamegeek")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut list_date = None;
                let mut price = None;
                let mut condition = None;
                let mut notes = None;
                let mut link = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ListDate => {
                            if list_date.is_some() {
                                return Err(serde::de::Error::duplicate_field("listdate"));
                            }
                            let list_date_xml: XmlDateTimeValue = map.next_value()?;
                            list_date = Some(list_date_xml.value);
                        },
                        Field::Price => {
                            if price.is_some() {
                                return Err(serde::de::Error::duplicate_field("price"));
                            }
                            price = Some(map.next_value()?);
                        },
                        Field::Condition => {
                            if condition.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            let condition_xml: XmlGameCondition = map.next_value()?;
                            condition = Some(condition_xml.value);
                        },
                        Field::Notes => {
                            if notes.is_some() {
                                return Err(serde::de::Error::duplicate_field("notes"));
                            }
                            let notes_xml: XmlStringValue = map.next_value()?;
                            notes = Some(notes_xml.value);
                        },
                        Field::Link => {
                            if link.is_some() {
                                return Err(serde::de::Error::duplicate_field("link"));
                            }
                            let link_xml: XmlMarketplaceLink = map.next_value()?;
                            link = Some(link_xml.href);
                        },
                    }
                }
                let list_date =
                    list_date.ok_or_else(|| serde::de::Error::missing_field("listdate"))?;
                let price = price.ok_or_else(|| serde::de::Error::missing_field("price"))?;
                let condition =
                    condition.ok_or_else(|| serde::de::Error::missing_field("condition"))?;
                let notes = notes.ok_or_else(|| serde::de::Error::missing_field("notes"))?;
                let link = link.ok_or_else(|| serde::de::Error::missing_field("link"))?;

                Ok(Self::Value {
                    list_date,
                    price,
                    condition,
                    notes,
                    link,
                })
            }
        }
        deserializer.deserialize_any(MarketplaceListingVisitor)
    }
}

/// A page of comments left on a game by a user. Can include a rating or a text comment or both.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RatingCommentPage {
    /// The total number of comments overall, not the number in this page.
    #[serde(rename = "totalitems")]
    pub total_items: u64,
    /// The index of this page, starting from 1.
    #[serde(rename = "page")]
    pub page_number: u64,
    /// A list of members in this guid.
    #[serde(rename = "comment")]
    pub comments: Vec<RatingComment>,
}

/// A comment left on a game by a user. Can include a rating or a text comment or both.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RatingComment {
    /// The user who left the comment.
    pub username: String,
    /// The rating, between 0 and 10, that the user left on the game.
    #[serde(deserialize_with = "deserialize_rating")]
    pub rating: Option<f64>,
    /// The text comment the user left on this game, may be empty.
    #[serde(rename = "value")]
    pub comment: String,
}

fn deserialize_rating<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;

    match s.as_str() {
        "N/A" => Ok(None),
        val => match val.parse() {
            Ok(rating) => Ok(Some(rating)),
            Err(e) => Err(serde::de::Error::custom(format!(
                "failed to parse rating \"{val}\" as float: {e}",
            ))),
        },
    }
}

impl<'de> Deserialize<'de> for GameDetails {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Type,
            Id,
            Thumbnail,
            Image,
            Name,
            Description,
            YearPublished,
            MinPlayers,
            MaxPlayers,
            PlayingTime,
            MinPlayTime,
            MaxPlayTime,
            MinAge,
            Link,
            Poll,
            Statistics,
            Versions,
            Videos,
            MarketPlaceListings,
            Comments,
        }

        struct GameDetailsVisitor;

        impl<'de> serde::de::Visitor<'de> for GameDetailsVisitor {
            type Value = GameDetails;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an XML object for a board game returned by the `thing` endpoint from boardgamegeek")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut game_type = None;
                let mut thumbnail = None;
                let mut image = None;
                let mut name = None;
                let mut alternate_names = vec![];
                let mut description = None;
                let mut year_published = None;
                let mut min_players = None;
                let mut max_players = None;
                let mut playing_time = None;
                let mut min_play_time = None;
                let mut max_play_time = None;
                let mut min_age = None;
                // Link tags
                let mut categories = vec![];
                let mut mechanics = vec![];
                let mut game_families = vec![];
                let mut expansion_links = vec![];
                let mut accessories = vec![];
                let mut compilations = vec![];
                let mut reimplementations = vec![];
                let mut designers = vec![];
                let mut artists = vec![];
                let mut publishers = vec![];
                // Polls
                let mut suggested_player_count = None;
                let mut suggested_player_age = None;
                let mut suggested_language_dependence = None;
                // Stats and optional
                let mut stats = None;
                let mut versions = None;
                let mut videos = None;
                let mut marketplace_listings = None;
                let mut rating_comments = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        },
                        Field::Type => {
                            if game_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            game_type = Some(map.next_value()?);
                        },
                        Field::Thumbnail => {
                            if thumbnail.is_some() {
                                return Err(serde::de::Error::duplicate_field("thumbnail"));
                            }
                            thumbnail = Some(map.next_value()?);
                        },
                        Field::Image => {
                            if image.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            image = Some(map.next_value()?);
                        },
                        Field::Name => {
                            let name_xml: XmlName = map.next_value()?;
                            match name_xml.name_type {
                                NameType::Primary => {
                                    if name.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "name type=\"primary\"",
                                        ));
                                    }
                                    name = Some(name_xml.value);
                                },
                                NameType::Alternate => {
                                    alternate_names.push(name_xml.value);
                                },
                            }
                        },
                        Field::Description => {
                            if description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        },
                        Field::YearPublished => {
                            if year_published.is_some() {
                                return Err(serde::de::Error::duplicate_field("yearpublished"));
                            }
                            let year_published_xml: XmlSignedValue = map.next_value()?;
                            year_published = Some(year_published_xml.value);
                        },
                        Field::MinPlayers => {
                            if min_players.is_some() {
                                return Err(serde::de::Error::duplicate_field("minplayers"));
                            }
                            let min_players_xml: XmlIntValue = map.next_value()?;
                            min_players = Some(min_players_xml.value);
                        },
                        Field::MaxPlayers => {
                            if max_players.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxplayers"));
                            }
                            let max_players_xml: XmlIntValue = map.next_value()?;
                            max_players = Some(max_players_xml.value);
                        },
                        Field::PlayingTime => {
                            if playing_time.is_some() {
                                return Err(serde::de::Error::duplicate_field("playingtime"));
                            }
                            let playing_time_xml: XmlSignedValue = map.next_value()?;
                            playing_time = Some(Duration::minutes(playing_time_xml.value));
                        },
                        Field::MinPlayTime => {
                            if min_play_time.is_some() {
                                return Err(serde::de::Error::duplicate_field("minplaytime"));
                            }
                            let min_play_time_xml: XmlSignedValue = map.next_value()?;
                            min_play_time = Some(Duration::minutes(min_play_time_xml.value));
                        },
                        Field::MaxPlayTime => {
                            if max_play_time.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxplaytime"));
                            }
                            let max_play_time_xml: XmlSignedValue = map.next_value()?;
                            max_play_time = Some(Duration::minutes(max_play_time_xml.value));
                        },
                        Field::MinAge => {
                            if min_age.is_some() {
                                return Err(serde::de::Error::duplicate_field("minage"));
                            }
                            let min_age_xml: XmlIntValue = map.next_value()?;
                            min_age = Some(min_age_xml.value);
                        },
                        Field::Link => {
                            let link: XmlLink = map.next_value()?;
                            match link.link_type {
                                crate::ItemType::BoardGameExpansion => {
                                    expansion_links.push(Game {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                crate::ItemType::BoardGameAccessory => {
                                    accessories.push(GameAccessory {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                crate::ItemType::BoardGameDesigner => {
                                    designers.push(GameDesigner {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                crate::ItemType::BoardGamePublisher => {
                                    publishers.push(GamePublisher {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                crate::ItemType::BoardGameArtist => {
                                    artists.push(GameArtist {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                crate::ItemType::BoardGameFamily => {
                                    game_families.push(GameFamilyName {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                crate::ItemType::BoardGameCategory => {
                                    categories.push(GameCategory {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                crate::ItemType::BoardGameMechanic => {
                                    mechanics.push(GameMechanic {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                crate::ItemType::BoardGameCompilation => {
                                    compilations.push(GameCompilation {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                crate::ItemType::BoardGameImplementation => {
                                    reimplementations.push(GameImplementation {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                link_type => {
                                    return Err(serde::de::Error::custom(format!(
                                        "found unexpected \"{link_type:?}\" link in game info",
                                    )));
                                },
                            }
                        },
                        Field::Poll => {
                            let poll: Poll = map.next_value()?;

                            if poll.name == "suggested_numplayers" {
                                if suggested_player_count.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "poll name=\"suggested_numplayers\"",
                                    ));
                                }
                                suggested_player_count =
                                    Some(poll.try_into().map_err(serde::de::Error::custom)?);
                            } else if poll.name == "suggested_playerage" {
                                if suggested_player_age.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "poll name=\"suggested_playerage\"",
                                    ));
                                }
                                suggested_player_age =
                                    Some(poll.try_into().map_err(serde::de::Error::custom)?);
                            } else if poll.name == "language_dependence" {
                                if suggested_language_dependence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "poll name=\"language_dependence\"",
                                    ));
                                }
                                suggested_language_dependence =
                                    Some(poll.try_into().map_err(serde::de::Error::custom)?);
                            } else {
                                return Err(serde::de::Error::custom(format!(
                                    "unexpected poll name `{}`",
                                    poll.name
                                )));
                            }
                        },
                        Field::Statistics => {
                            if stats.is_some() {
                                return Err(serde::de::Error::duplicate_field("statistics"));
                            }
                            let stats_xml: XmlGameStats = map.next_value()?;
                            let (rank, sub_family_ranks) =
                                xml_ranks_to_ranks::<'de, A>(stats_xml.ratings.ranks)?;
                            stats = Some(GameStats {
                                users_rated: stats_xml.ratings.usersrated.value,
                                average_rating: stats_xml.ratings.average.value,
                                bayesian_average: stats_xml.ratings.bayesaverage.value,
                                standard_deviation: stats_xml.ratings.stddev.value,
                                median: stats_xml.ratings.median.value,
                                rank,
                                sub_family_ranks,
                                users_owned: stats_xml.ratings.owned.value,
                                users_trading: stats_xml.ratings.trading.value,
                                users_want_in_trade: stats_xml.ratings.wanting.value,
                                users_wishlisted: stats_xml.ratings.wishing.value,
                                number_of_comments: stats_xml.ratings.numcomments.value,
                                number_of_weights: stats_xml.ratings.numweights.value,
                                weight_rating: stats_xml.ratings.averageweight.value,
                            });
                        },
                        Field::Versions => {
                            if versions.is_some() {
                                return Err(serde::de::Error::duplicate_field("versions"));
                            }
                            let versions_xml: VersionsXml = map.next_value()?;
                            versions = Some(versions_xml.versions);
                        },
                        Field::Videos => {
                            if videos.is_some() {
                                return Err(serde::de::Error::duplicate_field("videos"));
                            }
                            let videos_xml: VideosXml = map.next_value()?;
                            videos = Some(videos_xml.videos);
                        },
                        Field::MarketPlaceListings => {
                            if marketplace_listings.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "marketplacelistings",
                                ));
                            }
                            let marketplace_listings_xml: MarketplaceListingsXml =
                                map.next_value()?;
                            marketplace_listings = Some(marketplace_listings_xml.listings);
                        },
                        Field::Comments => {
                            if rating_comments.is_some() {
                                return Err(serde::de::Error::duplicate_field("comments"));
                            }
                            rating_comments = Some(map.next_value()?);
                        },
                    }
                }
                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let game_type = game_type.ok_or_else(|| serde::de::Error::missing_field("type"))?;
                let thumbnail =
                    thumbnail.ok_or_else(|| serde::de::Error::missing_field("thumbnail"))?;
                let image = image.ok_or_else(|| serde::de::Error::missing_field("image"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let description =
                    description.ok_or_else(|| serde::de::Error::missing_field("description"))?;
                let year_published = year_published
                    .ok_or_else(|| serde::de::Error::missing_field("yearpublished"))?;
                let min_players =
                    min_players.ok_or_else(|| serde::de::Error::missing_field("minplayers"))?;
                let max_players =
                    max_players.ok_or_else(|| serde::de::Error::missing_field("maxplayers"))?;
                let playing_time =
                    playing_time.ok_or_else(|| serde::de::Error::missing_field("playingtime"))?;
                let min_play_time =
                    min_play_time.ok_or_else(|| serde::de::Error::missing_field("minplaytime"))?;
                let max_play_time =
                    max_play_time.ok_or_else(|| serde::de::Error::missing_field("maxplaytime"))?;
                let min_age = min_age.ok_or_else(|| serde::de::Error::missing_field("minage"))?;

                let suggested_player_count = suggested_player_count.ok_or_else(|| {
                    serde::de::Error::missing_field("poll name=\"suggested_numplayers\"")
                })?;
                let suggested_player_age = suggested_player_age.ok_or_else(|| {
                    serde::de::Error::missing_field("poll name=\"suggested_playerage\"")
                })?;
                let suggested_language_dependence =
                    suggested_language_dependence.ok_or_else(|| {
                        serde::de::Error::missing_field("poll name=\"language_dependence\"")
                    })?;

                let stats = stats.ok_or_else(|| serde::de::Error::missing_field("statistics"))?;
                let versions = versions.unwrap_or_default();
                let videos = videos.unwrap_or_default();
                let marketplace_listings = marketplace_listings.unwrap_or_default();

                let (expansions, expansion_for) = match game_type {
                    GameType::BoardGame => (expansion_links, vec![]),
                    GameType::BoardGameExpansion => (vec![], expansion_links),
                };

                Ok(Self::Value {
                    id,
                    game_type,
                    name,
                    alternate_names,
                    description,
                    image,
                    thumbnail,
                    year_published,
                    min_players,
                    max_players,
                    suggested_player_count,
                    playing_time,
                    min_play_time,
                    max_play_time,
                    min_age,
                    suggested_player_age,
                    suggested_language_dependence,
                    categories,
                    mechanics,
                    game_families,
                    expansions,
                    expansion_for,
                    accessories,
                    compilations,
                    reimplementations,
                    designers,
                    artists,
                    publishers,
                    stats,
                    versions,
                    videos,
                    marketplace_listings,
                    rating_comments,
                })
            }
        }
        deserializer.deserialize_any(GameDetailsVisitor)
    }
}
