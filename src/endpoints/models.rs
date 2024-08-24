use serde::Deserialize;

/// The type of game, board game or expansion.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    /// A board game, or expansion, or board game accessory.
    BoardGame,
    /// A board game expansion.
    BoardGameExpansion,
    /// An accessory for a board game expansion.
    BoardGameAccessory,
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

/// A publisher of/ a game.
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
/// one or more langages.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Language {
    /// The ID of the language.
    pub id: u64,
    /// The name of the language, in English.
    #[serde(rename = "value")]
    pub name: String,
}

/// The dimensions of a game, in inches (TODO check this)
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Dimensions {
    /// The width of the game, in inches
    pub width: f64,
    /// The lenth of the game, in inches
    pub length: f64,
    /// The depth of the game, in inches
    pub depth: f64,
}
