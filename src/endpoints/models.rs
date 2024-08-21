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
