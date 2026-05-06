use crate::{BoardGameGeekApi, ForumGroup, IntoQueryParam, ItemDomain, QueryParam, Result};

#[derive(Clone, Debug)]
struct ForumGroupQueryBuilder {
    id: u64,
    forum_domain: ItemDomain,
}

impl<'builder> ForumGroupQueryBuilder {
    fn new(id: u64, forum_domain: ItemDomain) -> Self {
        Self { id, forum_domain }
    }

    fn build(self) -> Vec<QueryParam<'builder>> {
        vec![
            self.id.into_query_param("id"),
            self.forum_domain.into_query_param("type"),
        ]
    }
}

/// Forum group endpoint of the API. Used for returning list of details for forums specific to a
/// certain domain. For example, all the forums for a certain game, or game family.
pub struct ForumGroupApi<'api> {
    pub(crate) api: &'api BoardGameGeekApi,
    endpoint: &'static str,
}

impl<'api> ForumGroupApi<'api> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "forumlist",
        }
    }

    /// Get the list of forums that belong to a particular game family, by that game's ID.
    pub async fn get_game_forums(&self, id: u64) -> Result<ForumGroup> {
        self.get_forum_group_by_id_and_type(id, ItemDomain::Item)
            .await
    }

    /// Get the list of forums that belong to a particular game family, by that game family's ID.
    pub async fn get_game_family_forums(&self, id: u64) -> Result<ForumGroup> {
        self.get_forum_group_by_id_and_type(id, ItemDomain::Family)
            .await
    }

    async fn get_forum_group_by_id_and_type(
        &self,
        id: u64,
        domain_type: ItemDomain,
    ) -> Result<ForumGroup> {
        let query = ForumGroupQueryBuilder::new(id, domain_type);

        let request = self.api.build_request(self.endpoint, &query.build());
        let response = self.api.execute_request::<ForumGroup>(request).await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, Utc};
    use mockito::Matcher;

    use super::*;
    use crate::ForumDetails;

    #[tokio::test]
    async fn get_game_forums() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/forumlist")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("id".to_owned(), "13".to_owned()),
                Matcher::UrlEncoded("type".to_owned(), "thing".to_owned()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/forum_group/thing_forum_group.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let forum_group = api.forum_group().get_game_forums(13).await;
        mock.assert_async().await;

        assert!(forum_group.is_ok(), "error returned when okay expected");
        let forum_group = forum_group.unwrap();

        assert_eq!(
            forum_group,
            ForumGroup {
                domain_id: 13,
                forum_domain: ItemDomain::Item,
                forums: vec![
                    ForumDetails {
                        id: 297,
                        title: "Reviews".to_owned(),
                        description: "Post your game reviews in this forum.".to_owned(),
                        no_posting: false,
                        number_of_threads: 199,
                        number_of_posts: 1597,
                        last_post_date: None,
                    },
                    ForumDetails {
                        id: 926,
                        title: "Sessions".to_owned(),
                        description: "Post your session reports here.".to_owned(),
                        no_posting: false,
                        number_of_threads: 440,
                        number_of_posts: 1043,
                        last_post_date: Some(DateTime::from_naive_utc_and_offset(
                            NaiveDate::from_ymd_opt(2025, 4, 16)
                                .unwrap()
                                .and_hms_opt(21, 21, 12)
                                .unwrap(),
                            Utc
                        )),
                    },
                ],
            },
        );
    }

    #[tokio::test]
    async fn get_game_family_forums() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/forumlist")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("id".to_owned(), "1".to_owned()),
                Matcher::UrlEncoded("type".to_owned(), "family".to_owned()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/forum_group/family_forum_group.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let forum_group = api.forum_group().get_game_family_forums(1).await;
        mock.assert_async().await;

        assert!(forum_group.is_ok(), "error returned when okay expected");
        let forum_group = forum_group.unwrap();

        assert_eq!(
            forum_group,
            ForumGroup {
                domain_id: 1,
                forum_domain: ItemDomain::Family,
                forums: vec![ForumDetails {
                    id: 123,
                    title: "General".to_owned(),
                    description: "General discussion about this family of items.".to_owned(),
                    no_posting: false,
                    number_of_threads: 0,
                    number_of_posts: 0,
                    last_post_date: None,
                },],
            },
        );
    }
}
