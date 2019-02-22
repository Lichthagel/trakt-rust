use crate::{asyn::pagination::PaginationRequest, asyn::TraktApi, models::ListInfo};
use reqwest::Method;

impl<'a> TraktApi<'a> {
    pub fn lists_trending(&self) -> PaginationRequest<ListInfo> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("lists", "trending"))),
        )
    }

    pub fn lists_popular(&self) -> PaginationRequest<ListInfo> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("lists", "popular"))),
        )
    }
}
