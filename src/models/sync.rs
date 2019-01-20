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

pub struct SyncRequest {
    movies: Vec<Value>,
    shows: Vec<Value>,
    seasons: Vec<Value>,
    episodes: Vec<Value>,
    sync_type: SyncType,
}

impl SyncRequest {
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

    pub fn movie(&mut self, movie: Movie) -> Result<()> {
        self.movies.push(serde_json::to_value(movie)?);
        Ok(())
    }

    pub fn movie_at(&mut self, movie: Movie, collected_at: DateTime<Utc>) -> Result<()> {
        let movie = serde_json::to_value(movie)?;
        let mut movie = movie.as_object().ok_or(Error::NoneError)?.clone();

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.movies.push(Value::Object(movie));

        Ok(())
    }

    pub fn movie_id(&mut self, trakt_id: u64) {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        self.movies.push(Value::Object(movie))
    }

    pub fn movie_id_at(&mut self, trakt_id: u64, collected_at: DateTime<Utc>) {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.movies.push(Value::Object(movie))
    }

    pub fn movie_slug(&mut self, trakt_slug: String) {
        let mut ids = Map::new();
        ids.insert("slug".to_owned(), Value::String(trakt_slug));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        self.movies.push(Value::Object(movie))
    }

    pub fn movie_slug_at(&mut self, trakt_slug: String, collected_at: DateTime<Utc>) {
        let mut ids = Map::new();
        ids.insert("slug".to_owned(), Value::String(trakt_slug));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.movies.push(Value::Object(movie))
    }

    pub fn movie_imdb(&mut self, imdb_id: String) {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        self.movies.push(Value::Object(movie))
    }

    pub fn movie_imdb_at(&mut self, imdb_id: String, collected_at: DateTime<Utc>) {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.movies.push(Value::Object(movie))
    }

    pub fn show(&mut self, show: Show, f: impl Fn(&mut SyncRequestShow)) -> Result<()> {
        let show = serde_json::to_value(show)?;
        let show = show.as_object().ok_or(Error::NoneError)?.clone();

        let mut pshow = SyncRequestShow::new(self.sync_type.clone());

        f(&mut pshow);

        self.shows.push(pshow.build(show));
        Ok(())
    }

    pub fn show_id(&mut self, trakt_id: u64, f: impl Fn(&mut SyncRequestShow)) {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut show = Map::new();
        show.insert("ids".to_owned(), Value::Object(ids));

        let mut pshow = SyncRequestShow::new(self.sync_type.clone());

        f(&mut pshow);

        self.shows.push(pshow.build(show))
    }

    pub fn show_slug(&mut self, trakt_slug: String, f: impl Fn(&mut SyncRequestShow)) {
        let mut ids = Map::new();
        ids.insert("slug".to_owned(), Value::String(trakt_slug));

        let mut show = Map::new();
        show.insert("ids".to_owned(), Value::Object(ids));

        let mut pshow = SyncRequestShow::new(self.sync_type.clone());

        f(&mut pshow);

        self.shows.push(pshow.build(show))
    }

    pub fn show_imdb(&mut self, imdb_id: String, f: impl Fn(&mut SyncRequestShow)) {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut show = Map::new();
        show.insert("ids".to_owned(), Value::Object(ids));

        let mut pshow = SyncRequestShow::new(self.sync_type.clone());

        f(&mut pshow);

        self.shows.push(pshow.build(show))
    }

    pub fn season(&mut self, season: Season) -> Result<()> {
        self.seasons.push(serde_json::to_value(season)?);
        Ok(())
    }

    pub fn season_at(&mut self, season: Season, collected_at: DateTime<Utc>) -> Result<()> {
        let season = serde_json::to_value(season)?;
        let mut season = season.as_object().ok_or(Error::NoneError)?.clone();

        season.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.seasons.push(Value::Object(season));
        Ok(())
    }

    pub fn season_id(&mut self, trakt_id: u64) {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        self.seasons.push(Value::Object(movie))
    }

    pub fn season_id_at(&mut self, trakt_id: u64, collected_at: DateTime<Utc>) {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.seasons.push(Value::Object(movie))
    }

    pub fn season_imdb(&mut self, imdb_id: String) {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        self.seasons.push(Value::Object(movie))
    }

    pub fn season_imdb_at(&mut self, imdb_id: String, collected_at: DateTime<Utc>) {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.seasons.push(Value::Object(movie))
    }

    pub fn episode(&mut self, episode: Episode) -> Result<()> {
        self.episodes.push(serde_json::to_value(episode)?);
        Ok(())
    }

    pub fn episode_at(&mut self, episode: Episode, collected_at: DateTime<Utc>) -> Result<()> {
        let episode = serde_json::to_value(episode)?;
        let mut episode = episode.as_object().ok_or(Error::NoneError)?.clone();

        episode.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.episodes.push(Value::Object(episode));
        Ok(())
    }

    pub fn episode_id(&mut self, trakt_id: u64) {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        self.episodes.push(Value::Object(movie))
    }

    pub fn episode_id_at(&mut self, trakt_id: u64, collected_at: DateTime<Utc>) {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.episodes.push(Value::Object(movie))
    }

    pub fn episode_imdb(&mut self, imdb_id: String) {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        self.episodes.push(Value::Object(movie))
    }

    pub fn episode_imdb_at(&mut self, imdb_id: String, collected_at: DateTime<Utc>) {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            self.sync_type.get_date_name(),
            Value::String(collected_at.to_string()),
        );

        self.episodes.push(Value::Object(movie))
    }
}

pub struct SyncRequestShow {
    seasons: Vec<SyncRequestSeason>,
    sync_type: SyncType,
}

impl SyncRequestShow {
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

    pub fn season(&mut self, season_number: u32, f: impl Fn(&mut SyncRequestSeason)) {
        let mut s = SyncRequestSeason::new(season_number, None, self.sync_type.clone());
        f(&mut s);
        self.seasons.push(s)
    }

    pub fn season_at(
        &mut self,
        season_number: u32,
        collected_at: DateTime<Utc>,
        f: impl Fn(&mut SyncRequestSeason),
    ) {
        let mut s =
            SyncRequestSeason::new(season_number, Some(collected_at), self.sync_type.clone());
        f(&mut s);
        self.seasons.push(s)
    }
}

pub struct SyncRequestSeason {
    number: u32,
    collected_at: Option<DateTime<Utc>>,
    episodes: HashMap<u32, Option<DateTime<Utc>>>,
    sync_type: SyncType,
}

impl SyncRequestSeason {
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

    pub fn episode(&mut self, episode: u32) {
        self.episodes.insert(episode, None);
    }

    pub fn episode_at(&mut self, episode: u32, collected_at: DateTime<Utc>) {
        self.episodes.insert(episode, Some(collected_at));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncAddResponse {
    added: SyncResponseNumbers,
    updated: SyncResponseNumbers,
    existing: SyncResponseNumbers,
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
