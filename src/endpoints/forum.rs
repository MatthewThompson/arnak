use crate::{BoardGameGeekApi, Forum, IntoQueryParam, QueryParam, Result};

/// All optional query parameters for making a request to the forum endpoint.
#[derive(Clone, Debug, Default)]
struct ForumQueryParams {
    page: Option<u64>,
}

#[derive(Clone, Debug)]
struct ForumQueryBuilder<'builder> {
    forum_id: u64,
    params: &'builder ForumQueryParams,
}

impl<'builder> ForumQueryBuilder<'builder> {
    fn new(forum_id: u64, params: &'builder ForumQueryParams) -> Self {
        Self { forum_id, params }
    }

    fn build(self) -> Vec<QueryParam<'builder>> {
        let mut query_params = vec![];
        query_params.push(self.forum_id.into_query_param("id"));
        if let Some(page) = self.params.page {
            query_params.push(page.into_query_param("page"));
        }
        query_params
    }
}

/// The forum endpoint of the API.
pub struct ForumApi<'api> {
    pub(crate) api: &'api BoardGameGeekApi,
    endpoint: &'static str,
}

impl<'api> ForumApi<'api> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "forum",
        }
    }

    /// Get metadata for a forum by ID, contains the first page of threads.
    pub async fn get(&self, id: u64) -> Result<Forum> {
        let params = ForumQueryParams::default();
        let query = ForumQueryBuilder::new(id, &params);

        let request = self.api.build_request(self.endpoint, &query.build());
        let response = self.api.execute_request::<Forum>(request).await?;

        Ok(response)
    }

    /// Get metadata for a forum by ID, and a particular page of threads in this forum.
    pub async fn get_with_threads_page(&self, id: u64, page: u64) -> Result<Forum> {
        let params = ForumQueryParams { page: Some(page) };
        let query = ForumQueryBuilder::new(id, &params);

        let request = self.api.build_request(self.endpoint, &query.build());
        let response = self.api.execute_request::<Forum>(request).await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, Utc};
    use mockito::Matcher;

    use super::*;
    use crate::ThreadDetails;

    #[tokio::test]
    async fn get() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/forum")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "id".to_owned(),
                "1154020".to_owned(),
            )]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/forum/forum.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let forum = api.forum().get(1_154_020).await;
        mock.assert_async().await;

        assert!(forum.is_ok(), "error returned when okay expected");
        let forum = forum.unwrap();

        assert_eq!(
            forum,
            Forum {
                id: 1_154_020,
                title: "Geek Lobby".to_owned(),
                number_of_threads: 2874,
                number_of_posts: 125_454,
                threads: vec![
                    ThreadDetails {
                        id: 1_304_539,
                        subject: "Whoever gets in the last word wins.".to_owned(),
                        author: "ravager".to_owned(),
                        number_of_articles: 8598,
                        post_date: DateTime::from_naive_utc_and_offset(
                            NaiveDate::from_ymd_opt(2015, 1, 19)
                                .unwrap()
                                .and_hms_opt(14, 12, 57)
                                .unwrap(),
                            Utc,
                        ),
                        last_post_date: DateTime::from_naive_utc_and_offset(
                            NaiveDate::from_ymd_opt(2026, 5, 14)
                                .unwrap()
                                .and_hms_opt(11, 3, 35)
                                .unwrap(),
                            Utc,
                        ),
                    },
                    ThreadDetails {
                        id: 3_707_147,
                        subject: "1001 Game generator".to_owned(),
                        author: "Abarbesgaard".to_owned(),
                        number_of_articles: 2,
                        post_date: DateTime::from_naive_utc_and_offset(
                            NaiveDate::from_ymd_opt(2026, 5, 12)
                                .unwrap()
                                .and_hms_opt(6, 36, 59)
                                .unwrap(),
                            Utc,
                        ),
                        last_post_date: DateTime::from_naive_utc_and_offset(
                            NaiveDate::from_ymd_opt(2026, 5, 14)
                                .unwrap()
                                .and_hms_opt(3, 38, 58)
                                .unwrap(),
                            Utc,
                        ),
                    },
                ],
            }
        );
    }

    #[tokio::test]
    async fn get_with_threads_page() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/forum")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("id".to_owned(), "1154020".to_owned()),
                Matcher::UrlEncoded("page".to_owned(), "3".to_owned()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/forum/forum.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let forum = api.forum().get_with_threads_page(1_154_020, 3).await;
        mock.assert_async().await;

        assert!(forum.is_ok(), "error returned when okay expected");
        let forum = forum.unwrap();

        assert_eq!(
            forum,
            Forum {
                id: 1_154_020,
                title: "Geek Lobby".to_owned(),
                number_of_threads: 2874,
                number_of_posts: 125_454,
                threads: vec![
                    ThreadDetails {
                        id: 1_304_539,
                        subject: "Whoever gets in the last word wins.".to_owned(),
                        author: "ravager".to_owned(),
                        number_of_articles: 8598,
                        post_date: DateTime::from_naive_utc_and_offset(
                            NaiveDate::from_ymd_opt(2015, 1, 19)
                                .unwrap()
                                .and_hms_opt(14, 12, 57)
                                .unwrap(),
                            Utc,
                        ),
                        last_post_date: DateTime::from_naive_utc_and_offset(
                            NaiveDate::from_ymd_opt(2026, 5, 14)
                                .unwrap()
                                .and_hms_opt(11, 3, 35)
                                .unwrap(),
                            Utc,
                        ),
                    },
                    ThreadDetails {
                        id: 3_707_147,
                        subject: "1001 Game generator".to_owned(),
                        author: "Abarbesgaard".to_owned(),
                        number_of_articles: 2,
                        post_date: DateTime::from_naive_utc_and_offset(
                            NaiveDate::from_ymd_opt(2026, 5, 12)
                                .unwrap()
                                .and_hms_opt(6, 36, 59)
                                .unwrap(),
                            Utc,
                        ),
                        last_post_date: DateTime::from_naive_utc_and_offset(
                            NaiveDate::from_ymd_opt(2026, 5, 14)
                                .unwrap()
                                .and_hms_opt(3, 38, 58)
                                .unwrap(),
                            Utc,
                        ),
                    },
                ],
            }
        );
    }
}
