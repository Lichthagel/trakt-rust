//! All models related to [authentication]
//!
//! [authentication]: https://trakt.docs.apiary.io/#reference/authentication-oauth

#[cfg(feature = "async")]
use crate::asyn::{Result as AsyncResult, TraktApi as AsyncTraktApi};
#[cfg(feature = "sync")]
use crate::{Result, TraktApi};

/// The device codes required for device authentication
///
/// Look [here]
///
/// [here]: https://trakt.docs.apiary.io/#reference/authentication-devices/device-code/generate-new-device-codes
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthenticationDevices {
    pub device_code: String,
    pub user_code: String,
    pub verification_url: String,
    pub expires_in: u64,
    pub interval: u64,
}

impl AuthenticationDevices {
    #[cfg(feature = "sync")]
    pub fn poll(&self, client: &TraktApi) -> Result<AuthenticationTokenResponse> {
        client.oauth_device_token(&self.device_code)
    }

    #[cfg(feature = "async")]
    pub fn poll_async(&self, client: &AsyncTraktApi) -> AsyncResult<AuthenticationTokenResponse> {
        client.oauth_device_token(&self.device_code)
    }
}

/// The response of getting an access_token
///
/// Look [here]
///
/// [here]: https://trakt.docs.apiary.io/#reference/authentication-oauth/get-token/exchange-code-for-access_token
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthenticationTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub scope: String,
    pub created_at: u64,
}
