#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;

mod macros;
mod constants;
mod models;

use crate::{constants::API_URL, models::calendar::CalendarShow};
use chrono::{DateTime, Utc, Date};
use reqwest::Error;
use reqwest::Response;

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

    pub fn all_shows(&self, start_date: Date<Utc>, days: u32) -> Result<(Response, Option<Vec<CalendarShow>>), Error> {
        self.client
            .get(format!("{}/calendars/all/shows/{}/{}", API_URL, start_date.format("%Y-%m-%d"), days).as_str())
            .send()
            .map(|mut res| {
                println!("{:?}", res);
                if res.status().is_success() {
                    let text = res.text().unwrap();
                    (res, Some(serde_json::from_str(text.as_str()).unwrap()))
                } else {
                    (res, None)
                }
            })
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
                client_secret: String::from("def")
            },
            TraktApi::new(String::from("abc"), String::from("def"))
        );
    }
}
