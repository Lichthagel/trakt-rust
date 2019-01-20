use crate::{
    error::Result,
    models::{
        Alias, AnticipatedMovie, Comment, List, ListSort, ListType, MediaStats, Movie, MovieInfo,
        People, Ratings, TimePeriod, Translation, UpdatedMovie, User, WatchedMovie,
    },
    TraktApi,
};
use std::fmt::Display;

impl TraktApi {
    pub fn movies_trending(&self, page: u32, limit: u32) -> Result<Vec<MovieInfo>> {
        self.get(api_url!(
            ("movies", "trending"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movies_popular(&self, page: u32, limit: u32) -> Result<Vec<Movie>> {
        self.get(api_url!(
            ("movies", "popular"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movies_played(
        &self,
        page: u32,
        limit: u32,
        period: TimePeriod,
    ) -> Result<Vec<WatchedMovie>> {
        self.get(api_url!(
            ("movies", "played", period.to_string()),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movies_watched(
        &self,
        page: u32,
        limit: u32,
        period: TimePeriod,
    ) -> Result<Vec<WatchedMovie>> {
        self.get(api_url!(
            ("movies", "watched", period.to_string()),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movies_collected(
        &self,
        page: u32,
        limit: u32,
        period: TimePeriod,
    ) -> Result<Vec<WatchedMovie>> {
        self.get(api_url!(
            ("movies", "collected", period.to_string()),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movies_anticipated(&self, page: u32, limit: u32) -> Result<Vec<AnticipatedMovie>> {
        self.get(api_url!(
            ("movies", "anticipated"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movies_updates(&self, page: u32, limit: u32) -> Result<Vec<UpdatedMovie>> {
        self.get(api_url!(
            ("movies", "updates"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movie(&self, id: impl Display) -> Result<Movie> {
        self.get(api_url!(("movies", id)))
    }

    pub fn movie_aliases(&self, id: impl Display) -> Result<Vec<Alias>> {
        self.get(api_url!(("movies", id, "aliases")))
    }

    pub fn movie_translations(
        &self,
        id: impl Display,
        language: impl Display,
    ) -> Result<Vec<Translation>> {
        self.get(api_url!(("movies", id, "translations", language)))
    }

    pub fn movie_comments(&self, id: impl Display, page: u32, limit: u32) -> Result<Vec<Comment>> {
        self.get(api_url!(
            ("movies", id, "comments"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movie_lists(
        &self,
        id: impl Display,
        list_type: ListType,
        list_sorting: ListSort,
        page: u32,
        limit: u32,
    ) -> Result<Vec<List>> {
        self.get(api_url!(
            ("movies", id, "lists", list_type, list_sorting),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movie_people(&self, id: impl Display) -> Result<People> {
        self.get(api_url!(("movies", id, "people")))
    }

    pub fn movie_ratings(&self, id: impl Display) -> Result<Ratings> {
        self.get(api_url!(("movies", id, "ratings")))
    }

    pub fn movie_related(&self, id: impl Display, page: u32, limit: u32) -> Result<Vec<Movie>> {
        self.get(api_url!(
            ("movies", id, "related"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movie_stats(&self, id: impl Display) -> Result<MediaStats> {
        self.get(api_url!(("movies", id, "stats")))
    }

    pub fn movie_watching(&self, id: impl Display) -> Result<Vec<User>> {
        self.get(api_url!(("movies", id, "watching")))
    }
}
