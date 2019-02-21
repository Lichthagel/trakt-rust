use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::{Map, Number, Value};

pub trait Selector: Sized {
    fn build(self) -> Value;

    fn insert(self, k: String, v: Value) -> Self;

    fn map(self, map: Map<String, Value>) -> Self;

    /// #Panics
    ///
    /// Panics if value is not an [Value::Object]
    ///
    /// [Value::Object]: ../../serde_json/enum.Value.html#variant.Object
    fn value(self, value: Value) -> Self {
        if let Value::Object(value) = value {
            self.map(value)
        } else {
            panic!("value is not an object")
        }
    }

    fn item<T: Serialize>(self, item: T) -> Self {
        self.value(serde_json::to_value(item).unwrap())
    }

    /// #Panics
    ///
    /// Panics if json is not a valid JSON Object
    fn json(self, json: &str) -> Self {
        self.value(serde_json::from_str(json).unwrap())
    }

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
    fn ids(&mut self) -> &mut Map<String, Value>;

    fn slug(mut self, slug: &str) -> Self {
        self.ids()
            .insert("slug".to_owned(), Value::String(slug.to_owned()));
        self
    }

    fn id(mut self, trakt_id: u64) -> Self {
        self.ids()
            .insert("trakt".to_owned(), Value::Number(trakt_id.into()));
        self
    }

    fn tmdb(mut self, tmdb_id: u64) -> Self {
        self.ids()
            .insert("tmdb".to_owned(), Value::Number(tmdb_id.into()));
        self
    }

    fn imdb(mut self, imdb_id: &str) -> Self {
        self.ids()
            .insert("imdb".to_owned(), Value::String(imdb_id.to_owned()));
        self
    }

    fn tvdb(mut self, tvdb_id: u64) -> Self {
        self.ids()
            .insert("tvdb".to_owned(), Value::Number(tvdb_id.into()));
        self
    }

    fn tvrage(mut self, tvrage_id: u64) -> Self {
        self.ids()
            .insert("tvrage".to_owned(), Value::Number(tvrage_id.into()));
        self
    }
}

pub trait SelectMovie: Sized {
    fn movie_value(self, movie: Value) -> Self;

    fn movie_item<T: Serialize>(self, movie: T) -> Self {
        self.movie_value(serde_json::to_value(movie).unwrap())
    }

    /// # Panics
    ///
    /// Panics if movie isn't an valid JSON Object
    fn movie_json(self, movie: &str) -> Self {
        self.movie_value(serde_json::from_str(movie).unwrap())
    }

    fn movie(self, f: impl FnOnce(MovieSelector) -> MovieSelector) -> Self {
        self.movie_value(f(MovieSelector::default()).build())
    }
}

#[derive(Debug, PartialEq)]
pub struct MovieSelector {
    movie: Map<String, Value>,
}

impl Default for MovieSelector {
    fn default() -> Self {
        Self { movie: Map::new() }
    }
}

impl Selector for MovieSelector {
    fn build(self) -> Value {
        Value::Object(self.movie)
    }

    fn insert(mut self, k: String, v: Value) -> Self {
        self.movie.insert(k, v);
        self
    }

    fn map(mut self, map: Map<String, Value>) -> Self {
        for (k, v) in map {
            self.movie.insert(k, v);
        }
        self
    }
}

impl SelectIds for MovieSelector {
    fn ids(&mut self) -> &mut Map<String, Value> {
        if !self.movie.contains_key("ids") {
            self.movie
                .insert("ids".to_owned(), Value::Object(Map::new()));
        }

        self.movie.get_mut("ids").unwrap().as_object_mut().unwrap()
    }
}

pub trait SelectShow: Sized {
    fn show_value(self, show: Value) -> Self;

    fn show_item<T: Serialize>(self, show: T) -> Self {
        self.show_value(serde_json::to_value(show).unwrap())
    }

    fn show_json(self, show: &str) -> Self {
        self.show_value(serde_json::from_str(show).unwrap())
    }

    fn show(self, f: impl FnOnce(ShowSelector) -> ShowSelector) -> Self {
        self.show_value(f(ShowSelector::default()).build())
    }
}

pub struct ShowSelector {
    show: Map<String, Value>,
}

impl Default for ShowSelector {
    fn default() -> Self {
        Self { show: Map::new() }
    }
}

impl Selector for ShowSelector {
    fn build(self) -> Value {
        Value::Object(self.show)
    }

    fn insert(mut self, k: String, v: Value) -> Self {
        self.show.insert(k, v);
        self
    }

    fn map(mut self, map: Map<String, Value>) -> Self {
        for (k, v) in map {
            self.show.insert(k, v);
        }
        self
    }
}

impl ShowSelector {
    pub fn season(mut self, f: impl FnOnce(SeasonSelector) -> SeasonSelector) -> Self {
        match self.show.get_mut("seasons") {
            Some(arr) => {
                arr.as_array_mut()
                    .unwrap()
                    .push(f(SeasonSelector::default()).build());
            }
            None => {
                self.show.insert(
                    "seasons".to_owned(),
                    Value::Array(vec![f(SeasonSelector::default()).build()]),
                );
            }
        }
        self
    }
}

impl SelectIds for ShowSelector {
    fn ids(&mut self) -> &mut Map<String, Value> {
        if !self.show.contains_key("ids") {
            self.show
                .insert("ids".to_owned(), Value::Object(Map::new()));
        }

        self.show.get_mut("ids").unwrap().as_object_mut().unwrap()
    }
}

pub trait SelectSeason: Sized {
    fn season_value(self, season: Value) -> Self;

    fn season_item<T: Serialize>(self, season: T) -> Self {
        self.season_value(serde_json::to_value(season).unwrap())
    }

    fn season_json(self, season: &str) -> Self {
        self.season_value(serde_json::from_str(season).unwrap())
    }

    fn season(self, f: impl FnOnce(SeasonSelector) -> SeasonSelector) -> Self {
        self.season_value(f(SeasonSelector::default()).build())
    }
}

pub struct SeasonSelector {
    season: Map<String, Value>,
}

impl Default for SeasonSelector {
    fn default() -> Self {
        Self { season: Map::new() }
    }
}

impl Selector for SeasonSelector {
    fn build(self) -> Value {
        Value::Object(self.season)
    }

    fn insert(mut self, k: String, v: Value) -> Self {
        self.season.insert(k, v);
        self
    }

    fn map(mut self, map: Map<String, Value>) -> Self {
        for (k, v) in map {
            self.season.insert(k, v);
        }
        self
    }
}

impl SeasonSelector {
    pub fn number(self, season_number: u32) -> Self {
        self.insert("number".to_owned(), Value::Number(season_number.into()))
    }

    pub fn episode(mut self, f: impl FnOnce(EpisodeSelector) -> EpisodeSelector) -> Self {
        match self.season.get_mut("episodes") {
            Some(episodes) => {
                episodes
                    .as_array_mut()
                    .unwrap()
                    .push(f(EpisodeSelector::default()).build());
            }
            None => {
                self.season.insert(
                    "episodes".to_owned(),
                    Value::Array(vec![f(EpisodeSelector::default()).build()]),
                );
            }
        }
        self
    }
}

impl SelectIds for SeasonSelector {
    fn ids(&mut self) -> &mut Map<String, Value> {
        if !self.season.contains_key("ids") {
            self.season
                .insert("ids".to_owned(), Value::Object(Map::new()));
        }

        self.season.get_mut("ids").unwrap().as_object_mut().unwrap()
    }
}

pub trait SelectEpisode: Sized {
    fn episode_value(self, episode: Value) -> Self;

    fn episode_item<T: Serialize>(self, episode: T) -> Self {
        self.episode_value(serde_json::to_value(episode).unwrap())
    }

    fn episode_json(self, episode: &str) -> Self {
        self.episode_value(serde_json::from_str(episode).unwrap())
    }

    fn episode(self, f: impl FnOnce(EpisodeSelector) -> EpisodeSelector) -> Self {
        self.episode_value(f(EpisodeSelector::default()).build())
    }
}

pub struct EpisodeSelector {
    episode: Map<String, Value>,
}

impl Default for EpisodeSelector {
    fn default() -> Self {
        Self {
            episode: Map::new(),
        }
    }
}

impl Selector for EpisodeSelector {
    fn build(self) -> Value {
        Value::Object(self.episode)
    }

    fn insert(mut self, k: String, v: Value) -> Self {
        self.episode.insert(k, v);
        self
    }

    fn map(mut self, map: Map<String, Value>) -> Self {
        for (k, v) in map {
            self.episode.insert(k, v);
        }
        self
    }
}

impl EpisodeSelector {
    pub fn season(self, season_number: u32) -> Self {
        self.insert("season".to_owned(), season_number.into())
    }

    pub fn number(self, episode_number: u32) -> Self {
        self.insert("number".to_owned(), episode_number.into())
    }
}

impl SelectIds for EpisodeSelector {
    fn ids(&mut self) -> &mut Map<String, Value> {
        if !self.episode.contains_key("ids") {
            self.episode
                .insert("ids".to_owned(), Value::Object(Map::new()));
        }

        self.episode
            .get_mut("ids")
            .unwrap()
            .as_object_mut()
            .unwrap()
    }
}

pub trait SelectList: Sized {
    fn list_value(self, list: Value) -> Self;

    fn list_item<T: Serialize>(self, list: T) -> Self {
        self.list_value(serde_json::to_value(list).unwrap())
    }

    fn list_json(self, list: &str) -> Self {
        self.list_value(serde_json::from_str(list).unwrap())
    }

    fn list(self, f: impl FnOnce(ListSelector) -> ListSelector) -> Self {
        self.list_value(f(ListSelector::default()).build())
    }
}

pub struct ListSelector {
    list: Map<String, Value>,
}

impl Default for ListSelector {
    fn default() -> Self {
        Self { list: Map::new() }
    }
}

impl Selector for ListSelector {
    fn build(self) -> Value {
        Value::Object(self.list)
    }

    fn insert(mut self, k: String, v: Value) -> Self {
        self.list.insert(k, v);
        self
    }

    fn map(mut self, map: Map<String, Value>) -> Self {
        for (k, v) in map {
            self.list.insert(k, v);
        }
        self
    }
}

impl ListSelector {
    pub fn name(self, name: &str) -> Self {
        self.insert("name".to_owned(), name.into())
    }

    pub fn user(self, f: impl FnOnce(UserSelector) -> UserSelector) -> Self {
        self.insert("user".to_owned(), f(UserSelector::default()).build())
    }
}

impl SelectIds for ListSelector {
    fn ids(&mut self) -> &mut Map<String, Value> {
        if !self.list.contains_key("ids") {
            self.list
                .insert("ids".to_owned(), Value::Object(Map::new()));
        }

        self.list.get_mut("ids").unwrap().as_object_mut().unwrap()
    }
}

pub trait SelectUser: Sized {
    fn user_value(self, user: Value) -> Self;

    fn user_item<T: Serialize>(self, user: T) -> Self {
        self.user_value(serde_json::to_value(user).unwrap())
    }

    fn user_json(self, user: &str) -> Self {
        self.user_value(serde_json::from_str(user).unwrap())
    }

    fn user(self, f: impl FnOnce(UserSelector) -> UserSelector) -> Self {
        self.user_value(f(UserSelector::default()).build())
    }
}

pub struct UserSelector {
    user: Map<String, Value>,
}

impl Default for UserSelector {
    fn default() -> Self {
        Self { user: Map::new() }
    }
}

impl Selector for UserSelector {
    fn build(self) -> Value {
        Value::Object(self.user)
    }

    fn insert(mut self, k: String, v: Value) -> Self {
        self.user.insert(k, v);
        self
    }

    fn map(mut self, map: Map<String, Value>) -> Self {
        for (k, v) in map {
            self.user.insert(k, v);
        }
        self
    }
}

impl SelectIds for UserSelector {
    fn ids(&mut self) -> &mut Map<String, Value> {
        if !self.user.contains_key("ids") {
            self.user
                .insert("ids".to_owned(), Value::Object(Map::new()));
        }

        self.user.get_mut("ids").unwrap().as_object_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::selectors::*;

    #[test]
    fn movie_selector_value() {
        let s = MovieSelector::default().value(json!({
            "ids": {
                "slug": "warcraft-2016"
            }
        }));

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "slug": "warcraft-2016"
                }
            })
        );
    }

    #[test]
    fn movie_selector_json() {
        let s = MovieSelector::default().json(
            "{\
             \"ids\": {\
             \"slug\": \"warcraft-2016\"\
             }\
             }",
        );

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "slug": "warcraft-2016"
                }
            })
        );
    }

    #[test]
    fn movie_selector_slug() {
        let s = MovieSelector::default().slug("warcraft-2016");

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "slug": "warcraft-2016"
                }
            })
        );
    }

    #[test]
    fn show_selector_value() {
        let s = ShowSelector::default().value(json!({
            "ids": {
                "slug": "fairy-tail"
            },
            "seasons": [
                {
                    "number": 3,
                    "episodes": [{
                        "number": 3
                    }]
                }
            ]
        }));

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "slug": "fairy-tail"
                },
                "seasons": [
                    {
                        "number": 3,
                        "episodes": [{
                            "number": 3
                        }]
                    }
                ]
            })
        );
    }

    #[test]
    fn show_selector_json() {
        let s = ShowSelector::default().json(
            "{\"ids\":{\"slug\":\"fairy-tail\"},\"seasons\":[{\"number\":3,\"episodes\":[{\"number\":3}]}]}"
        );

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "slug": "fairy-tail"
                },
                "seasons": [
                    {
                        "number": 3,
                        "episodes": [{
                            "number": 3
                        }]
                    }
                ]
            })
        );
    }

    #[test]
    fn show_selector_slug() {
        let s = ShowSelector::default().slug("fairy-tail").value(json!({
            "seasons": [
                {
                    "number": 3,
                    "episodes": [{
                        "number": 3
                    }]
                }
            ]
        }));

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "slug": "fairy-tail"
                },
                "seasons": [
                    {
                        "number": 3,
                        "episodes": [{
                            "number": 3
                        }]
                    }
                ]
            })
        );
    }

    #[test]
    fn show_selector_season() {
        let s = ShowSelector::default().slug("fairy-tail").season(|season| {
            season.value(json!({
                "number": 3,
                "episodes": [{
                    "number": 3
                }]
            }))
        });

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "slug": "fairy-tail"
                },
                "seasons": [
                    {
                        "number": 3,
                        "episodes": [{
                            "number": 3
                        }]
                    }
                ]
            })
        );
    }

    #[test]
    fn season_selector_value() {
        let s = SeasonSelector::default().value(json!({
            "number": 3,
            "episodes": [{
                "number": 3
            }]
        }));

        assert_eq!(
            s.build(),
            json!({
                "number": 3,
                "episodes": [{
                    "number": 3
                }]
            })
        )
    }

    #[test]
    fn season_selector_json() {
        let s = SeasonSelector::default().json("{\"number\":3,\"episodes\":[{\"number\":3}]}");

        assert_eq!(
            s.build(),
            json!({
                "number": 3,
                "episodes": [{
                    "number": 3
                }]
            })
        )
    }

    #[test]
    fn season_selector_episode() {
        let s = SeasonSelector::default()
            .number(3)
            .episode(|ep| ep.number(3));

        assert_eq!(
            s.build(),
            json!({
                "number": 3,
                "episodes": [{
                    "number": 3
                }]
            })
        )
    }

    #[test]
    fn episode_selector_value() {
        let s = EpisodeSelector::default().value(json!({
            "ids": {
                "trakt": 3159393
            }
        }));

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "trakt": 3159393
                }
            })
        )
    }

    #[test]
    fn episode_selector_json() {
        let s = EpisodeSelector::default().json("{\"ids\":{\"trakt\":3159393}}");

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "trakt": 3159393
                }
            })
        )
    }

    #[test]
    fn episode_selector_id() {
        let s = EpisodeSelector::default().id(3159393);

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "trakt": 3159393
                }
            })
        )
    }

    #[test]
    fn list_selector_value() {
        let s = ListSelector::default().value(json!({
            "ids": {
                "trakt": 4834049
            }
        }));

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "trakt": 4834049
                }
            })
        )
    }

    #[test]
    fn list_selector_json() {
        let s = ListSelector::default().json("{\"ids\":{\"trakt\":4834049}}");

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "trakt": 4834049
                }
            })
        )
    }

    #[test]
    fn list_selector_id() {
        let s = ListSelector::default().id(4834049);

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "trakt": 4834049
                }
            })
        )
    }

    #[test]
    fn list_selector_user() {
        let s = ListSelector::default().user(|user| user.slug("lichthagel"));

        assert_eq!(
            s.build(),
            json!({
                "user": {
                    "ids": {
                        "slug": "lichthagel"
                    }
                }
            })
        )
    }

    #[test]
    fn user_selector_value() {
        let s = UserSelector::default().value(json!({
            "ids": {
                "slug": "lichthagel"
            }
        }));

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "slug": "lichthagel"
                }
            })
        )
    }

    #[test]
    fn user_selector_json() {
        let s = UserSelector::default().json("{\"ids\":{\"slug\":\"lichthagel\"}}");

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "slug": "lichthagel"
                }
            })
        )
    }

    #[test]
    fn user_selector_slug() {
        let s = UserSelector::default().slug("lichthagel");

        assert_eq!(
            s.build(),
            json!({
                "ids": {
                    "slug": "lichthagel"
                }
            })
        )
    }
}
