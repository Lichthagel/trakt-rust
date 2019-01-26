use crate::error::Result;
use crate::models::user::Settings;
use crate::TraktApi;

impl TraktApi {
    pub fn user_settings(&self, access_token: String) -> Result<Settings> {
        self.auth_get(api_url!(("users", "settings")), access_token)
    }
}
