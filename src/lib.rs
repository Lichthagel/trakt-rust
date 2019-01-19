#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod macros;
pub mod error;
pub mod models;

use chrono::{Date, Utc};
use serde::de::DeserializeOwned;
use serde_json::json;
use crate::{
    models::{
        Genre,
        Country,
        AllCommentableItemType,
        AuthenticationDevices,
        AuthenticationTokenResponse,
        CalendarMovie,
        CalendarShow,
        Certifications,
        CertificationsType,
        Comment,
        CommentAndItem,
        CommentItem,
        CommentType,
        Like,
        MediaType,
        Language,
        ListInfo,
        MovieInfo,
        Movie,
        WatchedMovie,
        TimePeriod
    },
    error::Error,
};

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
        self.get(api_parameter!(
            api_route!("comments", comment_id, "replies"),
            ("page", page),
            ("limit",limit)
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
        self.get(api_parameter!(
            api_route!("comments", comment_id, "likes"),
            ("page", page),
            ("limit",limit)
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
        self.get(api_parameter!(
            api_route!(
                "comments",
                "trending",
                comment_type.to_string(),
                item_type.to_string()
            ),
            ("page", page),
            ("limit", limit),
            ("include_replies", include_replies)
        ))
    }

    pub fn comments_recent(
        &self,
        comment_type: CommentType,
        item_type: AllCommentableItemType,
        include_replies: bool,
        page: u32,
        limit: u32,
    ) -> Result<Vec<CommentAndItem>, Error> {
        self.get(api_parameter!(
            api_route!(
                "comments",
                "recent",
                comment_type.to_string(),
                item_type.to_string()
            ),
            ("page", page),
            ("limit",limit),
            ("include_replies", include_replies)
        ))
    }

    pub fn comments_updates(
        &self,
        comment_type: CommentType,
        item_type: AllCommentableItemType,
        include_replies: bool,
        page: u32,
        limit: u32,
    ) -> Result<Vec<CommentAndItem>, Error> {
        self.get(api_parameter!(
            api_route!(
                "comments",
                "updates",
                comment_type.to_string(),
                item_type.to_string()
            ),
            ("page", page),
            ("limit",limit),
            ("include_replies", include_replies)
         ))
    }

    pub fn countries(
        &self,
        media_type: MediaType
    ) -> Result<Vec<Country>, Error> {
        self.get(api_route!("countries", media_type.to_string()))
    }

    pub fn genres(
        &self,
        media_type: MediaType
    ) -> Result<Vec<Genre>, Error> {
        self.get(api_route!("genres", media_type.to_string()))
    }

    pub fn languages(
        &self,
        media_type: MediaType
    ) -> Result<Vec<Language>, Error> {
        self.get(api_route!("languages", media_type.to_string()))
    }

    pub fn lists_trending(
        &self,
        page: u32,
        limit: u32
    ) -> Result<Vec<ListInfo>, Error> {
        self.get(api_parameter!(
            api_route!("lists", "trending"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn lists_popular(
        &self,
        page: u32,
        limit: u32
    ) -> Result<Vec<ListInfo>, Error> {
        self.get(api_parameter!(
            api_route!("lists", "popular"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movies_trending(
        &self,
        page: u32,
        limit: u32
    ) -> Result<Vec<MovieInfo>, Error> {
        self.get(api_parameter!(
            api_route!("movies", "trending"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movies_popular(
        &self,
        page: u32,
        limit: u32
    ) -> Result<Vec<Movie>, Error> {
        self.get(api_parameter!(
            api_route!("movies", "popular"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movies_played(
        &self,
        page: u32,
        limit: u32,
        period: TimePeriod
    ) -> Result<Vec<WatchedMovie>, Error> {
        self.get(api_parameter!(
            api_route!("movies", "played", period.to_string()),
            ("page", page),
            ("limit", limit)
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
