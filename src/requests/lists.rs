use crate::{
    error::{Error, Result},
    models::ListInfo,
    TraktApi,
};

impl TraktApi {
    pub fn lists_trending(&self, page: u32, limit: u32) -> Result<Vec<ListInfo>> {
        self.get(api_url!(
            ("lists", "trending"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn lists_popular(&self, page: u32, limit: u32) -> Result<Vec<ListInfo>> {
        self.get(api_url!(
            ("lists", "popular"),
            ("page", page),
            ("limit", limit)
        ))
    }
}
