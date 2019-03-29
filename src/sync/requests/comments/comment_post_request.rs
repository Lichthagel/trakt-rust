use crate::{models::Comment, Error, Result, TraktApi};
use reqwest::{Method, Request};
use serde_json::{Map, Value};

pub struct CommentPostRequest<'a> {
    client: &'a TraktApi<'a>,
    url: String,
    // false => POST, true => PUT
    method: bool,
    comment: String,
    spoiler: bool,
}

impl<'a> CommentPostRequest<'a> {
    pub fn new(client: &'a TraktApi, url: String, method: bool, comment: String) -> Self {
        Self {
            client,
            url,
            method,
            comment,
            spoiler: false,
        }
    }

    pub fn spoiler(mut self) -> Self {
        self.spoiler = true;
        self
    }

    pub fn build(&self, access_token: &str) -> Result<Request> {
        let mut m = Map::new();
        m.insert("comment".to_owned(), Value::String(self.comment.to_owned()));
        m.insert("spoiler".to_owned(), Value::Bool(self.spoiler));

        self.client
            .builder(
                if self.method {
                    Method::PUT
                } else {
                    Method::POST
                },
                self.url.to_owned(),
            )
            .body(serde_json::to_string(&m)?)
            .bearer_auth(access_token)
            .build()
            .map_err(Error::from)
    }

    pub fn execute(self, access_token: &str) -> Result<Comment> {
        self.client.execute(self.build(access_token)?)
    }
}
