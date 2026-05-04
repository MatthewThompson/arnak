use super::ItemType;
use crate::{
    Accessories, AccessoryDetails, BoardGameGeekApi, Error, IntoQueryParam, QueryParam, Result,
};

/// All optional query parameters for making a request to the accessory endpoint.
#[derive(Clone, Debug, Default)]
pub struct AccessoryQueryParams {
    // Whether to include the version information.
    include_versions: Option<bool>,
    // Whether to include marketplace data for the accessory.
    include_marketplace_data: Option<bool>,
    // Whether to include a page of comments for the accessory.
    //
    // Comment will include the rating too if there was one included. Sorted by username ascending.
    // Cannot be used in conjunction with rating comments.
    include_comments: Option<bool>,
    // Whether to include a page of rating comments for the accessory.
    //
    // A rating comment is a rating for a accessory, which will also include a comment if there was
    // one. Sorted by rating descending. Cannot be used in conjunction with comments.
    include_rating_comments: Option<bool>,
    // Which page of comments and videos to return. Default 1.
    page: Option<u64>,
    // Size of the comment and video pages, between 10 and 100.
    page_size: Option<u64>,
}

impl AccessoryQueryParams {
    /// Constructs a new accessory query with parameters set to None.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the `include_versions` query parameter. If set then information about different
    /// versions of the accessory will be included, if applicable.
    pub fn include_versions(mut self, include_versions: bool) -> Self {
        self.include_versions = Some(include_versions);
        self
    }

    /// Sets the `include_marketplace_data` query parameter. If set then information about where to
    /// buy the accessory and for what cost will be included.
    pub fn include_marketplace_data(mut self, include_marketplace_data: bool) -> Self {
        self.include_marketplace_data = Some(include_marketplace_data);
        self
    }

    /// Sets the `include_comments` query parameter. If set then comments on the accessory will be
    /// included, along with a rating if one was included with the comment.
    ///
    /// List of comments is paginated, where the page and page size are changed via the `page` and
    /// `page_size` query parameters. Ordered by username ascending.
    ///
    /// Note that this is not compatible with the `include_rating_comments` parameter.
    pub fn include_comments(mut self, include_comments: bool) -> Self {
        self.include_comments = Some(include_comments);
        self
    }

    /// Sets the `include_rating_comments` query parameter. If set then ratings on the accessory
    /// will be included, along with a comment if one was included with the rating.
    ///
    /// List of comments is paginated, where the page and page size are changed via the `page` and
    /// `page_size` query parameters. Ordered by rating descending.
    ///
    /// Note that this is not compatible with the `include_comments` parameter.
    pub fn include_rating_comments(mut self, include_rating_comments: bool) -> Self {
        self.include_rating_comments = Some(include_rating_comments);
        self
    }

    /// Sets the `page` query parameter. If set then this page of comments will be returned.
    pub fn page(mut self, page: u64) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the `page_size` query parameter. If set then comment pages will be this size. Minimum
    /// 10 and maximum 100, if unset or out of these bounds the page size will be 100.
    pub fn page_size(mut self, page_size: u64) -> Self {
        self.page_size = Some(page_size);
        self
    }
}

// Struct for building a query for the request to the accessory endpoint.
#[derive(Clone, Debug)]
struct AccessoryQueryBuilder<'builder> {
    game_ids: Vec<u64>,
    params: &'builder AccessoryQueryParams,
}

impl<'builder> AccessoryQueryBuilder<'builder> {
    // Constructs a new query builder from a list of IDs to request, and the rest of the
    // parameters.
    fn new(game_ids: Vec<u64>, params: &'builder AccessoryQueryParams) -> Self {
        Self { game_ids, params }
    }

    // Converts the list of parameters into a vector of
    // key value pairs that reqwest can use as HTTP query parameters.
    fn build(self) -> Vec<QueryParam<'builder>> {
        let mut query_params: Vec<_> = vec![];

        query_params.push(ItemType::BoardGameAccessory.into_query_param("type"));
        query_params.push(self.game_ids.into_query_param("id"));
        if let Some(include_versions) = self.params.include_versions {
            query_params.push(include_versions.into_query_param("versions"));
        }
        if let Some(include_marketplace_data) = self.params.include_marketplace_data {
            query_params.push(include_marketplace_data.into_query_param("marketplace"));
        }
        if let Some(include_comments) = self.params.include_comments {
            query_params.push(include_comments.into_query_param("comments"));
        }
        if let Some(include_rating_comments) = self.params.include_rating_comments {
            query_params.push(include_rating_comments.into_query_param("ratingcomments"));
        }
        if let Some(page) = self.params.page {
            query_params.push(page.into_query_param("page"));
        }
        if let Some(page_size) = self.params.page_size {
            query_params.push(page_size.into_query_param("pagesize"));
        }
        query_params
    }
}

/// Accessory endpoint for the API.
///
/// Retrieve one or more accessories by their IDs, up to a max of 20 at once.
/// Optionally more information can be included, such as comments or marketplace data.
///
/// This and the gamea endpoint use the same underlying "thing" API endpoint provided by
/// boardgamegeek.
pub struct AccessoryApi<'api> {
    pub(crate) api: &'api BoardGameGeekApi,
    endpoint: &'static str,
}

impl<'api> AccessoryApi<'api> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "thing",
        }
    }

    /// Searches for a board game or expansion by a given ID.
    pub async fn get_by_id(
        &self,
        id: u64,
        query_params: &AccessoryQueryParams,
    ) -> Result<AccessoryDetails> {
        let query = AccessoryQueryBuilder::new(vec![id], query_params);

        let request = self.api.build_request(self.endpoint, &query.build());
        let mut accessories = self.api.execute_request::<Accessories>(request).await?;

        match accessories.accessories.len() {
            0 => Err(Error::ItemNotFound),
            1 => Ok(accessories.accessories.remove(0)),
            len => Err(Error::UnexpectedResponseError(format!(
                "expected 1 game but got {len}",
            ))),
        }
    }

    /// Searches for board games or expansions by given IDs. Can return both games and expansions
    /// together.
    pub async fn get_by_ids(
        &self,
        ids: Vec<u64>,
        query_params: &AccessoryQueryParams,
    ) -> Result<Vec<AccessoryDetails>> {
        let query = AccessoryQueryBuilder::new(ids, query_params);

        let request = self.api.build_request(self.endpoint, &query.build());
        let accessories = self.api.execute_request::<Accessories>(request).await?;

        Ok(accessories.accessories)
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use crate::{AccessoryDetails, AccessoryQueryParams, BoardGameGeekApi, Game, GamePublisher};

    #[tokio::test]
    async fn get_by_id() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/thing")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("type".to_owned(), "boardgameaccessory".to_owned()),
                Matcher::UrlEncoded("id".to_owned(), "196929".to_owned()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/accessory/accessory.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let accessory = api
            .accessory()
            .get_by_id(196_929, &AccessoryQueryParams::new())
            .await;
        mock.assert_async().await;

        println!("{:?}", accessory);

        assert!(accessory.is_ok(), "error returned when okay expected");
        let accessory = accessory.unwrap();

        assert_eq!(
            accessory,
            AccessoryDetails {
                id: 196_929,
                name: "7 Wonders: Metal Coins".to_owned(),
                alternate_names: vec!["7 Wonders: Wondrous Metal Coins".to_owned()],
                description: "These high-quality metal coins replace the cardboard coins contained in the 7 Wonders core game and the 7 Wonders: Leaders expansion.".to_owned(),
                image: Some("https://cf.geekdo-images.com/fIVmsro-RGJVBQzWlDy3Jw__original/img/4OvrN4QJGvzMRx20XNx7FGhYJMo=/0x0/filters:format(jpeg)/pic7149814.jpg".to_owned()),
                thumbnail: Some("https://cf.geekdo-images.com/fIVmsro-RGJVBQzWlDy3Jw__small/img/GryodmBvp6vGamX_K2kZ1nuM-C0=/fit-in/200x150/filters:strip_icc()/pic7149814.jpg".to_owned()),
                year_published: 0,
                accessory_for: vec![
                    Game { id: 68448, name: "7 Wonders".to_owned() },
                    Game { id: 316377, name: "7 Wonders (Second Edition)".to_owned() },
                ],
                publishers: vec![
                    GamePublisher { id: 28595, name: "The Broken Token".to_owned() },
                    GamePublisher { id: 4384, name: "Repos Production".to_owned() },
                ],
                versions: vec![],
                marketplace_listings: vec![],
                rating_comments: None,
            },
        );
    }
}
