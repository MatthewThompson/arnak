use chrono::{DateTime, Duration, NaiveDateTime, ParseError, Utc};
use serde::Deserialize;

use crate::{ItemFamilyRank, ItemType, NameType, RankValue};

pub(crate) fn deserialize_xml_string<T: serde::de::DeserializeOwned>(
    xml: &str,
) -> core::result::Result<T, serde_xml_rs::Error> {
    // The parser config used by serde_xml
    let default_xml_reader_config = xml::ParserConfig::new()
        .trim_whitespace(true)
        .whitespace_to_characters(true)
        .cdata_to_characters(true)
        .ignore_comments(true)
        .coalesce_characters(true);
    // Not allowed by the default XML spec, so the underlying XML reader will return an error
    // while trying to deserialise. But this is used by boardgamegeek in the descriptions so
    // we need to add it here.
    let xml_reader_config = default_xml_reader_config.add_entity("mdash", "—");

    let xml_reader = xml::reader::EventReader::new_with_config(xml.as_bytes(), xml_reader_config);
    let mut deserializer = serde_xml_rs::Deserializer::new(xml_reader);
    T::deserialize(&mut deserializer)
}

// Types that only exist as intermediary values when deserialising more complex types.
// They appear in the form `<tag value="some_value">`

#[derive(Debug, Deserialize)]
pub(crate) struct XmlIntValue {
    pub(crate) value: u64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlSignedValue {
    pub(crate) value: i64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlFloatValue {
    pub(crate) value: f64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlStringValue {
    pub(crate) value: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlDateTimeValue {
    #[serde(deserialize_with = "deserialize_date_time_with_zone")]
    pub(crate) value: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlName {
    #[serde(rename = "type")]
    pub(crate) name_type: NameType,
    pub(crate) value: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct XmlLink {
    #[serde(rename = "type")]
    pub(crate) link_type: ItemType,
    pub(crate) id: u64,
    pub(crate) value: String,
}

pub(crate) fn deserialize_1_0_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;

    match s.as_str() {
        "1" => Ok(true),
        "0" => Ok(false),
        _ => Err(serde::de::Error::unknown_variant(&s, &["1", "0"])),
    }
}

pub(crate) fn deserialize_minutes<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;
    // Parse as unsigned because although Duration supports negative durations,
    // we do not want to support that for game playing time.
    let minutes = s.parse::<u32>().map_err(|e| {
        serde::de::Error::custom(format!(
            "unable to parse duration minutes string to u32: {e}"
        ))
    });
    minutes.map(|m| Duration::minutes(i64::from(m)))
}

const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
// e.g. 2024-07-22T16:33:30-05:00
// Used for the video post date returned from the game endpoint.
const DATE_TIME_ZONE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%:z";
// e.g. Thu, 14 Jun 2007 01:06:46 +0000
const DATE_TIME_ZONE_LONG_FORMAT: &str = "%a, %d %B %Y %H:%M:%S %z";

pub(crate) fn deserialize_date_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt =
        NaiveDateTime::parse_from_str(&s, DATE_TIME_FORMAT).map_err(serde::de::Error::custom)?;
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
}

pub(crate) fn date_time_with_zone_from_string(string: &str) -> Result<DateTime<Utc>, ParseError> {
    let date_time = DateTime::parse_from_str(string, DATE_TIME_ZONE_FORMAT)?;
    Ok(DateTime::<Utc>::from(date_time))
}

pub(crate) fn deserialize_date_time_with_zone<'de, D>(
    deserializer: D,
) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let date_time = DateTime::parse_from_str(&s, DATE_TIME_ZONE_LONG_FORMAT)
        .map_err(serde::de::Error::custom)?;
    Ok(DateTime::<Utc>::from(date_time))
}

// Intermediary struct needed due to the way the XML is structured
#[derive(Debug, Deserialize)]
pub(crate) struct XmlRanks {
    #[serde(rename = "rank")]
    pub(crate) ranks: Vec<XmlItemFamilyRank>,
}

// Type of game family.
//
// [`ItemFamilyType::Subtype`] is used for the `boardgame` family that includes all games.
// [`ItemFamilyType::Family`] is used for everything else. Such as party games or strategy games.
// Used only for identifying which ranks belong to sub families and which to the overall category
// (game/accessory) when deserialising.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub(crate) enum ItemFamilyType {
    // Used only for the generic `boardgame` family that includes all games.
    #[serde(rename = "subtype")]
    Subtype,
    // Used for all families of games such as party games and strategy games.
    #[serde(rename = "family")]
    Family,
}

// Helper function for deserialisers to convert a game's ranks as they are in the XML, to a single
// overall rank and a list of sub ranks. For a list of ranks in the XML, go through and return a the
// main one and a list of the rest. Will return an error if more than one generic rank is found.
pub(crate) fn xml_ranks_to_ranks<'de, D: serde::de::MapAccess<'de>>(
    xml_ranks: XmlRanks,
) -> core::result::Result<(ItemFamilyRank, Vec<ItemFamilyRank>), D::Error> {
    let mut rank = None;
    let mut sub_family_ranks = vec![];
    for family_rank in xml_ranks.ranks {
        match family_rank.game_family_type {
            ItemFamilyType::Subtype => {
                if rank.is_some() {
                    return Err(serde::de::Error::duplicate_field("rank type=\"subtype\""));
                }
                rank = Some(ItemFamilyRank {
                    id: family_rank.id,
                    name: family_rank.name,
                    friendly_name: family_rank.friendly_name,
                    value: family_rank.value,
                    bayesian_average: family_rank.bayesian_average,
                });
            },
            ItemFamilyType::Family => {
                sub_family_ranks.push(ItemFamilyRank {
                    id: family_rank.id,
                    name: family_rank.name,
                    friendly_name: family_rank.friendly_name,
                    value: family_rank.value,
                    bayesian_average: family_rank.bayesian_average,
                });
            },
        }
    }
    let rank = rank.ok_or_else(|| serde::de::Error::missing_field("rank type=\"subtype\""))?;

    Ok((rank, sub_family_ranks))
}

/// A struct containing the item's rank within a particular type of game.
#[derive(Debug, Deserialize)]
pub(crate) struct XmlItemFamilyRank {
    /// The type of this group of games. Can be `subtype` for rank within all
    /// board games. Or it can be `family` if it is the rank within a family of games
    /// such as party games or strategy games. We use this to determine whether to set it
    /// on the item's rank, or in their list of sub family ranks. But the field is not set
    /// on the actual [`GameFamilyRank`] struct
    #[serde(rename = "type")]
    pub(crate) game_family_type: ItemFamilyType,
    /// ID of the game family.
    pub(crate) id: u64,
    /// Name of the game type. "boardgame" used as the generic subtype that
    /// includes all board games.
    pub(crate) name: String,
    /// User friendly name in the format "GENRE game rank" e.g. "Party Game
    /// Rank".
    #[serde(rename = "friendlyname")]
    pub(crate) friendly_name: String,
    /// The overall rank on the site within this type of game.
    pub(crate) value: RankValue,
    /// The score out of 10, as a bayesian average.
    ///
    /// This is what boardgamegeek calls a Geek Rating. It is the average rating
    /// that the users have given it along with a few thousand 5.5 ratings added
    /// in too.
    #[serde(rename = "bayesaverage")]
    pub(crate) bayesian_average: f64,
}
