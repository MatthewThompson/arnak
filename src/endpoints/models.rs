use core::fmt::{self, Display};

use serde::Deserialize;

use super::{GameFamilyType, RankValue};
use crate::utils::{XmlFloatValue, XmlLink, XmlName, XmlSignedValue, XmlStringValue};

/// The type of the item. Either a board game, a board game expansion, or board game accessory.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    /// A board game. In many cases the underlying API will also include
    /// board game expansions under this type, unless explicitly excluded.
    BoardGame,
    /// A board game expansion.
    BoardGameExpansion,
    /// An accessory for a board game. This can include things such as playmats
    /// and miniatures.
    BoardGameAccessory,
    /// A designer of a board game.
    BoardGameDesigner,
    /// A publisher of a board game.
    BoardGamePublisher,
    /// An artist of a board game.
    BoardGameArtist,
    /// A group of games. A family of games might be all games that fall under a certain
    /// IP, or grouped by some other criteria such as game mechanic.
    BoardGameFamily,
    /// A category for a game.
    ///
    /// A category is a broad description mostly based on the theme of the game not the mechanics.
    /// Includes `Fantasy`, `Adventure`, `Animals`, and some mechanic based categories such as
    /// `Action / Dexterity`, and `Dice`.
    BoardGameCategory,
    /// A mechanic for a game.
    ///
    /// Mechanics can include `Worker Placement`, `Push Your Luck`, and `Negotiation`.
    BoardGameMechanic,
    /// A different edition of an existing game.
    BoardGameCompilation,
    /// A different implementation of an existing game.
    BoardGameImplementation,
    /// A different version of a game.
    ///
    /// Type used only in the version info for a game, and typically means translated versions
    /// of the game.
    BoardGameVersion,
    /// A language that a game supports.
    Language,
}

impl Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::BoardGame => write!(f, "boardgame"),
            ItemType::BoardGameExpansion => write!(f, "boardgameexpansion"),
            ItemType::BoardGameAccessory => write!(f, "boardgameaccessory"),
            ItemType::BoardGameDesigner => write!(f, "boardgamedesigner"),
            ItemType::BoardGamePublisher => write!(f, "boardgamepublisher"),
            ItemType::BoardGameArtist => write!(f, "boardgameartist"),
            ItemType::BoardGameFamily => write!(f, "boardgamefamily"),
            ItemType::BoardGameCategory => write!(f, "boardgamecategory"),
            ItemType::BoardGameMechanic => write!(f, "boardgamemechanic"),
            ItemType::BoardGameCompilation => write!(f, "boardgamecompilation"),
            ItemType::BoardGameImplementation => write!(f, "boardgameimplementation"),
            ItemType::BoardGameVersion => write!(f, "boardgameversion"),
            ItemType::Language => write!(f, "language"),
        }
    }
}

/// The type of an item that can be returned from the collections endpoint.
/// Either a board game, a board game expansion, or board game accessory, a subset ot
/// [`ItemType`].
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CollectionItemType {
    /// A board game. In many cases the underlying API will also include
    /// board game expansions under this type, unless explicitly excluded.
    BoardGame,
    /// A board game expansion.
    BoardGameExpansion,
    /// An accessory for a board game. This can include things such as playmats
    /// and miniatures.
    BoardGameAccessory,
}

impl Display for CollectionItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollectionItemType::BoardGame => write!(f, "boardgame"),
            CollectionItemType::BoardGameExpansion => write!(f, "boardgameexpansion"),
            CollectionItemType::BoardGameAccessory => write!(f, "boardgameaccessory"),
        }
    }
}

/// The type of a game.
///
/// Either [`GameType::BoardGame`] for a normal board game or [`GameType::BoardGameExpansion`]
/// for an expansion of another existing board game.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GameType {
    /// A board game. In many cases the underlying API will also include
    /// board game expansions under this type, unless explicitly excluded.
    BoardGame,
    /// A board game expansion.
    BoardGameExpansion,
}

impl Display for GameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameType::BoardGame => write!(f, "boardgame"),
            GameType::BoardGameExpansion => write!(f, "boardgameexpansion"),
        }
    }
}

/// The type of game, board game or expansion.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NameType {
    /// The primary name for a game or game family.
    Primary,
    /// An alternate name for a game or game family. Often a translation or name in a different
    /// locale.
    Alternate,
}

/// A game with minimal information, only the name and ID.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Game {
    /// The ID of the game.
    pub id: u64,
    /// The name of the game.
    #[serde(rename = "value")]
    pub name: String,
}

/// A game accessory with minimal information, only the name and ID.
///
/// More information can be retrieved from the accessory endpoint.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GameAccessory {
    /// The ID of the game.
    pub id: u64,
    /// The name of the game.
    #[serde(rename = "value")]
    pub name: String,
}

/// A game accessory with minimal information, only the name and ID.
///
/// More information can be retrieved from the accessory endpoint.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GameCategory {
    /// The ID of the game.
    pub id: u64,
    /// The name of the game.
    #[serde(rename = "value")]
    pub name: String,
}

/// A game accessory with minimal information, only the name and ID.
///
/// More information can be retrieved from the accessory endpoint.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GameMechanic {
    /// The ID of the game.
    pub id: u64,
    /// The name of the game.
    #[serde(rename = "value")]
    pub name: String,
}

/// A name and ID of a game family.
///
/// More information about the game family can be retrieved from the
/// game family endpoint. This will include a description and a list
/// of all games that belong to this game family.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GameFamilyName {
    /// The ID of the publisher.
    pub id: u64,
    /// The name of the publisher.
    #[serde(rename = "value")]
    pub name: String,
}

/// A different edition or compilation of a game.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GameCompilation {
    /// The ID of the compilation.
    pub id: u64,
    /// The name of the compilation.
    #[serde(rename = "value")]
    pub name: String,
}

/// A re-implementation of a game.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GameImplementation {
    /// The ID of the implementation.
    pub id: u64,
    /// The name of the implementation.
    #[serde(rename = "value")]
    pub name: String,
}

/// A designer of a game.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GameDesigner {
    /// The ID of the designer.
    pub id: u64,
    /// The name of the designer.
    #[serde(rename = "value")]
    pub name: String,
}

/// A publisher of a game.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GamePublisher {
    /// The ID of the publisher.
    pub id: u64,
    /// The name of the publisher.
    #[serde(rename = "value")]
    pub name: String,
}

/// An artist for a game.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GameArtist {
    /// The ID of the artist.
    pub id: u64,
    /// The name of the game.
    #[serde(rename = "value")]
    pub name: String,
}

/// A language, listed on versions of games that may support
/// one or more languages.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Language {
    /// The ID of the language.
    pub id: u64,
    /// The name of the language, in English.
    #[serde(rename = "value")]
    pub name: String,
}

/// The dimensions of a game, in inches.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Dimensions {
    /// The width of the game, in inches.
    pub width: f64,
    /// The length of the game, in inches.
    pub length: f64,
    /// The depth of the game, in inches.
    pub depth: f64,
}

// Intermediary struct needed due to the way the XML is structured
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct XmlRanks {
    #[serde(rename = "rank")]
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
    /// User friendly name in the format "GENRE game rank" e.g. "Party Game
    /// Rank".
    #[serde(rename = "friendlyname")]
    pub friendly_name: String,
    /// The overall rank on the site within this type of game.
    pub value: RankValue,
    /// The score out of 10, as a bayesian average.
    ///
    /// This is what boardgamegeek calls a Geek Rating. It is the average rating
    /// that the users have given it along with a few thousand 5.5 ratings added
    /// in too.
    #[serde(rename = "bayesaverage")]
    pub bayesian_average: f64,
}

// A user's collection on boardgamegeek.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct VersionsXml {
    // List of versions, each in an XML tag called `item`, within an outer
    // `version`. We use this intermediary type to get out just the first,
    // since we only expect 1.
    #[serde(rename = "item")]
    pub(crate) versions: Vec<GameVersion>,
}

/// Information about a game which is a version or re-implementation of another game, including the
/// link to the original.
///
/// Often this may be a translated version of a game. It is not the same as an expansion for a game.
#[derive(Clone, Debug, PartialEq)]
pub struct GameVersion {
    /// The ID of this game.
    pub id: u64,
    /// The name of the game.
    pub name: String,
    /// A list of alternate names for the game.
    pub alternate_names: Vec<String>,
    /// The year the game was first published.
    pub year_published: i64,
    /// A link to a jpg image for the game.
    pub image: String,
    /// A link to a jpg thumbnail image for the game.
    pub thumbnail: String,
    /// The name and ID of the game this version is based off of.
    pub original_game: Game,
    /// List of publishers for this game.
    pub publishers: Vec<GamePublisher>,
    /// List of game artists.
    pub artists: Vec<GameArtist>,
    /// Lists of languages that this version of the game supports.
    pub languages: Vec<Language>,
    /// The dimensions of the game, in inches, if included.
    pub dimensions: Option<Dimensions>,
    /// The weight of the game, in pounds, if included.
    pub weight: Option<f64>,
    /// Product code for the game, if included.
    pub product_code: Option<String>,
}

impl<'de> Deserialize<'de> for GameVersion {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Id,
            Name,
            Image,
            Thumbnail,
            YearPublished,
            ProductCode,
            Width,
            Length,
            Depth,
            Weight,
            // Game original version, publisher, artist, are each in an individual XML tag called
            // `link`
            Link,
            Type,
        }

        struct GameVersionVisitor;

        impl<'de> serde::de::Visitor<'de> for GameVersionVisitor {
            type Value = GameVersion;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string containing the XML for game version information.")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut name = None;
                let mut alternate_names = vec![];
                let mut publishers = vec![];
                let mut artists = vec![];
                let mut languages = vec![];
                let mut year_published = None;
                let mut image = None;
                let mut thumbnail = None;
                let mut original_game = None;
                let mut width = None;
                let mut length = None;
                let mut depth = None;
                let mut weight = None;
                let mut product_code = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
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
                        Field::Image => {
                            if image.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            image = Some(map.next_value()?);
                        },
                        Field::Thumbnail => {
                            if thumbnail.is_some() {
                                return Err(serde::de::Error::duplicate_field("thumbnail"));
                            }
                            thumbnail = Some(map.next_value()?);
                        },
                        Field::Link => {
                            let link: XmlLink = map.next_value()?;
                            match link.link_type {
                                ItemType::BoardGameVersion => {
                                    if original_game.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "link with type \"boardgameversion\"",
                                        ));
                                    }
                                    original_game = Some(Game {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                ItemType::BoardGamePublisher => {
                                    publishers.push(GamePublisher {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                ItemType::BoardGameArtist => {
                                    artists.push(GameArtist {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                ItemType::Language => {
                                    languages.push(Language {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                link_type => {
                                    return Err(serde::de::Error::custom(format!(
                                        "found unexpected \"{link_type:?}\" link in version",
                                    )));
                                },
                            }
                        },
                        Field::Width => {
                            if width.is_some() {
                                return Err(serde::de::Error::duplicate_field("width"));
                            }
                            let width_xml: XmlFloatValue = map.next_value()?;
                            if width_xml.value == 0.0 {
                                width = Some(None);
                            } else {
                                width = Some(Some(width_xml.value));
                            }
                        },
                        Field::Depth => {
                            if depth.is_some() {
                                return Err(serde::de::Error::duplicate_field("depth"));
                            }
                            let depth_xml: XmlFloatValue = map.next_value()?;
                            if depth_xml.value == 0.0 {
                                depth = Some(None);
                            } else {
                                depth = Some(Some(depth_xml.value));
                            }
                        },
                        Field::Length => {
                            if length.is_some() {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            let length_xml: XmlFloatValue = map.next_value()?;
                            if length_xml.value == 0.0 {
                                length = Some(None);
                            } else {
                                length = Some(Some(length_xml.value));
                            }
                        },
                        Field::Weight => {
                            if weight.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            let weight_xml: XmlFloatValue = map.next_value()?;
                            if weight_xml.value == 0.0 {
                                weight = Some(None);
                            } else {
                                weight = Some(Some(weight_xml.value));
                            }
                        },
                        Field::ProductCode => {
                            if product_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("product_code"));
                            }
                            let product_code_xml: XmlStringValue = map.next_value()?;
                            if product_code_xml.value.is_empty() {
                                product_code = Some(None);
                            } else {
                                product_code = Some(Some(product_code_xml.value));
                            }
                        },
                        Field::YearPublished => {
                            if year_published.is_some() {
                                return Err(serde::de::Error::duplicate_field("yearpublished"));
                            }
                            let year_published_xml: XmlSignedValue = map.next_value()?;
                            year_published = Some(year_published_xml.value);
                        },
                        Field::Type => {
                            // Type is fixed at "boardgameversion", even for the list of games
                            // contained so we don't add it. But we need
                            // to consume the value.
                            let _: String = map.next_value()?;
                        },
                    }
                }
                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let year_published = year_published
                    .ok_or_else(|| serde::de::Error::missing_field("yearpublished"))?;
                let image = image.ok_or_else(|| serde::de::Error::missing_field("image"))?;
                let thumbnail =
                    thumbnail.ok_or_else(|| serde::de::Error::missing_field("thumbnail"))?;
                let original_game = original_game.ok_or_else(|| {
                    serde::de::Error::missing_field("link with type \"boardgameversion\"")
                })?;
                let width = width.ok_or_else(|| serde::de::Error::missing_field("width"))?;
                let length = length.ok_or_else(|| serde::de::Error::missing_field("length"))?;
                let depth = depth.ok_or_else(|| serde::de::Error::missing_field("depth"))?;
                let weight = weight.ok_or_else(|| serde::de::Error::missing_field("weight"))?;
                let product_code =
                    product_code.ok_or_else(|| serde::de::Error::missing_field("productcode"))?;

                let dimensions;
                if let (Some(width), Some(length), Some(depth)) = (width, length, depth) {
                    dimensions = Some(Dimensions {
                        width,
                        length,
                        depth,
                    });
                } else if let (None, None, None) = (width, depth, length) {
                    dimensions = None;
                } else {
                    return Err(serde::de::Error::custom("Invalid game dimensions, some but not all of width, length, depth, were set."));
                }

                Ok(Self::Value {
                    id,
                    name,
                    alternate_names,
                    year_published,
                    image,
                    thumbnail,
                    original_game,
                    publishers,
                    artists,
                    languages,
                    dimensions,
                    weight,
                    product_code,
                })
            }
        }
        deserializer.deserialize_any(GameVersionVisitor)
    }
}

/// A user's username and ID.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct User {
    /// ID for the user.
    pub user_id: u64,
    /// Username, used to request collection information.
    pub username: String,
}
