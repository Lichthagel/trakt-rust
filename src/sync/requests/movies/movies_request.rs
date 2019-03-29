use crate::{
    extended_info::{ExtendedInfoFull, ExtendedInfoNone, WithFull, WithNone},
    filters::{CommonFilters, MovieFilters},
    pagination::Pagination,
    Error, Result, TraktApi,
};
use hashbrown::HashMap;
use reqwest::{Method, Request};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub struct MoviesRequest<'a, T> {
    client: &'a TraktApi<'a>,
    url: String,
    query: HashMap<String, String>,
    response_type: PhantomData<T>,
}

impl<'a, T: DeserializeOwned> MoviesRequest<'a, T> {
    pub fn new(client: &'a TraktApi, url: String) -> Self {
        Self {
            client,
            url,
            query: HashMap::new(),
            response_type: PhantomData,
        }
    }

    pub fn build(&self) -> crate::Result<Request> {
        let mut req = self
            .client
            .builder(Method::GET, format!("/movies/{}", self.url));

        if !self.query.is_empty() {
            req = req.query(&self.query);
        }

        req.build().map_err(Error::from)
    }

    pub fn execute(self) -> Result<Vec<T>> {
        self.client.execute(self.build()?)
    }
}

impl<'a, T: DeserializeOwned> Pagination for MoviesRequest<'a, T> {
    fn page(mut self, page: u32) -> Self {
        self.query.insert("page".to_owned(), format!("{}", page));
        self
    }

    fn limit(mut self, limit: u32) -> Self {
        self.query.insert("limit".to_owned(), format!("{}", limit));
        self
    }
}

impl<'a, T: WithFull> WithFull for MoviesRequest<'a, T> {
    type Full = MoviesRequest<'a, T::Full>;
}

impl<'a, T: WithNone> WithNone for MoviesRequest<'a, T> {
    type None = MoviesRequest<'a, T::None>;
}

impl<'a, T: WithFull + DeserializeOwned> ExtendedInfoFull for MoviesRequest<'a, T> {
    fn full(mut self) -> Self::Full {
        self.query.insert("extended".to_owned(), "full".to_owned());

        Self::Full {
            client: self.client,
            url: self.url,
            query: self.query,
            response_type: PhantomData,
        }
    }
}

impl<'a, T: WithNone + DeserializeOwned> ExtendedInfoNone for MoviesRequest<'a, T> {
    fn none(mut self) -> Self::None {
        self.query.remove("extended");

        Self::None {
            client: self.client,
            url: self.url,
            query: self.query,
            response_type: PhantomData,
        }
    }
}

impl<'a, T: DeserializeOwned> CommonFilters for MoviesRequest<'a, T> {
    fn query(mut self, query: &str) -> Self {
        self.query.insert("query".to_owned(), query.to_owned());
        self
    }

    fn year(mut self, year: u32) -> Self {
        self.query.insert("years".to_owned(), format!("{}", year));
        self
    }

    fn genre(mut self, genre_slug: &str) -> Self {
        self.query
            .insert("genres".to_owned(), genre_slug.to_owned());
        self
    }

    fn language(mut self, language_code: &str) -> Self {
        self.query
            .insert("languages".to_owned(), language_code.to_owned());
        self
    }

    fn country(mut self, country_code: &str) -> Self {
        self.query
            .insert("countries".to_owned(), country_code.to_owned());
        self
    }

    fn runtimes(mut self, from: u32, to: u32) -> Self {
        self.query
            .insert("runtimes".to_owned(), format!("{}-{}", from, to));
        self
    }

    fn ratings(mut self, from: u32, to: u32) -> Self {
        self.query
            .insert("ratings".to_owned(), format!("{}-{}", from, to));
        self
    }
}

impl<'a, T: DeserializeOwned> MovieFilters for MoviesRequest<'a, T> {
    fn certification(mut self, cert_slug: &str) -> Self {
        self.query
            .insert("certifications".to_owned(), cert_slug.to_owned());
        self
    }
}
