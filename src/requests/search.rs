use crate::{
    error::Result,
    models::{IdType, SearchItemType, SearchResult, SearchType},
    pagination::PaginationFactory,
    TraktApi,
};
use std::fmt::Display;

impl TraktApi {
    pub fn search(
        &self,
        item_type: SearchType,
        query: String,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<SearchResult>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("search", item_type),
            ("query", query),
            ("page", pf.page),
            ("limit", pf.limit)
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
