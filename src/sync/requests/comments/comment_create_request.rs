use crate::{
    models::Comment,
    selectors::{SelectEpisode, SelectList, SelectMovie, SelectSeason, SelectShow},
    Error, Result, TraktApi,
};
use reqwest::{Method, Request};
use serde_json::{Map, Value};

pub struct CommentCreateRequest<'a> {
    client: &'a TraktApi<'a>,
    url: String,
    body: Map<String, Value>,
}

impl<'a> CommentCreateRequest<'a> {
    pub fn new(client: &'a TraktApi, url: String, comment: &'a str) -> Self {
        let mut m = Map::new();
        m.insert("comment".to_owned(), Value::String(comment.to_owned()));
        Self {
            client,
            url,
            body: m,
        }
    }

    pub fn spoiler(mut self) -> Self {
        self.body.insert("spoiler".to_owned(), Value::Bool(true));
        self
    }

    pub fn sharing(mut self, network: String) -> Self {
        match self.body.get_mut("sharing") {
            Some(sharing) => {
                if let Some(sharing) = sharing.as_object_mut() {
                    sharing.insert(network, Value::Bool(true));
                }
            }
            None => {
                let mut m = Map::new();
                m.insert(network, Value::Bool(true));
                self.body.insert("sharing".to_owned(), Value::Object(m));
            }
        }
        self
    }

    pub fn twitter(self) -> Self {
        self.sharing("twitter".to_owned())
    }

    pub fn facebook(self) -> Self {
        self.sharing("facebook".to_owned())
    }

    pub fn tumblr(self) -> Self {
        self.sharing("tumblr".to_owned())
    }

    pub fn medium(self) -> Self {
        self.sharing("medium".to_owned())
    }

    pub fn build(&mut self, access_token: &str) -> std::result::Result<Request, Error> {
        self.client
            .builder(Method::POST, self.url.to_owned())
            .header("Authorization", format!("Bearer {}", access_token))
            .body(serde_json::to_string(&self.body).unwrap())
            .build()
            .map_err(Error::from)
    }

    pub fn execute(mut self, access_token: &str) -> Result<Comment> {
        self.client.execute(self.build(access_token)?)
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
