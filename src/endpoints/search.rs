use super::{ItemType, SearchResults};
use crate::{BoardGameGeekApi, QueryParam, Result};

/// All optional query parameters for making a request to the
/// search endpoint.
#[derive(Clone, Debug, Default)]
pub struct SearchQueryParams {
    /// Include only results for this game type. All apart from
    /// [ItemType::BoardGameAccessory] returned if the parameter is omitted.
    ///
    /// Note, if this is set to [ItemType::BoardGame] then it will include both
    /// board games and expansions, but set the type of all of them to be
    /// [ItemType::BoardGame] in the results. There does not seem to be a way
    /// around this.
    item_type: Option<ItemType>,
    /// Limit results to only exact matches of the search query.
    exact: Option<bool>,
}

impl SearchQueryParams {
    /// Constructs a new search query with parameters set to None.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the item_type query param, so that only a certain type of
    /// item will be returned. It should be noted that if [ItemType::BoardGame]
    /// is chosen then this will return both board games and board game expansions,
    /// with the type set to board game for both.
    ///
    /// If the param is omitted then all types will be returned apart from board
    /// game accessories. If these are to be searched the type must be set explicitly.
    ///
    /// Also, if the parameter is omitted, board game expansions will be returned twice,
    /// once with the type [ItemType::BoardGame] and once with the type
    /// [ItemType::BoardGameExpansion].
    pub fn item_type(mut self, item_type: ItemType) -> Self {
        self.item_type = Some(item_type);
        self
    }

    /// Sets the exact query param, so that exact matches will be returned if
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
        query_params.push(("query", self.search_query.to_string()));

        match self.params.exact {
            Some(true) => query_params.push(("exact", "1".to_string())),
            Some(false) => query_params.push(("exact", "0".to_string())),
            None => {},
        }
        match self.params.item_type {
            Some(ItemType::BoardGame) => query_params.push(("type", "boardgame".to_string())),
            Some(ItemType::BoardGameExpansion) => {
                query_params.push(("type", "boardgameexpansion".to_string()))
            },
            Some(ItemType::BoardGameAccessory) => {
                query_params.push(("type", "boardgameaccessory".to_string()))
            },
            None => {},
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

    /// Searches with a given query, and no additional query parameters set.
    /// This defaults to returning only games, returning both board games
    /// and expansions. However, expansions will be included in the results twice,
    /// once with the type [ItemType::BoardGame] and once with the type
    /// [ItemType::BoardGameExpansion].
    pub async fn search(&self, query: &str) -> Result<SearchResults> {
        let query = SearchQueryBuilder::new(query, SearchQueryParams::new());

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<SearchResults>(request).await
    }

    /// Searches for exact matches to a given query, and no additional query parameters set.
    /// This defaults to returning only games, returning both board games
    /// and expansions. However, expansions will be included in the results twice,
    /// once with the type [ItemType::BoardGame] and once with the type
    /// [ItemType::BoardGameExpansion].
    pub async fn search_exact(&self, query: &str) -> Result<SearchResults> {
        let query = SearchQueryBuilder::new(query, SearchQueryParams::new().exact(true));

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<SearchResults>(request).await
    }

    /// Makes a request from a [SearchQueryParams].
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
                id: 312484,
                item_type: ItemType::BoardGame,
                name: "Lost Ruins of Arnak".into(),
                year_published: 2020,
            },
        );
        assert_eq!(
            search_results.results[1],
            SearchResult {
                id: 341254,
                item_type: ItemType::BoardGameExpansion,
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
                item_type: ItemType::BoardGame,
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
                id: 341254,
                item_type: ItemType::BoardGameExpansion,
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
                id: 312484,
                item_type: ItemType::BoardGame,
                name: "Lost Ruins of Arnak".into(),
                year_published: 2020,
            },
        );
    }
}
