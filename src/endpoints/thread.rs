use chrono::{NaiveDate, NaiveDateTime};

use crate::{BoardGameGeekApi, IntoQueryParam, QueryParam, Result, Thread};

#[derive(Clone, Copy, Debug)]
enum DateOrDateTimeParam {
    Date(NaiveDate),
    DateTime(NaiveDateTime),
}

impl IntoQueryParam for &DateOrDateTimeParam {
    fn into_query_param(self, key: &str) -> QueryParam<'_> {
        match self {
            DateOrDateTimeParam::Date(date) => date.into_query_param(key),
            DateOrDateTimeParam::DateTime(date_time) => date_time.into_query_param(key),
        }
    }
}

/// All optional query parameters for making a request to the thread endpoint.
#[derive(Clone, Debug, Default)]
pub struct ThreadQueryParams {
    min_post_id: Option<u64>,
    min_post_date: Option<DateOrDateTimeParam>,
    post_count: Option<u64>,
}

impl ThreadQueryParams {
    /// Construct a default `ThreadQueryParams` with no parameters set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the `min_post_id` query parameter. When set, only include posts with an ID equal to or
    /// greater than the requested ID will be returned with the thread.
    pub fn min_post_id(mut self, min_post_id: u64) -> Self {
        self.min_post_id = Some(min_post_id);
        self
    }

    /// Sets the `min_post_date` query parameter. When set, only include posts with a post date
    /// equal to or after the the requested ID will be returned with the thread.
    pub fn min_post_date(mut self, min_post_date: NaiveDate) -> Self {
        self.min_post_date = Some(DateOrDateTimeParam::Date(min_post_date));
        self
    }

    /// Sets the `min_post_date` query parameter. When set, only include posts with a post date
    /// equal to or after the the requested ID will be returned with the thread.
    pub fn min_post_date_time(mut self, min_post_date_time: NaiveDateTime) -> Self {
        self.min_post_date = Some(DateOrDateTimeParam::DateTime(min_post_date_time));
        self
    }

    /// Sets the `post_count` query parameter. If set then the number of returned posts will be
    /// limited to the requested amount. Max 1000. If unset up to the max of 1000 will be returned.
    pub fn post_count(mut self, post_count: u64) -> Self {
        self.post_count = Some(post_count);
        self
    }
}

#[derive(Clone, Debug)]
struct ThreadQueryBuilder<'builder> {
    thread_id: u64,
    params: &'builder ThreadQueryParams,
}

impl<'builder> ThreadQueryBuilder<'builder> {
    fn new(thread_id: u64, params: &'builder ThreadQueryParams) -> Self {
        Self { thread_id, params }
    }

    fn build(self) -> Vec<QueryParam<'builder>> {
        let mut query_params: Vec<_> = vec![];
        query_params.push(self.thread_id.into_query_param("id"));

        if let Some(min_post_id) = self.params.min_post_id {
            query_params.push(min_post_id.into_query_param("minarticleid"));
        }
        if let Some(min_post_date) = self.params.min_post_date {
            query_params.push(min_post_date.into_query_param("minarticledate"));
        }
        if let Some(post_count) = self.params.post_count {
            query_params.push(post_count.into_query_param("count"));
        }
        query_params
    }
}

/// Thread endpoint of the API. Used for returning information about a single thread by its ID.
pub struct ThreadApi<'api> {
    pub(crate) api: &'api BoardGameGeekApi,
    endpoint: &'static str,
}

impl<'api> ThreadApi<'api> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "thread",
        }
    }

    /// Get a thread by ID, with optional query params.
    pub async fn get(&self, thread_id: u64, query_params: &ThreadQueryParams) -> Result<Thread> {
        let query = ThreadQueryBuilder::new(thread_id, query_params);

        let request = self.api.build_request(self.endpoint, &query.build());
        let thread = self.api.execute_request::<Thread>(request).await?;

        Ok(thread)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, NaiveDate, Utc};
    use mockito::Matcher;

    use crate::{BoardGameGeekApi, Thread, ThreadPost, ThreadQueryParams};

    #[tokio::test]
    async fn get() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/thread")
            .match_query(Matcher::AllOf(vec![Matcher::UrlEncoded(
                "id".to_owned(),
                "3707929".to_owned(),
            )]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/thread/thread.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let thread = api.thread().get(3_707_929, &ThreadQueryParams::new()).await;
        mock.assert_async().await;

        dbg!(&thread);

        assert!(thread.is_ok(), "error returned when okay expected");
        let thread = thread.unwrap();

        assert_eq!(
            thread,
            Thread {
                id: 3_707_929,
                number_of_articles: 25,
                link: "https://boardgamegeek.com/thread/3707929".to_owned(),
                subject: "Do you prefer video or text reviews?".to_owned(),
                posts: vec![
                    ThreadPost {
                        id: 47_675_007,
                        username: "SiarX".to_owned(),
                        link: "https://boardgamegeek.com/thread/3707929/article/47675007#47675007"
                            .to_owned(),
                        post_date: DateTime::from_naive_utc_and_offset(
                            NaiveDate::from_ymd_opt(2026, 5, 13)
                                .unwrap()
                                .and_hms_opt(17, 35, 39)
                                .unwrap(),
                            Utc,
                        ),
                        edit_date: DateTime::from_naive_utc_and_offset(
                            NaiveDate::from_ymd_opt(2026, 5, 13)
                                .unwrap()
                                .and_hms_opt(17, 35, 39)
                                .unwrap(),
                            Utc,
                        ),
                        number_of_edits: 0,
                        subject: "Do you prefer video or text reviews?".to_owned(),
                        body: "Which format you generally prefer? Video because it is more informative, text because it is faster to consume, or some other reason?".to_owned(),
                    },
                    ThreadPost {
                        id: 47_675_065,
                        username: "qrayx".to_owned(),
                        link: "https://boardgamegeek.com/thread/3707929/article/47675065#47675065"
                            .to_owned(),
                        post_date: DateTime::from_naive_utc_and_offset(
                            NaiveDate::from_ymd_opt(2026, 5, 13)
                                .unwrap()
                                .and_hms_opt(17, 45, 06)
                                .unwrap(),
                            Utc,
                        ),
                        edit_date: DateTime::from_naive_utc_and_offset(
                            NaiveDate::from_ymd_opt(2026, 5, 14)
                                .unwrap()
                                .and_hms_opt(18, 45, 2)
                                .unwrap(),
                            Utc,
                        ),
                        number_of_edits: 2,
                        subject: "Re: Do you prefer video or text reviews?".to_owned(),
                        body: "Text.".to_owned(),
                    },
                ],
            }
        );
    }

    #[tokio::test]
    async fn get_with_params() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/thread")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("id".to_owned(), "3707929".to_owned()),
                Matcher::UrlEncoded("minarticleid".to_owned(), "556".to_owned()),
                Matcher::UrlEncoded("minarticledate".to_owned(), "2025-05-10".to_owned()),
                Matcher::UrlEncoded("count".to_owned(), "10".to_owned()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/thread/thread.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let params = ThreadQueryParams::new()
            .min_post_date(NaiveDate::from_ymd_opt(2025, 5, 10).unwrap())
            .min_post_id(556)
            .post_count(10);
        let thread = api.thread().get(3_707_929, &params).await;
        mock.assert_async().await;

        assert!(thread.is_ok(), "error returned when okay expected");
    }

    #[tokio::test]
    async fn get_with_date_time_param() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/thread")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("id".to_owned(), "3707929".to_owned()),
                Matcher::UrlEncoded("minarticleid".to_owned(), "556".to_owned()),
                Matcher::UrlEncoded(
                    "minarticledate".to_owned(),
                    "2024-12-01 10:59:45".to_owned(),
                ),
                Matcher::UrlEncoded("count".to_owned(), "10".to_owned()),
            ]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/thread/thread.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let params = ThreadQueryParams::new()
            .min_post_date_time(
                NaiveDate::from_ymd_opt(2024, 12, 1)
                    .unwrap()
                    .and_hms_opt(10, 59, 45)
                    .unwrap(),
            )
            .min_post_id(556)
            .post_count(10);
        let thread = api.thread().get(3_707_929, &params).await;
        mock.assert_async().await;

        assert!(thread.is_ok(), "error returned when okay expected");
    }
}
