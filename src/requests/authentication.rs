use crate::{
    error::{Error, Result},
    models::{AuthenticationDevices, AuthenticationTokenResponse},
    TraktApi,
};
use serde_json::json;

impl TraktApi {
    pub fn oauth_authorize(
        &self,
        client_id: String,
        redirect_uri: String,
        state: Option<String>,
    ) -> Result<()> {
        match state {
            Some(state) => self.get(api_url!(
                ("oauth/authorize"),
                ("response_type", "code"),
                ("client_id", client_id),
                ("redirect_uri ", redirect_uri),
                ("state", state)
            )),
            None => self.get(api_url!(
                ("oauth/authorize"),
                ("response_type", "code"),
                ("client_id", client_id),
                ("redirect_uri ", redirect_uri)
            )),
        }
    }

    pub fn oauth_get_token(
        &self,
        code: String,
        redirect_uri: String,
    ) -> Result<AuthenticationTokenResponse> {
        if self.client_secret == None {
            return Err(Error::ClientSecretNeeded);
        }

        self.post(
            api_url!(("oauth/token")),
            json!({
                "code": code,
                "client_id": self.client_id,
                "client_secret": self.client_secret,
                "redirect_uri": redirect_uri,
                "grant_type": "authorization_code"
            })
            .to_string(),
        )
    }

    pub fn oauth_refresh_token(
        &self,
        refresh_token: String,
        redirect_uri: String,
    ) -> Result<AuthenticationTokenResponse> {
        if self.client_secret == None {
            return Err(Error::ClientSecretNeeded);
        }

        self.post(
            api_url!(("oauth/token")),
            json!({
                "refresh_token": refresh_token,
                "client_id": self.client_id,
                "client_secret": self.client_secret,
                "redirect_uri": redirect_uri,
                "grant_type": "refresh_token"
            })
            .to_string(),
        )
    }

    pub fn oauth_revoke_token(&self, token: String) -> Result<()> {
        if self.client_secret == None {
            return Err(Error::ClientSecretNeeded);
        }

        self.post(
            api_url!(("oauth/revoke")),
            json!({
                "token": token,
                "client_id": self.client_id,
                "client_secret": self.client_secret,
            })
            .to_string(),
        )
    }

    pub fn devices_authenticate(&self) -> Result<AuthenticationDevices> {
        self.post(
            api_url!(("oauth/device/code")),
            json!({"client_id": self.client_id}).to_string(),
        )
    }

    pub fn get_token(&self, device_code: String) -> Result<AuthenticationTokenResponse> {
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
