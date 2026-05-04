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
    use chrono::{DateTime, NaiveDate, Utc};
    use mockito::Matcher;

    use crate::{
        AccessoryDetails, AccessoryQueryParams, AccessoryVersion, BoardGameGeekApi, Game,
        GameArtist, GameDesigner, GamePublisher, ItemCondition, MarketplaceListing, Price,
        RatingComment, RatingCommentPage,
    };

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
                designers: vec![],
                artists: vec![],
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

    #[tokio::test]
    async fn get_by_ids() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/thing")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("type".to_owned(), "boardgameaccessory".to_owned()),
                Matcher::UrlEncoded("id".to_owned(), "22510,207791".to_owned()),
                Matcher::UrlEncoded("versions".to_owned(), "1".to_owned()),
                Matcher::UrlEncoded("marketplace".to_owned(), "1".to_owned()),
                Matcher::UrlEncoded("comments".to_owned(), "1".to_owned()),
                Matcher::UrlEncoded("ratingcomments".to_owned(), "1".to_owned()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/accessory/accessory_full.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let params = AccessoryQueryParams::new()
            .include_versions(true)
            .include_marketplace_data(true)
            .include_comments(true)
            .include_rating_comments(true);
        let accessory = api
            .accessory()
            .get_by_ids(vec![22_510, 207_791], &params)
            .await;
        mock.assert_async().await;

        assert!(accessory.is_ok(), "error returned when okay expected");
        let accessory = accessory.unwrap();

        assert_eq!(accessory.len(), 2);
        assert_eq!(
            accessory[0],
            AccessoryDetails {
                id: 22_510,
                name: "Wings of War: Miniatures".to_owned(),
                alternate_names: vec!["Wings of War: WW1 Airplane Packs".to_owned()],
                description: "Wings of War Airplane Packs provide miniatures for the Wings of War system. Each Airplane Pack includes one pre-painted, pre-assembled 1/144 scale model plane with gaming base, the relative airplane card and its deck of Maneuver cards. A Wings of War Deluxe set is also available.".to_owned(),
                image: Some("https://cf.geekdo-images.com/qGV1v8Ye0FKTxZNCF1ZINw__original/img/49pxPDdA4CHNFZOMQM1UTM8FNL4=/0x0/filters:format(jpeg)/pic830522.jpg".to_owned()),
                thumbnail: Some("https://cf.geekdo-images.com/qGV1v8Ye0FKTxZNCF1ZINw__small/img/vgAzbZuLXNSawia3yp4BAPT_2is=/fit-in/200x150/filters:strip_icc()/pic830522.jpg".to_owned()),
                year_published: 2007,
                accessory_for: vec![
                    Game { id: 15_953, name: "Wings of War: Burning Drachens".to_owned() },
                    Game { id: 31_552, name: "Wings of War: Deluxe Set".to_owned() },
                ],
                designers: vec![
                    GameDesigner {
                        id: 546,
                        name: "Andrea Angiolino".to_owned(),
                    },
                    GameDesigner {
                        id: 547,
                        name: "Pier Giorgio Paglia".to_owned(),
                    },
                ],
                artists: vec![
                    GameArtist { id: 12475, name: "Vincenzo Auletta".to_owned() },
                    GameArtist { id: 12474, name: "Dario Calì".to_owned() },
                    GameArtist { id: 20670, name: "Fabio Maiorana".to_owned() },
                ],
                publishers: vec![
                    GamePublisher { id: 17, name: "Fantasy Flight Games".to_owned() },
                    GamePublisher { id: 504, name: "Nexus Editrice".to_owned() },
                    GamePublisher { id: 3446, name: "Ubik".to_owned() },
                ],
                versions: vec![
                    AccessoryVersion {
                        id: 168_378,
                        name: "Wings of War: Miniatures".to_owned(),
                        thumbnail: None,
                        image: None,
                    },
                    AccessoryVersion {
                        id: 168_379,
                        name: "Wings of War: Miniatures".to_owned(),
                        thumbnail: Some("https://cf.geekdo-images.com/-qODJQlE2-T0ZhrcO6514g__small/img/SdSbI2zcepfXJqwWx-MOHG9vY9I=/fit-in/200x150/filters:strip_icc()/pic318897.jpg".to_owned()),
                        image: Some("https://cf.geekdo-images.com/-qODJQlE2-T0ZhrcO6514g__original/img/8y314LQOOa0dDqCWXwR_DK7LKbU=/0x0/filters:format(jpeg)/pic318897.jpg".to_owned()),
                    },
                ],
                marketplace_listings: vec![
                    MarketplaceListing {
                        list_date: DateTime::from_naive_utc_and_offset(NaiveDate::from_ymd_opt(2024, 9, 18).unwrap().and_hms_opt(21, 40, 57).unwrap(), Utc),
                        price: Price { currency: "USD".to_owned(), value: "44.99".to_owned() },
                        condition: ItemCondition::New,
                        notes: "Buy this".to_owned(),
                        link: "https://boardgamegeek.com/market/product/3549459".to_owned(),
                    },
                    MarketplaceListing {
                        list_date: DateTime::from_naive_utc_and_offset(NaiveDate::from_ymd_opt(2024, 9, 18).unwrap().and_hms_opt(21, 43, 24).unwrap(), Utc),
                        price: Price { currency: "USD".to_owned(), value: "44.99".to_owned() },
                        condition: ItemCondition::New,
                        notes: "and this!".to_owned(),
                        link: "https://boardgamegeek.com/market/product/3549461".to_owned(),
                    },
                ],
                rating_comments: Some(RatingCommentPage {
                    total_items: 502,
                    page_number: 1,
                    comments: vec![
                        RatingComment { username: "user1".to_owned(), rating: None, comment: "Looks interesting".to_owned() },
                        RatingComment { username: "user2".to_owned(), rating: Some(5.0), comment: "Must have more minis!".to_owned() },
                    ],
                }),
            },
        );
        assert_eq!(
            accessory[1],
            AccessoryDetails {
                id: 207_791,
                name: "Scythe: Board Extension".to_owned(),
                alternate_names: vec![],
                description: "The board extension slides next to the back side of the standard game board, creating a complete board with 70% bigger hexes (the content is the same). All units and resources in Scythe are kept on the board, so the larger hexes provide more space. The standard game board is 624x818mm (24.6 x 32.2 in), and it grows to 818x939mm (32.2 x 37.0 in) with this extension.".to_owned(),
                image: Some("https://cf.geekdo-images.com/Z9mTOM4Bvpuyg-D0s0IRZw__original/img/46Xr6DJoXX0YS3Fm4USxPd_UVA8=/0x0/filters:format(jpeg)/pic3403769.jpg".to_owned()),
                thumbnail: Some("https://cf.geekdo-images.com/Z9mTOM4Bvpuyg-D0s0IRZw__small/img/539zzfs3dwbbN2SNhuqpexiay-s=/fit-in/200x150/filters:strip_icc()/pic3403769.jpg".to_owned()),
                year_published: 2016,
                accessory_for: vec![
                    Game { id: 169_786, name: "Scythe".to_owned() },
                ],
                designers: vec![
                    GameDesigner { id: 62_640, name: "Jamey Stegmaier".to_owned() },
                ],
                artists: vec![
                    GameArtist { id: 33_148, name: "Jakub Rozalski".to_owned() },
                ],
                publishers: vec![
                    GamePublisher { id: 23_202, name: "Stonemaier Games".to_owned() },
                ],
                versions: vec![
                    AccessoryVersion {
                        id: 324_209,
                        name: "Scythe: Board Extension".to_owned(),
                        thumbnail: Some("https://cf.geekdo-images.com/Z9mTOM4Bvpuyg-D0s0IRZw__small/img/539zzfs3dwbbN2SNhuqpexiay-s=/fit-in/200x150/filters:strip_icc()/pic3403769.jpg".to_owned()),
                        image: Some("https://cf.geekdo-images.com/Z9mTOM4Bvpuyg-D0s0IRZw__original/img/46Xr6DJoXX0YS3Fm4USxPd_UVA8=/0x0/filters:format(jpeg)/pic3403769.jpg".to_owned()),
                    },
                ],
                marketplace_listings: vec![
                    MarketplaceListing {
                        list_date: DateTime::from_naive_utc_and_offset(NaiveDate::from_ymd_opt(2022, 10, 14).unwrap().and_hms_opt(11, 52, 3).unwrap(), Utc),
                        price: Price { currency: "GBP".to_owned(), value: "12.00".to_owned() },
                        condition: ItemCondition::New,
                        notes: "".to_owned(),
                        link: "https://boardgamegeek.com/market/product/2986606".to_owned(),
                    },
                    MarketplaceListing {
                        list_date: DateTime::from_naive_utc_and_offset(NaiveDate::from_ymd_opt(2023, 10, 28).unwrap().and_hms_opt(18, 11, 7).unwrap(), Utc),
                        price: Price { currency: "EUR".to_owned(), value: "10.00".to_owned() },
                        condition: ItemCondition::LikeNew,
                        notes: "Only one use.".to_owned(),
                        link: "https://boardgamegeek.com/market/product/3293476".to_owned(),
                    },
                    MarketplaceListing {
                        list_date: DateTime::from_naive_utc_and_offset(NaiveDate::from_ymd_opt(2025, 12, 8).unwrap().and_hms_opt(20, 22, 51).unwrap(), Utc),
                        price: Price { currency: "EUR".to_owned(), value: "19.00".to_owned() },
                        condition: ItemCondition::New,
                        notes: "Brand new copy of the game, in shrink wrap.".to_owned(),
                        link: "https://boardgamegeek.com/market/product/3908894".to_owned(),
                    },
                ],
                rating_comments: Some(RatingCommentPage {
                    total_items: 59,
                    page_number: 1,
                    comments: vec![
                        RatingComment {
                            username: "blah".to_owned(),
                            rating: Some(10.0),
                            comment: "Makes everything that much more epic.".to_owned(),
                        },
                        RatingComment {
                            username: "aaa".to_owned(),
                            rating: Some(10.0),
                            comment: "The big map does better the experience of the game in a way I didn't predict.".to_owned(),
                        },
                    ],
                }),
            },
        );
    }
}
