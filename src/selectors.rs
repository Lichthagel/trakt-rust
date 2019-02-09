use crate::models::User;
use crate::models::{
    list::OptionList, Episode, Ids, List, Movie, OptionEpisode, OptionMovie, OptionSeason,
    OptionShow, OptionUser, Season, Show,
};
use chrono::{DateTime, Utc};
use serde_json::{Map, Number, Value};

pub trait Selector: Sized {
    fn build(self) -> Value;

    fn insert(self, k: String, v: Value) -> Self;

    fn insert_num(self, k: String, v: impl Into<Number>) -> Self {
        self.insert(k, Value::Number(v.into()))
    }

    fn insert_str(self, k: String, v: String) -> Self {
        self.insert(k, Value::String(v))
    }

    fn insert_date(self, k: String, v: DateTime<Utc>) -> Self {
        self.insert_str(k, v.to_string())
    }

    fn rated_at(self, date: DateTime<Utc>) -> Self {
        self.insert_date("rated_at".to_owned(), date)
    }

    fn collected_at(self, date: DateTime<Utc>) -> Self {
        self.insert_date("collected_at".to_owned(), date)
    }

    fn watched_at(self, date: DateTime<Utc>) -> Self {
        self.insert_date("watched_at".to_owned(), date)
    }

    fn rating(self, rating: u8) -> Self {
        self.insert_num("rating".to_owned(), rating)
    }
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

pub struct MovieSelector {
    movie: OptionMovie,
    additional: Map<String, Value>,
}

impl Default for MovieSelector {
    fn default() -> Self {
        Self {
            movie: OptionMovie::default(),
            additional: Map::new(),
        }
    }
}

impl Selector for MovieSelector {
    fn build(self) -> Value {
        let v = serde_json::to_value(self.movie).unwrap();
        let mut m = v.as_object().unwrap().clone();
        drop(v);

        for (k, v) in self.additional {
            m.insert(k, v);
        }

        Value::Object(m)
    }

    fn insert(mut self, k: String, v: Value) -> Self {
        self.additional.insert(k, v);
        self
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
    additional: Map<String, Value>,
}

impl Default for ShowSelector {
    fn default() -> Self {
        Self {
            show: OptionShow::default(),
            seasons: Vec::new(),
            additional: Map::new(),
        }
    }
}

impl Selector for ShowSelector {
    fn build(self) -> Value {
        let v = serde_json::to_value(self.show).unwrap();
        let mut m = v.as_object().unwrap().clone();
        drop(v);

        m.insert("seasons".to_owned(), Value::Array(self.seasons));

        for (k, v) in self.additional {
            m.insert(k, v);
        }

        Value::Object(m)
    }

    fn insert(mut self, k: String, v: Value) -> Self {
        self.additional.insert(k, v);
        self
    }
}

impl ShowSelector {
    pub fn value(mut self, show: Value) -> Self {
        self.show = serde_json::from_value(show).unwrap();
        self
    }

    pub fn show(self, show: Show) -> Self {
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
    additional: Map<String, Value>,
}

impl Default for SeasonSelector {
    fn default() -> Self {
        Self {
            season: OptionSeason::default(),
            episodes: Vec::new(),
            additional: Map::new(),
        }
    }
}

impl Selector for SeasonSelector {
    fn build(self) -> Value {
        let v = serde_json::to_value(self.season).unwrap();
        let mut m = v.as_object().unwrap().clone();
        drop(v);

        m.insert("episodes".to_owned(), Value::Array(self.episodes));

        for (k, v) in self.additional {
            m.insert(k, v);
        }

        Value::Object(m)
    }

    fn insert(mut self, k: String, v: Value) -> Self {
        self.additional.insert(k, v);
        self
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
    additional: Map<String, Value>,
}

impl Default for EpisodeSelector {
    fn default() -> Self {
        Self {
            episode: OptionEpisode::default(),
            additional: Map::new(),
        }
    }
}

impl Selector for EpisodeSelector {
    fn build(self) -> Value {
        let v = serde_json::to_value(self.episode).unwrap();
        let mut m = v.as_object().unwrap().clone();
        drop(v);

        for (k, v) in self.additional {
            m.insert(k, v);
        }

        Value::Object(m)
    }

    fn insert(mut self, k: String, v: Value) -> Self {
        self.additional.insert(k, v);
        self
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

pub trait SelectList: Sized {
    fn list_v(self, list: Value) -> Self;

    fn list_obj(self, list: List) -> Self {
        self.list_v(serde_json::to_value(list).unwrap())
    }

    fn list(self, f: impl FnOnce(ListSelector) -> ListSelector) -> Self {
        self.list_v(f(ListSelector::default()).build())
    }
}

pub struct ListSelector {
    list: OptionList,
    additional: Map<String, Value>,
}

impl Default for ListSelector {
    fn default() -> Self {
        Self {
            list: OptionList::default(),
            additional: Map::new(),
        }
    }
}

impl Selector for ListSelector {
    fn build(self) -> Value {
        let v = serde_json::to_value(self.list).unwrap();
        let mut m = v.as_object().unwrap().clone();
        drop(v);

        for (k, v) in self.additional {
            m.insert(k, v);
        }

        Value::Object(m)
    }

    fn insert(mut self, k: String, v: Value) -> Self {
        self.additional.insert(k, v);
        self
    }
}

impl ListSelector {
    pub fn name(mut self, name: &str) -> Self {
        self.list.name = Some(name.to_owned());
        self
    }

    pub fn user(mut self, f: impl FnOnce(UserSelector) -> UserSelector) -> Self {
        self.list.user = Some(f(UserSelector::default()).user);
        self
    }
}

impl SelectIds for ListSelector {
    fn ids(&mut self) -> &mut Ids {
        if let None = self.list.ids {
            self.list.ids = Some(Ids::default())
        }

        self.list.ids.as_mut().unwrap()
    }
}

pub trait SelectUser: Sized {
    fn user_v(self, user: Value) -> Self;

    fn user_obj(self, user: User) -> Self {
        self.user_v(serde_json::to_value(user).unwrap())
    }

    fn user(self, f: impl FnOnce(UserSelector) -> UserSelector) -> Self {
        self.user_v(f(UserSelector::default()).build())
    }
}

pub struct UserSelector {
    user: OptionUser,
    additional: Map<String, Value>,
}

impl Default for UserSelector {
    fn default() -> Self {
        Self {
            user: OptionUser::default(),
            additional: Map::new(),
        }
    }
}

impl Selector for UserSelector {
    fn build(self) -> Value {
        let v = serde_json::to_value(self.user).unwrap();
        let mut m = v.as_object().unwrap().clone();
        drop(v);

        for (k, v) in self.additional {
            m.insert(k, v);
        }

        Value::Object(m)
    }

    fn insert(mut self, k: String, v: Value) -> Self {
        self.additional.insert(k, v);
        self
    }
}

impl SelectIds for UserSelector {
    fn ids(&mut self) -> &mut Ids {
        if let None = self.user.ids {
            self.user.ids = Some(Ids::default())
        }

        self.user.ids.as_mut().unwrap()
    }
}
