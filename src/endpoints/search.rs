use super::{ItemType, SearchResults};
use crate::{BoardGameGeekApi, IntoQueryParam, QueryParam, Result};

/// All optional query parameters for making a request to the
/// search endpoint.
#[derive(Clone, Debug, Default)]
pub struct SearchQueryParams {
    /// Include only results for the provided item types. If none are provided
    /// then it will default to [`ItemType::BoardGame`].
    ///
    /// Note, if this is set to [`ItemType::BoardGame`], or left unset, then it will include both
    /// board games and expansions, but set the type of all of them to be
    /// [`ItemType::BoardGame`] in the results. There does not seem to be a way
    /// around this.
    item_types: Vec<ItemType>,
    /// Limit results to only exact matches of the search query.
    exact: Option<bool>,
}

impl SearchQueryParams {
    /// Constructs a new search query with parameters set to None.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an item to the `item_type` query param, so that that type of
    /// item will be returned from the search. It should be noted that if [`ItemType::BoardGame`]
    /// is chosen then this will return both board games and board game expansions,
    /// with the type set to board game for both.
    ///
    /// If the parameter is omitted then it will default to [`ItemType::BoardGame`].
    ///
    /// If the parameter includes both [`ItemType::BoardGame`], and [`ItemType::BoardGameExpansion`]
    /// then board game expansions will be returned twice, once with the type
    /// [`ItemType::BoardGame`] and once with the type [`ItemType::BoardGameExpansion`].
    pub fn item_type(mut self, item_type: ItemType) -> Self {
        self.item_types.push(item_type);
        self
    }

    /// Adds a list of item types to the `item_types` query param, so that items of these types
    /// will be returned from the search. It should be noted that if [`ItemType::BoardGame`]
    /// is chosen then this will return both board games and board game expansions,
    /// with the type set to board game for both.
    ///
    /// If the parameter is omitted then it will default to [`ItemType::BoardGame`].
    ///
    /// If the parameter includes both [`ItemType::BoardGame`], and [`ItemType::BoardGameExpansion`]
    /// then board game expansions will be returned twice, once with the type
    /// [`ItemType::BoardGame`] and once with the type [`ItemType::BoardGameExpansion`].
    pub fn item_types(mut self, item_types: Vec<ItemType>) -> Self {
        self.item_types.extend(item_types);
        self
    }

    /// Sets the `exact` query param, so that exact matches will be returned if
    /// set to true.
    pub fn exact(mut self, exact: bool) -> Self {
        self.exact = Some(exact);
        self
    }
}

// Struct for building a query for the request to the search endpoint.
#[derive(Clone, Debug)]
struct SearchQueryBuilder<'q> {
    search_query: &'q str,
    params: SearchQueryParams,
}

impl<'builder> SearchQueryBuilder<'builder> {
    // Constructs a new query builder from a search query, and the rest of the
    // parameters.
    fn new(search_query: &'builder str, params: SearchQueryParams) -> Self {
        Self {
            search_query,
            params,
        }
    }

    // Converts the list of parameters into a vector of
    // key value pairs that reqwest can use as HTTP query parameters.
    fn build(self) -> Vec<QueryParam<'builder>> {
        let mut query_params: Vec<_> = vec![];
        query_params.push(self.search_query.into_query_param("query"));

        if let Some(value) = self.params.exact {
            query_params.push(value.into_query_param("exact"));
        }
        if self.params.item_types.is_empty() {
            query_params.push(ItemType::BoardGame.into_query_param("type"));
        } else {
            query_params.push(self.params.item_types.into_query_param("type"));
        }
        query_params
    }
}

/// Search endpoint of the API. Used for searching for games and other items by name.
///
/// A maximum of 500 items will be returned by the API per type provided, with no option
/// for pagination. So if the page doesn't include the desired item the query must be made
/// more specific.
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

    /// Searches with a given query, and no additional query parameters set.
    /// This defaults to returning only games, returning both board games
    /// and expansions. However, expansions will be included in the results twice,
    /// once with the type [`ItemType::BoardGame`] and once with the type
    /// [`ItemType::BoardGameExpansion`].
    pub async fn search(&self, query: &str) -> Result<SearchResults> {
        let query = SearchQueryBuilder::new(query, SearchQueryParams::new());

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<SearchResults>(request).await
    }

    /// Searches for exact matches to a given query, and no additional query parameters set.
    /// This defaults to returning only games, returning both board games
    /// and expansions. However, expansions will be included in the results twice,
    /// once with the type [`ItemType::BoardGame`] and once with the type
    /// [`ItemType::BoardGameExpansion`].
    pub async fn search_exact(&self, query: &str) -> Result<SearchResults> {
        let query = SearchQueryBuilder::new(query, SearchQueryParams::new().exact(true));

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<SearchResults>(request).await
    }

    /// Makes a request from a [`SearchQueryParams`].
    pub async fn search_with_query_params(
        &self,
        query: &str,
        query_params: SearchQueryParams,
    ) -> Result<SearchResults> {
        let query = SearchQueryBuilder::new(query, query_params);

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<SearchResults>(request).await
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use super::*;
    use crate::{ItemType, SearchResult};

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
                id: 312_484,
                item_type: ItemType::BoardGame,
                name: "Lost Ruins of Arnak".into(),
                year_published: Some(2020),
            },
        );
        assert_eq!(
            search_results.results[1],
            SearchResult {
                id: 341_254,
                item_type: ItemType::BoardGameExpansion,
                name: "Lost Ruins of Arnak: Expedition Leaders".into(),
                year_published: Some(2021),
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
                id: 312_484,
                item_type: ItemType::BoardGame,
                name: "Lost Ruins of Arnak".into(),
                year_published: Some(2020),
            },
        );
    }

    #[tokio::test]
    async fn search_double_quotes() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/search")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "query".into(),
                "a".into(),
            )]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/search/search_result_quotes.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let search_results = api.search().search("a").await;
        mock.assert_async().await;

        assert!(search_results.is_ok(), "error returned when okay expected");
        let search_results = search_results.unwrap();

        assert_eq!(search_results.results.len(), 2);
        assert_eq!(
            search_results.results[0],
            SearchResult {
                id: 12668,
                item_type: ItemType::BoardGame,
                name: "\"Get Smart\"".into(),
                year_published: Some(1965),
            },
        );
        assert_eq!(
            search_results.results[1],
            SearchResult {
                id: 30346,
                item_type: ItemType::BoardGame,
                name: "\"Get Smart\" Card Game".into(),
                year_published: None,
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
                    .item_type(ItemType::BoardGameExpansion)
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
                id: 341_254,
                item_type: ItemType::BoardGameExpansion,
                name: "Lost Ruins of Arnak: Expedition Leaders".into(),
                year_published: Some(2021),
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
                    .item_type(ItemType::BoardGame),
            )
            .await;
        mock.assert_async().await;

        assert!(search_results.is_ok(), "error returned when okay expected");
        let search_results = search_results.unwrap();

        assert_eq!(search_results.results.len(), 1);
        assert_eq!(
            search_results.results[0],
            SearchResult {
                id: 312_484,
                item_type: ItemType::BoardGame,
                name: "Lost Ruins of Arnak".into(),
                year_published: Some(2020),
            },
        );
    }

    #[tokio::test]
    async fn search_multiple_types() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/search")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("query".into(), "arnak".into()),
                Matcher::UrlEncoded(
                    "type".into(),
                    "boardgame,boardgameaccessory,boardgameartist".into(),
                ),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/search/search_game_and_accessories.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let search_results = api
            .search()
            .search_with_query_params(
                "arnak",
                SearchQueryParams::new()
                    .item_type(ItemType::BoardGame)
                    .item_types(vec![
                        ItemType::BoardGameAccessory,
                        ItemType::BoardGameArtist,
                    ]),
            )
            .await;
        mock.assert_async().await;

        assert!(search_results.is_ok(), "error returned when okay expected");
        let search_results = search_results.unwrap();

        assert_eq!(search_results.results.len(), 2);
        assert_eq!(
            search_results.results[0],
            SearchResult {
                id: 403_238,
                item_type: ItemType::BoardGameAccessory,
                name: "Lost Ruins of Arnak + Expansions: The GiftForge Insert".into(),
                year_published: Some(2023),
            },
        );
        assert_eq!(
            search_results.results[1],
            SearchResult {
                id: 312_484,
                item_type: ItemType::BoardGame,
                name: "Lost Ruins of Arnak".into(),
                year_published: Some(2020),
            },
        );
    }
}
