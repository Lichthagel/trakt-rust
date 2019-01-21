use crate::{
    error::{Error, Result},
    models::{Episode, Movie, Season, Show},
};
use chrono::{DateTime, Utc};
use serde_json::{Map, Value};
use std::collections::HashMap;

pub struct RatingFactory {
    movies: Vec<Value>,
    shows: Vec<Value>,
    seasons: Vec<Value>,
    episodes: Vec<Value>,
}

impl RatingFactory {
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

    pub fn movie(mut self, movie: Movie, rating: u8) -> Result<Self> {
        let movie = serde_json::to_value(movie)?;
        let mut movie = movie.as_object().ok_or(Error::NoneError)?.clone();

        movie.insert("rating".to_owned(), Value::Number(rating.into()));

        self.movies.push(Value::Object(movie));
        Ok(self)
    }

    pub fn movie_at(
        mut self,
        movie: Movie,
        collected_at: DateTime<Utc>,
        rating: u8,
    ) -> Result<Self> {
        let movie = serde_json::to_value(movie)?;
        let mut movie = movie.as_object().ok_or(Error::NoneError)?.clone();

        movie.insert(
            "rated_at".to_owned(),
            Value::String(collected_at.to_string()),
        );

        movie.insert("rating".to_owned(), Value::Number(rating.into()));

        self.movies.push(Value::Object(movie));

        Ok(self)
    }

    pub fn movie_id(mut self, trakt_id: u64, rating: u8) -> Self {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert("rating".to_owned(), Value::Number(rating.into()));

        self.movies.push(Value::Object(movie));
        self
    }

    pub fn movie_id_at(mut self, trakt_id: u64, collected_at: DateTime<Utc>, rating: u8) -> Self {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            "rated_at".to_owned(),
            Value::String(collected_at.to_string()),
        );

        movie.insert("rating".to_owned(), Value::Number(rating.into()));

        self.movies.push(Value::Object(movie));
        self
    }

    pub fn movie_slug(mut self, trakt_slug: String, rating: u8) -> Self {
        let mut ids = Map::new();
        ids.insert("slug".to_owned(), Value::String(trakt_slug));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert("rating".to_owned(), Value::Number(rating.into()));

        self.movies.push(Value::Object(movie));
        self
    }

    pub fn movie_slug_at(
        mut self,
        trakt_slug: String,
        collected_at: DateTime<Utc>,
        rating: u8,
    ) -> Self {
        let mut ids = Map::new();
        ids.insert("slug".to_owned(), Value::String(trakt_slug));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            "rated_at".to_owned(),
            Value::String(collected_at.to_string()),
        );

        movie.insert("rating".to_owned(), Value::Number(rating.into()));

        self.movies.push(Value::Object(movie));
        self
    }

    pub fn movie_imdb(mut self, imdb_id: String, rating: u8) -> Self {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert("rating".to_owned(), Value::Number(rating.into()));

        self.movies.push(Value::Object(movie));
        self
    }

    pub fn movie_imdb_at(
        mut self,
        imdb_id: String,
        collected_at: DateTime<Utc>,
        rating: u8,
    ) -> Self {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut movie = Map::new();
        movie.insert("ids".to_owned(), Value::Object(ids));

        movie.insert(
            "rated_at".to_owned(),
            Value::String(collected_at.to_string()),
        );

        movie.insert("rating".to_owned(), Value::Number(rating.into()));

        self.movies.push(Value::Object(movie));
        self
    }

    pub fn show(
        mut self,
        show: Show,
        f: impl FnOnce(RatingShowFactory) -> RatingShowFactory,
    ) -> Result<Self> {
        let show = serde_json::to_value(show)?;
        let show = show.as_object().ok_or(Error::NoneError)?.clone();

        self.shows.push(f(RatingShowFactory::new()).build(show));
        Ok(self)
    }

    pub fn show_id(
        mut self,
        trakt_id: u64,
        f: impl Fn(RatingShowFactory) -> RatingShowFactory,
    ) -> Self {
        let mut ids = Map::new();
        ids.insert("trakt".to_owned(), Value::Number(trakt_id.into()));

        let mut show = Map::new();
        show.insert("ids".to_owned(), Value::Object(ids));

        self.shows.push(f(RatingShowFactory::new()).build(show));
        self
    }

    pub fn show_slug(
        mut self,
        trakt_slug: String,
        f: impl Fn(RatingShowFactory) -> RatingShowFactory,
    ) -> Self {
        let mut ids = Map::new();
        ids.insert("slug".to_owned(), Value::String(trakt_slug));

        let mut show = Map::new();
        show.insert("ids".to_owned(), Value::Object(ids));

        self.shows.push(f(RatingShowFactory::new()).build(show));
        self
    }

    pub fn show_imdb(
        mut self,
        imdb_id: String,
        f: impl Fn(RatingShowFactory) -> RatingShowFactory,
    ) -> Self {
        let mut ids = Map::new();
        ids.insert("imdb".to_owned(), Value::String(imdb_id));

        let mut show = Map::new();
        show.insert("ids".to_owned(), Value::Object(ids));

        self.shows.push(f(RatingShowFactory::new()).build(show));
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
            "rated_at".to_owned(),
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
            "rated_at".to_owned(),
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
            "rated_at".to_owned(),
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
            "rated_at".to_owned(),
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
            "rated_at".to_owned(),
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
            "rated_at".to_owned(),
            Value::String(collected_at.to_string()),
        );

        self.episodes.push(Value::Object(movie));
        self
    }
}

pub struct RatingShowFactory {
    seasons: Vec<RatingSeasonFactory>,
    rating: Option<u8>,
    rated_at: Option<DateTime<Utc>>,
}

impl RatingShowFactory {
    fn new() -> Self {
        Self {
            seasons: Vec::new(),
            rating: None,
            rated_at: None,
        }
    }

    fn build(mut self, mut show: Map<String, Value>) -> Value {
        let mut seasons: Vec<Value> = self.seasons.iter_mut().map(|i| i.build()).collect();

        seasons.dedup();

        show.insert("seasons".to_owned(), Value::Array(seasons));

        if let Some(rate) = self.rating {
            show.insert("rating".to_owned(), Value::Number(rate.into()));
        }

        if let Some(date) = self.rated_at {
            show.insert("rated_at".to_owned(), Value::String(date.to_string()));
        }

        Value::Object(show)
    }

    pub fn season(
        mut self,
        season_number: u32,
        f: impl Fn(RatingSeasonFactory) -> RatingSeasonFactory,
    ) -> Self {
        self.seasons
            .push(f(RatingSeasonFactory::new(season_number)));
        self
    }

    pub fn rating(mut self, rating: u8) -> Self {
        self.rating = Some(rating);
        self
    }

    pub fn rated_at(mut self, rated_at: DateTime<Utc>) -> Self {
        self.rated_at = Some(rated_at);
        self
    }

    pub fn rating_at(self, rating: u8, rated_at: DateTime<Utc>) -> Self {
        self.rating(rating).rated_at(rated_at)
    }
}

pub struct RatingSeasonFactory {
    number: u32,
    rated_at: Option<DateTime<Utc>>,
    rating: Option<u8>,
    episodes: HashMap<u32, (u8, Option<DateTime<Utc>>)>,
}

impl RatingSeasonFactory {
    fn new(season_number: u32) -> Self {
        Self {
            number: season_number,
            rated_at: None,
            rating: None,
            episodes: HashMap::new(),
        }
    }

    fn build(&mut self) -> Value {
        let mut season = Map::new();

        let mut episodes: Vec<Value> = self
            .episodes
            .iter()
            .map(|(num, (rating, date))| {
                let mut episode = Map::new();
                episode.insert("number".to_owned(), Value::Number(num.clone().into()));
                if let Some(date) = date {
                    episode.insert(
                        "rated_at".to_owned(),
                        Value::String(date.to_owned().to_string()),
                    );
                }

                episode.insert("rating".to_owned(), Value::Number(rating.clone().into()));

                Value::Object(episode)
            })
            .collect();

        episodes.dedup();

        season.insert("number".to_owned(), Value::Number(self.number.into()));
        season.insert("episodes".to_owned(), Value::Array(episodes));
        if let Some(date) = self.rated_at {
            season.insert("rated_at".to_owned(), Value::String(date.to_string()));
        }

        Value::Object(season)
    }

    pub fn episode(mut self, episode: u32, rating: u8) -> Self {
        self.episodes.insert(episode, (rating, None));
        self
    }

    pub fn episode_at(mut self, episode: u32, rating: u8, collected_at: DateTime<Utc>) -> Self {
        self.episodes.insert(episode, (rating, Some(collected_at)));
        self
    }

    pub fn rating(mut self, rating: u8) -> Self {
        self.rating = Some(rating);
        self
    }

    pub fn rated_at(mut self, rated_at: DateTime<Utc>) -> Self {
        self.rated_at = Some(rated_at);
        self
    }

    pub fn rating_at(self, rating: u8, rated_at: DateTime<Utc>) -> Self {
        self.rating(rating).rated_at(rated_at)
    }
}
