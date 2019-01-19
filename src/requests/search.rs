use crate::{
    error::Result,
    models::{IdType, SearchItemType, SearchResult, SearchType},
    TraktApi,
};
use std::fmt::Display;

impl TraktApi {
    pub fn search(
        &self,
        item_type: SearchType,
        query: String,
        page: u32,
        limit: u32,
    ) -> Result<Vec<SearchResult>> {
        self.get(api_url!(
            ("search", item_type),
            ("query", query),
            ("page", page),
            ("limit", limit)
        ))
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
