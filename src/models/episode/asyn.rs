use crate::{
    asyn::{pagination::PaginationRequest, Result, TraktApi},
    models::{
        Comment, Episode, FullEpisode, FullUser, List, ListFactory, MediaStats, Ratings, ToId,
        Translation, User,
    },
};
use std::fmt::Display;

pub trait EpisodeMethods: ToId<(u32, u32)> {
    fn translations(
        &self,
        client: &TraktApi,
        show: impl Display,
        language: impl Display,
    ) -> Result<Vec<Translation>> {
        client.episode_translations(show, self.id().0, self.id().1, language)
    }

    fn comments<'a>(
        &self,
        client: &'a TraktApi,
        show: impl Display,
    ) -> PaginationRequest<'a, Comment> {
        client.episode_comments(show, self.id().0, self.id().1)
    }

    fn lists<'a>(
        &self,
        client: &'a TraktApi,
        show: impl Display,
        f: impl FnOnce(ListFactory) -> ListFactory,
    ) -> PaginationRequest<'a, List> {
        client.episode_lists(show, self.id().0, self.id().1, f)
    }

    fn ratings(&self, client: &TraktApi, show: impl Display) -> Result<Ratings> {
        client.episode_ratings(show, self.id().0, self.id().1)
    }

    fn stats(&self, client: &TraktApi, show: impl Display) -> Result<MediaStats> {
        client.episode_stats(show, self.id().0, self.id().1)
    }

    fn watching(&self, client: &TraktApi, show: impl Display) -> Result<Vec<User>> {
        client.episode_watching(show, self.id().0, self.id().1)
    }

    fn watching_full(&self, client: &TraktApi, show: impl Display) -> Result<Vec<FullUser>> {
        client.episode_watching_full(show, self.id().0, self.id().1)
    }

    fn fetch(&self, client: &TraktApi, show: impl Display) -> Result<Episode> {
        client.episode(show, self.id().0, self.id().1)
    }

    fn fetch_full(&self, client: &TraktApi, show: impl Display) -> Result<FullEpisode> {
        client.episode_full(show, self.id().0, self.id().1)
    }
}

impl EpisodeMethods for Episode {}
impl EpisodeMethods for FullEpisode {}
