use crate::{
    asyn::{Result, TraktApi},
    extended_info::{ExtendedInfoFull, ExtendedInfoNone, WithFull, WithNone},
    models::{AllCommentableItemType, CommentType},
    pagination::Pagination,
    Error,
};
use hashbrown::HashMap;
use reqwest::{r#async::Request, Method};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub struct CommentsRequest<'a, T> {
    client: &'a TraktApi<'a>,
    url: &'a str,
    comment_type: CommentType,
    item_type: AllCommentableItemType,
    query: HashMap<String, String>,
    response_type: PhantomData<T>,
}

impl<'a, T: DeserializeOwned + Send + 'static> CommentsRequest<'a, T> {
    pub fn new(client: &'a TraktApi, url: &'a str) -> Self {
        Self {
            client,
            url,
            comment_type: CommentType::All,
            item_type: AllCommentableItemType::All,
            query: HashMap::new(),
            response_type: PhantomData,
        }
    }

    fn comment_type(mut self, comment_type: CommentType) -> Self {
        self.comment_type = comment_type;
        self
    }

    pub fn shouts(self) -> Self {
        self.comment_type(CommentType::Shouts)
    }

    pub fn reviews(self) -> Self {
        self.comment_type(CommentType::Reviews)
    }

    fn item_type(mut self, item_type: AllCommentableItemType) -> Self {
        self.item_type = item_type;
        self
    }

    pub fn movie(self) -> Self {
        self.item_type(AllCommentableItemType::Movie)
    }

    pub fn show(self) -> Self {
        self.item_type(AllCommentableItemType::Show)
    }

    pub fn season(self) -> Self {
        self.item_type(AllCommentableItemType::Season)
    }

    pub fn episode(self) -> Self {
        self.item_type(AllCommentableItemType::Episode)
    }

    pub fn list(self) -> Self {
        self.item_type(AllCommentableItemType::List)
    }

    pub fn include_replies(mut self) -> Self {
        self.query
            .insert("include_replies".to_owned(), "true".to_owned());
        self
    }

    pub fn build(&self) -> std::result::Result<Request, Error> {
        let mut url = format!(
            "/comments/{}/{}/{}",
            self.url, self.comment_type, self.item_type
        );

        if !self.query.is_empty() {
            url.push('?');
            url.push_str(&serde_urlencoded::to_string(&self.query)?)
        }

        self.client
            .builder(Method::GET, url)
            .build()
            .map_err(Error::from)
    }

    pub fn execute(self) -> Result<Vec<T>> {
        match self.build() {
            Ok(req) => self.client.execute(req),
            Err(e) => Box::new(futures::future::err(e)),
        }
    }
}

impl<'a, T> Pagination for CommentsRequest<'a, T> {
    fn page(mut self, page: u32) -> Self {
        self.query.insert("page".to_owned(), format!("{}", page));
        self
    }

    fn limit(mut self, limit: u32) -> Self {
        self.query.insert("limit".to_owned(), format!("{}", limit));
        self
    }
}

impl<'a, T: WithFull> WithFull for CommentsRequest<'a, T> {
    type Full = CommentsRequest<'a, T::Full>;
}

impl<'a, T: WithNone> WithNone for CommentsRequest<'a, T> {
    type None = CommentsRequest<'a, T::None>;
}

impl<'a, T: WithFull> ExtendedInfoFull for CommentsRequest<'a, T> {
    fn full(mut self) -> Self::Full {
        self.query.insert("extended".to_owned(), "full".to_owned());

        Self::Full {
            client: self.client,
            url: self.url,
            comment_type: self.comment_type,
            item_type: self.item_type,
            query: self.query,
            response_type: PhantomData,
        }
    }
}

impl<'a, T: WithNone> ExtendedInfoNone for CommentsRequest<'a, T> {
    fn none(mut self) -> Self::None {
        self.query.remove("extended");

        Self::None {
            client: self.client,
            url: self.url,
            comment_type: self.comment_type,
            item_type: self.item_type,
            query: self.query,
            response_type: PhantomData,
        }
    }
}

#[cfg(tests)]
mod tests {

    #[test]
    fn comments_request() {}

}
