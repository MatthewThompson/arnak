use serde::Deserialize;

use crate::BoardGameGeekApi;
use crate::Result;

/// The returned struct containing a list of hot board games.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct HotList {
    /// The list of hot board games.
    #[serde(rename = "$value")]
    pub items: Vec<HotItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HotItem {
    /// The ID of the game.
    pub id: u64,
    /// The rank within the hotlist, should be ordered from 1 to 50.
    pub rank: u64,
    /// A link to a jpg thumbnail image for the game.
    pub thumbnail: String,
    /// The name of the game.
    pub name: String,
    /// The year the game was first published.
    pub year_published: i64,
}

/// Hot list endpoint of the API. Used for returning the current trending board games.
pub struct HotListApi<'api> {
    pub(crate) api: &'api BoardGameGeekApi<'api>,
    endpoint: &'api str,
}

impl<'api> HotListApi<'api> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "hot",
        }
    }

    /// Gets the hot list.
    pub async fn get(&self) -> Result<HotList> {
        let request = self.api.build_request(self.endpoint, &[]);
        self.api.execute_request(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Matcher;

    #[tokio::test]
    async fn get() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let api = BoardGameGeekApi {
            base_url: &url,
            client: reqwest::Client::new(),
        };

        let mock = server
            .mock("GET", "/hot")
            .match_query(Matcher::AllOf(vec![]))
            .with_status(200)
            .with_body(
                std::fs::read_to_string("test_data/hot_list/hot_list.xml")
                    .expect("failed to load test data"),
            )
            .create_async()
            .await;

        let hot_list = api.hot_list().get().await;
        mock.assert();

        assert!(hot_list.is_ok(), "error returned when okay expected");
        let hot_list = hot_list.unwrap();

        assert_eq!(hot_list.items.len(), 50);
        assert_eq!(
            hot_list.items[0],
            HotItem {
                id: 359871,
                rank: 1,
                thumbnail: "https://cf.geekdo-images.com/XWImAu_3RK61wbzcKboVdA__thumb/img/Ry-6KHwNgERWadyxs1X1_P3dMvY=/fit-in/200x150/filters:strip_icc()/pic8145530.png".into(),
                name: "Arcs".into(),
                year_published: 2024,
            }
        )
    }
}
