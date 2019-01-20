use crate::models::ListFactory;
use crate::{
    error::Result,
    models::{Comment, Episode, List, MediaStats, Ratings, Season, User},
    TraktApi,
};
use std::fmt::Display;

impl TraktApi {
    pub fn seasons(&self, show_id: impl Display) -> Result<Vec<Season>> {
        self.get(api_url!(("shows", show_id, "seasons")))
    }

    pub fn season(&self, show_id: impl Display, season_number: u32) -> Result<Vec<Episode>> {
        self.get(api_url!(("shows", show_id, "seasons", season_number)))
    }

    pub fn season_comments(
        &self,
        show_id: impl Display,
        season_number: u32,
    ) -> Result<Vec<Comment>> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "comments"
        )))
    }

    pub fn season_lists(
        &self,
        show_id: impl Display,
        season_number: u32,
        f: impl FnOnce(ListFactory) -> ListFactory,
    ) -> Result<Vec<List>> {
        let list_factory = f(ListFactory::default());
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "lists",
            list_factory.list_filter,
            list_factory.sorting
        )))
    }

    pub fn season_ratings(&self, show_id: impl Display, season_number: u32) -> Result<Ratings> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "ratings"
        )))
    }

    pub fn season_stats(&self, show_id: impl Display, season_number: u32) -> Result<MediaStats> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "stats"
        )))
    }

    pub fn season_watching(&self, show_id: impl Display, season_number: u32) -> Result<Vec<User>> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "watching"
        )))
    }
}
