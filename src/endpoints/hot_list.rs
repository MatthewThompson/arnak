use super::{HotList, HotListGame};
use crate::{BoardGameGeekApi, Result};

/// Hot list endpoint of the API. Used for returning the current trending board
/// games.
pub struct HotListApi<'api> {
    pub(crate) api: &'api BoardGameGeekApi,
    endpoint: &'static str,
}

impl<'api> HotListApi<'api> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "hot",
        }
    }

    /// Gets the current list of hot board games.
    pub async fn get(&self) -> Result<Vec<HotListGame>> {
        let request = self.api.build_request(self.endpoint, &[]);
        let response = self.api.execute_request::<HotList>(request).await?;

        Ok(response.games)
    }
}

#[cfg(test)]
mod tests {
    use mockito::Matcher;

    use super::*;
    use crate::HotListGame;

    #[tokio::test]
    async fn get() {
        let mut server = mockito::Server::new_async().await;
        let api = BoardGameGeekApi {
            base_url: server.url(),
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
        mock.assert_async().await;

        assert!(hot_list.is_ok(), "error returned when okay expected");
        let hot_list = hot_list.unwrap();

        assert_eq!(hot_list.len(), 50);
        assert_eq!(
            hot_list[0],
            HotListGame {
                id: 359_871,
                rank: 1,
                thumbnail: "https://cf.geekdo-images.com/XWImAu_3RK61wbzcKboVdA__thumb/img/Ry-6KHwNgERWadyxs1X1_P3dMvY=/fit-in/200x150/filters:strip_icc()/pic8145530.png".to_owned(),
                name: "Arcs".to_owned(),
                year_published: 2024,
            },
        );
    }
}
