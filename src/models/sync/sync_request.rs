use crate::{
    error::{Error, Result},
    models::{Episode, Season, Show},
    selectors::{SelectMovie, SelectMovieData},
};
use chrono::{DateTime, Utc};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SyncType {
    Watch,
    Collect,
    Rating,
    Watchlist,
}

impl SyncType {
    pub fn get_date_name(&self) -> String {
        match self {
            SyncType::Watch => "watched_at",
            SyncType::Collect => "collected_at",
            _ => "",
        }
        .to_owned()
    }
}

pub struct SyncFactory {
    movies: Vec<Value>,
    shows: Vec<Value>,
    seasons: Vec<Value>,
    episodes: Vec<Value>,
    sync_type: SyncType,
}

impl SyncFactory {
    pub fn new(sync_type: SyncType) -> Self {
        Self {
            movies: Vec::new(),
            shows: Vec::new(),
            seasons: Vec::new(),
            episodes: Vec::new(),
            sync_type,
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

    pub fn show(
        mut self,
        show: Show,
        f: impl FnOnce(SyncShowFactory) -> SyncShowFactory,
    ) -> Result<Self> {
        let show = serde_json::to_value(show)?;
        let show = show.as_object().ok_or(Error::NoneError)?.clone();

        self.shows
            .push(f(SyncShowFactory::new(self.sync_type.clone())).build(show));
        Ok(self)
    }

    pub fn show_id(
        mut self,
        trakt_id: u64,
        f: impl Fn(SyncShowFactory) -> SyncShowFactory,
    ) -> Self {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut show = Map::new();
        show.insert("ids".to_owned(), Value::Object(ids));

        self.shows
            .push(f(SyncShowFactory::new(self.sync_type.clone())).build(show));
        self
    }

    pub fn show_slug(
        mut self,
        trakt_slug: String,
        f: impl Fn(SyncShowFactory) -> SyncShowFactory,
    ) -> Self {
        let mut ids = Map::new();
        ids.insert("slug".to_owned(), Value::String(trakt_slug));

        let mut show = Map::new();
        show.insert("ids".to_owned(), Value::Object(ids));

        self.shows
            .push(f(SyncShowFactory::new(self.sync_type.clone())).build(show));
        self
    }

    pub fn show_imdb(
        mut self,
        imdb_id: String,
        f: impl Fn(SyncShowFactory) -> SyncShowFactory,
    ) -> Self {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut show = Map::new();
        show.insert("ids".to_owned(), Value::Object(ids));

        self.shows
            .push(f(SyncShowFactory::new(self.sync_type.clone())).build(show));
        self
    }

    pub fn season(mut self, season: Season) -> Result<Self> {
        self.seasons.push(serde_json::to_value(season)?);
        Ok(self)
    }

    pub fn season_at(mut self, season: Season, collected_at: DateTime<Utc>) -> Result<Self> {
        let season = serde_json::to_value(season)?;
        let mut season = season.as_object().ok_or(Error::NoneError)?.clone();

        season.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.seasons.push(Value::Object(season));
        Ok(self)
    }

    pub fn season_id(mut self, trakt_id: u64) -> Self {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        self.seasons.push(Value::Object(movie));
        self
    }

    pub fn season_id_at(mut self, trakt_id: u64, collected_at: DateTime<Utc>) -> Self {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.seasons.push(Value::Object(movie));
        self
    }

    pub fn season_imdb(mut self, imdb_id: String) -> Self {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        self.seasons.push(Value::Object(movie));
        self
    }

    pub fn season_imdb_at(mut self, imdb_id: String, collected_at: DateTime<Utc>) -> Self {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.seasons.push(Value::Object(movie));
        self
    }

    pub fn episode(mut self, episode: Episode) -> Result<Self> {
        self.episodes.push(serde_json::to_value(episode)?);
        Ok(self)
    }

    pub fn episode_at(mut self, episode: Episode, collected_at: DateTime<Utc>) -> Result<Self> {
        let episode = serde_json::to_value(episode)?;
        let mut episode = episode.as_object().ok_or(Error::NoneError)?.clone();

        episode.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.episodes.push(Value::Object(episode));
        Ok(self)
    }

    pub fn episode_id(mut self, trakt_id: u64) -> Self {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        self.episodes.push(Value::Object(movie));
        self
    }

    pub fn episode_id_at(mut self, trakt_id: u64, collected_at: DateTime<Utc>) -> Self {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.episodes.push(Value::Object(movie));
        self
    }

    pub fn episode_imdb(mut self, imdb_id: String) -> Self {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        self.episodes.push(Value::Object(movie));
        self
    }

    pub fn episode_imdb_at(mut self, imdb_id: String, collected_at: DateTime<Utc>) -> Self {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.episodes.push(Value::Object(movie));
        self
    }
}

impl SelectMovie for SyncFactory {
    fn movie_v(mut self, movie: Value) -> Self {
        self.movies.push(movie);
        self
    }
}

impl SelectMovieData<DateTime<Utc>> for SyncFactory {
    fn movie_v_d(mut self, movie: Value, data: DateTime<Utc>) -> Self {
        let mut movie = movie.as_object().unwrap().clone();

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(data.to_string()),
        );

        self.movies.push(Value::Object(movie));
        self
    }
}

pub struct SyncShowFactory {
    seasons: Vec<SyncSeasonFactory>,
    sync_type: SyncType,
    date_at: Option<DateTime<Utc>>,
}

impl SyncShowFactory {
    fn new(sync_type: SyncType) -> Self {
        Self {
            seasons: Vec::new(),
            sync_type,
            date_at: None,
        }
    }

    fn build(mut self, mut show: Map<String, Value>) -> Value {
        let mut seasons: Vec<Value> = self.seasons.iter_mut().map(|i| i.build()).collect();

        seasons.dedup();

        show.insert("seasons".to_owned(), Value::Array(seasons));

        if let Some(date) = self.date_at {
            show.insert(
                self.sync_type.get_date_name(),
                Value::String(date.to_string()),
            );
        }

        Value::Object(show)
    }

    pub fn season(
        mut self,
        season_number: u32,
        f: impl Fn(SyncSeasonFactory) -> SyncSeasonFactory,
    ) -> Self {
        self.seasons.push(f(SyncSeasonFactory::new(
            season_number,
            self.sync_type.clone(),
        )));
        self
    }

    pub fn at(mut self, date_at: DateTime<Utc>) -> Self {
        self.date_at = Some(date_at);
        self
    }
}

pub struct SyncSeasonFactory {
    number: u32,
    date_at: Option<DateTime<Utc>>,
    episodes: HashMap<u32, Option<DateTime<Utc>>>,
    sync_type: SyncType,
}

impl SyncSeasonFactory {
    fn new(season_number: u32, sync_type: SyncType) -> Self {
        Self {
            number: season_number,
            date_at: None,
            episodes: HashMap::new(),
            sync_type,
        }
    }

    fn build(&mut self) -> Value {
        let mut season = Map::new();

        let mut episodes: Vec<Value> = self
            .episodes
            .iter()
            .map(|(num, date)| {
                let mut episode = Map::new();
                episode.insert("number".to_owned(), Value::Number(num.clone().into()));
                if let Some(date) = date {
                    episode.insert(
                        self.sync_type.get_date_name(),
                        Value::String(date.to_owned().to_string()),
                    );
                }
                Value::Object(episode)
            })
            .collect();

        episodes.dedup();

        season.insert("number".to_owned(), Value::Number(self.number.into()));
        season.insert("episodes".to_owned(), Value::Array(episodes));
        if let Some(date) = self.date_at {
            season.insert(
                self.sync_type.get_date_name(),
                Value::String(date.to_string()),
            );
        }

        Value::Object(season)
    }

    pub fn episode(mut self, episode: u32) -> Self {
        self.episodes.insert(episode, None);
        self
    }

    pub fn episode_at(mut self, episode: u32, date_at: DateTime<Utc>) -> Self {
        self.episodes.insert(episode, Some(date_at));
        self
    }

    pub fn at(mut self, date: DateTime<Utc>) -> Self {
        self.date_at = Some(date);
        self
    }
}
