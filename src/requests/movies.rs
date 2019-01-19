use crate::{
    error::Error,
    models::{
        Alias, AnticipatedMovie, Comment, List, ListSort, ListType, Movie, MovieInfo, Ratings,
        TimePeriod, Translation, UpdatedMovie, WatchedMovie,
    },
    TraktApi,
};
use std::fmt::Display;

impl TraktApi {
    pub fn movies_trending(&self, page: u32, limit: u32) -> Result<Vec<MovieInfo>, Error> {
        self.get(api_url!(
            ("movies", "trending"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movies_popular(&self, page: u32, limit: u32) -> Result<Vec<Movie>, Error> {
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
    ) -> Result<Vec<WatchedMovie>, Error> {
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
    ) -> Result<Vec<WatchedMovie>, Error> {
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
    ) -> Result<Vec<WatchedMovie>, Error> {
        self.get(api_url!(
            ("movies", "collected", period.to_string()),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movies_anticipated(
        &self,
        page: u32,
        limit: u32,
    ) -> Result<Vec<AnticipatedMovie>, Error> {
        self.get(api_url!(
            ("movies", "anticipated"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movies_updates(&self, page: u32, limit: u32) -> Result<Vec<UpdatedMovie>, Error> {
        self.get(api_url!(
            ("movies", "updates"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movie(&self, id: impl Display) -> Result<Movie, Error> {
        self.get(api_url!(("movies", id)))
    }

    pub fn movie_aliases(&self, id: impl Display) -> Result<Vec<Alias>, Error> {
        self.get(api_url!(("movies", id, "aliases")))
    }

    pub fn movie_translations(
        &self,
        id: impl Display,
        language: impl Display,
    ) -> Result<Vec<Translation>, Error> {
        self.get(api_url!(("movies", id, "translations", language)))
    }

    pub fn movie_comments(
        &self,
        id: impl Display,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Comment>, Error> {
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
    ) -> Result<Vec<List>, Error> {
        self.get(api_url!(
            ("movies", id, "lists", list_type, list_sorting),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn movie_people(&self, _id: impl Display) -> Result<Vec<()>, Error> {
        unimplemented!() // TODO
    }

    pub fn movie_ratings(&self, id: impl Display) -> Result<Ratings, Error> {
        self.get(api_url!(("movies", id, "ratings")))
    }

    pub fn movie_related(
        &self,
        id: impl Display,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Movie>, Error> {
        self.get(api_url!(
            ("movies", id, "related"),
            ("page", page),
            ("limit", limit)
        ))
    }
}
