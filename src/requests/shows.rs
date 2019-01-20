use crate::models::Episode;
use crate::models::Ratings;
use crate::models::ShowStats;
use crate::models::User;
use crate::{
    error::Result,
    models::{
        Alias, AnticipatedShow, CollectionProgress, Comment, List, People, Show, ShowInfo,
        TimePeriod, Translation, UpdatedShow, WatchedProgress, WatchedShow,
    },
    TraktApi,
};
use std::fmt::Display;

impl TraktApi {
    pub fn shows_trending(&self, page: u32, limit: u32) -> Result<Vec<ShowInfo>> {
        self.get(api_url!(
            ("shows", "trending"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn shows_popular(&self, page: u32, limit: u32) -> Result<Vec<Show>> {
        self.get(api_url!(
            ("shows", "popular"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn shows_played(
        &self,
        page: u32,
        limit: u32,
        period: TimePeriod,
    ) -> Result<Vec<WatchedShow>> {
        self.get(api_url!(
            ("shows", "played", period),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn shows_watched(
        &self,
        page: u32,
        limit: u32,
        period: TimePeriod,
    ) -> Result<Vec<WatchedShow>> {
        self.get(api_url!(
            ("shows", "watched", period),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn shows_collected(
        &self,
        page: u32,
        limit: u32,
        period: TimePeriod,
    ) -> Result<Vec<WatchedShow>> {
        self.get(api_url!(
            ("shows", "collected", period),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn shows_anticipated(&self, page: u32, limit: u32) -> Result<Vec<AnticipatedShow>> {
        self.get(api_url!(
            ("shows", "anticipated"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn shows_updates(&self, page: u32, limit: u32) -> Result<Vec<UpdatedShow>> {
        self.get(api_url!(
            ("shows", "updates"),
            ("page", page),
            ("limit", limit)
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
        language: String,
    ) -> Result<Vec<Translation>> {
        self.get(api_url!(("shows", id, "translations", language)))
    }

    pub fn show_comments(&self, id: impl Display, page: u32, limit: u32) -> Result<Vec<Comment>> {
        self.get(api_url!(
            ("shows", id, "comments"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn show_lists(&self, id: impl Display, page: u32, limit: u32) -> Result<Vec<List>> {
        self.get(api_url!(
            ("shows", id, "lists"),
            ("page", page),
            ("limit", limit)
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

    pub fn show_related(&self, id: impl Display, page: u32, limit: u32) -> Result<Vec<Show>> {
        self.get(api_url!(
            ("shows", id, "related"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn show_stats(&self, id: impl Display) -> Result<ShowStats> {
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
