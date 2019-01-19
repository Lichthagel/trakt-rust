use crate::{
    error::Error,
    models::{AllCommentableItemType, Comment, CommentAndItem, CommentItem, CommentType, Like},
    TraktApi,
};

impl TraktApi {
    pub fn comments(&self, id: u32) -> Result<Comment, Error> {
        self.get(api_url!(("comments", id)))
    }

    pub fn replies(&self, comment_id: u32, page: u32, limit: u32) -> Result<Vec<Comment>, Error> {
        self.get(api_url!(
            ("comments", comment_id, "replies"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn comment_item(&self, comment_id: u32) -> Result<CommentItem, Error> {
        self.get(api_url!(("comments", comment_id, "item")))
    }

    pub fn comment_likes(
        &self,
        comment_id: u32,
        page: u32,
        limit: u32,
    ) -> Result<Vec<Like>, Error> {
        self.get(api_url!(
            ("comments", comment_id, "likes"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn comments_trending(
        &self,
        comment_type: CommentType,
        item_type: AllCommentableItemType,
        include_replies: bool,
        page: u32,
        limit: u32,
    ) -> Result<Vec<CommentAndItem>, Error> {
        self.get(api_url!(
            (
                "comments",
                "trending",
                comment_type.to_string(),
                item_type.to_string()
            ),
            ("page", page),
            ("limit", limit),
            ("include_replies", include_replies)
        ))
    }

    pub fn comments_recent(
        &self,
        comment_type: CommentType,
        item_type: AllCommentableItemType,
        include_replies: bool,
        page: u32,
        limit: u32,
    ) -> Result<Vec<CommentAndItem>, Error> {
        self.get(api_url!(
            (
                "comments",
                "recent",
                comment_type.to_string(),
                item_type.to_string()
            ),
            ("page", page),
            ("limit", limit),
            ("include_replies", include_replies)
        ))
    }

    pub fn comments_updates(
        &self,
        comment_type: CommentType,
        item_type: AllCommentableItemType,
        include_replies: bool,
        page: u32,
        limit: u32,
    ) -> Result<Vec<CommentAndItem>, Error> {
        self.get(api_url!(
            (
                "comments",
                "updates",
                comment_type.to_string(),
                item_type.to_string()
            ),
            ("page", page),
            ("limit", limit),
            ("include_replies", include_replies)
        ))
    }
}
