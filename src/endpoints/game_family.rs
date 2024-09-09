use super::{GameFamilies, GameFamily, ItemType};
use crate::{BoardGameGeekApi, Error, IntoQueryParam, QueryParam, Result};

// Query parameters for making a request to the game family endpoint.
#[derive(Clone, Debug, Default)]
struct GameFamilyQueryParams {
    // ID for the game families to retrieve.
    game_family_ids: Vec<u64>,
}

impl GameFamilyQueryParams {
    // Constructs a new search query with the list of family IDs empty.
    fn new() -> Self {
        Self::default()
    }

    // Adds an ID to the list of game family IDs to retrieve.
    fn game_family_id(mut self, id: u64) -> Self {
        self.game_family_ids.push(id);
        self
    }

    // Adds a list of IDs to the list of game family IDs to retrieve.
    fn game_family_ids(mut self, ids: Vec<u64>) -> Self {
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
        query_params.push(ItemType::BoardGameFamily.into_query_param("type"));

        if !self.params.game_family_ids.is_empty() {
            query_params.push(self.params.game_family_ids.into_query_param("id"));
        }
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
    pub async fn get_by_id(&self, id: u64) -> Result<GameFamily> {
        let query = GameFamilyQueryBuilder::new(GameFamilyQueryParams::new().game_family_id(id));

        let request = self.api.build_request(self.endpoint, &query.build());
        let mut response = self.api.execute_request::<GameFamilies>(request).await?;

        match response.game_families.len() {
            0 => Err(Error::ItemNotFound),
            1 => Ok(response.game_families.remove(0)),
            len => Err(Error::UnexpectedResponseError(format!(
                "expected 1 game family but got {len}",
            ))),
        }
    }

    /// Gets families of games by their IDs.
    pub async fn get_by_ids(&self, ids: Vec<u64>) -> Result<Vec<GameFamily>> {
        let query = GameFamilyQueryBuilder::new(GameFamilyQueryParams::new().game_family_ids(ids));

        let request = self.api.build_request(self.endpoint, &query.build());
        let response = self.api.execute_request::<GameFamilies>(request).await?;

        Ok(response.game_families)
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

        let game_family = api.game_family().get_by_id(2).await;
        mock.assert_async().await;

        assert!(game_family.is_ok(), "error returned when okay expected");
        let game_family = game_family.unwrap();

        assert_eq!(
            game_family,
            GameFamily {
                id: 2,
                name: "Game: Carcassonne".into(),
                alternate_names: vec!["Carcassonne: Solo-Variante".into()],
                image: "https://cf.geekdo-images.com/c_pg0WfJKn7_P33AsDS5EA__original/img/k2t0IHkPo0nzLadfSxXhtAzyU5I=/0x0/filters:format(jpeg)/pic453826.jpg".into(),
                thumbnail: "https://cf.geekdo-images.com/c_pg0WfJKn7_P33AsDS5EA__thumb/img/8RgZmSChaxESGjIdhMeIg0C9OZk=/fit-in/200x150/filters:strip_icc()/pic453826.jpg".into(),
                description: "Games (expansions, promos, etc.) in the \"Carcassonne\" family of games, published by Hans im GlÃ¼ck.\n\n\nSee this Carcassonne Series wiki for more details.".into(),
                games: vec![
                    Game {
                        id: 822,
                        name: "Carcassonne".into(),
                    },
                    Game {
                        id: 142_057,
                        name: "Carcassonne Big Box".into(),
                    },
                    Game {
                        id: 141_008,
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

        let game_families = api.game_family().get_by_ids(vec![2, 3]).await;
        mock.assert_async().await;

        assert!(game_families.is_ok(), "error returned when okay expected");
        let game_families = game_families.unwrap();

        assert_eq!(game_families.len(), 2);
        assert_eq!(
            game_families[0],
            GameFamily {
                id: 2,
                name: "Game: Carcassonne".into(),
                alternate_names: vec!["Carcassonne: Solo-Variante".into()],
                image: "https://cf.geekdo-images.com/c_pg0WfJKn7_P33AsDS5EA__original/img/k2t0IHkPo0nzLadfSxXhtAzyU5I=/0x0/filters:format(jpeg)/pic453826.jpg".into(),
                thumbnail: "https://cf.geekdo-images.com/c_pg0WfJKn7_P33AsDS5EA__thumb/img/8RgZmSChaxESGjIdhMeIg0C9OZk=/fit-in/200x150/filters:strip_icc()/pic453826.jpg".into(),
                description: "Games (expansions, promos, etc.) in the \"Carcassonne\" family of games, published by Hans im GlÃ¼ck.\n\n\nSee this Carcassonne Series wiki for more details.".into(),
                games: vec![
                    Game {
                        id: 822,
                        name: "Carcassonne".into(),
                    },
                    Game {
                        id: 142_057,
                        name: "Carcassonne Big Box".into(),
                    },
                    Game {
                        id: 141_008,
                        name: "Carcassonne Big Box 2".into(),
                    },
                ],
            },
        );
        assert_eq!(
            game_families[1],
            GameFamily {
                id: 3,
                name: "Game: Catan".into(),
                alternate_names: vec![],
                image: "https://cf.geekdo-images.com/FFUKDbZw6d9mAKaL9U3ymg__original/img/rulpehNOumO24_7WzaHvl7P2aac=/0x0/filters:format(jpeg)/pic1446957.jpg".into(),
                thumbnail: "https://cf.geekdo-images.com/FFUKDbZw6d9mAKaL9U3ymg__thumb/img/o06DBHHSC9Yck1WmSkp-rK360QI=/fit-in/200x150/filters:strip_icc()/pic1446957.jpg".into(),
                description: "This is the family of Settlers of Catan games, meant to include any game in the Game: Catan universe.\n\nA detailed overview is given on the Catan Series wiki.".into(),
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
    async fn get_by_id_not_found() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/family")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("type".into(), "boardgamefamily".into()),
                Matcher::UrlEncoded("id".into(), "9000".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/game_family/game_family_not_found.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let game_families = api.game_family().get_by_id(9000).await;
        mock.assert_async().await;

        assert!(game_families.is_err());
        assert!(matches!(game_families.err().unwrap(), Error::ItemNotFound));
    }
}
