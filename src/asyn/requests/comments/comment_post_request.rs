use crate::{asyn::TraktApi, models::Comment, Error, asyn::Result};
use reqwest::{r#async::Request, Method};
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

    pub fn build(&self, access_token: &'a str) -> std::result::Result<Request, Error> {
        let mut m = Map::new();
        m.insert("comment".to_owned(), Value::String(self.comment.clone()));
        m.insert("spoiler".to_owned(), Value::Bool(self.spoiler));

        serde_json::to_string(&m)
            .map_err(Error::from)
            .and_then(|body| {
                self.client
                    .builder(
                        if self.method {
                            Method::PUT
                        } else {
                            Method::POST
                        },
                        self.url.to_owned(),
                    )
                    .body(body)
                    .header("Authorization", format!("Bearer {}", access_token))
                    .build()
                    .map_err(Error::from)
            })
    }

    pub fn execute(self, access_token: &'a str) -> Result<Comment> {
        match self.build(access_token) {
            Ok(req) => self.client.execute(req),
            Err(e) => Box::new(futures::future::err(e)),
        }
    }
}
