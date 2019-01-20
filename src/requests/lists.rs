use crate::{error::Result, models::ListInfo, pagination::PaginationFactory, TraktApi};

impl TraktApi {
    pub fn lists_trending(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<ListInfo>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("lists", "trending"),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn lists_popular(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<ListInfo>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("lists", "popular"),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }
}
