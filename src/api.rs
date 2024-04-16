use std::future::Future;
use std::time::Duration;

use reqwest::{RequestBuilder, Response};
use serde_xml_rs::from_str;
use tokio::time::sleep;

use crate::endpoints::collection::CollectionApi;
use crate::{BoardGameGeekApiError, Result};

pub struct BoardGameGeekApi {
    pub(crate) base_url: &'static str,
    /// Http client for making requests.
    pub(crate) client: reqwest::Client,
}

impl Default for BoardGameGeekApi {
    fn default() -> Self {
        Self::new()
    }
}

impl BoardGameGeekApi {
    const BASE_URL: &'static str = "https://boardgamegeek.com/xmlapi2";

    pub fn new() -> Self {
        Self {
            base_url: BoardGameGeekApi::BASE_URL,
            client: reqwest::Client::new(),
        }
    }

    /// Returns the collection endpoint of the API, which is used for querying a specific
    /// user's board game collection.
    pub fn collection(&self) -> CollectionApi {
        CollectionApi::new(self)
    }

    // Creates a reqwest::RequestBuilder from the base url and the provided
    // endpoint and query.
    pub(crate) fn build_request(
        &self,
        endpoint: &str,
        query: &[(&str, &str)],
    ) -> reqwest::RequestBuilder {
        self.client
            .get(format!("{}/{}", self.base_url, endpoint))
            .query(query)
    }

    // Handles a HTTP request by calling execute_request_raw, then parses the response
    // to the expected type.
    pub(crate) async fn execute_request<'a, T: serde::de::DeserializeOwned + 'a>(
        &'a self,
        request: RequestBuilder,
    ) -> Result<T> {
        let response = self.execute_request_raw(request).await?;
        let response_text = response.text().await?;

        from_str(&response_text).map_err(BoardGameGeekApiError::ParseError)
    }

    // Handles an HTTP request. execute_request_raw accepts a reqwest::ReqwestBuilder,
    // sends it and awaits. If the response is Accepted (202), it will wait for the data to
    // be ready and try again. Any errors are wrapped in the local BoardGameGeekApiError
    // enum before being returned.
    fn execute_request_raw<'a>(
        &self,
        request: RequestBuilder,
    ) -> impl Future<Output = Result<Response>> + 'a {
        let retries: u32 = 0;
        async move {
            loop {
                let request_clone = request.try_clone().ok_or(BoardGameGeekApiError::ApiError(
                    "Unknown error, failed to clone request".to_string(),
                ))?;
                let response = match request_clone.send().await {
                    Ok(response) => response,
                    Err(e) => break Err(BoardGameGeekApiError::HttpError(e)),
                };
                if response.status() == reqwest::StatusCode::ACCEPTED {
                    // Request has been accepted but the data isn't ready yet, we wait a short amount of time
                    // before trying again, with exponential backoff.
                    let backoff_multiplier = 2_u64.pow(retries);
                    let delay = Duration::from_millis(200 * backoff_multiplier);
                    sleep(delay).await;
                    continue;
                }
                break match response.error_for_status() {
                    Err(e) => Err(BoardGameGeekApiError::HttpError(e)),
                    Ok(res) => Ok(res),
                };
            }
        }
    }
}
