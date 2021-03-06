use crate::{
    asyn::{
        pagination::PaginationRequest,
        requests::comments::comment_post_request::CommentPostRequest, Result, TraktApi,
    },
    models::{Comment, CommentAndItem, CommentItem, FullComment, FullCommentAndItem, Like, ToId},
};

pub trait CommentMethods<'b>: ToId<'b, u32> {
    fn update_async<'a>(&'b self, client: &'a TraktApi, comment: String) -> CommentPostRequest<'a> {
        client.comment_update(self.id(), comment)
    }

    fn delete_async(&'b self, client: &TraktApi, access_token: &str) -> Result<()> {
        client.comment_delete(self.id(), access_token)
    }

    fn replies_async<'a>(&'b self, client: &'a TraktApi) -> PaginationRequest<'a, Comment> {
        client.replies(self.id())
    }

    fn reply_post_async<'a>(
        &'b self,
        client: &'a TraktApi,
        comment: String,
    ) -> CommentPostRequest<'a> {
        client.replies_post(self.id(), comment)
    }

    fn item_async(&'b self, client: &TraktApi) -> Result<CommentItem> {
        client.comment_item(self.id())
    }

    fn likes_async<'a>(&'b self, client: &'a TraktApi) -> PaginationRequest<'a, Like> {
        client.comment_likes(self.id())
    }

    fn like_async(&'b self, client: &TraktApi, access_token: &str) -> Result<()> {
        client.comment_like(self.id(), access_token)
    }

    fn like_delete_async(&'b self, client: &TraktApi, access_token: &str) -> Result<()> {
        client.comment_like_delete(self.id(), access_token)
    }

    fn fetch(&'b self, client: &TraktApi) -> Result<Comment> {
        client.comment(self.id())
    }
}

impl<'a> CommentMethods<'a> for Comment {}
impl<'a> CommentMethods<'a> for FullComment {}
impl<'a> CommentMethods<'a> for CommentAndItem {}
impl<'a> CommentMethods<'a> for FullCommentAndItem {}
