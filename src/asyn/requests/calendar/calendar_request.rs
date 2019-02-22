use crate::{
    asyn::{Result, TraktApi},
    extended_info::{ExtendedInfoFull, ExtendedInfoNone, WithFull, WithNone},
};
use chrono::{Date, Utc};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

/// A request to a calendar endpoint
#[derive(Debug, Clone)]
pub struct CalendarRequest<'a, T: DeserializeOwned> {
    client: &'a TraktApi<'a>,
    url: &'a str,
    start_date: Option<Date<Utc>>,
    days: Option<u32>,
    access_token: Option<&'a str>,
    full: bool,
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
            full: false,
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

    /// Execute this request
    pub fn execute(self) -> Result<Vec<T>> {
        let mut url = self.url.to_owned();

        if let Some(start_date) = self.start_date {
            url = format!("{}/{}", url, start_date.format("%Y-%m-%d"));

            if let Some(days) = self.days {
                url = format!("{}/{}", url, days);
            }
        }

        if self.full {
            url.push_str("?extended=full");
        }

        match self.access_token {
            Some(access_token) => self
                .client
                .auth_get(api_url!(("calendars", url)), access_token),
            None => self.client.get(api_url!(("calendars", url))),
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
    fn full(self) -> CalendarRequest<'a, T::Full> {
        CalendarRequest {
            client: self.client,
            url: self.url,
            start_date: self.start_date,
            days: self.days,
            access_token: self.access_token,
            full: true,
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
    fn none(self) -> CalendarRequest<'a, T::None> {
        CalendarRequest {
            client: self.client,
            url: self.url,
            start_date: self.start_date,
            days: self.days,
            access_token: self.access_token,
            full: false,
            response_type: PhantomData,
        }
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
            && self.full == other.full
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        asyn::{requests::calendar::calendar_request::CalendarRequest, TraktApi},
        extended_info::ExtendedInfoFull,
        models::{CalendarShow, FullCalendarShow},
    };
    use chrono::Utc;
    use std::marker::PhantomData;

    #[test]
    fn calendar_request() {
        let api = TraktApi::staging("...".to_owned(), Some("...".to_owned()));

        assert_eq!(
            CalendarRequest::<CalendarShow>::new(&api.clone(), "some_url", None)
                .start_date(Utc::today())
                .days(3)
                .full(),
            CalendarRequest::<FullCalendarShow> {
                client: &api,
                url: "some_url",
                start_date: Some(Utc::today()),
                days: Some(3),
                access_token: None,
                full: true,
                response_type: PhantomData
            }
        )
    }

}
