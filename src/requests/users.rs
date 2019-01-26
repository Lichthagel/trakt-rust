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
}
