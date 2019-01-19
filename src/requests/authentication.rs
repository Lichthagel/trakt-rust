use crate::{
    error::Error,
    models::{AuthenticationDevices, AuthenticationTokenResponse},
    TraktApi,
};
use serde_json::json;

impl TraktApi {
    pub fn authenticate_devices(&self) -> Result<AuthenticationDevices, Error> {
        self.post(
            api_url!(("oauth/device/code")),
            json!({"client_id": self.client_id}).to_string(),
        )
    }

    pub fn get_token(&self, device_code: String) -> Result<AuthenticationTokenResponse, Error> {
        if self.client_secret == None {
            return Err(Error::ClientSecretNeeded);
        }

        self.post(
            api_url!(("oauth/device/token")),
            json!({
                "code": device_code,
                "client_id": self.client_id,
                "client_secret": self.client_secret
            })
            .to_string(),
        )
    }
}
