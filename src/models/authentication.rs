//! All models related to [authentication]
//!
//! [authentication]: https://trakt.docs.apiary.io/#reference/authentication-oauth

/// The body for getting an access_token.
///
/// Look [here]
///
/// [here]: https://trakt.docs.apiary.io/#reference/authentication-oauth/get-token/exchange-code-for-access_token
#[derive(Debug, Serialize, Deserialize)]
pub struct Authentication {
    pub code: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: Option<String>,
    pub grant_type: Option<String>,
}

/// The device codes required for device authentication
///
/// Look [here]
///
/// [here]: https://trakt.docs.apiary.io/#reference/authentication-devices/device-code/generate-new-device-codes
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationDevices {
    pub device_code: String,
    pub user_code: String,
    pub verification_url: String,
    pub expires_in: u64,
    pub interval: u64,
}

/// The response of getting an access_token
///
/// Look [here]
///
/// [here]: https://trakt.docs.apiary.io/#reference/authentication-oauth/get-token/exchange-code-for-access_token
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub scope: String,
    pub created_at: u64,
}
