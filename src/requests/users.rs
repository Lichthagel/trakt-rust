use crate::models::like::LikeableType;
use crate::models::like::UserLike;
use crate::models::user::FullUser;
use crate::models::User;
use crate::pagination::PaginationFactory;
use crate::{
    error::Result,
    models::user::{FollowRequest, FollowRequestApprove, Settings},
    TraktApi,
};

impl TraktApi {
    pub fn user_settings(&self, access_token: String) -> Result<Settings> {
        self.auth_get(api_url!(("users", "settings")), access_token)
    }

    pub fn user_requests(&self, access_token: String) -> Result<Vec<FollowRequest>> {
        self.auth_get(api_url!(("users", "requests")), access_token)
    }

    pub fn user_request_approve(
        &self,
        id: u32,
        access_token: String,
    ) -> Result<FollowRequestApprove> {
        self.auth_post(
            api_url!(("users", "requests", id)),
            "".to_owned(),
            access_token,
        )
    }

    pub fn user_request_deny(&self, id: u32, access_token: String) -> Result<()> {
        self.auth_delete(api_url!(("users", "requests", id)), access_token)
    }

    // TODO hidden items

    pub fn user_likes(
        &self,
        item_type: Option<LikeableType>,
        f: impl FnOnce(PaginationFactory) -> PaginationFactory,
        access_token: String,
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

    pub fn user_profile(&self, slug: String, access_token: Option<String>) -> Result<User> {
        match access_token {
            Some(access_token) => self.auth_get(api_url!(("users", slug)), access_token),
            None => self.get(api_url!(("users", slug))),
        }
    }

    pub fn user_profile_full(
        &self,
        slug: String,
        access_token: Option<String>,
    ) -> Result<FullUser> {
        match access_token {
            Some(access_token) => self.auth_get(
                api_url!(("users", slug), ("extended", "full")),
                access_token,
            ),
            None => self.get(api_url!(("users", slug), ("extended", "full"))),
        }
    }
}
