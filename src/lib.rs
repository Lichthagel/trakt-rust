#[macro_use]
extern crate serde_derive;
extern crate chrono;

mod models;
mod constants;

use crate::{
    models::calendar::CalendarShow,
    constants::API_URL
};
use chrono::{
    DateTime,
    Utc
};

#[derive(Debug)]
pub struct TraktApi {
    client: reqwest::Client,
    client_id: String,
    client_secret: String
}

impl TraktApi {
    pub fn new(client_id: String, client_secret: String) -> TraktApi {
        TraktApi {
            client: reqwest::Client::new(),
            client_id,
            client_secret
        }
    }

    pub fn all_shows(&self, timespan: Option<(DateTime<Utc>, u32)>) -> CalendarShow {
        self.client
            .get(format!("{}/calendars/all/shows", API_URL).as_str())
            .send()
            .map(|mut res| {
                if res.status().is_success() {
                    let text = res.text().unwrap();
                }
            });
        unimplemented!()
    }

}

impl PartialEq for TraktApi {
    fn eq(&self, other: &TraktApi) -> bool {
        self.client_id == other.client_id && self.client_secret == other.client_secret
    }

    fn ne(&self, other: &TraktApi) -> bool {
        self.client_id != other.client_id ||self.client_secret != other.client_secret
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
