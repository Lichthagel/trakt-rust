use crate::{
    asyn::{pagination::PaginationRequest, Result, TraktApi},
    models::{
        comment::GetComments,
        like::{LikeableType, UserLike},
        user::{FollowRequest, FollowRequestApprove, FullUser, Settings},
        CollectionMovie, CollectionShow, CommentAndItem, User,
    },
};
use reqwest::Method;

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
        access_token: &str,
    ) -> PaginationRequest<UserLike> {
        PaginationRequest::new(
            self,
            self.builder(
                Method::GET,
                match item_type {
                    Some(item_type) => api_url!(("users", "likes", item_type)),
                    None => api_url!(("users", "likes")),
                },
            )
            .header("Authorization", format!("Bearer {}", access_token)),
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
        access_token: Option<&str>,
    ) -> PaginationRequest<CommentAndItem> {
        let gc = f(GetComments::default());

        PaginationRequest::new(
            self,
            match access_token {
                Some(access_token) => self
                    .builder(
                        Method::GET,
                        api_url!(("users", slug, "comments", gc.comment_type, gc.item_type)),
                    )
                    .query(&[("include_replies", gc.include_replies)])
                    .header("Authorization", format!("Bearer {}", access_token)),
                None => self
                    .builder(
                        Method::GET,
                        api_url!(("users", slug, "comments", gc.comment_type, gc.item_type)),
                    )
                    .query(&[("include_replies", gc.include_replies)]),
            },
        )
    }
}
