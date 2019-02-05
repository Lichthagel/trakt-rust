/// A trait that allows for pagination being added to a request. [More]
///
/// [More]: https://trakt.docs.apiary.io/#introduction/pagination
pub trait Pagination {

    /// Set requested page
    fn page(self, page: u32) -> Self;

    /// Set requested entry
    fn limit(self, limit: u32) -> Self;
}

/// A simple implementation of [Pagination]
///
/// [Pagination]: trait.Pagination.html
pub struct PaginationFactory {
    pub page: u32,
    pub limit: u32,
}

impl Default for PaginationFactory {

    /// Creates a default [PaginationFactory] with page 1 and 10 entries
    ///
    /// [PaginationFactory]: struct.PaginationFactory.html
    fn default() -> Self {
        Self { page: 1, limit: 10 }
    }
}

/// Implementation of [Pagination] for a simple [PaginationFactory]
///
/// [Pagination]: trait.Pagination.html
/// [PaginationFactory]: struct.PaginationFactory.html
impl Pagination for PaginationFactory {

    /// Set requested page
    fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    /// Set number of requested entries
    fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }
}
