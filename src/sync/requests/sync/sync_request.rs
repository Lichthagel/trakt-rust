use crate::{
    selectors::{SelectEpisode, SelectMovie, SelectSeason, SelectShow},
    sync::Result,
    TraktApi,
};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use std::marker::PhantomData;

/// A struct for making changes to your watchlist, collection, ratings etc.
///
/// # Example
///
/// ```rust,ignore
/// extern crate chrono;
///
/// use chrono::{offset::TimeZone, Utc};
/// use trakt::{selectors::*, TraktApi};
///
/// fn main() {
///     let api = TraktApi::new(
///         env!("CLIENT_ID").to_owned(),
///         None,
///     );
///
///     let access_token = "";
///
///     dbg!(api
///         .sync_ratings_add()
///         .movie(|m| m
///             .slug("warcraft-2016")
///             .rating(3)
///             .rated_at(Utc.ymd(2010, 10, 10).and_hms(10, 10, 10)))
///         .execute(access_token)
///         .unwrap());
/// }
/// ```
///
/// **Note**: [TraktApi.sync_ratings_add()] returns SyncRequest<SyncAddResponse>
///
/// [TraktApi.sync_ratings_add()]: ../../struct.TraktApi.html#method.sync_ratings_add
pub struct SyncRequest<'a, R: DeserializeOwned> {
    movies: Vec<Value>,
    shows: Vec<Value>,
    seasons: Vec<Value>,
    episodes: Vec<Value>,
    url: String,
    client: &'a TraktApi<'a>,
    response_type: PhantomData<R>,
}

impl<'a, R: DeserializeOwned> SyncRequest<'a, R> {
    pub fn new(url: String, client: &'a TraktApi) -> Self {
        Self {
            movies: Vec::new(),
            shows: Vec::new(),
            seasons: Vec::new(),
            episodes: Vec::new(),
            url,
            client,
            response_type: PhantomData,
        }
    }

    pub fn execute(self, access_token: &str) -> Result<R> {
        let mut obj = Map::new();
        obj.insert("movies".to_owned(), Value::Array(self.movies));
        obj.insert("shows".to_owned(), Value::Array(self.shows));
        obj.insert("seasons".to_owned(), Value::Array(self.seasons));
        obj.insert("episodes".to_owned(), Value::Array(self.episodes));
        let body = Value::Object(obj);

        self.client
            .auth_post(self.url, serde_json::to_string(&body)?, access_token)
    }
}

impl<'a, R: DeserializeOwned> SelectMovie for SyncRequest<'a, R> {
    fn movie_value(mut self, movie: Value) -> Self {
        self.movies.push(movie);
        self
    }
}

impl<'a, R: DeserializeOwned> SelectShow for SyncRequest<'a, R> {
    fn show_value(mut self, show: Value) -> Self {
        self.shows.push(show);
        self
    }
}

impl<'a, R: DeserializeOwned> SelectSeason for SyncRequest<'a, R> {
    fn season_value(mut self, season: Value) -> Self {
        self.seasons.push(season);
        self
    }
}

impl<'a, R: DeserializeOwned> SelectEpisode for SyncRequest<'a, R> {
    fn episode_value(mut self, episode: Value) -> Self {
        self.episodes.push(episode);
        self
    }
}
