use serde::Deserialize;

/// The type of game, board game or expansion.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    /// A board game, or expansion, or board game accessory.
    ///
    /// Due to the way the API works, this type can include expansions too.
    /// If a request is made for just board games, or the game type is not
    /// filtered, then both games with a type of [GameType::BoardGame] and
    /// those with a type of [GameType::BoardGameExpansion] will be returned,
    /// and they will ALL have the type of [GameType::BoardGame]. However when
    /// requesting just expansions, the returned games will correctly have the
    /// type [GameType::BoardGameExpansion].
    BoardGame,
    /// A board game expansion.
    BoardGameExpansion,
    /// An accessory for a board game expansion.
    BoardGameAccessory,
}
