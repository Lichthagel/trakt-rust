use crate::{
    error::{Error, Result},
    models::{
        Episode, Movie, OptionEpisode, OptionMovie, OptionSeason, OptionShow, Season, Show,
        WatchableType,
    },
};
use chrono::{DateTime, Utc};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LastActivities {
    all: DateTime<Utc>,
    movies: LastActivitiesElement,
    episodes: LastActivitiesElement,
    shows: LastActivitiesElement,
    seasons: LastActivitiesElement,
    comments: LastActivitiesElement,
    lists: LastActivitiesElement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastActivitiesElement {
    watched_at: Option<DateTime<Utc>>,
    collected_at: Option<DateTime<Utc>>,
    rated_at: Option<DateTime<Utc>>,
    watchlisted_at: Option<DateTime<Utc>>,
    commented_at: Option<DateTime<Utc>>,
    paused_at: Option<DateTime<Utc>>,
    hidden_at: Option<DateTime<Utc>>,
    liked_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playback {
    progress: f64,
    paused_at: Option<DateTime<Utc>>,
    id: u64,
    #[serde(rename = "type")]
    item_type: WatchableType,
    movie: Option<Movie>,
    episode: Option<Episode>,
    show: Option<Show>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SyncType {
    Watch,
    Collect,
}

impl SyncType {
    pub fn get_date_name(&self) -> String {
        match self {
            SyncType::Watch => "watched_at",
            SyncType::Collect => "collected_at",
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

    pub fn movie(mut self, movie: Movie) -> Result<Self> {
        self.movies.push(serde_json::to_value(movie)?);
        Ok(self)
    }

    pub fn movie_at(mut self, movie: Movie, collected_at: DateTime<Utc>) -> Result<Self> {
        let movie = serde_json::to_value(movie)?;
        let mut movie = movie.as_object().ok_or(Error::NoneError)?.clone();

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.movies.push(Value::Object(movie));

        Ok(self)
    }

    pub fn movie_id(mut self, trakt_id: u64) -> Self {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        self.movies.push(Value::Object(movie));
        self
    }

    pub fn movie_id_at(mut self, trakt_id: u64, collected_at: DateTime<Utc>) -> Self {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.movies.push(Value::Object(movie));
        self
    }

    pub fn movie_slug(mut self, trakt_slug: String) -> Self {
        let mut ids = Map::new();
        ids.insert("slug".to_owned(), Value::String(trakt_slug));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        self.movies.push(Value::Object(movie));
        self
    }

    pub fn movie_slug_at(mut self, trakt_slug: String, collected_at: DateTime<Utc>) -> Self {
        let mut ids = Map::new();
        ids.insert("slug".to_owned(), Value::String(trakt_slug));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.movies.push(Value::Object(movie));
        self
    }

    pub fn movie_imdb(mut self, imdb_id: String) -> Self {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        self.movies.push(Value::Object(movie));
        self
    }

    pub fn movie_imdb_at(mut self, imdb_id: String, collected_at: DateTime<Utc>) -> Self {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.movies.push(Value::Object(movie));
        self
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

pub struct SyncShowFactory {
    seasons: Vec<SyncSeasonFactory>,
    sync_type: SyncType,
}

impl SyncShowFactory {
    fn new(sync_type: SyncType) -> Self {
        Self {
            seasons: Vec::new(),
            sync_type,
        }
    }

    fn build(mut self, mut show: Map<String, Value>) -> Value {
        let mut seasons: Vec<Value> = self.seasons.iter_mut().map(|i| i.build()).collect();

        seasons.dedup();

        show.insert("seasons".to_owned(), Value::Array(seasons));

        Value::Object(show)
    }

    pub fn season(
        mut self,
        season_number: u32,
        f: impl Fn(SyncSeasonFactory) -> SyncSeasonFactory,
    ) -> Self {
        self.seasons.push(f(SyncSeasonFactory::new(
            season_number,
            None,
            self.sync_type.clone(),
        )));
        self
    }

    pub fn season_at(
        mut self,
        season_number: u32,
        collected_at: DateTime<Utc>,
        f: impl Fn(&mut SyncSeasonFactory),
    ) -> Self {
        let mut s =
            SyncSeasonFactory::new(season_number, Some(collected_at), self.sync_type.clone());
        f(&mut s);
        self.seasons.push(s);
        self
    }
}

pub struct SyncSeasonFactory {
    number: u32,
    collected_at: Option<DateTime<Utc>>,
    episodes: HashMap<u32, Option<DateTime<Utc>>>,
    sync_type: SyncType,
}

impl SyncSeasonFactory {
    fn new(season_number: u32, collected_at: Option<DateTime<Utc>>, sync_type: SyncType) -> Self {
        Self {
            number: season_number,
            collected_at,
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
        if let Some(date) = self.collected_at {
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

    pub fn episode_at(mut self, episode: u32, collected_at: DateTime<Utc>) -> Self {
        self.episodes.insert(episode, Some(collected_at));
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncAddResponse {
    added: SyncResponseNumbers,
    updated: Option<SyncResponseNumbers>,
    existing: Option<SyncResponseNumbers>,
    not_found: SyncResponseNotFound,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRemoveResponse {
    deleted: SyncResponseNumbers,
    not_found: SyncResponseNotFound,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResponseNumbers {
    movies: u32,
    episodes: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResponseNotFound {
    movies: Vec<OptionMovie>,
    shows: Vec<OptionShow>,
    seasons: Vec<OptionSeason>,
    episodes: Vec<OptionEpisode>,
}
