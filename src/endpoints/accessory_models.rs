use core::fmt;

use serde::Deserialize;

use crate::deserialize::{XmlLink, XmlName, XmlSignedValue, XmlStringValue};
use crate::{
    Game, GameArtist, GameDesigner, GamePublisher, MarketplaceListing, NameType, RatingCommentPage,
    XmlMarketplaceListings,
};

// A struct containing the list of requested accessories with the full details.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Accessories {
    // List of accessories.
    #[serde(default, rename = "item")]
    pub(crate) accessories: Vec<AccessoryDetails>,
}

/// A game, or expansion, with full details.
///
/// A game returned from the game endpoint which includes all details about a game.
/// This includes the name, description, image and thumbnail. As well as lists of all
/// the alternate names, expansions, artists and publishers.
///
/// Some information, such as version info, comments, and marketplace data is only
/// optionally included if requested.
#[derive(Clone, Debug, PartialEq)]
pub struct AccessoryDetails {
    /// The ID of the accessory.
    pub id: u64,
    /// The name of the accessory.
    pub name: String,
    /// A list of alternate names for the accessory, usually translations of the primary name.
    pub alternate_names: Vec<String>,
    /// A brief description of the accessory.
    pub description: String,
    /// A link to a jpg image for the accessory. Can by empty.
    pub image: Option<String>,
    /// A link to a jpg thumbnail image for the accessory. Can by empty.
    pub thumbnail: Option<String>,
    // TODO check if this is always 0
    /// The year the accessory was first published.
    pub year_published: i64,
    /// A list of games that this is an accessory for.
    pub accessory_for: Vec<Game>,
    /// The designer of this accessory.
    pub designers: Vec<GameDesigner>,
    /// A list of artists for this game.
    pub artists: Vec<GameArtist>,
    /// The list of publishers for this accessory.
    pub publishers: Vec<GamePublisher>,
    /// Information for the various versions of the accessory.
    pub versions: Vec<AccessoryVersion>,
    /// Information of where to buy the accessory and for how much.
    pub marketplace_listings: Vec<MarketplaceListing>,
    /// List of comments and ratings users have given to the accessory.
    ///
    /// Each comment may have a rating but no comment, a comment but no rating, or both. However
    /// the underlying API will only return all comments, whether or not they have ratings,
    /// with the `include_comments` query parameter. Or all ratings, whether or not they have
    /// comments. However the same tag is used to return them so there is no way to return all
    /// comments and ratings together.
    ///
    /// Page number and page size can be controlled via query parameters.
    pub rating_comments: Option<RatingCommentPage>,
}

// A user's collection on boardgamegeek.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) struct XmlAccessoryVersions {
    // List of versions, each in an XML tag called `item`, within an outer
    // `version`. We use this intermediary type to get out just the first,
    // since we only expect 1.
    #[serde(rename = "item")]
    pub(crate) versions: Vec<AccessoryVersion>,
}

/// Information about a version of this accessory
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AccessoryVersion {
    /// The ID of this accessory.
    pub id: u64,
    /// The name of the accessory.
    #[serde(
        deserialize_with = "deserialize_accessory_version_name",
        rename = "canonicalname",
    )]
    pub name: String,
    /// A link to a jpg image for the accessory.
    pub image: Option<String>,
    /// A link to a jpg thumbnail image for the accessory.
    pub thumbnail: Option<String>,
}

fn deserialize_accessory_version_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let name_value_xml = XmlStringValue::deserialize(deserializer)?;
    Ok(name_value_xml.value)
}

impl<'de> Deserialize<'de> for AccessoryDetails {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Id,
            Type,
            Thumbnail,
            Image,
            Name,
            Description,
            YearPublished,
            Link,
            Versions,
            MarketPlaceListings,
            Comments,
        }

        struct AccessoryDetailsVisitor;

        impl<'de> serde::de::Visitor<'de> for AccessoryDetailsVisitor {
            type Value = AccessoryDetails;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an XML object for a board game accessory returned by the `thing` endpoint from boardgamegeek")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut id = None;
                let mut thumbnail = None;
                let mut image = None;
                let mut name = None;
                let mut alternate_names = vec![];
                let mut description = None;
                let mut year_published = None;
                // Link tags
                let mut accessory_for = vec![];
                let mut designers = vec![];
                let mut artists = vec![];
                let mut publishers = vec![];
                // Other
                let mut versions = None;
                let mut marketplace_listings = None;
                let mut rating_comments = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        },
                        Field::Type => {
                            // Ignore, it's always the same thing
                            let _: String = map.next_value()?;
                        },
                        Field::Thumbnail => {
                            if thumbnail.is_some() {
                                return Err(serde::de::Error::duplicate_field("thumbnail"));
                            }
                            thumbnail = Some(map.next_value()?);
                        },
                        Field::Image => {
                            if image.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            image = Some(map.next_value()?);
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
                        Field::Description => {
                            if description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        },
                        Field::YearPublished => {
                            if year_published.is_some() {
                                return Err(serde::de::Error::duplicate_field("yearpublished"));
                            }
                            let year_published_xml: XmlSignedValue = map.next_value()?;
                            year_published = Some(year_published_xml.value);
                        },
                        Field::Link => {
                            let link: XmlLink = map.next_value()?;
                            match link.link_type {
                                crate::ItemType::BoardGameAccessory => {
                                    // The type "boardgameaccessory" with "inbound=true" is used to
                                    // list games that this is an accessory for.
                                    accessory_for.push(Game {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                crate::ItemType::BoardGameDesigner => {
                                    designers.push(GameDesigner {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                crate::ItemType::BoardGameArtist => {
                                    artists.push(GameArtist {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                crate::ItemType::BoardGamePublisher => {
                                    publishers.push(GamePublisher {
                                        id: link.id,
                                        name: link.value,
                                    });
                                },
                                link_type => {
                                    return Err(serde::de::Error::custom(format!(
                                        "found unexpected \"{link_type:?}\" link in game info",
                                    )));
                                },
                            }
                        },
                        Field::Versions => {
                            if versions.is_some() {
                                return Err(serde::de::Error::duplicate_field("versions"));
                            }
                            let versions_xml: XmlAccessoryVersions = map.next_value()?;
                            versions = Some(versions_xml.versions);
                        },
                        Field::MarketPlaceListings => {
                            if marketplace_listings.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "marketplacelistings",
                                ));
                            }
                            let marketplace_listings_xml: XmlMarketplaceListings =
                                map.next_value()?;
                            marketplace_listings = Some(marketplace_listings_xml.listings);
                        },
                        Field::Comments => {
                            if rating_comments.is_some() {
                                return Err(serde::de::Error::duplicate_field("comments"));
                            }
                            rating_comments = Some(map.next_value()?);
                        },
                    }
                }
                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let thumbnail =
                    thumbnail.ok_or_else(|| serde::de::Error::missing_field("thumbnail"))?;
                let image = image.ok_or_else(|| serde::de::Error::missing_field("image"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;
                let description =
                    description.ok_or_else(|| serde::de::Error::missing_field("description"))?;
                let year_published = year_published
                    .ok_or_else(|| serde::de::Error::missing_field("yearpublished"))?;

                let versions = versions.unwrap_or_default();
                let marketplace_listings = marketplace_listings.unwrap_or_default();

                Ok(Self::Value {
                    id,
                    name,
                    alternate_names,
                    description,
                    image,
                    thumbnail,
                    year_published,
                    accessory_for,
                    designers,
                    artists,
                    publishers,
                    versions,
                    marketplace_listings,
                    rating_comments,
                })
            }
        }
        deserializer.deserialize_any(AccessoryDetailsVisitor)
    }
}
