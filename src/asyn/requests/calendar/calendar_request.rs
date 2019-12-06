use crate::{
    asyn::{Result, TraktApi},
    extended_info::{ExtendedInfoFull, ExtendedInfoNone, WithFull, WithNone},
    filters::Filters,
    models::ShowStatus,
    Error,
};
use chrono::{Date, Utc};
use reqwest::{r#async::Request, Method};
use serde::de::DeserializeOwned;
use std::{collections::HashMap, marker::PhantomData};

/// A request to a calendar endpoint
#[derive(Debug, Clone)]
pub struct CalendarRequest<'a, T: DeserializeOwned> {
    client: &'a TraktApi<'a>,
    url: &'a str,
    start_date: Option<Date<Utc>>,
    days: Option<u32>,
    access_token: Option<&'a str>,
    query: HashMap<String, String>,
    response_type: PhantomData<T>,
}

impl<'a, T: DeserializeOwned + Send + 'static> CalendarRequest<'a, T> {
    pub fn new(client: &'a TraktApi, url: &'a str, access_token: Option<&'a str>) -> Self {
        Self {
            client,
            url,
            start_date: None,
            days: None,
            access_token,
            query: HashMap::new(),
            response_type: PhantomData,
        }
    }

    /// Set the start date
    pub fn start_date(mut self, start_date: Date<Utc>) -> Self {
        self.start_date = Some(start_date);
        self
    }

    /// Set the number of days you want entries for. Will be ignored if you don't use [start_date].
    ///
    /// [start_date]: struct.CalendarRequest.html#method.start_date
    pub fn days(mut self, days: u32) -> Self {
        self.days = Some(days);
        self
    }

    /// Build a [reqwest::Request]
    ///
    /// [reqwest::Request]: ../../../../../reqwest/struct.Request.html
    pub fn build(&self) -> std::result::Result<Request, Error> {
        let mut url = "/calendars/".to_owned();
        url.push_str(self.url);

        if let Some(start_date) = &self.start_date {
            url = format!("{}/{}", url, start_date.format("%Y-%m-%d"));

            if let Some(days) = self.days {
                url = format!("{}/{}", url, days);
            }
        }

        let mut req = self.client.builder(Method::GET, url);

        if let Some(access_token) = self.access_token {
            req = req.header("Authorization", format!("Bearer {}", access_token));
        }

        if !self.query.is_empty() {
            req = req.query(&self.query);
        }

        req.build().map_err(Error::from)
    }

    /// Execute this request
    pub fn execute(self) -> Result<Vec<T>> {
        match self.build() {
            Ok(req) => self.client.execute(req),
            Err(e) => Box::new(futures::future::err(e)),
        }
    }
}

impl<'a, T> WithFull for CalendarRequest<'a, T>
where
    T: DeserializeOwned + WithFull,
    T::Full: DeserializeOwned,
{
    type Full = CalendarRequest<'a, T::Full>;
}

impl<'a, T> WithNone for CalendarRequest<'a, T>
where
    T: DeserializeOwned + WithNone,
    T::None: DeserializeOwned,
{
    type None = CalendarRequest<'a, T::None>;
}

impl<'a, T> ExtendedInfoFull for CalendarRequest<'a, T>
where
    T: DeserializeOwned + WithFull,
    T::Full: DeserializeOwned,
{
    /// Request full extended info
    fn full(mut self) -> CalendarRequest<'a, T::Full> {
        self.query.insert("extended".to_owned(), "full".to_owned());

        CalendarRequest {
            client: self.client,
            url: self.url,
            start_date: self.start_date,
            days: self.days,
            access_token: self.access_token,
            query: self.query,
            response_type: PhantomData,
        }
    }
}

impl<'a, T> ExtendedInfoNone for CalendarRequest<'a, T>
where
    T: DeserializeOwned + WithNone,
    T::None: DeserializeOwned,
{
    /// Request no extended info
    fn none(mut self) -> CalendarRequest<'a, T::None> {
        self.query.remove("extended");

        CalendarRequest {
            client: self.client,
            url: self.url,
            start_date: self.start_date,
            days: self.days,
            access_token: self.access_token,
            query: self.query,
            response_type: PhantomData,
        }
    }
}

impl<'a, T: DeserializeOwned> Filters for CalendarRequest<'a, T> {
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

    fn certification(mut self, cert_slug: &str) -> Self {
        self.query
            .insert("certifications".to_owned(), cert_slug.to_owned());
        self
    }

    fn network(mut self, network_name: &str) -> Self {
        self.query
            .insert("networks".to_owned(), network_name.to_owned());
        self
    }

    fn status(mut self, status: ShowStatus) -> Self {
        self.query.insert("status".to_owned(), status.to_string());
        self
    }
}

impl<'a, T> PartialEq<CalendarRequest<'a, T>> for CalendarRequest<'a, T>
where
    T: DeserializeOwned,
{
    fn eq(&self, other: &CalendarRequest<'a, T>) -> bool {
        self.client == other.client
            && self.url == other.url
            && self.days == other.days
            && self.start_date == other.start_date
            && self.query == other.query
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        asyn::{requests::calendar::calendar_request::CalendarRequest, TraktApi},
        extended_info::ExtendedInfoFull,
        filters::Filters,
        models::{CalendarShow, FullCalendarShow},
    };
    use chrono::Utc;
    use std::{collections::HashMap, marker::PhantomData};

    #[test]
    fn calendar_request() {
        let api = TraktApi::staging("...".to_owned(), Some("...".to_owned()));

        let mut query = HashMap::new();
        query.insert("extended".to_owned(), "full".to_owned());
        query.insert("languages".to_owned(), "de".to_owned());
        query.insert("query".to_owned(), "tron".to_owned());
        query.insert("countries".to_owned(), "us".to_owned());

        assert_eq!(
            CalendarRequest::<CalendarShow>::new(&api.clone(), "some_url", None)
                .start_date(Utc::today())
                .days(3)
                .language("de")
                .query("tron")
                .country("us")
                .full(),
            CalendarRequest::<FullCalendarShow> {
                client: &api,
                url: "some_url",
                start_date: Some(Utc::today()),
                days: Some(3),
                access_token: None,
                query,
                response_type: PhantomData
            }
        )
    }
}
