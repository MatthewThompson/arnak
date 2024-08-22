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
