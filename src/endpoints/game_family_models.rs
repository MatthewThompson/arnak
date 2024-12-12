use core::fmt;

use serde::Deserialize;

use super::Game;
use crate::deserialize::{XmlLink, XmlName};
use crate::{ItemType, NameType};

// A list of game families. Which are groups of games in a particular series.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct GameFamilies {
    // List of game families, and the games that they contain.
    #[serde(default, rename = "item")]
    pub(crate) game_families: Vec<GameFamily>,
}

/// A family of games in a particular series or group. Contains the description for the
/// family as well as the list of games.
#[derive(Clone, Debug, PartialEq)]
pub struct GameFamily {
    /// The ID of the game family.
    pub id: u64,
    /// The name of the game family.
    pub name: String,
    /// A list of alternate names for the game family.
    pub alternate_names: Vec<String>,
    /// A link to a jpg image for the game family.
    pub image: Option<String>,
    /// A link to a jpg thumbnail image for the game family.
    pub thumbnail: Option<String>,
    /// A description of the group of games.
    pub description: String,
    /// The list of games in this game family.
    pub games: Vec<Game>,
}

impl<'de> Deserialize<'de> for GameFamily {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Id,
            Name,
            Image,
            Thumbnail,
            Description,
            // Each game is in an individual XML tag called `link`
            Link,
            Type,
        }

        struct GameFamilyVisitor;

        impl<'de> serde::de::Visitor<'de> for GameFamilyVisitor {
            type Value = GameFamily;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string containing the XML for a family of games.")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut name = None;
                let mut alternate_names = vec![];
                let mut image = None;
                let mut thumbnail = None;
                let mut description = None;
                let mut games = vec![];
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
                        Field::Description => {
                            if description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        },
                        Field::Link => {
                            let link: XmlLink = map.next_value()?;
                            match link.link_type {
                                ItemType::BoardGameFamily => {
                                    games.push(Game {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                link_type => {
                                    return Err(serde::de::Error::custom(format!(
                                        "found unexpected \"{link_type:?}\" link in game family",
                                    )));
                                },
                            }
                        },
                        Field::Type => {
                            // Type is fixed at "boardgamefamily", even for the list of games
                            // contained so we don't add it. But we need
                            // to consume the value.
                            let _: String = map.next_value()?;
                        },
                    }
                }
                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let thumbnail =
                    thumbnail.ok_or_else(|| serde::de::Error::missing_field("thumbnail"))?;
                let image = image.ok_or_else(|| serde::de::Error::missing_field("image"))?;
                let description =
                    description.ok_or_else(|| serde::de::Error::missing_field("description"))?;
                Ok(Self::Value {
                    id,
                    name,
                    alternate_names,
                    image,
                    thumbnail,
                    description,
                    games,
                })
            }
        }
        deserializer.deserialize_any(GameFamilyVisitor)
    }
}
