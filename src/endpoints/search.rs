use core::fmt;

use serde::Deserialize;

use super::GameType;
use crate::utils::{XmlSignedValue, XmlStringValue};
use crate::{BoardGameGeekApi, Result};

/// The returned struct containing a list of search results.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SearchResults {
    /// The list of board games returned by the search.
    #[serde(rename = "$value")]
    pub results: Vec<SearchResult>,
}

/// A result when searching for a name. Includes the game's name, type, and year published.
#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult {
    /// The ID of the game.
    pub id: u64,
    /// The type of game, which will either be boardgame or expansion.
    pub item_type: GameType,
    /// The name of the game.
    pub name: String,
    /// The year the game was first published.
    pub year_published: i64,
}

impl<'de> Deserialize<'de> for SearchResult {
    fn deserialize<D: serde::de::Deserializer<'de>>(
        deserializer: D,
    ) -> core::result::Result<Self, D::Error> {
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

            fn visit_map<A>(self, mut map: A) -> core::result::Result<Self::Value, A::Error>
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
                        }
                        Field::Type => {
                            if item_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            item_type = Some(map.next_value()?);
                        }
                        Field::Name => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            let name_xml_tag: XmlStringValue = map.next_value()?;
                            name = Some(name_xml_tag.value);
                        }
                        Field::YearPublished => {
                            if year_published.is_some() {
                                return Err(serde::de::Error::duplicate_field("yearpublished"));
                            }
                            let year_published_xml_tag: XmlSignedValue = map.next_value()?;
                            year_published = Some(year_published_xml_tag.value);
                        }
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

/// Required query paramters.
#[derive(Clone, Debug)]
pub struct BaseSearchQuery<'q> {
    pub(crate) query: &'q str,
}

/// All optional query parameters for making a request to the
/// search endpoint.
#[derive(Clone, Debug, Default)]
pub struct SearchQueryParams {
    /// Include only results for this game type.
    ///
    /// Note, if this is set to [GameType::BoardGame] then it will include both
    /// board games and expansions, but set the type of all of them to be
    /// [GameType::BoardGame] in the results. There does not seem to be a way around
    /// this.
    game_type: Option<GameType>,
    /// Limit results to only exact matches of the search query.
    exact: Option<bool>,
}

impl SearchQueryParams {
    /// Constructs a new search query with parameters set to None.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the game_type query param, so that only expansions or board games can be filtered when searching.
    pub fn game_type(mut self, game_type: GameType) -> Self {
        self.game_type = Some(game_type);
        self
    }

    /// Sets the exact query param, so that exact matches will be returned if set to true.
    pub fn exact(mut self, exact: bool) -> Self {
        self.exact = Some(exact);
        self
    }
}

/// Struct for building a query for the request to the search endpoint.
#[derive(Clone, Debug)]
struct SearchQueryBuilder<'q> {
    base: BaseSearchQuery<'q>,
    params: SearchQueryParams,
}

impl<'builder> SearchQueryBuilder<'builder> {
    /// Constructs a new query builder from a base query, and the rest of the parameters.
    fn new(base: BaseSearchQuery<'builder>, params: SearchQueryParams) -> Self {
        Self { base, params }
    }

    pub fn build(self) -> Vec<(&'builder str, String)> {
        let mut query_params: Vec<_> = vec![];
        query_params.push(("query", self.base.query.to_string()));

        match self.params.exact {
            Some(true) => query_params.push(("exact", "1".to_string())),
            Some(false) => query_params.push(("exact", "0".to_string())),
            None => {}
        }
        match self.params.game_type {
            Some(GameType::BoardGame) => query_params.push(("type", "boardgame".to_string())),
            Some(GameType::BoardGameExpansion) => {
                query_params.push(("type", "boardgameexpansion".to_string()))
            }
            None => {}
        }
        query_params
    }
}

/// Search endpoint of the API. Used for searching for games by name.
pub struct SearchApi<'api> {
    pub(crate) api: &'api BoardGameGeekApi,
    endpoint: &'static str,
}

impl<'api> SearchApi<'api> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "search",
        }
    }

    /// Searches with a given query.
    pub async fn search(&self, query: &str) -> Result<SearchResults> {
        let query = SearchQueryBuilder::new(BaseSearchQuery { query }, SearchQueryParams::new());

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<SearchResults>(request).await
    }

    /// Searches for exact matches to a given query.
    pub async fn search_exact(&self, query: &str) -> Result<SearchResults> {
        let query = SearchQueryBuilder::new(
            BaseSearchQuery { query },
            SearchQueryParams::new().exact(true),
        );

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<SearchResults>(request).await
    }

    /// Makes a request from a [SearchQueryParams].
    pub async fn search_with_query_params(
        &self,
        query: &str,
        query_params: SearchQueryParams,
    ) -> Result<SearchResults> {
        let query = SearchQueryBuilder::new(BaseSearchQuery { query }, query_params);

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<SearchResults>(request).await
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use super::*;

    #[tokio::test]
    async fn search() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/search")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "query".into(),
                "some search term".into(),
            )]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/search/search.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let search_results = api.search().search("some search term").await;
        mock.assert_async().await;

        assert!(search_results.is_ok(), "error returned when okay expected");
        let search_results = search_results.unwrap();

        assert_eq!(search_results.results.len(), 2);
        assert_eq!(
            search_results.results[0],
            SearchResult {
                id: 312484,
                item_type: GameType::BoardGame,
                name: "Lost Ruins of Arnak".into(),
                year_published: 2020,
            },
        );
        assert_eq!(
            search_results.results[1],
            SearchResult {
                id: 341254,
                item_type: GameType::BoardGameExpansion,
                name: "Lost Ruins of Arnak: Expedition Leaders".into(),
                year_published: 2021,
            },
        );
    }

    #[tokio::test]
    async fn search_exact() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/search")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("query".into(), "lost ruins of arnak".into()),
                Matcher::UrlEncoded("exact".into(), "1".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/search/search_exact.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let search_results = api.search().search_exact("lost ruins of arnak").await;
        mock.assert_async().await;

        assert!(search_results.is_ok(), "error returned when okay expected");
        let search_results = search_results.unwrap();

        assert_eq!(search_results.results.len(), 1);
        assert_eq!(
            search_results.results[0],
            SearchResult {
                id: 312484,
                item_type: GameType::BoardGame,
                name: "Lost Ruins of Arnak".into(),
                year_published: 2020,
            },
        );
    }

    #[tokio::test]
    async fn search_with_query_params() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/search")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("query".into(), "arnak".into()),
                Matcher::UrlEncoded("exact".into(), "0".into()),
                Matcher::UrlEncoded("type".into(), "boardgameexpansion".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/search/search_expansions.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let search_results = api
            .search()
            .search_with_query_params(
                "arnak",
                SearchQueryParams::new()
                    .game_type(GameType::BoardGameExpansion)
                    .exact(false),
            )
            .await;
        mock.assert_async().await;

        assert!(search_results.is_ok(), "error returned when okay expected");
        let search_results = search_results.unwrap();

        assert_eq!(search_results.results.len(), 1);
        assert_eq!(
            search_results.results[0],
            SearchResult {
                id: 341254,
                item_type: GameType::BoardGameExpansion,
                name: "Lost Ruins of Arnak: Expedition Leaders".into(),
                year_published: 2021,
            },
        );

        let mock = server
            .mock("GET", "/search")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("query".into(), "lost ruins of arnak".into()),
                Matcher::UrlEncoded("exact".into(), "1".into()),
                Matcher::UrlEncoded("type".into(), "boardgame".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/search/search_exact.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let search_results = api
            .search()
            .search_with_query_params(
                "lost ruins of arnak",
                SearchQueryParams::new()
                    .exact(true)
                    .game_type(GameType::BoardGame),
            )
            .await;
        mock.assert_async().await;

        assert!(search_results.is_ok(), "error returned when okay expected");
        let search_results = search_results.unwrap();

        assert_eq!(search_results.results.len(), 1);
        assert_eq!(
            search_results.results[0],
            SearchResult {
                id: 312484,
                item_type: GameType::BoardGame,
                name: "Lost Ruins of Arnak".into(),
                year_published: 2020,
            },
        );
    }
}
