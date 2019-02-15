use crate::{
    asyn::{Result, TraktApi},
    error::Error,
    models::{Episode, Movie, Show},
    selectors::{SelectEpisode, SelectMovie, SelectShow},
};
use chrono::{DateTime, NaiveDate, Utc};
use futures::future::Future;
use serde_json::{Map, Value};

/// A struct for creating a checkin. [More]
///
/// # Example
///
/// ```rust,no_run
/// use trakt::{
///     selectors::{MovieSelector, SelectIds, SelectMovie},
///     TraktApi,
/// };
///
/// fn main() {
///     let api = TraktApi::new(
///         "...".to_owned(),
///         Some("...".to_owned()),
///     );
///
///     let access_token = "...";
///
///     dbg!(api
///         .checkin()
///         .movie(|m| m.slug("warcraft-2016"))
///         .execute(access_token)
///         .unwrap());
/// }
/// ```
///
/// [More]: https://trakt.docs.apiary.io/#reference/checkin/checkin/check-into-an-item
#[derive(Debug)]
pub struct Checkin<'a> {
    pub client: &'a TraktApi,
    pub body: Map<String, Value>,
    pub sharing: CheckinSharing,
}

impl<'a> Checkin<'a> {
    fn new(client: &'a TraktApi) -> Self {
        Self {
            client,
            body: Map::new(),
            sharing: CheckinSharing::new(false, false, false),
        }
    }

    /// Share checkin on twitter
    pub fn twitter(mut self) -> Self {
        self.sharing.twitter = true;
        self
    }

    /// Share checkin on tumblr
    pub fn tumblr(mut self) -> Self {
        self.sharing.tumblr = true;
        self
    }

    /// Share checkin on facebook
    pub fn facebook(mut self) -> Self {
        self.sharing.facebook = true;
        self
    }

    /// Set the message of a checkin
    pub fn message(mut self, message: &str) -> Self {
        self.body
            .insert("message".to_owned(), Value::String(message.to_owned()));
        self
    }

    /// Set the app version in a checkin
    pub fn app_version(mut self, app_version: &str) -> Self {
        self.body.insert(
            "app_version".to_owned(),
            Value::String(app_version.to_owned()),
        );
        self
    }

    /// Set the app build date in a checkin
    ///
    /// Panics if app_date can't be serialized
    pub fn app_date(mut self, app_date: NaiveDate) -> Self {
        self.body.insert(
            "app_date".to_owned(),
            serde_json::to_value(app_date).unwrap(),
        );
        self
    }

    /// Executes the checkin
    ///
    /// Panics if self.sharing or self.body can't be serialized
    pub fn execute(mut self, access_token: &str) -> Result<CheckinResponse> {
        self.body.insert(
            "sharing".to_owned(),
            serde_json::to_value(self.sharing).unwrap(),
        );

        self.client.auth_post(
            api_url!(("checkin")),
            serde_json::to_string(&self.body).unwrap(),
            access_token,
        )
    }
}

/// Select the movie of a checkin
impl<'a> SelectMovie for Checkin<'a> {
    fn movie_v(mut self, movie: Value) -> Self {
        self.body.insert("movie".to_owned(), movie);
        self
    }
}

/// Select the episode of a checkin
impl<'a> SelectEpisode for Checkin<'a> {
    fn episode_v(mut self, episode: Value) -> Self {
        self.body.insert("episode".to_owned(), episode);
        self
    }
}

/// Select the show of a checkin
impl<'a> SelectShow for Checkin<'a> {
    fn show_v(mut self, show: Value) -> Self {
        self.body.insert("show".to_owned(), show);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckinSharing {
    pub twitter: bool,
    pub tumblr: bool,
    pub facebook: bool,
}

impl CheckinSharing {
    pub fn new(twitter: bool, tumblr: bool, facebook: bool) -> CheckinSharing {
        CheckinSharing {
            twitter,
            tumblr,
            facebook,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckinResponse {
    pub id: u64,
    pub watched_at: DateTime<Utc>,
    pub sharing: CheckinSharing,
    pub movie: Option<Movie>,
    pub episode: Option<Episode>,
    pub show: Option<Show>,
}

impl TraktApi {
    pub fn checkin(&self) -> Checkin {
        Checkin::new(self)
    }

    pub fn checkout(&self, access_token: &str) -> Result<()> {
        Box::new(
            self.client
                .delete(&api_url!(("checkin")))
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", access_token))
                .header("trakt-api-version", 2)
                .header("trakt-api-key", self.client_id.as_str())
                .send()
                .and_then(|mut res| res.json())
                .map_err(Error::from),
        )
    }
}