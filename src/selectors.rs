use crate::models::{
    Episode, Ids, Movie, OptionEpisode, OptionMovie, OptionSeason, OptionShow, Season, Show,
};
use serde_json::Value;

pub trait Selector {
    fn build(self) -> Value;
}

pub trait SelectIds: Sized {
    fn ids(&mut self) -> &mut Ids;

    fn slug(mut self, slug: &str) -> Self {
        self.ids().slug = Some(slug.to_owned());
        self
    }

    fn id(mut self, trakt_id: u64) -> Self {
        self.ids().trakt = Some(trakt_id);
        self
    }

    fn tmdb(mut self, tmdb_id: u64) -> Self {
        self.ids().tmdb = Some(tmdb_id);
        self
    }

    fn imdb(mut self, imdb_id: &str) -> Self {
        self.ids().imdb = Some(imdb_id.to_owned());
        self
    }

    fn tvdb(mut self, tvdb_id: u64) -> Self {
        self.ids().tvdb = Some(tvdb_id);
        self
    }

    fn tvrage(mut self, tvrage_id: u64) -> Self {
        self.ids().tvrage = Some(tvrage_id);
        self
    }
}

pub trait SelectMovie: Sized {
    fn movie_v(self, movie: Value) -> Self;

    fn movie_obj(self, movie: Movie) -> Self {
        self.movie_v(serde_json::to_value(movie).unwrap())
    }

    fn movie(self, f: impl FnOnce(MovieSelector) -> MovieSelector) -> Self {
        self.movie_v(f(MovieSelector::default()).build())
    }
}

pub trait SelectMovieData<T>: Sized {
    fn movie_v_d(self, movie: Value, data: T) -> Self;

    fn movie_obj_d(self, movie: Movie, data: T) -> Self {
        self.movie_v_d(serde_json::to_value(movie).unwrap(), data)
    }

    fn movie_d(self, f: impl FnOnce(MovieSelector) -> MovieSelector, data: T) -> Self {
        self.movie_v_d(f(MovieSelector::default()).build(), data)
    }
}

pub struct MovieSelector {
    movie: OptionMovie,
}

impl Default for MovieSelector {
    fn default() -> Self {
        Self {
            movie: OptionMovie::default(),
        }
    }
}

impl Selector for MovieSelector {
    fn build(self) -> Value {
        serde_json::to_value(self.movie).unwrap()
    }
}

impl MovieSelector {
    pub fn value(mut self, movie: Value) -> Self {
        self.movie = serde_json::from_value(movie).unwrap();
        self
    }

    pub fn movie(self, movie: Movie) -> Self {
        self.value(serde_json::to_value(movie).unwrap())
    }

    pub fn optmovie(mut self, movie: OptionMovie) -> Self {
        self.movie += movie;
        self
    }
}

impl SelectIds for MovieSelector {
    fn ids(&mut self) -> &mut Ids {
        if let None = self.movie.ids {
            self.movie.ids = Some(Ids::default())
        }

        self.movie.ids.as_mut().unwrap()
    }
}

pub trait SelectShow: Sized {
    fn show_v(self, show: Value) -> Self;

    fn show_obj(self, show: Show) -> Self {
        self.show_v(serde_json::to_value(show).unwrap())
    }

    fn show(self, f: impl FnOnce(ShowSelector) -> ShowSelector) -> Self {
        self.show_v(f(ShowSelector::default()).build())
    }
}

pub struct ShowSelector {
    show: OptionShow,
    seasons: Vec<Value>,
}

impl Default for ShowSelector {
    fn default() -> Self {
        Self {
            show: OptionShow::default(),
            seasons: Vec::new(),
        }
    }
}

impl Selector for ShowSelector {
    fn build(self) -> Value {
        let v = serde_json::to_value(self.show).unwrap();
        let mut m = v.as_object().unwrap().clone();
        drop(v);

        m.insert("seasons".to_owned(), Value::Array(self.seasons));

        Value::Object(m)
    }
}

impl ShowSelector {
    pub fn value(mut self, show: Value) -> Self {
        self.show = serde_json::from_value(show).unwrap();
        self
    }

    pub fn show(mut self, show: Show) -> Self {
        self.value(serde_json::to_value(show).unwrap())
    }

    pub fn optshow(mut self, show: OptionShow) -> Self {
        self.show += show;
        self
    }

    pub fn season(mut self, f: impl FnOnce(SeasonSelector) -> SeasonSelector) -> Self {
        self.seasons.push(f(SeasonSelector::default()).build());
        self
    }
}

impl SelectIds for ShowSelector {
    fn ids(&mut self) -> &mut Ids {
        if let None = self.show.ids {
            self.show.ids = Some(Ids::default());
        }

        self.show.ids.as_mut().unwrap()
    }
}

pub trait SelectSeason: Sized {
    fn season_v(self, season: Value) -> Self;

    fn season_obj(self, season: Season) -> Self {
        self.season_v(serde_json::to_value(season).unwrap())
    }

    fn season(self, f: impl FnOnce(SeasonSelector) -> SeasonSelector) -> Self {
        self.season_v(f(SeasonSelector::default()).build())
    }
}

pub struct SeasonSelector {
    season: OptionSeason,
    episodes: Vec<Value>,
}

impl Default for SeasonSelector {
    fn default() -> Self {
        Self {
            season: OptionSeason::default(),
            episodes: Vec::new(),
        }
    }
}

impl Selector for SeasonSelector {
    fn build(self) -> Value {
        let v = serde_json::to_value(self.season).unwrap();
        let mut m = v.as_object().unwrap().clone();
        drop(v);

        m.insert("episodes".to_owned(), Value::Array(self.episodes));

        Value::Object(m)
    }
}

impl SeasonSelector {
    pub fn number(mut self, season_number: u32) -> Self {
        self.season.number = Some(season_number);
        self
    }

    pub fn episode(mut self, f: impl FnOnce(EpisodeSelector) -> EpisodeSelector) -> Self {
        self.episodes.push(f(EpisodeSelector::default()).build());
        self
    }
}

impl SelectIds for SeasonSelector {
    fn ids(&mut self) -> &mut Ids {
        if let None = self.season.ids {
            self.season.ids = Some(Ids::default());
        }
        self.season.ids.as_mut().unwrap()
    }
}

pub trait SelectEpisode: Sized {
    fn episode_v(self, episode: Value) -> Self;

    fn episode_obj(self, episode: Episode) -> Self {
        self.episode_v(serde_json::to_value(episode).unwrap())
    }

    fn episode(self, f: impl FnOnce(EpisodeSelector) -> EpisodeSelector) -> Self {
        self.episode_v(f(EpisodeSelector::default()).build())
    }
}

pub struct EpisodeSelector {
    episode: OptionEpisode,
}

impl Default for EpisodeSelector {
    fn default() -> Self {
        Self {
            episode: OptionEpisode::default(),
        }
    }
}

impl Selector for EpisodeSelector {
    fn build(self) -> Value {
        serde_json::to_value(self.episode).unwrap()
    }
}

impl EpisodeSelector {
    pub fn season(mut self, season_number: u32) -> Self {
        self.episode.season = Some(season_number);
        self
    }

    pub fn number(mut self, episode_number: u32) -> Self {
        self.episode.number = Some(episode_number);
        self
    }
}

impl SelectIds for EpisodeSelector {
    fn ids(&mut self) -> &mut Ids {
        if let None = self.episode.ids {
            self.episode.ids = Some(Ids::default());
        }

        self.episode.ids.as_mut().unwrap()
    }
}
