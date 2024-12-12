use core::fmt;

use serde::Deserialize;

use crate::deserialize::{XmlSignedValue, XmlStringValue};

/// The returned struct containing a list of hot board games.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct HotList {
    /// The list of trending board games currently on the hot list.
    #[serde(default, rename = "item")]
    pub(crate) games: Vec<HotListGame>,
}

/// An game on the hot list, has the rank from 1 to 50 on the list,
/// as well as some basic information about the game like the name
/// and year published.
#[derive(Clone, Debug, PartialEq)]
pub struct HotListGame {
    /// The ID of the game.
    pub id: u64,
    /// The rank within the hot list, should be ordered from 1 to 50.
    pub rank: u64,
    /// A link to a jpg thumbnail image for the game.
    pub thumbnail: Option<String>,
    /// The name of the game.
    pub name: String,
    /// The year the game was first published.
    pub year_published: i64,
}

impl<'de> Deserialize<'de> for HotListGame {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            ID,
            Rank,
            Thumbnail,
            Name,
            YearPublished,
        }

        struct HotListGameVisitor;

        impl<'de> serde::de::Visitor<'de> for HotListGameVisitor {
            type Value = HotListGame;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string containing the XML a game off the hot list.")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut rank = None;
                let mut thumbnail = None;
                let mut name = None;
                let mut year_published = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ID => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            let id_str: String = map.next_value()?;
                            id = Some(id_str.parse::<u64>().map_err(|e| {
                                serde::de::Error::custom(format!(
                                    "failed to parse value a u64: {e}"
                                ))
                            })?);
                        },
                        Field::Rank => {
                            if rank.is_some() {
                                return Err(serde::de::Error::duplicate_field("rank"));
                            }
                            let rank_str: String = map.next_value()?;
                            rank = Some(rank_str.parse::<u64>().map_err(|e| {
                                serde::de::Error::custom(format!(
                                    "failed to parse value a u64: {e}"
                                ))
                            })?);
                        },
                        Field::Thumbnail => {
                            if thumbnail.is_some() {
                                return Err(serde::de::Error::duplicate_field("thumbnail"));
                            }
                            let thumbnail_xml_tag: XmlStringValue = map.next_value()?;
                            thumbnail = Some(thumbnail_xml_tag.value);
                        },
                        Field::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            let name_xml_tag: XmlStringValue = map.next_value()?;
                            name = Some(name_xml_tag.value);
                        },
                        Field::YearPublished => {
                            if year_published.is_some() {
                                return Err(serde::de::Error::duplicate_field("yearpublished"));
                            }
                            let year_published_xml_tag: XmlSignedValue = map.next_value()?;
                            year_published = Some(year_published_xml_tag.value);
                        },
                    }
                }
                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let rank = rank.ok_or_else(|| serde::de::Error::missing_field("rank"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let year_published = year_published
                    .ok_or_else(|| serde::de::Error::missing_field("yearpublished"))?;
                Ok(Self::Value {
                    id,
                    rank,
                    thumbnail,
                    name,
                    year_published,
                })
            }
        }
        deserializer.deserialize_any(HotListGameVisitor)
    }
}
