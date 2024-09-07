use std::future::Future;
use std::time::Duration;

use reqwest::{RequestBuilder, Response};
use tokio::time::sleep;

use crate::endpoints::collection::CollectionApi;
use crate::utils::deserialise_xml_string;
use crate::{
    deserialise_maybe_error, CollectionItem, CollectionItemBrief, Error, GameApi, GameFamilyApi,
    GuildApi, HotListApi, Result, SearchApi,
};

/// API for making requests to the [Board Game Geek API](https://boardgamegeek.com/wiki/page/BGG_XML_API2).
pub struct BoardGameGeekApi {
    // URL for the board game geek API.
    // Note this is a String instead of a 'static &str for unit test purposes.
    pub(crate) base_url: String,
    // Http client for making requests to the underlying API.
    pub(crate) client: reqwest::Client,
}

impl Default for BoardGameGeekApi {
    fn default() -> Self {
        Self::new()
    }
}

impl BoardGameGeekApi {
    const BASE_URL: &'static str = "https://boardgamegeek.com/xmlapi2";

    /// Creates a new API from a default HTTP client.
    pub fn new() -> Self {
        Self {
            base_url: String::from(BoardGameGeekApi::BASE_URL),
            client: reqwest::Client::new(),
        }
    }

    /// Returns the collection endpoint of the API, which is used for querying a
    /// specific user's collections.
    ///
    /// A collection can be a set of games, or game accessories,
    /// and doesn't necessarily just include items that the user owns, but also
    /// items on the user's wishlist or ones they have previously owned, or even
    /// items they have manually added to the collection.
    pub fn collection(&self) -> CollectionApi<CollectionItem> {
        CollectionApi::new(self)
    }

    /// Returns the brief collection endpoint of the API, which is used for querying a
    /// specific user's collections, but in a more brief format. Data such as game images is
    /// omitted.
    ///
    /// A collection can be a set of games, or game accessories,
    /// and doesn't necessarily just include items that the user owns, but also
    /// items on the user's wishlist or ones they have previously owned, or even
    /// items they have manually added to the collection.
    pub fn collection_brief(&self) -> CollectionApi<CollectionItemBrief> {
        CollectionApi::new(self)
    }

    /// Returns the game family endpoint of the API, which is used for querying
    /// families of games by their IDs.
    pub fn game_family(&self) -> GameFamilyApi {
        GameFamilyApi::new(self)
    }

    /// Returns the game endpoint of the API, which is used for querying
    /// full game details by their IDs.
    pub fn game(&self) -> GameApi {
        GameApi::new(self)
    }

    /// Returns the guild endpoint of the API, which is used for querying
    /// guilds by their IDs.
    pub fn guild(&self) -> GuildApi {
        GuildApi::new(self)
    }

    /// Returns the hot list endpoint of the API, which is used for querying the
    /// current trending board games.
    pub fn hot_list(&self) -> HotListApi {
        HotListApi::new(self)
    }

    /// Returns the search endpoint of the API, which is used for searching for
    /// board games by name.
    pub fn search(&self) -> SearchApi {
        SearchApi::new(self)
    }

    // Creates a reqwest::RequestBuilder from the base url and the provided
    // endpoint and query.
    pub(crate) fn build_request(
        &self,
        endpoint: &str,
        query: &[(&str, String)],
    ) -> reqwest::RequestBuilder {
        self.client
            .get(format!("{}/{}", self.base_url, endpoint))
            .query(query)
    }

    // Handles a HTTP request by calling execute_request_raw, then parses the
    // response to the expected type.
    pub(crate) async fn execute_request<T: serde::de::DeserializeOwned>(
        &self,
        request: RequestBuilder,
    ) -> Result<T> {
        let response = self.send_request(request).await?;
        let response_text = response.text().await?;

        let parse_result = deserialise_xml_string(&response_text);
        match parse_result {
            Ok(result) => Ok(result),
            Err(e) => {
                // The API returns a 200 but with an XML error in some cases,
                // such as a username not found, so we try to parse that first
                // for a more specific error.
                match deserialise_maybe_error(&response_text) {
                    Some(api_error) => Err(api_error),
                    // If the error cannot be parsed, that likely means it was a successful response
                    // that we failed to parse. So return an unexpected response with the original
                    // error.
                    None => Err(Error::InvalidResponseError(e)),
                }
            },
        }
    }

    // Handles an HTTP request. send_request accepts a reqwest::ReqwestBuilder,
    // sends it and awaits. If the response is Accepted (202), it will wait for the
    // data to be ready and try again.
    fn send_request(&self, request: RequestBuilder) -> impl Future<Output = Result<Response>> {
        let mut retries: u32 = 0;
        async move {
            loop {
                let request_clone = request.try_clone().expect("Couldn't clone request");
                let response = match request_clone.send().await {
                    Ok(response) => response,
                    Err(e) => break Err(Error::HttpError(e)),
                };
                if response.status() == reqwest::StatusCode::ACCEPTED {
                    if retries >= 4 {
                        break Err(Error::MaxRetryError(retries + 1));
                    }
                    // Request has been accepted but the data isn't ready yet, we wait a short
                    // amount of time before trying again, with exponential
                    // backoff.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn send_request() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/some_endpoint")
            .with_status(200)
            .with_body("hello there")
            .create_async()
            .await;

        let request = api.build_request("some_endpoint", &[]);
        let res = api.send_request(request).await;

        mock.assert_async().await;
        assert!(res.is_ok());
        assert!(res.unwrap().text().await.unwrap() == "hello there");
    }

    #[tokio::test]
    async fn send_failed_request() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/some_endpoint")
            .with_status(500)
            .create_async()
            .await;

        let request = api.build_request("some_endpoint", &[]);
        let res = api.send_request(request).await;

        mock.assert_async().await;
        assert!(res.is_err());
    }

    #[tokio::test(start_paused = true)]
    async fn send_request_202_retries() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/some_endpoint")
            .with_status(202)
            .create_async()
            .await;

        let request = api.build_request("some_endpoint", &[]);
        let res = api.send_request(request).await;

        mock.expect(1);

        let mock = server
            .mock("GET", "/some_endpoint")
            .with_status(202)
            .create_async()
            .await;
        tokio::time::sleep(Duration::from_millis(200)).await;
        mock.expect(2);

        let mock = server
            .mock("GET", "/some_endpoint")
            .with_status(202)
            .create_async()
            .await;
        tokio::time::sleep(Duration::from_millis(400)).await;
        mock.expect(3);

        let mock = server
            .mock("GET", "/some_endpoint")
            .with_status(202)
            .create_async()
            .await;
        tokio::time::sleep(Duration::from_millis(800)).await;
        mock.expect(4);

        let mock = server
            .mock("GET", "/some_endpoint")
            .with_status(202)
            .create_async()
            .await;
        tokio::time::sleep(Duration::from_millis(1600)).await;
        mock.expect(5);

        let mock = server
            .mock("GET", "/some_endpoint")
            .with_status(202)
            .create_async()
            .await;
        tokio::time::sleep(Duration::from_millis(3200)).await;
        mock.expect(5);

        assert!(res.is_err());
    }
}
