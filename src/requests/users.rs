use crate::models::comment::GetComments;
use crate::models::CommentAndItem;
use crate::{
    error::Result,
    models::{
        like::{LikeableType, UserLike},
        user::{FollowRequest, FollowRequestApprove, FullUser, Settings},
        CollectionMovie, CollectionShow, User,
    },
    pagination::PaginationFactory,
    TraktApi,
};

impl TraktApi {
    pub fn user_settings(&self, access_token: &str) -> Result<Settings> {
        self.auth_get(api_url!(("users", "settings")), access_token)
    }

    pub fn user_requests(&self, access_token: &str) -> Result<Vec<FollowRequest>> {
        self.auth_get(api_url!(("users", "requests")), access_token)
    }

    pub fn user_request_approve(
        &self,
        id: u32,
        access_token: &str,
    ) -> Result<FollowRequestApprove> {
        self.auth_post(
            api_url!(("users", "requests", id)),
            "".to_owned(),
            access_token,
        )
    }

    pub fn user_request_deny(&self, id: u32, access_token: &str) -> Result<()> {
        self.auth_delete(api_url!(("users", "requests", id)), access_token)
    }

    // TODO hidden items

    pub fn user_likes(
        &self,
        item_type: Option<LikeableType>,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
        access_token: &str,
    ) -> Result<Vec<UserLike>> {
        let pf = f(PaginationFactory::default());

        self.auth_get(
            match item_type {
                Some(item_type) => api_url!(
                    ("users", "likes", item_type),
                    ("page", pf.page),
                    ("limit", pf.limit)
                ),
                None => api_url!(("users", "likes"), ("page", pf.page), ("limit", pf.limit)),
            },
            access_token,
        )
    }

    pub fn user_profile(&self, slug: &str, access_token: Option<&str>) -> Result<User> {
        match access_token {
            Some(access_token) => self.auth_get(api_url!(("users", slug)), access_token),
            None => self.get(api_url!(("users", slug))),
        }
    }

    pub fn user_profile_full(&self, slug: &str, access_token: Option<&str>) -> Result<FullUser> {
        match access_token {
            Some(access_token) => self.auth_get(
                api_url!(("users", slug), ("extended", "full")),
                access_token,
            ),
            None => self.get(api_url!(("users", slug), ("extended", "full"))),
        }
    }

    pub fn user_collection_movies(
        &self,
        slug: &str,
        access_token: Option<&str>,
    ) -> Result<Vec<CollectionMovie>> {
        match access_token {
            Some(access_token) => self.auth_get(
                api_url!(("users", slug, "collection", "movies")),
                access_token,
            ),
            None => self.get(api_url!(("users", slug, "collection", "movies"))),
        }
    }

    pub fn user_collection_shows(
        &self,
        slug: &str,
        access_token: Option<&str>,
    ) -> Result<Vec<CollectionShow>> {
        match access_token {
            Some(access_token) => self.auth_get(
                api_url!(("users", slug, "collection", "shows")),
                access_token,
            ),
            None => self.get(api_url!(("users", slug, "collection", "shows"))),
        }
    }

    pub fn user_comments(
        &self,
        slug: &str,
        f: impl FnOnce(GetComments) -> GetComments,
        g: impl FnOnce(PaginationFactory) -> PaginationFactory,
        access_token: Option<&str>,
    ) -> Result<Vec<CommentAndItem>> {
        let gc = f(GetComments::default());
        let pf = g(PaginationFactory::default());

        match access_token {
            Some(access_token) => self.auth_get(
                api_url!(
                    ("users", slug, "comments", gc.comment_type, gc.item_type),
                    ("include_replies", gc.include_replies),
                    ("page", pf.page),
                    ("limit", pf.limit)
                ),
                access_token,
            ),
            None => self.get(api_url!(
                ("users", slug, "comments", gc.comment_type, gc.item_type),
                ("include_replies", gc.include_replies),
                ("page", pf.page),
                ("limit", pf.limit)
            )),
        }
    }
}
