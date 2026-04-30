use chrono::{Duration, NaiveDate};
use serde::Deserialize;

use crate::deserialize::deserialize_minutes;
use crate::ItemSubType;

/// A play is a recorded instance of someone playing a game. This struct includes one page of a list
/// of plays, along with the total number in the list.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Plays {
    // TODO check if this is a different object when requesting for a thing/family
    /// The username of the user that these plays were requested for.
    pub username: String,
    /// The ID of the user that these plays were requested for.
    #[serde(rename = "userid")]
    pub user_id: u64,
    /// The total number of plays for the query, only up to 100 being returned depending on the
    /// requested page.
    pub total: u64,
    /// The page of plays that is returned in the `plays` field of this object. Page size is 100.
    pub page: u64,
    /// The list of plays for this
    #[serde(default = "Vec::new", rename = "play")]
    pub plays: Vec<Play>,
}

/// A recorded instance of a game being played, the date it was played on and the players involved.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Play {
    /// ID of this play.
    pub id: u64,
    /// The date that the session happened.
    pub date: NaiveDate,
    /// The number of times the game was played in this session.
    pub quantity: u64,
    /// How long the play session lasted for.
    #[serde(default, rename = "length", deserialize_with = "deserialize_minutes")]
    pub duration: Duration,
    /// True if the game was not finished in this play.
    pub incomplete: bool,
    /// Where the game was played.
    pub location: String,
    /// An option to "Save play details but don't use when analyzing plays.".
    #[serde(rename = "nowinstats")]
    pub do_not_count_win_stats: bool,
    /// The game or other item that was played in this session.
    #[serde(rename = "item")]
    pub played_item: PlayedItem,
    /// The players who played.
    #[serde(
        default = "Vec::new",
        deserialize_with = "deserialize_nested_players_list"
    )]
    pub players: Vec<Player>,
    /// Any user written comments about this session.
    #[serde(default)]
    pub comments: Option<String>,
}

/// The item, usually a game, that was played.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PlayedItem {
    /// The name of the item that was played.
    pub name: String,
    /// The unique identifier for this item.
    #[serde(rename = "objectid")]
    pub id: u64,
    /// The list of types the item is, such as board game, or board game and board game expansion.
    #[serde(
        default = "Vec::new",
        deserialize_with = "deserialize_nested_sub_types_list",
        rename = "subtypes"
    )]
    pub sub_types: Vec<ItemSubType>,
}

// Since the list of sub types in nested inside a `subtypes` xml tag. We need to use this struct
// with a custom deserializer in order to have just a vec on the returned object.
#[derive(Clone, Debug, Deserialize, PartialEq)]
struct SubTypesXml {
    #[serde(default = "Vec::new", rename = "subtype")]
    sub_types: Vec<SubTypeXml>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct SubTypeXml {
    value: ItemSubType,
}

fn deserialize_nested_sub_types_list<'de, D>(deserializer: D) -> Result<Vec<ItemSubType>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let sub_types_xml = SubTypesXml::deserialize(deserializer)?;
    Ok(sub_types_xml
        .sub_types
        .into_iter()
        .map(|xml| xml.value)
        .collect())
}

// Since the list of players in nested inside a `players` xml tag. We need to use this struct with a
// custom deserializer in order to have just a vec on the returned object.
#[derive(Clone, Debug, Deserialize, PartialEq)]
struct PlayersXml {
    #[serde(default = "Vec::new", rename = "player")]
    players: Vec<Player>,
}

fn deserialize_nested_players_list<'de, D>(deserializer: D) -> Result<Vec<Player>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let players_xml = PlayersXml::deserialize(deserializer)?;
    Ok(players_xml.players)
}

/// Details for a player for a game session, as well as information involving that particular game
/// play such as whether or not they won.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Player {
    /// The player's Boardgamegeek username, if they have one.
    pub username: Option<String>,
    /// The player's Boardgamegeek user ID, if they have one.
    #[serde(rename = "userid")]
    pub user_id: Option<u64>,
    /// The player's name.
    pub name: String,
    /// Their in game start position. Arbitrary user input is allowed so this is a string not a
    /// number.
    #[serde(rename = "startposition")]
    pub start_position: String,
    /// Their in game color.
    pub color: String,
    /// Their in game score.
    pub score: String,
    /// True if this was the first time this player played this game.
    #[serde(rename = "new")]
    pub first_time_playing: bool,
    /// The player's rating of the game.
    pub rating: u64,
    /// True if this player won the game.
    #[serde(rename = "win")]
    pub won: bool,
}
