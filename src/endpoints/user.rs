use super::User;
use crate::{BoardGameGeekApi, IntoQueryParam, QueryParam, Result};

/// All optional query parameters for making a request to the user endpoint.
#[derive(Clone, Debug, Default)]
pub struct UserQueryParams {
    include_buddies: Option<bool>,
    include_guilds: Option<bool>,
    include_top_list: Option<bool>,
    include_hot_list: Option<bool>,
    page: Option<u64>,
}

impl UserQueryParams {
    /// Construct a default `UserQueryParams` with no parameters set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the `include_buddies` query parameter. If set to true then the list of buddies will be
    /// returned on the user object.
    pub fn include_buddies(mut self, include_buddies: bool) -> Self {
        self.include_buddies = Some(include_buddies);
        self
    }

    /// Sets the `include_guilds` query parameter. If set to true then the list of guilds the user
    /// belongs to will be included in the user object.
    pub fn include_guilds(mut self, include_guilds: bool) -> Self {
        self.include_guilds = Some(include_guilds);
        self
    }

    /// Sets the `include_top_list` query parameter. If set to true the user's list of top
    /// games will be included in the result.
    pub fn include_top_list(mut self, include_top_list: bool) -> Self {
        self.include_top_list = Some(include_top_list);
        self
    }

    /// Sets the `include_hot_list` query parameter. If set to true the user's list of hot
    /// games will be included in the result.
    pub fn include_hot_list(mut self, include_hot_list: bool) -> Self {
        self.include_hot_list = Some(include_hot_list);
        self
    }

    /// Sets the `page` query parameter. If set then the requested page
    /// of buddies and guilds will be returned. If unset the first page will
    /// be returned.
    pub fn page(mut self, page: u64) -> Self {
        self.page = Some(page);
        self
    }
}

#[derive(Clone, Debug)]
struct UserQueryBuilder<'q> {
    username: &'q str,
    params: UserQueryParams,
}

impl<'builder> UserQueryBuilder<'builder> {
    fn new(username: &'builder str, params: UserQueryParams) -> Self {
        Self { username, params }
    }

    fn build(self) -> Vec<QueryParam<'builder>> {
        let mut query_params: Vec<_> = vec![];
        query_params.push(self.username.into_query_param("name"));
        // Set the domain as board game. The internal API allows for rpg and
        // video game to be allowed here but we hide these.
        query_params.push("boardgame".into_query_param("domain"));

        if let Some(include_buddies) = self.params.include_buddies {
            query_params.push(include_buddies.into_query_param("buddies"));
        }
        if let Some(include_guilds) = self.params.include_guilds {
            query_params.push(include_guilds.into_query_param("guilds"));
        }
        if let Some(include_top_list) = self.params.include_top_list {
            query_params.push(include_top_list.into_query_param("top"));
        }
        if let Some(include_hot_list) = self.params.include_hot_list {
            query_params.push(include_hot_list.into_query_param("hot"));
        }
        if let Some(page) = self.params.page {
            query_params.push(page.into_query_param("page"));
        }
        query_params
    }
}

/// User endpoint of the API. Used for returning information about a single user by their username.
pub struct UserApi<'api> {
    pub(crate) api: &'api BoardGameGeekApi,
    endpoint: &'static str,
}

impl<'api> UserApi<'api> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "user",
        }
    }

    /// Get a user by their username.
    pub async fn get(&self, username: &str, query_params: UserQueryParams) -> Result<User> {
        let query = UserQueryBuilder::new(username, query_params);

        let request = self.api.build_request(self.endpoint, &query.build());
        let response = self.api.execute_request::<User>(request).await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use mockito::Matcher;

    use super::*;
    use crate::{Buddy, BuddyList, GuildBrief, GuildList, ListItem, ListItemType};

    #[tokio::test]
    async fn get() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/user")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("name".to_owned(), "bluebearbgg".to_owned()),
                Matcher::UrlEncoded("domain".to_owned(), "boardgame".to_owned()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/user/user.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let user = api.user().get("bluebearbgg", UserQueryParams::new()).await;
        mock.assert_async().await;

        assert!(user.is_ok(), "error returned when okay expected");
        let user = user.unwrap();

        assert_eq!(
            user,
            User {
                id: 3_855_477,
                username: "BluebearBGG".to_owned(),
                first_name: "Matthew".to_owned(),
                last_name: "".to_owned(),
                avatar_link: None,
                year_registered: 2024,
                last_login: NaiveDate::from_ymd_opt(2025, 10, 26).unwrap(),
                state_or_province: Some("England".to_owned()),
                country: Some("United Kingdom".to_owned()),
                web_address: None,
                xbox_account: None,
                wii_account: None,
                psn_account: None,
                battlenet_account: Some("account name".to_owned()),
                steam_account: None,
                trade_rating: 0,
                top_list: vec![],
                hot_list: vec![],
                buddies: BuddyList {
                    total: 0,
                    page: 0,
                    buddies: vec![]
                },
                guilds: GuildList {
                    total: 0,
                    page: 0,
                    guilds: vec![]
                },
            }
        );
    }

    #[tokio::test]
    async fn get_with_all_params() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/user")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("name".to_owned(), "bluebearbgg".to_owned()),
                Matcher::UrlEncoded("domain".to_owned(), "boardgame".to_owned()),
                Matcher::UrlEncoded("buddies".to_owned(), "1".to_owned()),
                Matcher::UrlEncoded("guilds".to_owned(), "1".to_owned()),
                Matcher::UrlEncoded("hot".to_owned(), "1".to_owned()),
                Matcher::UrlEncoded("top".to_owned(), "1".to_owned()),
                Matcher::UrlEncoded("page".to_owned(), "4".to_owned()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/user/user_full.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let params = UserQueryParams::new()
            .include_buddies(true)
            .include_guilds(true)
            .include_hot_list(true)
            .include_top_list(true)
            .page(4);
        let user = api.user().get("bluebearbgg", params).await;
        mock.assert_async().await;

        assert!(user.is_ok(), "error returned when okay expected");
        let user = user.unwrap();

        assert_eq!(
            user,
            User {
                id: 3_855_477,
                username: "BluebearBGG".to_owned(),
                first_name: "Matthew".to_owned(),
                last_name: "".to_owned(),
                avatar_link: None,
                year_registered: 2024,
                last_login: NaiveDate::from_ymd_opt(2025, 10, 26).unwrap(),
                state_or_province: Some("England".to_owned()),
                country: Some("United Kingdom".to_owned()),
                web_address: None,
                xbox_account: None,
                wii_account: None,
                psn_account: None,
                battlenet_account: Some("account name".to_owned()),
                steam_account: None,
                trade_rating: 0,
                top_list: vec![
                    ListItem {
                        id: 338_960,
                        name: "Slay the Spire: The Board Game".to_owned(),
                        rank: 1,
                        item_type: ListItemType::Thing,
                    },
                    ListItem {
                        id: 103_500,
                        name: "Bobby Hill".to_owned(),
                        rank: 2,
                        item_type: ListItemType::Person,
                    },
                    ListItem {
                        id: 5774,
                        name: "Bézier Games".to_owned(),
                        rank: 3,
                        item_type: ListItemType::Company,
                    },
                    ListItem {
                        id: 79006,
                        name: "Components:  Meeples".to_owned(),
                        rank: 4,
                        item_type: ListItemType::Family,
                    },
                    ListItem {
                        id: 2082,
                        name: "Worker Placement".to_owned(),
                        rank: 5,
                        item_type: ListItemType::Property,
                    },
                    ListItem {
                        id: 38,
                        name: "Gen Con 2020".to_owned(),
                        rank: 6,
                        item_type: ListItemType::Event,
                    },
                ],
                hot_list: vec![
                    ListItem {
                        id: 454_971,
                        name: "Slay the Spire: The Board Game – Downfall".to_owned(),
                        rank: 1,
                        item_type: ListItemType::Thing,
                    },
                    ListItem {
                        id: 2082,
                        name: "Worker Placement".to_owned(),
                        rank: 2,
                        item_type: ListItemType::Property,
                    },
                ],
                buddies: BuddyList {
                    total: 1,
                    page: 1,
                    buddies: vec![Buddy {
                        id: 379_939,
                        name: "happy_squid".to_owned(),
                    },],
                },
                guilds: GuildList {
                    total: 2,
                    page: 1,
                    guilds: vec![
                        GuildBrief {
                            id: 1062,
                            name: "Kickstarter Games".to_owned(),
                        },
                        GuildBrief {
                            id: 999_999,
                            name: "A guild".to_owned(),
                        },
                    ],
                },
            }
        );
    }
}
