use crate::{pagination::Pagination, sync::Result, TraktApi};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

/// A simple implementation of [Pagination]
///
/// [Pagination]: ../trait.Pagination.html
pub struct PaginationRequest<'a, T> {
    client: &'a TraktApi<'a>,
    request: RequestBuilder,
    response_type: PhantomData<T>,
}

impl<'a, T: DeserializeOwned> PaginationRequest<'a, T> {
    pub fn new(client: &'a TraktApi, request: RequestBuilder) -> Self {
        Self {
            client,
            request,
            response_type: PhantomData,
        }
    }

    fn apply(self, f: impl FnOnce(RequestBuilder) -> RequestBuilder) -> Self {
        Self {
            client: self.client,
            request: f(self.request),
            response_type: PhantomData,
        }
    }

    pub fn execute(self) -> Result<Vec<T>> {
        self.client.execute(self.request)
    }
}

/// Implementation of [Pagination] for a simple [PaginationFactory]
///
/// [Pagination]: ../trait.Pagination.html
/// [PaginationFactory]: struct.PaginationFactory.html
impl<'a, T: DeserializeOwned> Pagination for PaginationRequest<'a, T> {
    /// Set requested page
    fn page(self, page: u32) -> Self {
        self.apply(|b| b.query(&[("page", page)]))
    }

    /// Set number of requested entries
    fn limit(self, limit: u32) -> Self {
        self.apply(|b| b.query(&[("limit", limit)]))
    }
}
