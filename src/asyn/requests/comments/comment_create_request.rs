use crate::{
    asyn::{Result, TraktApi},
    models::{Comment, CommentSharing},
    selectors::{SelectEpisode, SelectList, SelectMovie, SelectSeason, SelectShow},
};
use serde_json::{Map, Value};

pub struct CommentCreateRequest<'a> {
    client: &'a TraktApi,
    url: String,
    body: Map<String, Value>,
    sharing: CommentSharing,
}

impl<'a> CommentCreateRequest<'a> {
    pub fn new(client: &'a TraktApi, url: String, comment: &'a str) -> Self {
        let mut m = Map::new();
        m.insert("comment".to_owned(), Value::String(comment.to_owned()));
        Self {
            client,
            url,
            body: m,
            sharing: CommentSharing::new(false, false, false, false),
        }
    }

    pub fn spoiler(mut self) -> Self {
        self.body.insert("spoiler".to_owned(), Value::Bool(true));
        self
    }

    pub fn twitter(mut self) -> Self {
        self.sharing.twitter = true;
        self
    }

    pub fn facebook(mut self) -> Self {
        self.sharing.facebook = true;
        self
    }

    pub fn tumblr(mut self) -> Self {
        self.sharing.tumblr = true;
        self
    }

    pub fn medium(mut self) -> Self {
        self.sharing.medium = true;
        self
    }

    pub fn execute(mut self, access_token: &str) -> Result<Comment> {
        self.body.insert(
            "sharing".to_owned(),
            serde_json::to_value(self.sharing).unwrap(),
        );
        self.client._auth_post(
            &self.url,
            dbg!(serde_json::to_string(&self.body).unwrap()),
            access_token,
        )
    }
}

impl<'a> SelectMovie for CommentCreateRequest<'a> {
    fn movie_value(mut self, movie: Value) -> Self {
        self.body.insert("movie".to_owned(), movie);
        self
    }
}

impl<'a> SelectEpisode for CommentCreateRequest<'a> {
    fn episode_value(mut self, episode: Value) -> Self {
        self.body.insert("episode".to_ascii_lowercase(), episode);
        self
    }
}

impl<'a> SelectShow for CommentCreateRequest<'a> {
    fn show_value(mut self, show: Value) -> Self {
        self.body.insert("show".to_owned(), show);
        self
    }
}

impl<'a> SelectSeason for CommentCreateRequest<'a> {
    fn season_value(mut self, season: Value) -> Self {
        self.body.insert("season".to_owned(), season);
        self
    }
}

impl<'a> SelectList for CommentCreateRequest<'a> {
    fn list_value(mut self, list: Value) -> Self {
        self.body.insert("list".to_owned(), list);
        self
    }
}
