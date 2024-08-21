use core::fmt;

use serde::Deserialize;

use super::ItemType;
use crate::utils::{XmlSignedValue, XmlStringValue};

/// The returned struct containing a list of search results.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SearchResults {
    /// The list of board games returned by the search.
    #[serde(rename = "$value")]
    pub results: Vec<SearchResult>,
}

/// A result when searching for a name. Includes the game's name, type, and year
/// published.
#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult {
    /// The ID of the game.
    pub id: u64,
    /// The type of game, which will either be a board game, expansion or accessory for a board
    /// game.
    pub item_type: ItemType,
    /// The name of the game.
    pub name: String,
    /// The year the game was first published.
    pub year_published: i64,
}

impl<'de> Deserialize<'de> for SearchResult {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            ID,
            Type,
            Name,
            YearPublished,
        }

        struct SearchResultVisitor;

        impl<'de> serde::de::Visitor<'de> for SearchResultVisitor {
            type Value = SearchResult;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string containing the XML for a search result.")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut item_type = None;
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
                        Field::Type => {
                            if item_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            item_type = Some(map.next_value()?);
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
                let item_type =
                    item_type.ok_or_else(|| serde::de::Error::missing_field("item_type"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let year_published = year_published
                    .ok_or_else(|| serde::de::Error::missing_field("yearpublished"))?;
                Ok(Self::Value {
                    id,
                    item_type,
                    name,
                    year_published,
                })
            }
        }
        deserializer.deserialize_any(SearchResultVisitor)
    }
}
