use crate::selectors::{SelectEpisode, SelectMovie, SelectSeason, SelectShow};
use serde_json::{Map, Value};

pub struct SyncFactory {
    movies: Vec<Value>,
    shows: Vec<Value>,
    seasons: Vec<Value>,
    episodes: Vec<Value>,
}

impl SyncFactory {
    pub fn new() -> Self {
        Self {
            movies: Vec::new(),
            shows: Vec::new(),
            seasons: Vec::new(),
            episodes: Vec::new(),
        }
    }

    pub fn build(self) -> Value {
        let mut obj = Map::new();
        obj.insert("movies".to_owned(), Value::Array(self.movies));
        obj.insert("shows".to_owned(), Value::Array(self.shows));
        obj.insert("seasons".to_owned(), Value::Array(self.seasons));
        obj.insert("episodes".to_owned(), Value::Array(self.episodes));
        Value::Object(obj)
    }
}

impl SelectMovie for SyncFactory {
    fn movie_v(mut self, movie: Value) -> Self {
        self.movies.push(movie);
        self
    }
}

impl SelectShow for SyncFactory {
    fn show_v(mut self, show: Value) -> Self {
        self.shows.push(show);
        self
    }
}

impl SelectSeason for SyncFactory {
    fn season_v(mut self, season: Value) -> Self {
        self.seasons.push(season);
        self
    }
}

impl SelectEpisode for SyncFactory {
    fn episode_v(mut self, episode: Value) -> Self {
        self.episodes.push(episode);
        self
    }
}
