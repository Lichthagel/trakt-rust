use crate::{
    asyn::{Result, TraktApi},
    error::Error,
    selectors::{SelectEpisode, SelectMovie, SelectSeason, SelectShow},
};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use std::marker::PhantomData;

/// A struct for making changes to your watchlist, collection, ratings etc.
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

impl<'a, R: DeserializeOwned + Send + 'static> SyncRequest<'a, R> {
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

        match serde_json::to_string(&body) {
            Ok(body) => self.client.auth_post(self.url, body, access_token),
            Err(e) => Box::new(futures::future::err(Error::from(e))),
        }
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
