#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod macros;
pub mod error;
pub mod models;

use crate::{
    error::Error,
    models::{
        AllCommentableItemType, AuthenticationDevices, AuthenticationTokenResponse, CalendarMovie,
        CalendarShow, Certifications, CertificationsType, Comment, CommentAndItem, CommentItem,
        CommentType, Like,
    },
};
use chrono::{Date, Utc};
use serde::de::DeserializeOwned;
use serde_json::json;

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

    fn get<T: DeserializeOwned>(&self, url: String) -> Result<T, Error> {
        match self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(serde_json::from_reader(res).unwrap())
                } else {
                    Err(Error::Response(res))
                }
            }
            Err(e) => Err(Error::Connection(e)),
        }
    }

    fn post<T: DeserializeOwned>(&self, url: String, body: String) -> Result<T, Error> {
        match self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .body(body)
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(serde_json::from_reader(res).unwrap())
                } else {
                    Err(Error::Response(res))
                }
            }
            Err(e) => Err(Error::Connection(e)),
        }
    }

    pub fn authenticate_devices(&self) -> Result<AuthenticationDevices, Error> {
        self.post(
            api_route!("oauth/device/code"),
            json!({"client_id": self.client_id}).to_string(),
        )
    }

    pub fn get_token(&self, device_code: String) -> Result<AuthenticationTokenResponse, Error> {
        self.post(
            api_route!("oauth/device/token"),
            json!({
                "code": device_code,
                "client_id": self.client_id,
                "client_secret": self.client_secret
            })
            .to_string(),
        )
    }

    pub fn certifications(&self, ct: CertificationsType) -> Result<Certifications, Error> {
        self.get(api_route!("certifications", ct.to_string()))
    }

    pub fn calendar_all_shows(
        &self,
        start_date: Date<Utc>,
        days: u32,
    ) -> Result<Vec<CalendarShow>, Error> {
        self.get(api_route!(
            "calendars",
            "all",
            "shows",
            start_date.format("%Y-%m-%d"),
            days
        ))
    }

    pub fn calendar_all_new_shows(
        &self,
        start_date: Date<Utc>,
        days: u32,
    ) -> Result<Vec<CalendarShow>, Error> {
        self.get(api_route!(
            "calendars",
            "all",
            "shows",
            "new",
            start_date.format("%Y-%m-%d"),
            days
        ))
    }

    pub fn calendar_all_season_premieres(
        &self,
        start_date: Date<Utc>,
        days: u32,
    ) -> Result<Vec<CalendarShow>, Error> {
        self.get(api_route!(
            "calendars",
            "all",
            "shows",
            "premieres",
            start_date.format("%Y-%m-%d"),
            days
        ))
    }

    pub fn calendar_all_movies(
        &self,
        start_date: Date<Utc>,
        days: u32,
    ) -> Result<Vec<CalendarMovie>, Error> {
        self.get(api_route!(
            "calendars",
            "all",
            "movies",
            start_date.format("%Y-%m-%d"),
            days
        ))
    }

    pub fn calendar_all_dvd(
        &self,
        start_date: Date<Utc>,
        days: u32,
    ) -> Result<Vec<CalendarMovie>, Error> {
        self.get(api_route!(
            "calendars",
            "all",
            "dvd",
            start_date.format("%Y-%m-%d"),
            days
        ))
    }

    pub fn comments(&self, id: u32) -> Result<Comment, Error> {
        self.get(api_route!("comments", id))
    }

    pub fn replies(&self, comment_id: u32, page: u32, limit: u32) -> Result<Vec<Comment>, Error> {
        self.get(api_pagination!(
            api_route!("comments", comment_id, "replies"),
            page,
            limit
        ))
    }

    pub fn comment_item(&self, comment_id: u32) -> Result<CommentItem, Error> {
        self.get(api_route!("comments", comment_id, "item"))
    }

    pub fn comment_likes(
        &self,
        comment_id: u32,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Like>, Error> {
        self.get(api_pagination!(
            api_route!("comments", comment_id, "likes"),
            page,
            limit
        ))
    }

    pub fn comments_trending(
        &self,
        comment_type: CommentType,
        item_type: AllCommentableItemType,
        include_replies: bool,
        page: u32,
        limit: u32,
    ) -> Result<Vec<CommentAndItem>, Error> {
        self.get(format!(
            "{}&include_replies={}",
            api_pagination!(
                api_route!(
                    "comments",
                    "trending",
                    comment_type.to_string(),
                    item_type.to_string()
                ),
                page,
                limit
            ),
            include_replies
        ))
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
