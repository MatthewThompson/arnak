use core::fmt::Display;

use serde::Deserialize;

use super::{GameFamilyType, RankValue};

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

// TODO: is there a nice way to have a separate display and to_string implementation?
// to_string is needed for converting this type into query params, but it would be good
// to have a separate user facing display that would look like "board game" instead of
// "boardgame" for example.
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
/// [ItemType].
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
/// Either [GameType::BoardGame] for a normal board game or [GameType::BoardGameExpansion]
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
