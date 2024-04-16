use reqwest::RequestBuilder;
use serde_xml_rs::from_str;

use crate::endpoints::collection::CollectionApi;

pub struct BoardGameGeekAPI {
    pub(crate) base_url: &'static str,
    /// Http client for making requests.
    pub(crate) client: reqwest::Client,
}

impl Default for BoardGameGeekAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl BoardGameGeekAPI {
    const BASE_URL: &'static str = "https://boardgamegeek.com/xmlapi2";

    pub fn new() -> Self {
        Self {
            base_url: BoardGameGeekAPI::BASE_URL,
            client: reqwest::Client::new(),
        }
    }

    pub fn collection(&self) -> CollectionApi {
        CollectionApi::new(self)
    }

    pub(crate) fn build_request(
        &self,
        endpoint: &str,
        query: &[(&str, &str)],
    ) -> reqwest::RequestBuilder {
        self.client
            .get(format!("{}/{}", self.base_url, endpoint))
            .query(query)
    }

    // TODO split this into an async move execute_request function that can then handle 202 and retries
    //  and another function which just calls that and deserialises the response
    // TODO make custom error and handle/wrap reqwest errors properly
    pub(crate) async fn execute_request<'a, T: serde::de::DeserializeOwned + 'a>(
        &'a self,
        request: RequestBuilder,
    ) -> Result<T, reqwest::Error> {
        let response = request.send().await?;
        let response_text = response.text().await?;

        let output: T = from_str(&response_text).unwrap();

        Ok(output)
    }
}
