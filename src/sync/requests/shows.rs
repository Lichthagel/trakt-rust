use crate::{
    models::{
        Alias, AnticipatedShow, CollectionProgress, Comment, Episode, List, ListFactory,
        MediaStats, People, Ratings, Show, ShowInfo, TimePeriod, Translation, UpdatedShow, User,
        WatchedProgress, WatchedShow,
    },
    sync::pagination::PaginationRequest,
    Result, TraktApi,
};
use reqwest::Method;
use std::fmt::Display;

impl<'a> TraktApi<'a> {
    pub fn shows_trending(&self) -> PaginationRequest<ShowInfo> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("shows", "trending"))),
        )
    }

    pub fn shows_popular(&self) -> PaginationRequest<Show> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("shows", "popular"))),
        )
    }

    pub fn shows_played(&self, period: TimePeriod) -> PaginationRequest<WatchedShow> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("shows", "played", period))),
        )
    }

    pub fn shows_watched(&self, period: TimePeriod) -> PaginationRequest<WatchedShow> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("shows", "watched", period))),
        )
    }

    pub fn shows_collected(&self, period: TimePeriod) -> PaginationRequest<WatchedShow> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("shows", "collected", period))),
        )
    }

    pub fn shows_anticipated(&self) -> PaginationRequest<AnticipatedShow> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("shows", "anticipated"))),
        )
    }

    pub fn shows_updates(&self) -> PaginationRequest<UpdatedShow> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("shows", "updates"))),
        )
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

    pub fn show_comments(&self, id: impl Display) -> PaginationRequest<Comment> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("shows", id, "comments"))),
        )
    }

    pub fn show_lists(
        &self,
        id: impl Display,
        f: impl FnOnce(ListFactory) -> ListFactory,
    ) -> PaginationRequest<List> {
        let list_factory = f(ListFactory::default());

        PaginationRequest::new(
            self,
            self.builder(
                Method::GET,
                api_url!((
                    "shows",
                    id,
                    "lists",
                    list_factory.list_filter,
                    list_factory.sorting
                )),
            ),
        )
    }

    pub fn show_progress_collection(
        &self,
        id: impl Display,
        access_token: &str,
    ) -> Result<CollectionProgress> {
        self.auth_get(
            api_url!(("shows", id, "progress", "collection")),
            access_token,
        )
    }

    pub fn show_progress_watched(
        &self,
        id: impl Display,
        access_token: &str,
    ) -> Result<WatchedProgress> {
        self.auth_get(api_url!(("shows", id, "progress", "watched")), access_token)
    }

    pub fn show_people(&self, id: impl Display) -> Result<People> {
        self.get(api_url!(("shows", id, "people")))
    }

    pub fn show_ratings(&self, id: impl Display) -> Result<Ratings> {
        self.get(api_url!(("shows", id, "ratings")))
    }

    pub fn show_related(&self, id: impl Display) -> PaginationRequest<Show> {
        PaginationRequest::new(
            self,
            self.builder(Method::GET, api_url!(("shows", id, "related"))),
        )
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
