use crate::{
    asyn::{Result, TraktApi},
    error::Error,
    pagination::Pagination,
};
use reqwest::r#async::RequestBuilder;
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

impl<'a, T: DeserializeOwned + Send + 'static> PaginationRequest<'a, T> {
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
        match self.request.build() {
            Ok(req) => self.client.execute(req),
            Err(e) => Box::new(futures::future::err(Error::from(e))),
        }
    }
}

/// Implementation of [Pagination] for a simple [PaginationFactory]
///
/// [Pagination]: ../trait.Pagination.html
/// [PaginationFactory]: struct.PaginationFactory.html
impl<'a, T: DeserializeOwned + Send + 'static> Pagination for PaginationRequest<'a, T> {
    /// Set requested page
    fn page(self, page: u32) -> Self {
        self.apply(|b| b.query(&[("page", page)]))
    }

    /// Set number of requested entries
    fn limit(self, limit: u32) -> Self {
        self.apply(|b| b.query(&[("limit", limit)]))
    }
}
