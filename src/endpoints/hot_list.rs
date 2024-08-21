use core::fmt;

use serde::Deserialize;

use crate::utils::{XmlSignedValue, XmlStringValue};
use crate::{BoardGameGeekApi, Result};

/// The returned struct containing a list of hot board games.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct HotList {
    /// The list of hot board games.
    #[serde(rename = "$value")]
    pub items: Vec<HotItem>,
}

/// An item on the hot list, has the rank from 1 to 50 on the list,
/// as well as some basic information about the game like the name
/// and year published.
#[derive(Clone, Debug, PartialEq)]
pub struct HotItem {
    /// The ID of the game.
    pub id: u64,
    /// The rank within the hotlist, should be ordered from 1 to 50.
    pub rank: u64,
    /// A link to a jpg thumbnail image for the game.
    pub thumbnail: String,
    /// The name of the game.
    pub name: String,
    /// The year the game was first published.
    pub year_published: i64,
}

impl<'de> Deserialize<'de> for HotItem {
    fn deserialize<D: serde::de::Deserializer<'de>>(
        deserializer: D,
    ) -> core::result::Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            ID,
            Rank,
            Thumbnail,
            Name,
            YearPublished,
        }

        struct HotItemVisitor;

        impl<'de> serde::de::Visitor<'de> for HotItemVisitor {
            type Value = HotItem;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string containing the XML an item off the hot list.")
            }

            fn visit_map<A>(self, mut map: A) -> core::result::Result<Self::Value, A::Error>
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
                let thumbnail =
                    thumbnail.ok_or_else(|| serde::de::Error::missing_field("thumbnail"))?;
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
        deserializer.deserialize_any(HotItemVisitor)
    }
}

/// Hot list endpoint of the API. Used for returning the current trending board
/// games.
pub struct HotListApi<'api> {
    pub(crate) api: &'api BoardGameGeekApi,
    endpoint: &'static str,
}

impl<'api> HotListApi<'api> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "hot",
        }
    }

    /// Gets the hot list.
    pub async fn get(&self) -> Result<HotList> {
        let request = self.api.build_request(self.endpoint, &[]);
        self.api.execute_request(request).await
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use super::*;

    #[tokio::test]
    async fn get() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/hot")
            .match_query(Matcher::AllOf(vec![]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/hot_list/hot_list.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let hot_list = api.hot_list().get().await;
        mock.assert_async().await;

        assert!(hot_list.is_ok(), "error returned when okay expected");
        let hot_list = hot_list.unwrap();

        assert_eq!(hot_list.items.len(), 50);
        assert_eq!(
            hot_list.items[0],
            HotItem {
                id: 359871,
                rank: 1,
                thumbnail: "https://cf.geekdo-images.com/XWImAu_3RK61wbzcKboVdA__thumb/img/Ry-6KHwNgERWadyxs1X1_P3dMvY=/fit-in/200x150/filters:strip_icc()/pic8145530.png".into(),
                name: "Arcs".into(),
                year_published: 2024,
            }
        )
    }
}
