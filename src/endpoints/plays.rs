use chrono::NaiveDate;

use crate::{BoardGameGeekApi, IntoQueryParam, ItemSubType, Plays, QueryParam, Result};

/// All optional query parameters for making a request to the plays endpoint.
#[derive(Clone, Debug, Default)]
pub struct PlaysQueryParams {
    min_date: Option<NaiveDate>,
    max_date: Option<NaiveDate>,
    sub_type: Option<ItemSubType>,
    page: Option<u64>,
}

impl PlaysQueryParams {
    /// Constructs a new plays query with parameters set to None.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the `min_date` parameter. Will only return plays from after this date.
    pub fn min_date(mut self, min_date: NaiveDate) -> Self {
        self.min_date = Some(min_date);
        self
    }

    /// Sets the `max_date` parameter. Will only return plays from up to this date.
    pub fn max_date(mut self, max_date: NaiveDate) -> Self {
        self.max_date = Some(max_date);
        self
    }

    /// Sets the `sub_type` parameter. Will only return items of the specified type.
    pub fn sub_type(mut self, sub_type: ItemSubType) -> Self {
        self.sub_type = Some(sub_type);
        self
    }

    /// The page of results to return, if unset defaults to the first page.
    pub fn page(mut self, page: u64) -> Self {
        self.page = Some(page);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum PlaysItemType {
    Thing,
    Family,
}

#[derive(Clone, Debug)]
enum PlaysQuery<'q> {
    QueryByUser(&'q str),
    QueryById {
        id: u64,
        plays_item_type: PlaysItemType,
    },
}

#[derive(Clone, Debug)]
struct PlaysQueryBuilder<'q> {
    query: PlaysQuery<'q>,
    params: PlaysQueryParams,
}

impl<'builder> PlaysQueryBuilder<'builder> {
    fn new(query: PlaysQuery<'builder>, params: PlaysQueryParams) -> Self {
        Self { query, params }
    }

    fn build(self) -> Vec<QueryParam<'builder>> {
        let mut query_params = vec![];
        // The endpoint requires either a username param, or both an ID and a type, for it to return
        // anything.
        match self.query {
            PlaysQuery::QueryByUser(username) => {
                query_params.push(username.into_query_param("username"));
            },
            PlaysQuery::QueryById {
                id,
                plays_item_type,
            } => {
                query_params.push(id.into_query_param("id"));
                query_params.push(plays_item_type.into_query_param("type"));
            },
        }

        if let Some(min_date) = self.params.min_date {
            query_params.push(min_date.into_query_param("mindate"));
        }
        if let Some(max_date) = self.params.max_date {
            query_params.push(max_date.into_query_param("maxdate"));
        }
        if let Some(sub_type) = self.params.sub_type {
            query_params.push(sub_type.into_query_param("subtype"));
        }
        if let Some(page) = self.params.page {
            query_params.push(page.into_query_param("page"));
        }
        query_params
    }
}

/// Plays endpoint of the API. Used for returning information about recordings of instances of games
/// being played. Plays can be queried either by user or by item ID, either way they are returned in
/// reverse chronological order.
pub struct PlaysApi<'api> {
    pub(crate) api: &'api BoardGameGeekApi,
    endpoint: &'static str,
}

impl<'api> PlaysApi<'api> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "plays",
        }
    }

    /// Get a list of recorded game plays for a specific user
    pub async fn get_by_username(
        &self,
        username: &str,
        query_params: PlaysQueryParams,
    ) -> Result<Plays> {
        let query = PlaysQueryBuilder::new(PlaysQuery::QueryByUser(username), query_params);

        let request = self.api.build_request(self.endpoint, &query.build());
        let response = self.api.execute_request::<Plays>(request).await?;

        Ok(response)
    }

    /// Get a list of recorded game plays for a specific item that can be played.
    pub async fn get_by_item_id(
        &self,
        item_id: u64,
        query_params: PlaysQueryParams,
    ) -> Result<Plays> {
        let query = PlaysQueryBuilder::new(
            PlaysQuery::QueryById {
                id: item_id,
                plays_item_type: PlaysItemType::Thing,
            },
            query_params,
        );

        let request = self.api.build_request(self.endpoint, &query.build());
        let response = self.api.execute_request::<Plays>(request).await?;

        Ok(response)
    }

    /// Get a list of recorded game plays for a specific game family.
    pub async fn get_by_family_id(
        &self,
        family_id: u64,
        query_params: PlaysQueryParams,
    ) -> Result<Plays> {
        let query = PlaysQueryBuilder::new(
            PlaysQuery::QueryById {
                id: family_id,
                plays_item_type: PlaysItemType::Family,
            },
            query_params,
        );

        let request = self.api.build_request(self.endpoint, &query.build());
        let response = self.api.execute_request::<Plays>(request).await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;
    use mockito::Matcher;

    use super::*;
    use crate::{Play, PlayedItem, Player};

    #[tokio::test]
    async fn get_by_username() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/plays")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "username".to_owned(),
                "BluebearBgg".to_owned(),
            )]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/plays/user_plays.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let plays = api
            .plays()
            .get_by_username("BluebearBgg", PlaysQueryParams::new())
            .await;
        mock.assert_async().await;

        assert!(plays.is_ok(), "error returned when okay expected");
        let plays = plays.unwrap();

        assert_eq!(
            plays,
            Plays {
                username: "bluebearbgg".to_owned(),
                user_id: 3_855_477,
                total: 3,
                page: 1,
                plays: vec![
                    Play {
                        id: 113_391_260,
                        date: NaiveDate::from_ymd_opt(2026, 4, 30).unwrap(),
                        quantity: 1,
                        duration: Duration::minutes(60),
                        incomplete: false,
                        location: "kitchen".to_owned(),
                        do_not_count_win_stats: false,
                        comments: Some("blah".to_owned()),
                        played_item: PlayedItem {
                            name: "Lost Ruins of Arnak: The Missing Expedition".to_owned(),
                            id: 382_350,
                            sub_types: vec![
                                ItemSubType::BoardGame,
                                ItemSubType::BoardGameExpansion
                            ],
                        },
                        players: vec![],
                    },
                    Play {
                        id: 112_947_972,
                        date: NaiveDate::from_ymd_opt(2026, 4, 18).unwrap(),
                        quantity: 2,
                        duration: Duration::minutes(1310),
                        incomplete: true,
                        location: "kitchen".to_owned(),
                        do_not_count_win_stats: false,
                        comments: None,
                        played_item: PlayedItem {
                            name: "Lost Ruins of Arnak".to_owned(),
                            id: 312_484,
                            sub_types: vec![ItemSubType::BoardGame],
                        },
                        players: vec![Player {
                            username: Some("BluebearBGG".to_owned()),
                            user_id: Some(3_855_477),
                            name: "Matthew Thompson".to_owned(),
                            start_position: "1".to_owned(),
                            color: "blue".to_owned(),
                            score: "999".to_owned(),
                            first_time_playing: true,
                            rating: 0,
                            won: true,
                        },],
                    },
                    Play {
                        id: 83_820_037,
                        date: NaiveDate::from_ymd_opt(2024, 4, 13).unwrap(),
                        quantity: 1,
                        duration: Duration::minutes(120),
                        incomplete: false,
                        location: "".to_owned(),
                        do_not_count_win_stats: false,
                        comments: Some(
                            "Fun game, first time playing. Played with 4 people.".to_owned()
                        ),
                        played_item: PlayedItem {
                            name: "Lost Ruins of Arnak".to_owned(),
                            id: 312_484,
                            sub_types: vec![ItemSubType::BoardGame],
                        },
                        players: vec![Player {
                            username: Some("BluebearBGG".to_owned()),
                            user_id: Some(3_855_477),
                            name: "Matthew".to_owned(),
                            start_position: "".to_owned(),
                            color: "".to_owned(),
                            score: "".to_owned(),
                            first_time_playing: false,
                            rating: 0,
                            won: false,
                        },],
                    },
                ],
            }
        );
    }

    #[tokio::test]
    async fn get_by_item_id() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/plays")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("id".to_owned(), "382350".to_owned()),
                Matcher::UrlEncoded("type".to_owned(), "thing".to_owned()),
                Matcher::UrlEncoded("mindate".to_owned(), "2026-01-01".to_owned()),
                Matcher::UrlEncoded("maxdate".to_owned(), "2026-06-02".to_owned()),
                Matcher::UrlEncoded("subtype".to_owned(), "boardgameexpansion".to_owned()),
                Matcher::UrlEncoded("page".to_owned(), "1".to_owned()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/plays/thing_plays.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let params = PlaysQueryParams::new()
            .min_date(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap())
            .max_date(NaiveDate::from_ymd_opt(2026, 6, 2).unwrap())
            .sub_type(ItemSubType::BoardGameExpansion)
            .page(1);
        let plays = api.plays().get_by_item_id(382_350, params).await;
        mock.assert_async().await;

        assert!(plays.is_ok(), "error returned when okay expected");
        let plays = plays.unwrap();

        assert_eq!(
            plays,
            Plays {
                username: "".to_owned(),
                user_id: 0,
                total: 1,
                page: 1,
                plays: vec![Play {
                    id: 113_391_260,
                    date: NaiveDate::from_ymd_opt(2026, 4, 30).unwrap(),
                    quantity: 1,
                    duration: Duration::minutes(60),
                    incomplete: false,
                    location: "kitchen".to_owned(),
                    do_not_count_win_stats: false,
                    comments: Some("blah".to_owned()),
                    played_item: PlayedItem {
                        name: "Lost Ruins of Arnak: The Missing Expedition".to_owned(),
                        id: 382_350,
                        sub_types: vec![ItemSubType::BoardGame, ItemSubType::BoardGameExpansion],
                    },
                    players: vec![],
                },],
            }
        );
    }
}
