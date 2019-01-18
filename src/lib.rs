#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod macros;
pub mod models;
pub mod error;

use crate::models::comment::CommentItem;
use crate::models::{
    AuthenticationDeviceGetToken, AuthenticationDeviceGetTokenResponse, AuthenticationDeviceId,
    AuthenticationDevices, CalendarMovie, CalendarShow, Certifications, CertificationsType,
    Comment,
    CommentItem,
    Like
};
use chrono::{Date, Utc};
use reqwest::{Error as ReqError, Response};
use std::option::Option::Some;
use std::option::Option::None;
use crate::error::Error;

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

    pub fn authenticate_devices(&self) -> Result<(Response, Option<AuthenticationDevices>), ReqError> {
        self.client
            .post(&api_route!("oauth/device/code"))
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .body(
                serde_json::to_string(&AuthenticationDeviceId {
                    client_id: self.client_id.clone(),
                })
                    .unwrap(),
            )
            .send()
            .map(|mut res| {
                if res.status().is_success() {
                    let text = res.text().unwrap();
                    (res, Some(serde_json::from_str(text.as_str()).unwrap()))
                } else {
                    (res, None)
                }
            })
    }

    pub fn get_token(
        &self,
        code: String,
    ) -> Result<(Response, Option<AuthenticationDeviceGetTokenResponse>), ReqError> {
        self.client
            .post(&api_route!("oauth/device/token"))
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .body(
                serde_json::to_string(&AuthenticationDeviceGetToken {
                    code,
                    client_id: self.client_id.clone(),
                    client_secret: self.client_secret.clone(),
                })
                    .unwrap(),
            )
            .send()
            .map(|mut res| {
                if res.status().is_success() {
                    let text = res.text().unwrap();
                    (res, Some(serde_json::from_str(text.as_str()).unwrap()))
                } else {
                    (res, None)
                }
            })
    }

    pub fn certifications(
        &self,
        ct: CertificationsType,
    ) -> Result<Certifications, Error> {
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
    ) -> Result<Vec<CalendarShow>, Error> {
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
    ) -> Result<Vec<CalendarShow>, Error> {
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
    ) -> Result<Vec<CalendarShow>, Error> {
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
    ) -> Result<Vec<CalendarMovie>, Error> {
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
    ) -> Result<Vec<CalendarMovie>, Error> {
        api_request!(
            self.client,
            self.client_id.as_str(),
            api_route!("calendars/all/dvd", start_date.format("%Y-%m-%d"), days)
        )
    }

    pub fn comments(&self, id: u32) -> Result<Comment, Error> {
        api_request!(
            self.client,
            self.client_id.as_str(),
            api_route!("comments", id)
        )
    }

    pub fn replies(
        &self,
        comment_id: u32,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Comment>, Error> {
        api_request!(
            self.client,
            self.client_id.as_str(),
            api_pagination!(api_route!("comments", comment_id, "replies"), page, limit)
        )
    }

    pub fn comment_item(&self, comment_id: u32) -> Result<CommentItem, Error> {
        api_request!(
            self.client,
            self.client_id.as_str(),
            api_route!("comments", comment_id, "item")
        )
    }

    pub fn comment_likes(
        &self,
        comment_id: u32,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Like>, Error> {
        api_request!(
            self.client,
            self.client_id.as_str(),
            api_pagination!(
                api_route!("comments", comment_id, "likes"),
                page,
                limit
            )
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
