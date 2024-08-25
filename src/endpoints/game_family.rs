use super::GameFamilies;
use crate::{BoardGameGeekApi, QueryParam, Result};

/// Query parameters for making a request to the game family endpoint.
#[derive(Clone, Debug, Default)]
pub struct GameFamilyQueryParams {
    /// ID for the game families to retrieve.
    game_family_ids: Vec<u64>,
}

impl GameFamilyQueryParams {
    /// Constructs a new search query with the list of family IDs empty.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an ID to the list of game family IDs to retrieve.
    pub fn game_family_id(mut self, id: u64) -> Self {
        self.game_family_ids.push(id);
        self
    }

    /// Adds a list of IDs to the list of game family IDs to retrieve.
    pub fn game_family_ids(mut self, ids: Vec<u64>) -> Self {
        self.game_family_ids.extend(ids);
        self
    }
}

// Struct for building a query for the request to the game family endpoint.
#[derive(Clone, Debug)]
struct GameFamilyQueryBuilder {
    params: GameFamilyQueryParams,
}

impl<'builder> GameFamilyQueryBuilder {
    // Constructs a new query builder from the query params.
    fn new(params: GameFamilyQueryParams) -> Self {
        Self { params }
    }

    // Converts the list of parameters into a vector of
    // key value pairs that reqwest can use as HTTP query parameters.
    fn build(self) -> Vec<QueryParam<'builder>> {
        let mut query_params: Vec<_> = vec![];
        // Underlying endpoint supports RPG and Video game families too but we hide those.
        query_params.push(("type", "boardgamefamily".to_string()));

        let id_list_string = self
            .params
            .game_family_ids
            .iter()
            .map(u64::to_string)
            .collect::<Vec<String>>()
            .join(",");
        query_params.push(("id", id_list_string));
        query_params
    }
}

/// Game family endpoint of the API. Used for searching for game families by ID.
pub struct GameFamilyApi<'api> {
    pub(crate) api: &'api BoardGameGeekApi,
    endpoint: &'static str,
}

impl<'api> GameFamilyApi<'api> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "family",
        }
    }

    /// Gets a family of games by ID.
    pub async fn get_by_id(&self, id: u64) -> Result<GameFamilies> {
        let query = GameFamilyQueryBuilder::new(GameFamilyQueryParams::new().game_family_id(id));

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<GameFamilies>(request).await
    }

    /// Gets families of games by their IDs.
    pub async fn get_by_ids(&self, ids: Vec<u64>) -> Result<GameFamilies> {
        let query = GameFamilyQueryBuilder::new(GameFamilyQueryParams::new().game_family_ids(ids));

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<GameFamilies>(request).await
    }

    /// Gets families of games by the given query parameters.
    pub async fn get_from_query(&self, query: GameFamilyQueryParams) -> Result<GameFamilies> {
        let query = GameFamilyQueryBuilder::new(query);

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<GameFamilies>(request).await
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use super::*;
    use crate::{Game, GameFamily};

    #[tokio::test]
    async fn get_by_id() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/family")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("type".into(), "boardgamefamily".into()),
                Matcher::UrlEncoded("id".into(), "2".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/game_family/game_family_single.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let families = api.game_family().get_by_id(2).await;
        mock.assert_async().await;

        assert!(families.is_ok(), "error returned when okay expected");
        let families = families.unwrap();

        assert_eq!(families.game_families.len(), 1);
        assert_eq!(
            families.game_families[0],
            GameFamily {
                id: 2,
                name: "Game: Carcassonne".into(),
                alternate_names: vec!["Carcassonne: Solo-Variante".into()],
                image: "https://cf.geekdo-images.com/c_pg0WfJKn7_P33AsDS5EA__original/img/k2t0IHkPo0nzLadfSxXhtAzyU5I=/0x0/filters:format(jpeg)/pic453826.jpg".into(),
                thumbnail: "https://cf.geekdo-images.com/c_pg0WfJKn7_P33AsDS5EA__thumb/img/8RgZmSChaxESGjIdhMeIg0C9OZk=/fit-in/200x150/filters:strip_icc()/pic453826.jpg".into(),
                description: "Games (expansions, promos, etc.) in the \"Carcassonne\" family of games, published by Hans im GlÃ¼ck.\n\n\nSee this Carcassonne Series wiki for more details.\n\n".into(),
                games: vec![
                    Game {
                        id: 822,
                        name: "Carcassonne".into(),
                    },
                    Game {
                        id: 142057,
                        name: "Carcassonne Big Box".into(),
                    },
                    Game {
                        id: 141008,
                        name: "Carcassonne Big Box 2".into(),
                    },
                ],
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
            .mock("GET", "/family")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("type".into(), "boardgamefamily".into()),
                Matcher::UrlEncoded("id".into(), "2,3".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/game_family/game_family_multiple.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let families = api.game_family().get_by_ids(vec![2, 3]).await;
        mock.assert_async().await;

        assert!(families.is_ok(), "error returned when okay expected");
        let families = families.unwrap();

        assert_eq!(families.game_families.len(), 2);
        assert_eq!(
            families.game_families[0],
            GameFamily {
                id: 2,
                name: "Game: Carcassonne".into(),
                alternate_names: vec!["Carcassonne: Solo-Variante".into()],
                image: "https://cf.geekdo-images.com/c_pg0WfJKn7_P33AsDS5EA__original/img/k2t0IHkPo0nzLadfSxXhtAzyU5I=/0x0/filters:format(jpeg)/pic453826.jpg".into(),
                thumbnail: "https://cf.geekdo-images.com/c_pg0WfJKn7_P33AsDS5EA__thumb/img/8RgZmSChaxESGjIdhMeIg0C9OZk=/fit-in/200x150/filters:strip_icc()/pic453826.jpg".into(),
                description: "Games (expansions, promos, etc.) in the \"Carcassonne\" family of games, published by Hans im GlÃ¼ck.\n\n\nSee this Carcassonne Series wiki for more details.\n\n".into(),
                games: vec![
                    Game {
                        id: 822,
                        name: "Carcassonne".into(),
                    },
                    Game {
                        id: 142057,
                        name: "Carcassonne Big Box".into(),
                    },
                    Game {
                        id: 141008,
                        name: "Carcassonne Big Box 2".into(),
                    },
                ],
            },
        );
        assert_eq!(
            families.game_families[1],
            GameFamily {
                id: 3,
                name: "Game: Catan".into(),
                alternate_names: vec![],
                image: "https://cf.geekdo-images.com/FFUKDbZw6d9mAKaL9U3ymg__original/img/rulpehNOumO24_7WzaHvl7P2aac=/0x0/filters:format(jpeg)/pic1446957.jpg".into(),
                thumbnail: "https://cf.geekdo-images.com/FFUKDbZw6d9mAKaL9U3ymg__thumb/img/o06DBHHSC9Yck1WmSkp-rK360QI=/fit-in/200x150/filters:strip_icc()/pic1446957.jpg".into(),
                description: "This is the family of Settlers of Catan games, meant to include any game in the Game: Catan universe.\n\nA detailed overview is given on the Catan Series wiki.\n\n".into(),
                games: vec![
                    Game {
                        id: 13,
                        name: "CATAN".into(),
                    },
                    Game {
                        id: 27710,
                        name: "Catan Dice Game".into(),
                    },
                ],
            },
        );
    }

    #[tokio::test]
    async fn get_by_params() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/family")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("type".into(), "boardgamefamily".into()),
                Matcher::UrlEncoded("id".into(), "2,3,4,5,101,200,6".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/game_family/game_family_multiple.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let query = GameFamilyQueryParams::new()
            .game_family_ids(vec![2, 3])
            .game_family_id(4)
            .game_family_id(5)
            .game_family_ids(vec![101, 200, 6]);
        let families = api.game_family().get_from_query(query).await;
        mock.assert_async().await;

        assert!(families.is_ok(), "error returned when okay expected");
    }
}
