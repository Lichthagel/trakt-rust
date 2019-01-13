#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod macros;
pub mod models;

use crate::models::{
    CalendarMovie,
    CalendarShow,
    Certifications,
    CertificationsType,
};
use chrono::{Date, Utc};
use reqwest::{Error, Response};

#[derive(Debug)]
pub struct TraktApi {
    client: reqwest::Client,
    client_id: String,
    client_secret: String,
}

impl TraktApi {
    pub fn new(client_id: String, client_secret: String) -> TraktApi {
        TraktApi {
            client: reqwest::Client::new(),
            client_id,
            client_secret,
        }
    }

    pub fn certifications(
        &self,
        ct: CertificationsType,
    ) -> Result<(Response, Option<Certifications>), Error> {
        api_request!(
        self.client,
        self.client_id.as_str(),
        api_route!("certifications", ct.to_string())
        )
    }

    pub fn calendar_all_shows(
        &self,
        start_date: Date<Utc>,
        days: u32,
    ) -> Result<(Response, Option<Vec<CalendarShow>>), Error> {
        api_request!(
            self.client,
            self.client_id.as_str(),
            api_route!("calendars/all/shows", start_date.format("%Y-%m-%d"), days)
        )
    }

    pub fn calendar_all_new_shows(
        &self,
        start_date: Date<Utc>,
        days: u32,
    ) -> Result<(Response, Option<Vec<CalendarShow>>), Error> {
        api_request!(
            self.client,
            self.client_id.as_str(),
            api_route!(
                "calendars/all/shows/new",
                start_date.format("%Y-%m-%d"),
                days
            )
        )
    }

    pub fn calendar_all_season_premieres(
        &self,
        start_date: Date<Utc>,
        days: u32,
    ) -> Result<(Response, Option<Vec<CalendarShow>>), Error> {
        api_request!(
            self.client,
            self.client_id.as_str(),
            api_route!(
                "calendars/all/shows/premieres",
                start_date.format("%Y-%m-%d"),
                days
            )
        )
    }

    pub fn calendar_all_movies(
        &self,
        start_date: Date<Utc>,
        days: u32,
    ) -> Result<(Response, Option<Vec<CalendarMovie>>), Error> {
        api_request!(
            self.client,
            self.client_id.as_str(),
            api_route!("calendars/all/movies", start_date.format("%Y-%m-%d"), days)
        )
    }

    pub fn calendar_all_dvd(
        &self,
        start_date: Date<Utc>,
        days: u32,
    ) -> Result<(Response, Option<Vec<CalendarMovie>>), Error> {
        api_request!(
            self.client,
            self.client_id.as_str(),
            api_route!("calendars/all/dvd", start_date.format("%Y-%m-%d"), days)
        )
    }
}

impl PartialEq for TraktApi {
    fn eq(&self, other: &TraktApi) -> bool {
        self.client_id == other.client_id && self.client_secret == other.client_secret
    }

    fn ne(&self, other: &TraktApi) -> bool {
        self.client_id != other.client_id || self.client_secret != other.client_secret
    }
}

#[cfg(test)]
mod tests {
    use crate::TraktApi;

    #[test]
    fn new_trakt_api() {
        assert_eq!(
            TraktApi {
                client: reqwest::Client::new(),
                client_id: String::from("abc"),
                client_secret: String::from("def"),
            },
            TraktApi::new(String::from("abc"), String::from("def"))
        );
    }
}
