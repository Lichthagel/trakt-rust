/// A trait that allows for pagination being added to a request. [More]
///
/// [More]: https://trakt.docs.apiary.io/#introduction/pagination
pub trait Pagination {
    /// Set requested page
    fn page(self, page: u32) -> Self;

    /// Set requested entry
    fn limit(self, limit: u32) -> Self;
}
