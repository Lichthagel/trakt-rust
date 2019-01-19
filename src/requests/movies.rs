use crate::{
    error::Error,
    models::{AnticipatedMovie, Movie, MovieInfo, TimePeriod, UpdatedMovie, WatchedMovie},
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
}
