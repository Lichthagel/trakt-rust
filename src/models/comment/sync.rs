use crate::{
    models::{Comment, CommentAndItem, CommentItem, FullComment, FullCommentAndItem, Like, ToId},
    sync::{
        pagination::PaginationRequest, requests::comments::comment_post_request::CommentPostRequest,
    },
    Result, TraktApi,
};

pub trait CommentMethods: ToId<u32> {
    fn update<'a>(&self, client: &'a TraktApi, comment: String) -> CommentPostRequest<'a> {
        client.comment_update(self.id(), comment)
    }

    fn delete(&self, client: &TraktApi, access_token: &str) -> Result<()> {
        client.comment_delete(self.id(), access_token)
    }

    fn replies<'a>(&self, client: &'a TraktApi) -> PaginationRequest<'a, Comment> {
        client.replies(self.id())
    }

    fn reply_post<'a>(&self, client: &'a TraktApi, comment: String) -> CommentPostRequest<'a> {
        client.replies_post(self.id(), comment)
    }

    fn item(&self, client: &TraktApi) -> Result<CommentItem> {
        client.comment_item(self.id())
    }

    fn likes<'a>(&self, client: &'a TraktApi) -> PaginationRequest<'a, Like> {
        client.comment_likes(self.id())
    }

    fn like(&self, client: &TraktApi, access_token: &str) -> Result<()> {
        client.comment_like(self.id(), access_token)
    }

    fn like_delete(&self, client: &TraktApi, access_token: &str) -> Result<()> {
        client.comment_like_delete(self.id(), access_token)
    }

    fn fetch(&self, client: &TraktApi) -> Result<Comment> {
        client.comment(self.id())
    }
}

impl CommentMethods for Comment {}
impl CommentMethods for FullComment {}
impl CommentMethods for CommentAndItem {}
impl CommentMethods for FullCommentAndItem {}
