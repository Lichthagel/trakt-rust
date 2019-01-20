use crate::models::ListFactory;
use crate::{
    error::Result,
    models::{Comment, Episode, List, MediaStats, Ratings, Translation, User},
    TraktApi,
};
use std::fmt::Display;

impl TraktApi {
    pub fn episode(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
    ) -> Result<Episode> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "episodes",
            episode_number
        )))
    }

    pub fn episode_translations(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
        language: impl Display,
    ) -> Result<Vec<Translation>> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "episodes",
            episode_number,
            "translations",
            language
        )))
    }

    pub fn episode_comments(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Comment>> {
        self.get(api_url!(
            (
                "shows",
                show_id,
                "seasons",
                season_number,
                "episodes",
                episode_number,
                "comments"
            ),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn episode_lists(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
        f: impl FnOnce(ListFactory) -> ListFactory,
        page: u32,
        limit: u32,
    ) -> Result<Vec<List>> {
        let list_factory = f(ListFactory::default());
        self.get(api_url!(
            (
                "shows",
                show_id,
                "seasons",
                season_number,
                "episodes",
                episode_number,
                "lists",
                list_factory.list_filter,
                list_factory.sorting
            ),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn episode_ratings(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
    ) -> Result<Ratings> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "episodes",
            episode_number,
            "ratings"
        )))
    }

    pub fn episode_stats(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
    ) -> Result<MediaStats> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "episodes",
            episode_number,
            "stats"
        )))
    }

    pub fn episode_watching(
        &self,
        show_id: impl Display,
        season_number: u32,
        episode_number: u32,
    ) -> Result<Vec<User>> {
        self.get(api_url!((
            "shows",
            show_id,
            "seasons",
            season_number,
            "episodes",
            episode_number,
            "watching"
        )))
    }
}
