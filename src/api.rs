use std::future::Future;
use std::time::Duration;

use reqwest::{RequestBuilder, Response};
use serde_xml_rs::from_str;
use tokio::time::sleep;

use crate::endpoints::collection::CollectionApi;
use crate::{ApiXmlErrors, Error, Result};

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

    /// Creates a new API with a default constructed reqwest client.
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

        let parse_result = from_str(&response_text);
        match parse_result {
            Ok(result) => Ok(result),
            Err(e) => {
                // The API returns a 200 but with an XML error in some cases,
                // such as a usename not found, so we try to parse that first
                // for a more specific error.
                let api_error = from_str::<ApiXmlErrors>(&response_text);
                match api_error {
                    Ok(api_error) => Err(Error::ApiError(api_error.errors[0].message.to_string())),
                    // If it's not a parseable error, we want to return the orignal error
                    // from failing to parse the output type.
                    Err(_) => Err(Error::ParseError(e)),
                }
            }
        }
    }

    // Handles an HTTP request. execute_request_raw accepts a reqwest::ReqwestBuilder,
    // sends it and awaits. If the response is Accepted (202), it will wait for the data to
    // be ready and try again. Any errors are wrapped in the local BoardGameGeekApiError
    // enum before being returned.
    fn execute_request_raw<'a>(
        &self,
        request: RequestBuilder,
    ) -> impl Future<Output = Result<Response>> + 'a {
        let mut retries: u32 = 0;
        async move {
            loop {
                let request_clone = request.try_clone().ok_or(Error::LocalError(
                    "Unknown error, failed to clone request".to_string(),
                ))?;
                let response = match request_clone.send().await {
                    Ok(response) => response,
                    Err(e) => break Err(Error::HttpError(e)),
                };
                if response.status() == reqwest::StatusCode::ACCEPTED {
                    if retries > 4 {
                        break Err(Error::LocalError(format!("Response was accepted but maximum retries hit. Retried {retries} times.")))
                    }
                    // Request has been accepted but the data isn't ready yet, we wait a short amount of time
                    // before trying again, with exponential backoff.
                    let backoff_multiplier = 2_u64.pow(retries);
                    retries += 1;
                    let delay = Duration::from_millis(200 * backoff_multiplier);
                    sleep(delay).await;
                    continue;
                }
                break match response.error_for_status() {
                    Err(e) => Err(Error::HttpError(e)),
                    Ok(res) => Ok(res),
                };
            }
        }
    }
}
