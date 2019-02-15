use crate::{
    asyn::{pagination::PaginationRequest, Result, TraktApi},
    models::{IdType, SearchItemType, SearchResult, SearchType},
};
use reqwest::Method;
use std::fmt::Display;

impl TraktApi {
    pub fn search(&self, item_type: SearchType, query: &str) -> PaginationRequest<SearchResult> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("search", item_type)))
                .query(&[("query", query)]),
        )
    }

    pub fn id_lookup(
        &self,
        id_type: IdType,
        id: impl Display,
        item_type: Option<SearchItemType>,
    ) -> Result<Vec<SearchResult>> {
        self.get(api_url!(
            ("search", id_type, id),
            (
                "type",
                match item_type {
                    Some(t) => t.to_string(),
                    None => String::new(),
                }
            )
        ))
    }
}
