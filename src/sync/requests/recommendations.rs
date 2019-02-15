use crate::{models::Movie, Result, TraktApi};
use std::fmt::Display;

impl TraktApi {
    pub fn recommendations_movie(&self, access_token: &str) -> Result<Vec<Movie>> {
        self.auth_get(api_url!(("recommendations", "movies")), access_token)
    }

    pub fn recommendations_movie_hide(&self, id: impl Display, access_token: &str) -> Result<()> {
        self.auth_delete(api_url!(("recommendations", "movies", id)), access_token)
    }

    pub fn recommendations_show(&self, access_token: &str) -> Result<Vec<Movie>> {
        self.auth_get(api_url!(("recommendations", "shows")), access_token)
    }

    pub fn recommendations_show_hide(&self, id: impl Display, access_token: &str) -> Result<()> {
        self.auth_delete(api_url!(("recommendations", "shows", id)), access_token)
    }
}
