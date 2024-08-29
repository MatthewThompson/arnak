use core::fmt;

use chrono::Duration;
use serde::Deserialize;

use super::{
    Game, GameAccessory, GameArtist, GameCategory, GameCompilation, GameDesigner, GameFamilyName,
    GameFamilyRank, GameImplementation, GameMechanic, GamePublisher, GameType, XmlRanks,
};
use crate::utils::{XmlFloatValue, XmlIntValue, XmlLink, XmlName, XmlSignedValue};
use crate::NameType;

/// A list of requested games with the full details.
#[derive(Clone, Debug, Deserialize)]
pub struct Games {
    /// List of games.
    #[serde(rename = "$value")]
    pub games: Vec<GameDetails>,
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
    pub suggested_player_count: Poll,
    /// The amount of time the game is suggested to take to play.
    pub playing_time: Duration,
    /// Minimum amount of time the game is suggested to take to play.
    pub min_playtime: Duration,
    /// Maximum amount of time the game is suggested to take to play.
    pub max_playtime: Duration,
    /// The minimum suggested age suitable for playing this game.
    pub min_age: u64,
    /// The suggested number minimum suitable age for playing this game.
    ///
    /// Poll results for at which age is a suitable minimum age for playing this game.
    pub suggested_player_age: Poll,
    /// The suggested dependence on knowing the game's language in order to be able to play it.
    ///
    /// Poll results, with five options, for whether the game can be played without knowing the
    /// language, if it does not have much in game text. Through being completely unplayable
    /// due to extensive text in the game.
    pub suggested_language_dependence: Poll,
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
    pub stats: GameStats, // TODO type
}

/// blah
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GameStats {
    /// The number of users who have rated this game.
    pub users_rated: u64,
    /// The average rating from users, from 0-10.
    pub average: f64,
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
    /// The list of ranks the item is on the site within each of its item types.
    pub ranks: Vec<GameFamilyRank>,
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

#[derive(Debug, Deserialize)]
struct XmlGameStats {
    #[serde(rename = "$value")]
    ratings: StatsRatings,
}

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

/// A poll for
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Poll {
    /// Fixed slug
    pub name: String,
    /// Pretty title
    pub title: String,
    /// List of results TODO unwrap this with custom des
    pub results: Vec<PollResults>,
}

/// A results for a poll
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PollResults {
    /// List of results
    #[serde(rename = "$value")]
    pub results: Vec<PollResult>,
}

/// A result for a poll
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PollResult {
    /// Name of the vote option
    pub value: String,
    /// How many people voted for it
    #[serde(rename = "numvotes")]
    pub number_of_votes: u64,
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
                let mut expansions = vec![];
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
                                    expansions.push(Game {
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
                                    })
                                },
                                crate::ItemType::BoardGameImplementation => {
                                    reimplementations.push(GameImplementation {
                                        id: link.id,
                                        name: link.value,
                                    })
                                },
                                link_type => {
                                    return Err(serde::de::Error::custom(format!(
                                        "found unexpected \"{:?}\" link in game info",
                                        link_type,
                                    )));
                                },
                            }
                        },
                        Field::Poll => {
                            // TODO
                            // check name
                            // suggested_numplayers/suggested_playerage/language_dependence
                            // Decode as appropriate
                            // Struct with hard coded 5 values for lang dep??
                            let poll: Poll = map.next_value()?;

                            if poll.name == "suggested_numplayers" {
                                if suggested_player_count.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "poll name=\"suggested_numplayers\"",
                                    ));
                                }
                                suggested_player_count = Some(poll);
                            } else if poll.name == "suggested_playerage" {
                                if suggested_player_age.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "poll name=\"suggested_playerage\"",
                                    ));
                                }
                                suggested_player_age = Some(poll);
                            } else if poll.name == "language_dependence" {
                                if suggested_language_dependence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "poll name=\"language_dependence\"",
                                    ));
                                }
                                suggested_language_dependence = Some(poll);
                            }
                        },
                        Field::Statistics => {
                            if stats.is_some() {
                                return Err(serde::de::Error::duplicate_field("stats"));
                            }
                            let stats_xml: XmlGameStats = map.next_value()?;
                            stats = Some(GameStats {
                                users_rated: stats_xml.ratings.usersrated.value,
                                average: stats_xml.ratings.average.value,
                                bayesian_average: stats_xml.ratings.bayesaverage.value,
                                standard_deviation: stats_xml.ratings.stddev.value,
                                median: stats_xml.ratings.median.value,
                                ranks: stats_xml.ratings.ranks.ranks,
                                users_owned: stats_xml.ratings.owned.value,
                                users_trading: stats_xml.ratings.trading.value,
                                users_want_in_trade: stats_xml.ratings.wanting.value,
                                users_wishlisted: stats_xml.ratings.wishing.value,
                                number_of_comments: stats_xml.ratings.numcomments.value,
                                number_of_weights: stats_xml.ratings.numweights.value,
                                weight_rating: stats_xml.ratings.averageweight.value,
                            });
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
                let min_playtime =
                    min_play_time.ok_or_else(|| serde::de::Error::missing_field("minplaytime"))?;
                let max_playtime =
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

                let stats = stats.ok_or_else(|| serde::de::Error::missing_field("stats"))?;

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
                    min_playtime,
                    max_playtime,
                    min_age,
                    suggested_player_age,
                    suggested_language_dependence,
                    categories,
                    mechanics,
                    game_families,
                    expansions,
                    accessories,
                    compilations,
                    reimplementations,
                    designers,
                    artists,
                    publishers,
                    stats,
                })
            }
        }
        deserializer.deserialize_any(GameDetailsVisitor)
    }
}
