use chrono::NaiveDate;

use crate::{BoardGameGeekApi, IntoQueryParam, ItemType, Plays, QueryParam, Result};

/// All optional query parameters for making a request to the plays endpoint.
#[derive(Clone, Debug)]
pub struct PlaysQueryParams {
    min_date: Option<NaiveDate>,
    max_date: Option<NaiveDate>,
    // TODO new type for this? (it's a subset)
    sub_type: Option<ItemType>,
    page: Option<u64>,
}

#[derive(Clone, Debug)]
enum PlaysQuery<'n> {
    QueryByUser(&'n str),
    // TODO replace with new type (only thing and family)
    QueryById { id: u64, item_type: ItemType },
}

#[derive(Clone, Debug)]
struct PlaysQueryBuilder<'q> {
    query: PlaysQuery<'q>,
    params: PlaysQueryParams,
}

impl<'builder> PlaysQueryBuilder<'builder> {
    fn new(query: PlaysQuery<'builder>, params: PlaysQueryParams) -> Self {
        Self { query, params }
    }

    fn build(self) -> Vec<QueryParam<'builder>> {
        let mut query_params = vec![];
        // The endpoint requires either a username param, or both an ID and a type, for it to return
        // anything.
        match self.query {
            PlaysQuery::QueryByUser(username) => {
                query_params.push(username.into_query_param("username"));
            },
            PlaysQuery::QueryById { id, item_type } => {
                query_params.push(id.into_query_param("id"));
                query_params.push(item_type.into_query_param("type"));
            },
        }

        if let Some(min_date) = self.params.min_date {
            query_params.push(min_date.into_query_param("mindate"));
        }
        if let Some(max_date) = self.params.max_date {
            query_params.push(max_date.into_query_param("maxdate"));
        }
        if let Some(sub_type) = self.params.sub_type {
            query_params.push(sub_type.into_query_param("subtype"));
        }
        if let Some(page) = self.params.page {
            query_params.push(page.into_query_param("page"));
        }
        query_params
    }
}

/// Plays endpoint of the API. Used for returning information about recordings of instances of games
/// being played. Plays can be queried either by user or by ID, either way they are returned in
/// reverse chronological order.
pub struct PlaysApi<'api> {
    pub(crate) api: &'api BoardGameGeekApi,
    endpoint: &'static str,
}

impl<'api> PlaysApi<'api> {
    pub(crate) fn new(api: &'api BoardGameGeekApi) -> Self {
        Self {
            api,
            endpoint: "plays",
        }
    }

    /// Get a list of recorded game plays for a specific user
    pub async fn get_by_username(
        &self,
        username: &str,
        query_params: PlaysQueryParams,
    ) -> Result<Plays> {
        let query = PlaysQueryBuilder::new(PlaysQuery::QueryByUser(username), query_params);

        let request = self.api.build_request(self.endpoint, &query.build());
        let response = self.api.execute_request::<Plays>(request).await?;

        Ok(response)
    }

    /// Get a list of recorded game plays for a specific item that can be played.
    pub async fn get_by_item_id(
        &self,
        item_id: u64,
        item_type: ItemType,
        query_params: PlaysQueryParams,
    ) -> Result<Plays> {
        let query = PlaysQueryBuilder::new(
            PlaysQuery::QueryById {
                id: item_id,
                item_type,
            },
            query_params,
        );

        let request = self.api.build_request(self.endpoint, &query.build());
        let response = self.api.execute_request::<Plays>(request).await?;

        Ok(response)
    }
}
