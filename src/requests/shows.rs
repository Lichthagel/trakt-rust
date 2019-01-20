use crate::{
    error::Result,
    models::{
        Alias, AnticipatedShow, CollectionProgress, Comment, Episode, List, ListFactory,
        MediaStats, People, Ratings, Show, ShowInfo, TimePeriod, Translation, UpdatedShow, User,
        WatchedProgress, WatchedShow,
    },
    pagination::PaginationFactory,
    TraktApi,
};
use std::fmt::Display;

impl TraktApi {
    pub fn shows_trending(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<ShowInfo>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("shows", "trending"),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn shows_popular(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<Show>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("shows", "popular"),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn shows_played(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
        period: TimePeriod,
    ) -> Result<Vec<WatchedShow>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("shows", "played", period),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn shows_watched(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
        period: TimePeriod,
    ) -> Result<Vec<WatchedShow>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("shows", "watched", period),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn shows_collected(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
        period: TimePeriod,
    ) -> Result<Vec<WatchedShow>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("shows", "collected", period),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn shows_anticipated(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<AnticipatedShow>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("shows", "anticipated"),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn shows_updates(
        &self,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<UpdatedShow>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("shows", "updates"),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn show(&self, id: impl Display) -> Result<Show> {
        self.get(api_url!(("shows", id)))
    }

    pub fn show_aliases(&self, id: impl Display) -> Result<Vec<Alias>> {
        self.get(api_url!(("shows", id, "aliases")))
    }

    pub fn show_translations(
        &self,
        id: impl Display,
        language: impl Display,
    ) -> Result<Vec<Translation>> {
        self.get(api_url!(("shows", id, "translations", language)))
    }

    pub fn show_comments(
        &self,
        id: impl Display,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<Comment>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("shows", id, "comments"),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn show_lists(
        &self,
        id: impl Display,
        f: impl FnOnce(ListFactory) -> ListFactory,
        g: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<List>> {
        let list_factory = f(ListFactory::default());
        let pf = g(PaginationFactory::default());
        self.get(api_url!(
            (
                "shows",
                id,
                "lists",
                list_factory.list_filter,
                list_factory.sorting
            ),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn show_progress_collection(
        &self,
        id: impl Display,
        access_token: String,
    ) -> Result<CollectionProgress> {
        self.auth_get(
            api_url!(("shows", id, "progress", "collection")),
            access_token,
        )
    }

    pub fn show_progress_watched(
        &self,
        id: impl Display,
        access_token: String,
    ) -> Result<WatchedProgress> {
        self.auth_get(api_url!(("shows", id, "progress", "watched")), access_token)
    }

    pub fn show_people(&self, id: impl Display) -> Result<People> {
        self.get(api_url!(("shows", id, "people")))
    }

    pub fn show_ratings(&self, id: impl Display) -> Result<Ratings> {
        self.get(api_url!(("shows", id, "ratings")))
    }

    pub fn show_related(
        &self,
        id: impl Display,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<Show>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("shows", id, "related"),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn show_stats(&self, id: impl Display) -> Result<MediaStats> {
        self.get(api_url!(("shows", id, "stats")))
    }

    pub fn show_watching(&self, id: impl Display) -> Result<Vec<User>> {
        self.get(api_url!(("shows", id, "watching")))
    }

    pub fn show_next_episode(&self, id: impl Display) -> Result<Option<Episode>> {
        self.get(api_url!(("shows", id, "next_episode")))
    }

    pub fn show_last_episode(&self, id: impl Display) -> Result<Option<Episode>> {
        self.get(api_url!(("shows", id, "last_episode")))
    }
}
