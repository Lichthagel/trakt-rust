#[derive(Debug, Serialize, Deserialize)]
pub struct Authentication {
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: Option<String>,
    grant_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationDevices {
    pub device_code: String,
    pub user_code: String,
    pub verification_url: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationDeviceId {
    pub client_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationDeviceGetToken {
    pub code: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationDeviceGetTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: String,
    pub refresh_token: String,
    pub scope: String,
    pub created_at: String,
}
