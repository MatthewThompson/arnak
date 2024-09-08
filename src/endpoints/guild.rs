use super::Guild;
use crate::{BoardGameGeekApi, IntoQueryParam, QueryParam, Result};

/// Which field to sort the list of members by, either username or date joined.
#[derive(Clone, Debug)]
pub enum GuildMemberSortBy {
    /// Sort by username alphabetically, ascending.
    Username,
    /// Sort by date joined, descending starting from the user who most recently joined the guild.
    DateJoined,
}

impl IntoQueryParam for GuildMemberSortBy {
    fn into_query_param(self, key: &str) -> QueryParam<'_> {
        match self {
            Self::Username => (key, "username".to_owned()),
            Self::DateJoined => (key, "date".to_owned()),
        }
    }
}

/// Optional query parameters that can be made when retrieving guilds.
#[derive(Clone, Debug, Default)]
pub struct GuildQueryParams {
    // Whether to include a page of members, and which one to include.
    // Setting this will ensure the `members` flag is set too.
    include_member_page: Option<u64>,
    // If set, will sort the list of members by the selected option.
    // Defaults to username.
    sort_by: Option<GuildMemberSortBy>,
}

impl GuildQueryParams {
    /// Constructs a new guild query with parameters set to None.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the `include_member_page` query parameter.
    pub fn include_member_page(mut self, member_page: u64) -> Self {
        self.include_member_page = Some(member_page);
        self
    }

    /// Sets the `sort_by` query parameter.
    pub fn sort_by(mut self, sort_by: GuildMemberSortBy) -> Self {
        self.sort_by = Some(sort_by);
        self
    }
}

#[derive(Clone, Debug)]
struct GuildQueryBuilder {
    guild_id: u64,
    params: GuildQueryParams,
}

impl<'builder> GuildQueryBuilder {
    fn new(guild_id: u64, params: GuildQueryParams) -> Self {
        Self { guild_id, params }
    }

    // Converts the list of parameters into a vector of
    // key value pairs that reqwest can use as HTTP query parameters.
    fn build(self) -> Vec<QueryParam<'builder>> {
        let mut query_params: Vec<_> = vec![];
        query_params.push(self.guild_id.into_query_param("id"));

        if let Some(member_page) = self.params.include_member_page {
            query_params.push(true.into_query_param("members"));
            query_params.push(member_page.into_query_param("page"));
        }
        if let Some(sort_by) = self.params.sort_by {
            query_params.push(sort_by.into_query_param("sort"));
        }
        query_params
    }
}

/// Endpoint for getting guilds by their ID.
///
/// A guild is a group of members on the site, with a specific purpose based around events, clubs,
/// or location.
pub struct GuildApi<'api> {
    pub(crate) api: &'api BoardGameGeekApi,
    endpoint: &'static str,
}

impl<'api> GuildApi<'api> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "guild",
        }
    }

    /// Gets a guild via the provided query params.
    pub async fn get(&self, guild_id: u64, query_params: GuildQueryParams) -> Result<Guild> {
        let query = GuildQueryBuilder::new(guild_id, query_params);

        let request = self.api.build_request(self.endpoint, &query.build());
        self.api.execute_request::<Guild>(request).await
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use mockito::Matcher;

    use super::*;
    use crate::{Guild, Location, Member, MemberPage};

    #[tokio::test]
    async fn get_by_id() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/guild")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "id".into(),
                "13".into(),
            )]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/guild/guild.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let guild = api.guild().get(13, GuildQueryParams::new()).await;
        mock.assert_async().await;

        assert!(guild.is_ok(), "error returned when okay expected");
        let guild = guild.unwrap();

        assert_eq!(
            guild,
            Guild {
                id: 13,
                name: "Con of the North".to_owned(),
                created_at: Utc.with_ymd_and_hms(2007, 6, 14, 1, 6, 46).unwrap(),
                category: "event".to_owned(),
                website: "http://www.website.org/".to_owned(),
                manager: "ManagerName".to_owned(),
                description:
                    "A group to discuss the Con of the North, held in February in Minnesota."
                        .to_owned(),
                location: Location {
                    address_line_1: String::new(),
                    address_line_2: String::new(),
                    city: "Saint Paul".to_owned(),
                    state: "Minnesota".to_owned(),
                    country: "United States".to_owned(),
                    postal_code: String::new(),
                },
                member_page: None,
            },
        );
    }

    #[tokio::test]
    async fn get_with_member_page() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/guild")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("id".into(), "13".into()),
                Matcher::UrlEncoded("members".into(), "1".into()),
                Matcher::UrlEncoded("page".into(), "2".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/guild/guild_with_member_page.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let guild = api
            .guild()
            .get(13, GuildQueryParams::new().include_member_page(2))
            .await;
        mock.assert_async().await;

        assert!(guild.is_ok(), "error returned when okay expected");
        let guild = guild.unwrap();

        assert_eq!(
            guild,
            Guild {
                id: 13,
                name: "Con of the North".to_owned(),
                created_at: Utc.with_ymd_and_hms(2007, 6, 14, 1, 6, 46).unwrap(),
                category: "event".to_owned(),
                website: "http://www.website.org/".to_owned(),
                manager: "ManagerName".to_owned(),
                description:
                    "A group to discuss the Con of the North, held in February in Minnesota."
                        .to_owned(),
                location: Location {
                    address_line_1: String::new(),
                    address_line_2: String::new(),
                    city: "Saint Paul".to_owned(),
                    state: "Minnesota".to_owned(),
                    country: "United States".to_owned(),
                    postal_code: String::new(),
                },
                member_page: Some(MemberPage {
                    total_members: 27,
                    page_number: 2,
                    members: vec![
                        Member {
                            name: "SomeMember".to_owned(),
                            date_joined: Utc.with_ymd_and_hms(2016, 5, 19, 10, 53, 1).unwrap(),
                        },
                        Member {
                            name: "SomeOtherMember".to_owned(),
                            date_joined: Utc.with_ymd_and_hms(2009, 4, 24, 0, 1, 22).unwrap(),
                        },
                    ],
                }),
            },
        );
    }

    #[tokio::test]
    async fn get_from_query_params() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/guild")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("id".into(), "13".into()),
                Matcher::UrlEncoded("members".into(), "1".into()),
                Matcher::UrlEncoded("page".into(), "5".into()),
                Matcher::UrlEncoded("sort".into(), "date".into()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/guild/guild_with_member_page.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let params = GuildQueryParams::new()
            .include_member_page(5)
            .sort_by(GuildMemberSortBy::DateJoined);
        let guild = api.guild().get(13, params).await;
        mock.assert_async().await;

        assert!(guild.is_ok(), "error returned when okay expected");
    }
}
