use crate::{
    error::Result,
    models::{
        AllCommentableItemType, Comment, CommentAndItem, CommentItem, CommentNew, CommentType,
        CommentPost, Like,
    },
    TraktApi,
};

impl TraktApi {
    pub fn comment_post(&self, comment: CommentNew, access_token: String) -> Result<Comment> {
        self.auth_post(
            api_url!(("comments")),
            comment.to_json_string()?,
            access_token,
        )
    }

    pub fn comment(&self, id: u32) -> Result<Comment> {
        self.get(api_url!(("comments", id)))
    }

    pub fn comment_update(
        &self,
        comment_id: u32,
        comment_update: CommentPost,
        access_token: String,
    ) -> Result<Comment> {
        self.auth_put(
            api_url!(("comments", comment_id)),
            comment_update.to_json_string()?,
            access_token
        )
    }

    pub fn comment_delete(
        &self,
        comment_id: u32,
        access_token: String
    ) -> Result<()> {
        self.auth_delete(
            api_url!(("comments", comment_id)),
            access_token
        )
    }

    pub fn replies(&self, comment_id: u32, page: u32, limit: u32) -> Result<Vec<Comment>> {
        self.get(api_url!(
            ("comments", comment_id, "replies"),
            ("page", page),
            ("limit", limit)
        ))
    }

    pub fn replies_post(
        &self,
        comment_id: u32,
        comment: CommentPost,
        access_token: String
    ) -> Result<Comment> {
        self.auth_post(
            api_url!(("comments", comment_id, "replies")),
            comment.to_json_string()?,
            access_token
        )
    }

    pub fn comment_item(&self, comment_id: u32) -> Result<CommentItem> {
        self.get(api_url!(("comments", comment_id, "item")))
    }

    pub fn comment_likes(&self, comment_id: u32, page: u32, limit: u32) -> Result<Vec<Like>> {
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
    ) -> Result<Vec<CommentAndItem>> {
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
    ) -> Result<Vec<CommentAndItem>> {
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
    ) -> Result<Vec<CommentAndItem>> {
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
