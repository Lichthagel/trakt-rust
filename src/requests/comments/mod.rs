pub mod comment_create_request;
pub mod comment_post_request;

pub use crate::requests::comments::comment_create_request::CommentCreateRequest;
pub use crate::requests::comments::comment_post_request::CommentPostRequest;

use crate::{
    error::Result,
    models::{AllCommentableItemType, Comment, CommentAndItem, CommentItem, CommentType, Like},
    pagination::PaginationFactory,
    TraktApi,
};

impl TraktApi {
    pub fn comment_create<'a>(&'a self, comment: &'a str) -> CommentCreateRequest<'a> {
        CommentCreateRequest::new(self, api_url!(("comments")), comment)
    }

    pub fn comment(&self, id: u32) -> Result<Comment> {
        self.get(api_url!(("comments", id)))
    }

    pub fn comment_update(&self, comment_id: u32, comment: String) -> CommentPostRequest {
        CommentPostRequest::new(self, api_url!(("comments", comment_id)), true, comment)
    }

    pub fn comment_delete(&self, comment_id: u32, access_token: &str) -> Result<()> {
        self.auth_delete(api_url!(("comments", comment_id)), access_token)
    }

    pub fn replies(
        &self,
        comment_id: u32,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<Comment>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("comments", comment_id, "replies"),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn replies_post(&self, comment_id: u32, comment: String) -> CommentPostRequest {
        CommentPostRequest::new(
            self,
            api_url!(("comments", comment_id, "replies")),
            false,
            comment,
        )
    }

    pub fn comment_item(&self, comment_id: u32) -> Result<CommentItem> {
        self.get(api_url!(("comments", comment_id, "item")))
    }

    pub fn comment_likes(
        &self,
        comment_id: u32,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<Like>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            ("comments", comment_id, "likes"),
            ("page", pf.page),
            ("limit", pf.limit)
        ))
    }

    pub fn comment_like(&self, comment_id: u32, access_token: &str) -> Result<()> {
        self.auth_post_no_body(
            api_url!(("comments", comment_id, "like")),
            String::from(""),
            access_token,
        )
    }

    pub fn comment_like_delete(&self, comment_id: u32, access_token: &str) -> Result<()> {
        self.auth_delete(api_url!(("comments", comment_id, "like")), access_token)
    }

    pub fn comments_trending(
        &self,
        comment_type: CommentType,
        item_type: AllCommentableItemType,
        include_replies: bool,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<CommentAndItem>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            (
                "comments",
                "trending",
                comment_type.to_string(),
                item_type.to_string()
            ),
            ("page", pf.page),
            ("limit", pf.limit),
            ("include_replies", include_replies)
        ))
    }

    pub fn comments_recent(
        &self,
        comment_type: CommentType,
        item_type: AllCommentableItemType,
        include_replies: bool,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<CommentAndItem>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            (
                "comments",
                "recent",
                comment_type.to_string(),
                item_type.to_string()
            ),
            ("page", pf.page),
            ("limit", pf.limit),
            ("include_replies", include_replies)
        ))
    }

    pub fn comments_updates(
        &self,
        comment_type: CommentType,
        item_type: AllCommentableItemType,
        include_replies: bool,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
    ) -> Result<Vec<CommentAndItem>> {
        let pf = f(PaginationFactory::default());
        self.get(api_url!(
            (
                "comments",
                "updates",
                comment_type.to_string(),
                item_type.to_string()
            ),
            ("page", pf.page),
            ("limit", pf.limit),
            ("include_replies", include_replies)
        ))
    }
}
