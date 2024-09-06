use super::{GameDetails, Games, ItemType};
use crate::{BoardGameGeekApi, Error, IntoQueryParam, QueryParam, Result};

/// All optional query parameters for making a request to the game endpoint.
#[derive(Clone, Debug, Default)]
pub struct GameQueryParams {
    // Whether to include the version information.
    include_versions: Option<bool>,
    // Whether to include links to related videos for the game.
    include_videos: Option<bool>,
    // Whether to include marketplace data for the game.
    include_marketplace_data: Option<bool>,
    // Whether to include a page of comments for the game.
    //
    // Comment will include the rating too if there was one included. Sorted by username ascending.
    // Cannot be used in conjunction with rating comments.
    include_comments: Option<bool>,
    // Whether to include a page of rating comments for the game.
    //
    // A rating comment is a rating for a game, which will also include a comment if there was one.
    // Sorted by rating descending. Cannot be used in conjunction with comments.
    include_rating_comments: Option<bool>,
    // Which page of comments and videos to return. Default 1.
    page: Option<u64>,
    // Size of the comment and video pages, between 10 and 100.
    page_size: Option<u64>,
}

impl GameQueryParams {
    /// Constructs a new game query with parameters set to None.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the include_versions query parameter. If set then information about different
    /// versions of the game will be included, if applicable.
    pub fn include_versions(mut self, include_versions: bool) -> Self {
        self.include_versions = Some(include_versions);
        self
    }

    /// Sets the include_videos query parameter. If set then links to related videos will be
    /// included. Page and pagesize seem not to work for the list of videos.
    pub fn include_videos(mut self, include_videos: bool) -> Self {
        self.include_videos = Some(include_videos);
        self
    }

    /// Sets the include_marketplace_data query parameter. If set then information about where to
    /// buy the game and for what cost will be included.
    pub fn include_marketplace_data(mut self, include_marketplace_data: bool) -> Self {
        self.include_videos = Some(include_marketplace_data);
        self
    }

    /// Sets the include_comments query parameter. If set then comments on the game will be
    /// included, along with a rating if one was included with the comment.
    ///
    /// List of comments is paginated, where the page and page size are changed via the `page` and
    /// `page_size` query parameters. Ordered by username ascending.
    ///
    /// Note that this is not compatible with the include_rating_comments parameter.
    pub fn include_comments(mut self, include_comments: bool) -> Self {
        self.include_videos = Some(include_comments);
        self
    }

    /// Sets the include_rating_comments query parameter. If set then ratings on the game will be
    /// included, along with a comment if one was included with the rating.
    ///
    /// List of comments is paginated, where the page and page size are changed via the `page` and
    /// `page_size` query parameters. Ordered by rating descending.
    ///
    /// Note that this is not compatible with the include_comments parameter.
    pub fn include_rating_comments(mut self, include_rating_comments: bool) -> Self {
        self.include_videos = Some(include_rating_comments);
        self
    }

    /// Sets the page query parameter. If set then this page of comments will be returned.
    pub fn page(mut self, page: u64) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the page_size query parameter. If set then comment pages will be this size. Minimum 10
    /// and maximum 100, if unset or out of these bounds the page size will be 100.
    pub fn page_size(mut self, page_size: u64) -> Self {
        self.page_size = Some(page_size);
        self
    }
}

// Struct for building a query for the request to the game endpoint.
#[derive(Clone, Debug)]
struct GameQueryBuilder {
    game_ids: Vec<u64>,
    params: GameQueryParams,
}

impl<'builder> GameQueryBuilder {
    // Constructs a new query builder from a list of IDs to request, and the rest of the
    // parameters.
    fn new(game_ids: Vec<u64>, params: GameQueryParams) -> Self {
        Self { game_ids, params }
    }

    // Converts the list of parameters into a vector of
    // key value pairs that reqwest can use as HTTP query parameters.
    fn build(self) -> Vec<QueryParam<'builder>> {
        let mut query_params: Vec<_> = vec![];
        let default_types = vec![ItemType::BoardGame, ItemType::BoardGameExpansion];

        query_params.push(default_types.into_query_param("type"));
        query_params.push(true.into_query_param("stats"));
        query_params.push(self.game_ids.into_query_param("id"));
        if let Some(include_versions) = self.params.include_versions {
            query_params.push(include_versions.into_query_param("versions"));
        }
        if let Some(include_videos) = self.params.include_videos {
            query_params.push(include_videos.into_query_param("videos"));
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

/// Game endpoint for the API.
///
/// Retrieve one or more games or game expansions by their IDs, up to a max of 20 at once.
/// Optionally more information can be included, such as comments or marketplace data.
pub struct GameApi<'api> {
    pub(crate) api: &'api BoardGameGeekApi,
    endpoint: &'static str,
}

impl<'api> GameApi<'api> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "thing",
        }
    }

    /// Searches for a board game or expansion by a given ID.
    pub async fn get_by_id(&self, id: u64, query_params: GameQueryParams) -> Result<GameDetails> {
        let query = GameQueryBuilder::new(vec![id], query_params);

        let request = self.api.build_request(self.endpoint, &query.build());
        let mut games = self.api.execute_request::<Games>(request).await?;

        match games.games.len() {
            0 => Err(Error::ItemNotFound),
            1 => Ok(games.games.remove(0)),
            len => Err(Error::UnexpectedResponseError(format!(
                "expected 1 game but got {}",
                len
            ))),
        }
    }

    /// Searches for a board game or expansion by given IDs.
    pub async fn get_by_ids(
        &self,
        ids: Vec<u64>,
        query_params: GameQueryParams,
    ) -> Result<GameDetails> {
        let query = GameQueryBuilder::new(ids, query_params);

        let request = self.api.build_request(self.endpoint, &query.build());
        let mut games = self.api.execute_request::<Games>(request).await?;

        match games.games.len() {
            0 => Err(Error::ItemNotFound),
            1 => Ok(games.games.remove(0)),
            len => Err(Error::UnexpectedResponseError(format!(
                "expected 1 game but got {}",
                len
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;
    use mockito::Matcher;

    use super::*;
    use crate::{
        Game, GameAccessory, GameArtist, GameCategory, GameDesigner, GameFamilyName,
        GameFamilyRank, GameFamilyType, GameMechanic, GamePublisher, GameStats, GameType, Poll,
        PollResult, PollResults, RankValue,
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
                Matcher::UrlEncoded("type".into(), "boardgame,boardgameexpansion".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
                Matcher::UrlEncoded("id".into(), "312484".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/game/game.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let game = api.game().get_by_id(312484, GameQueryParams::new()).await;
        mock.assert_async().await;

        assert!(game.is_ok(), "error returned when okay expected");
        let game = game.unwrap();

        assert_eq!(
            game,
            GameDetails {
                id:312484,
                game_type: GameType::BoardGame,
                name: "Lost Ruins of Arnak".to_owned(),
                alternate_names: vec![
                    "アルナックの失われし遺跡".to_owned(),
                ],
                description: "On an uninhabited island in uncharted seas, explorers have found traces of a great civilization. Now you will lead an expedition to explore the island, find lost artifacts, and face fearsome guardians, all in a quest to learn the island's secrets.\n\nLost Ruins of Arnak combines deck-building and worker placement in a game of exploration, resource management, and discovery. In addition to traditional deck-builder effects, cards can also be used to place workers, and new worker actions become available as players explore the island. Some of these actions require resources instead of workers, so building a solid resource base will be essential. You are limited to only one action per turn, so make your choice carefully... what action will benefit you most now? And what can you afford to do later... assuming someone else doesn't take the action first!?\n\nDecks are small, and randomness in the game is heavily mitigated by the wealth of tactical decisions offered on the game board. With a variety of worker actions, artifacts, and equipment cards, the set-up for each game will be unique, encouraging players to explore new strategies to meet the challenge.\n\nDiscover the Lost Ruins of Arnak!\n\n—description from the publisher".to_owned(),
                image: "https://cf.geekdo-images.com/U4aoXbKATU7YbA8bAT73FQ__original/img/TKJnD49aci6Soc214_MTUe1iNmg=/0x0/filters:format(png)/pic6253876.png".to_owned(),
                thumbnail: "https://cf.geekdo-images.com/U4aoXbKATU7YbA8bAT73FQ__thumb/img/g0aac2-OQvMbEPXv1vIvSumPmkA=/fit-in/200x150/filters:strip_icc()/pic6253876.png".to_owned(),
                year_published: 2020,
                min_players: 1,
                max_players: 4,
                suggested_player_count: Poll {
                    name: "suggested_numplayers".to_owned(),
                    title: "User Suggested Number of Players".to_owned(),
                    results: vec![
                        PollResults {
                            results: vec![
                                PollResult { value: "Best".to_owned(), number_of_votes: 88 },
                                PollResult { value: "Recommended".to_owned(), number_of_votes: 337 },
                                PollResult { value: "Not Recommended".to_owned(), number_of_votes: 126 },
                            ],
                        },
                        PollResults {
                            results: vec![
                                PollResult { value: "Best".to_owned(), number_of_votes: 225 },
                                PollResult { value: "Recommended".to_owned(), number_of_votes: 506 },
                                PollResult { value: "Not Recommended".to_owned(), number_of_votes: 35 },
                            ],
                        },
                        PollResults {
                            results: vec![
                                PollResult { value: "Best".to_owned(), number_of_votes: 512},
                                PollResult { value: "Recommended".to_owned(), number_of_votes: 202},
                                PollResult { value: "Not Recommended".to_owned(), number_of_votes: 12},
                            ],
                        },
                        PollResults {
                            results: vec![
                                PollResult { value: "Best".to_owned(), number_of_votes: 176 },
                                PollResult { value: "Recommended".to_owned(), number_of_votes: 385 },
                                PollResult { value: "Not Recommended".to_owned(), number_of_votes: 95 },
                            ],
                        },
                        PollResults {
                            results: vec![
                                PollResult { value: "Best".to_owned(), number_of_votes: 1 },
                                PollResult { value: "Recommended".to_owned(), number_of_votes: 0 },
                                PollResult { value: "Not Recommended".to_owned(), number_of_votes: 361 },
                            ],
                        },
                    ],
                },
                playing_time: Duration::minutes(120),
                min_playtime: Duration::minutes(30),
                max_playtime: Duration::minutes(120),
                min_age: 12,
                suggested_player_age: Poll {
                    name: "suggested_playerage".to_owned(),
                    title: "User Suggested Player Age".to_owned(),
                    results: vec![
                        PollResults {
                            results: vec![
                                PollResult { value: "6".to_owned(), number_of_votes: 3 },
                                PollResult { value: "8".to_owned(), number_of_votes: 17 },
                                PollResult { value: "10".to_owned(), number_of_votes: 75 },
                                PollResult { value: "14".to_owned(), number_of_votes: 10 },
                                PollResult { value: "16".to_owned(), number_of_votes: 1 },
                                PollResult { value: "18".to_owned(), number_of_votes: 0 },
                                PollResult { value: "21 and up".to_owned(), number_of_votes: 0 },
                            ],
                        },
                    ],
                },
                suggested_language_dependence: Poll {
                    name: "language_dependence".to_owned(),
                    title: "Language Dependence".to_owned(),
                    results: vec![
                        PollResults {
                            results: vec![
                                PollResult { value: "No necessary in-game text".to_owned(), number_of_votes: 0 },
                                PollResult { value: "Some necessary text - easily memorized or small crib sheet".to_owned(), number_of_votes: 4 },
                                PollResult { value: "Moderate in-game text - needs crib sheet or paste ups".to_owned(), number_of_votes: 28 },
                                PollResult { value: "Extensive use of text - massive conversion needed to be playable".to_owned(), number_of_votes: 5 },
                                PollResult { value: "Unplayable in another language".to_owned(), number_of_votes: 2 },
                            ],
                        },
                    ],
                },
                categories: vec![
                    GameCategory {
                        id: 1020,
                        name: "Exploration".to_owned(),
                    },
                    GameCategory {
                        id: 1097,
                        name: "Travel".to_owned(),
                    },
                ],
                mechanics: vec![
                    GameMechanic {
                        id: 2664,
                        name: "Deck, Bag, and Pool Building".to_owned(),
                    },
                    GameMechanic {
                        id: 2041,
                        name: "Open Drafting".to_owned(),
                    },
                    GameMechanic {
                        id: 2082,
                        name: "Worker Placement".to_owned(),
                    },
                ],
                game_families: vec![
                    GameFamilyName {
                        id: 5666,
                        name: "Players: Games with Solitaire Rules".to_owned(),
                    },
                    GameFamilyName {
                        id: 21940,
                        name: "Theme: Archaeology / Paleontology".to_owned(),
                    },
                ],
                expansions: vec![
                    Game {
                        id: 341254,
                        name: "Lost Ruins of Arnak: Expedition Leaders".to_owned(),
                    },
                ],
                expansion_for: vec![],
                accessories: vec![],
                compilations: vec![],
                reimplementations: vec![],
                designers: vec![
                    GameDesigner {
                        id: 127823,
                        name: "Design".to_owned(),
                    },
                    GameDesigner {
                        id: 127822,
                        name: "Er".to_owned(),
                    },
                ],
                artists: vec![
                    GameArtist {
                        id: 152613,
                        name: "Artist person".to_owned(),
                    },
                    GameArtist {
                        id: 115373,
                        name: "Another Artist person".to_owned(),
                    },
                ],
                publishers: vec![
                    GamePublisher {
                        id: 1391,
                        name: "Hobby Japan".to_owned(),
                    },
                ],
                stats: GameStats {
                    users_rated: 45233,
                    average_rating: 8.07243,
                    bayesian_average: 7.89555,
                    standard_deviation: 1.24187,
                    median: 0.0,
                    ranks: vec![
                        GameFamilyRank {
                            game_family_type: GameFamilyType::Subtype,
                            id: 1,
                            name: "boardgame".to_owned(),
                            friendly_name: "Board Game Rank".to_owned(),
                            value: RankValue::Ranked(28),
                            bayesian_average: 7.89555,
                        },
                        GameFamilyRank {
                            game_family_type: GameFamilyType::Family,
                            id: 5497,
                            name: "strategygames".to_owned(),
                            friendly_name: "Strategy Game Rank".to_owned(),
                            value: RankValue::Ranked(29),
                            bayesian_average: 7.89048,
                        },
                    ],
                    users_owned: 68393,
                    users_trading: 456,
                    users_want_in_trade: 1056,
                    users_wishlisted: 13287,
                    number_of_comments: 5633,
                    number_of_weights: 1466,
                    weight_rating: 2.9216,
                },
                versions: vec![],
            },
        );
    }

    #[tokio::test]
    async fn get_by_id_expansion() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/thing")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("type".into(), "boardgame,boardgameexpansion".into()),
                Matcher::UrlEncoded("stats".into(), "1".into()),
                Matcher::UrlEncoded("id".into(), "341254".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/game/game_expansion.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let game = api.game().get_by_id(341254, GameQueryParams::new()).await;
        mock.assert_async().await;

        println!("{:?}", game);

        assert!(game.is_ok(), "error returned when okay expected");
        let game = game.unwrap();

        assert_eq!(
            game,
            GameDetails {
                id: 341254,
                game_type: GameType::BoardGameExpansion,
                name: "Lost Ruins of Arnak: Expedition Leaders".to_owned(),
                alternate_names: vec![
                    "アルナックの失われし遺跡：調査隊長".to_owned(),
                ],
                description: "Return to the mysterious island of Arnak in Lost Ruins of Arnak: Expedition Leaders!\n\nGive your expedition an edge by choosing one of six unique leaders, each equipped with different abilities, skills, and starting decks that offer different strategies and styles of play for you to explore.\n\nIn addition to the leader abilities, which bring a new element of asymmetry to the game, this expansion contains alternative research tracks that offer even more variety and a bigger challenge, new item and artifact cards to create new combos and synergies, along with more guardians & assistants to meet and sites to explore.".to_owned(),
                image: "https://cf.geekdo-images.com/U4aoXbKATU7YbA8bAT73FQ__original/img/TKJnD49aci6Soc214_MTUe1iNmg=/0x0/filters:format(png)/pic6253876.png".to_owned(),
                thumbnail: "https://cf.geekdo-images.com/U4aoXbKATU7YbA8bAT73FQ__thumb/img/g0aac2-OQvMbEPXv1vIvSumPmkA=/fit-in/200x150/filters:strip_icc()/pic6253876.png".to_owned(),
                year_published: 2021,
                min_players: 1,
                max_players: 4,
                suggested_player_count: Poll {
                    name: "suggested_numplayers".to_owned(),
                    title: "User Suggested Number of Players".to_owned(),
                    results: vec![
                        PollResults {
                            results: vec![
                                PollResult { value: "Best".to_owned(), number_of_votes: 8 },
                                PollResult { value: "Recommended".to_owned(), number_of_votes: 45 },
                                PollResult { value: "Not Recommended".to_owned(), number_of_votes: 16 },
                            ],
                        },
                        PollResults {
                            results: vec![
                                PollResult { value: "Best".to_owned(), number_of_votes: 26 },
                                PollResult { value: "Recommended".to_owned(), number_of_votes: 50 },
                                PollResult { value: "Not Recommended".to_owned(), number_of_votes: 1 },
                            ],
                        },
                        PollResults {
                            results: vec![
                                PollResult { value: "Best".to_owned(), number_of_votes: 53 },
                                PollResult { value: "Recommended".to_owned(), number_of_votes: 18 },
                                PollResult { value: "Not Recommended".to_owned(), number_of_votes: 1 },
                            ],
                        },
                        PollResults {
                            results: vec![
                                PollResult { value: "Best".to_owned(), number_of_votes: 15 },
                                PollResult { value: "Recommended".to_owned(), number_of_votes: 44 },
                                PollResult { value: "Not Recommended".to_owned(), number_of_votes: 9 },
                            ],
                        },
                        PollResults {
                            results: vec![
                                PollResult { value: "Best".to_owned(), number_of_votes: 0 },
                                PollResult { value: "Recommended".to_owned(), number_of_votes: 1 },
                                PollResult { value: "Not Recommended".to_owned(), number_of_votes: 42 },
                            ],
                        },
                    ],
                },
                playing_time: Duration::minutes(120),
                min_playtime: Duration::minutes(30),
                max_playtime: Duration::minutes(120),
                min_age: 12,
                suggested_player_age: Poll {
                    name: "suggested_playerage".to_owned(),
                    title: "User Suggested Player Age".to_owned(),
                    results: vec![
                        PollResults {
                            results: vec![
                                PollResult { value: "6".to_owned(), number_of_votes: 0 },
                                PollResult { value: "8".to_owned(), number_of_votes: 0 },
                                PollResult { value: "10".to_owned(), number_of_votes: 6 },
                                PollResult { value: "14".to_owned(), number_of_votes: 1 },
                                PollResult { value: "16".to_owned(), number_of_votes: 0 },
                                PollResult { value: "18".to_owned(), number_of_votes: 0 },
                                PollResult { value: "21 and up".to_owned(), number_of_votes: 0 },
                            ],
                        },
                    ],
                },
                suggested_language_dependence: Poll {
                    name: "language_dependence".to_owned(),
                    title: "Language Dependence".to_owned(),
                    results: vec![
                        PollResults {
                            results: vec![
                                PollResult { value: "No necessary in-game text".to_owned(), number_of_votes: 0 },
                                PollResult { value: "Some necessary text - easily memorized or small crib sheet".to_owned(), number_of_votes: 0 },
                                PollResult { value: "Moderate in-game text - needs crib sheet or paste ups".to_owned(), number_of_votes: 5 },
                                PollResult { value: "Extensive use of text - massive conversion needed to be playable".to_owned(), number_of_votes: 0 },
                                PollResult { value: "Unplayable in another language".to_owned(), number_of_votes: 1 },
                            ],
                        },
                    ],
                },
                categories: vec![
                    GameCategory {
                        id: 1042,
                        name: "Expansion for Base-game".to_owned(),
                    },
                    GameCategory {
                        id: 1020,
                        name: "Exploration".to_owned(),
                    },
                    GameCategory {
                        id: 1097,
                        name: "Travel".to_owned(),
                    },
                ],
                mechanics: vec![
                    GameMechanic {
                        id: 2664,
                        name: "Deck, Bag, and Pool Building".to_owned(),
                    },
                    GameMechanic {
                        id: 2041,
                        name: "Open Drafting".to_owned(),
                    },
                    GameMechanic {
                        id: 2082,
                        name: "Worker Placement".to_owned(),
                    },
                ],
                game_families: vec![
                    GameFamilyName {
                        id: 5666,
                        name: "Players: Games with Solitaire Rules".to_owned(),
                    },
                    GameFamilyName {
                        id: 21940,
                        name: "Theme: Archaeology / Paleontology".to_owned(),
                    },
                ],
                expansions: vec![],
                expansion_for: vec![
                    Game {
                        id: 312484,
                        name: "Lost Ruins of Arnak".to_owned(),
                    },
                ],
                accessories: vec![
                    GameAccessory {
                        id: 363147,
                        name: "Lost Ruins of Arnak + Expedition Leaders: Insert".to_owned(),
                    },
                ],
                compilations: vec![],
                reimplementations: vec![],
                designers: vec![
                    GameDesigner {
                        id: 127823,
                        name: "Design".to_owned(),
                    },
                    GameDesigner {
                        id: 127822,
                        name: "Er".to_owned(),
                    },
                ],
                artists: vec![],
                publishers: vec![
                    GamePublisher {
                        id: 1391,
                        name: "Hobby Japan".to_owned(),
                    },
                ],
                stats: GameStats {
                    users_rated: 7103,
                    average_rating: 8.7037,
                    bayesian_average: 7.92384,
                    standard_deviation: 1.00019,
                    median: 0.0,
                    ranks: vec![
                        GameFamilyRank {
                            game_family_type: GameFamilyType::Subtype,
                            id: 1,
                            name: "boardgame".to_owned(),
                            friendly_name: "Board Game Rank".to_owned(),
                            value: RankValue::NotRanked,
                            bayesian_average: 7.92384,
                        },
                        GameFamilyRank {
                            game_family_type: GameFamilyType::Family,
                            id: 5497,
                            name: "strategygames".to_owned(),
                            friendly_name: "Strategy Game Rank".to_owned(),
                            value: RankValue::NotRanked,
                            bayesian_average: 8.06708,
                        },
                    ],
                    users_owned: 26790,
                    users_trading: 119,
                    users_want_in_trade: 227,
                    users_wishlisted: 1547,
                    number_of_comments: 1129,
                    number_of_weights: 146,
                    weight_rating: 3.1301,
                },
                versions: vec![],
            },
        );
    }
}
