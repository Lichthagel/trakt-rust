use crate::{
    error::Result,
    extended_info::{ExtendedInfoFull, ExtendedInfoNone, WithFull, WithNone},
    filters::{CommonFilters, MovieFilters},
    pagination::Pagination,
    TraktApi,
};
use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub struct MoviesRequest<'a, T> {
    client: &'a TraktApi,
    request: RequestBuilder,
    response_type: PhantomData<T>,
}

impl<'a, T: DeserializeOwned> MoviesRequest<'a, T> {
    pub fn new(client: &'a TraktApi, url: String) -> Self {
        Self {
            client,
            request: client.builder(Method::GET, &api_url!(("movies", url))),
            response_type: PhantomData,
        }
    }

    fn apply<U>(self, f: impl FnOnce(RequestBuilder) -> RequestBuilder) -> MoviesRequest<'a, U> {
        MoviesRequest {
            client: self.client,
            request: f(self.request),
            response_type: PhantomData,
        }
    }

    pub fn execute(self) -> Result<Vec<T>> {
        self.client.execute(self.request)
    }
}

impl<'a, T: DeserializeOwned> Pagination for MoviesRequest<'a, T> {
    fn page(self, page: u32) -> Self {
        self.apply(|b| b.query(&[("page", page)]))
    }

    fn limit(self, limit: u32) -> Self {
        self.apply(|b| b.query(&[("limit", limit)]))
    }
}

impl<'a, T: WithFull> WithFull for MoviesRequest<'a, T> {
    type Full = MoviesRequest<'a, T::Full>;
}

impl<'a, T: WithNone> WithNone for MoviesRequest<'a, T> {
    type None = MoviesRequest<'a, T::None>;
}

impl<'a, T: WithFull + DeserializeOwned> ExtendedInfoFull for MoviesRequest<'a, T> {
    fn full(self) -> Self::Full {
        self.apply(|b| b.query(&[("extended", "full")]))
    }
}

impl<'a, T: WithNone + DeserializeOwned> ExtendedInfoNone for MoviesRequest<'a, T> {
    fn none(self) -> Self::None {
        self.apply(|b| b.query(&[("extended", "none")]))
    }
}

impl<'a, T: DeserializeOwned> CommonFilters for MoviesRequest<'a, T> {
    fn query(self, query: &str) -> Self {
        self.apply(|b| b.query(&[("query", query)]))
    }

    fn year(self, year: u32) -> Self {
        self.apply(|b| b.query(&[("years", year)]))
    }

    fn genre(self, genre_slug: &str) -> Self {
        self.apply(|b| b.query(&[("genres", genre_slug)]))
    }

    fn language(self, language_code: &str) -> Self {
        self.apply(|b| b.query(&[("languages", language_code)]))
    }

    fn country(self, country_code: &str) -> Self {
        self.apply(|b| b.query(&[("countries", country_code)]))
    }

    fn runtimes(self, from: u32, to: u32) -> Self {
        self.apply(|b| b.query(&[("runtimes", format!("{}-{}", from, to))]))
    }

    fn ratings(self, from: u32, to: u32) -> Self {
        self.apply(|b| b.query(&[("ratings", format!("{}-{}", from, to))]))
    }
}

impl<'a, T: DeserializeOwned> MovieFilters for MoviesRequest<'a, T> {
    fn certification(self, cert_slug: &str) -> Self {
        self.apply(|b| b.query(&[("certifications", cert_slug)]))
    }
}
