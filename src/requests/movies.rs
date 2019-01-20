use crate::{
    error::Result,
    models::{
        Alias, AnticipatedMovie, Comment, List, ListFactory, MediaStats, Movie, MovieInfo, People,
        Ratings, TimePeriod, Translation, UpdatedMovie, User, WatchedMovie,
    },
    pagination::PaginationFactory,
    TraktApi,
};
use std::fmt::Display;

impl TraktApi {
    pub fn movies_trending(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<MovieInfo>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("movies", "trending"),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn movies_popular(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<Movie>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("movies", "popular"),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn movies_played(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
        period: TimePeriod,
    ) -> Result<Vec<WatchedMovie>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("movies", "played", period.to_string()),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn movies_watched(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
        period: TimePeriod,
    ) -> Result<Vec<WatchedMovie>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("movies", "watched", period.to_string()),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn movies_collected(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
        period: TimePeriod,
    ) -> Result<Vec<WatchedMovie>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("movies", "collected", period.to_string()),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn movies_anticipated(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<AnticipatedMovie>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("movies", "anticipated"),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn movies_updates(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<UpdatedMovie>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("movies", "updates"),
            ("page", pf.page),
            ("limit", pf.limit)
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

    pub fn movie_comments(
        &self,
        id: impl Display,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<Comment>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("movies", id, "comments"),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn movie_lists(
        &self,
        id: impl Display,
        f: impl FnOnce(ListFactory) -> ListFactory,
        g: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<List>> {
        let list_factory = f(ListFactory::default());
        let pf = g(PaginationFactory::default());
        self.get(api_url!(
            (
                "movies",
                id,
                "lists",
                list_factory.list_filter,
                list_factory.sorting
            ),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn movie_people(&self, id: impl Display) -> Result<People> {
        self.get(api_url!(("movies", id, "people")))
    }

    pub fn movie_ratings(&self, id: impl Display) -> Result<Ratings> {
        self.get(api_url!(("movies", id, "ratings")))
    }

    pub fn movie_related(
        &self,
        id: impl Display,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<Movie>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("movies", id, "related"),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn movie_stats(&self, id: impl Display) -> Result<MediaStats> {
        self.get(api_url!(("movies", id, "stats")))
    }

    pub fn movie_watching(&self, id: impl Display) -> Result<Vec<User>> {
        self.get(api_url!(("movies", id, "watching")))
    }
}
