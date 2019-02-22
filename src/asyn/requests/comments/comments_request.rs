use crate::{
    asyn::{Result, TraktApi},
    extended_info::{ExtendedInfoFull, ExtendedInfoNone, WithFull, WithNone},
    models::{AllCommentableItemType, CommentType},
    pagination::Pagination,
};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub struct CommentsRequest<'a, T> {
    client: &'a TraktApi<'a>,
    url: &'a str,
    comment_type: CommentType,
    item_type: AllCommentableItemType,
    include_replies: bool,
    page: u32,
    limit: u32,
    full: bool,
    response_type: PhantomData<T>,
}

impl<'a, T: DeserializeOwned + Send + 'static> CommentsRequest<'a, T> {
    pub fn new(client: &'a TraktApi, url: &'a str) -> Self {
        Self {
            client,
            url,
            comment_type: CommentType::All,
            item_type: AllCommentableItemType::All,
            include_replies: false,
            page: 1,
            limit: 10,
            full: false,
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
        self.include_replies = true;
        self
    }

    pub fn execute(self) -> Result<Vec<T>> {
        self.client.get(if self.full {
            api_url!(
                ("comments", self.url, self.comment_type, self.item_type),
                ("page", self.page),
                ("limit", self.limit),
                ("extended", "full")
            )
        } else {
            api_url!(
                ("comments", self.url, self.comment_type, self.item_type),
                ("page", self.page),
                ("limit", self.limit)
            )
        })
    }
}

impl<'a, T> Pagination for CommentsRequest<'a, T> {
    fn page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
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
    fn full(self) -> Self::Full {
        Self::Full {
            client: self.client,
            url: self.url,
            comment_type: self.comment_type,
            item_type: self.item_type,
            include_replies: self.include_replies,
            page: self.page,
            limit: self.limit,
            full: true,
            response_type: PhantomData,
        }
    }
}

impl<'a, T: WithNone> ExtendedInfoNone for CommentsRequest<'a, T> {
    fn none(self) -> Self::None {
        Self::None {
            client: self.client,
            url: self.url,
            comment_type: self.comment_type,
            item_type: self.item_type,
            include_replies: self.include_replies,
            page: self.page,
            limit: self.limit,
            full: false,
            response_type: PhantomData,
        }
    }
}
