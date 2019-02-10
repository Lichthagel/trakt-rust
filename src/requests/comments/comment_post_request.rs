use crate::{error::Result, models::Comment, TraktApi};
use serde_json::{Map, Value};

pub struct CommentPostRequest<'a> {
    client: &'a TraktApi,
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

    pub fn execute(self, access_token: &str) -> Result<Comment> {
        let mut m = Map::new();
        m.insert("comment".to_owned(), Value::String(self.comment));
        m.insert("spoiler".to_owned(), Value::Bool(self.spoiler));

        if self.method {
            self.client
                .auth_put(self.url, serde_json::to_string(&m)?, access_token)
        } else {
            self.client
                .auth_post(self.url, serde_json::to_string(&m)?, access_token)
        }
    }
}
