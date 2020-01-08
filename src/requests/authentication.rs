use crate::models::{AuthenticationDevices, AuthenticationTokenResponse};
use crate::TraktApi;
use crate::{Error, Result};
use serde_json::Value;

impl TraktApi<'_> {
    /// Returns the url where a user needs to login (always uses trakt.tv as base url).
    /// [API docs]
    ///
    /// [API docs]: https://trakt.docs.apiary.io/#reference/authentication-oauth/authorize/authorize-application
    pub fn oauth_authorize(&self, redirect_uri: &str, state: Option<&str>) -> String {
        match state {
            Some(state) => format!("https://trakt.tv/oauth/authorize?response_type=code&client_id={}&redirect_uri={}&state={}", self.client_id, redirect_uri, state),
            None => format!("https://trakt.tv/oauth/authorize?response_type=code&client_id={}&redirect_uri={}", self.client_id, redirect_uri)
        }
    }

    /// Returns an access token using the code sent to redirect uri.
    /// [API docs]
    ///
    /// [API docs]: https://trakt.docs.apiary.io/#reference/authentication-oauth/get-token/exchange-code-for-access_token
    pub async fn oauth_get_token(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<AuthenticationTokenResponse> {
        match self.client_secret {
            Some(client_secret) => self.post("/oauth/token", format!("{{\"code\":\"{}\",\"client_id\":\"{}\",\"client_secret\":\"{}\",\"redirect_uri\":\"{}\",\"grant_type\":\"authorization_code\"}}", code, self.client_id, client_secret, redirect_uri)).await,
            None => Err(Error::ClientSecretNeeded)
        }
    }

    /// Returns an access token using a refresh token
    /// [API docs]
    ///
    /// [API docs]: https://trakt.docs.apiary.io/#reference/authentication-oauth/get-token/exchange-code-for-access_token
    pub async fn oauth_refresh_token(
        &self,
        refresh_token: &str,
        redirect_uri: &str,
    ) -> Result<AuthenticationTokenResponse> {
        match self.client_secret {
            Some(client_secret) =>  self.post("/oauth/token", format!("{{\"refresh_token\":\"{}\",\"client_id\":\"{}\",\"client_secret\":\"{}\",\"redirect_uri\":\"{}\",\"grant_type\":\"refresh_token\"}}", refresh_token, self.client_id, client_secret, redirect_uri)).await,
            None => Err(Error::ClientSecretNeeded)
        }
    }

    /// Invalidates an access token
    /// [API docs]
    ///
    /// [API docs]: https://trakt.docs.apiary.io/#reference/authentication-oauth/revoke-token/revoke-an-access_token
    pub async fn oauth_revoke_token(&self, token: &str) -> Result<()> {
        match self.client_secret {
            Some(client_secret) => self
                .post(
                    "/oauth/revoke",
                    format!(
                        "{{\"token\":\"{}\",\"client_id\":\"{}\",\"client_secret\":\"{}\"}}",
                        token, self.client_id, client_secret
                    ),
                )
                .await
                .map(|_: Value| ()),
            None => Err(Error::ClientSecretNeeded),
        }
    }

    pub async fn oauth_device_code(&self) -> Result<AuthenticationDevices> {
        self.post(
            "/oauth/device/code",
            format!("{{\"client_id\":\"{}\"}}", self.client_id),
        )
        .await
    }

    pub async fn oauth_device_token(
        &self,
        device_code: &str,
    ) -> Result<AuthenticationTokenResponse> {
        match self.client_secret {
            Some(client_secret) => {
                self.post(
                    "/oauth/device/token",
                    format!(
                        "{{\"code\":\"{}\", \"client_id\":\"{}\", \"client_secret\":\"{}\"}}",
                        device_code, self.client_id, client_secret
                    ),
                )
                .await
            }
            None => Err(Error::ClientSecretNeeded),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{AuthenticationDevices, AuthenticationTokenResponse};
    use crate::tests::mock;
    use crate::TraktApi;
    use mockito::Matcher;
    use std::fs;

    #[test]
    fn oauth_authorize() {
        let c = TraktApi::new("...", None);

        assert_eq!(
            c.oauth_authorize("http://localhost:8080/auth", None),
            "https://trakt.tv/oauth/authorize?response_type=code&client_id=...&redirect_uri=http://localhost:8080/auth".to_owned()
        )
    }

    #[tokio::test]
    async fn oauth_get_token() {
        let m = mock("POST", "/oauth/token", "CLIENT_ID")
            .with_status(200)
            .match_body(Matcher::JsonString(
                fs::read_to_string("mock_data/oauth_get_token_req.json").unwrap(),
            ))
            .with_body_from_file("mock_data/oauth_get_token.json")
            .create();

        let s = TraktApi::with_url(&mockito::server_url(), "CLIENT_ID", Some("CLIENT_SECRET"))
            .oauth_get_token("CODE", "http://localhost:8080/auth")
            .await
            .unwrap();

        assert_eq!(
            s,
            AuthenticationTokenResponse {
                access_token: "dbaf9757982a9e738f05d249b7b5b4a266b3a139049317c4909f2f263572c781"
                    .to_string(),
                token_type: "bearer".to_string(),
                expires_in: 7200,
                refresh_token: "76ba4c5c75c96f6087f58a4de10be6c00b29ea1ddc3b2022ee2016d1363e3a7c"
                    .to_string(),
                scope: "public".to_string(),
                created_at: 1487889741
            }
        );

        m.assert();
    }

    #[tokio::test]
    async fn oauth_refresh_token() {
        let m = mock("POST", "/oauth/token", "CLIENT_ID")
            .with_status(200)
            .match_body(Matcher::JsonString(
                fs::read_to_string("mock_data/oauth_refresh_token_req.json").unwrap(),
            ))
            .with_body_from_file("mock_data/oauth_get_token.json")
            .create();

        let s = TraktApi::with_url(&mockito::server_url(), "CLIENT_ID", Some("CLIENT_SECRET"))
            .oauth_refresh_token("REFRESH_TOKEN", "http://localhost:8080/auth")
            .await
            .unwrap();

        assert_eq!(
            s,
            AuthenticationTokenResponse {
                access_token: "dbaf9757982a9e738f05d249b7b5b4a266b3a139049317c4909f2f263572c781"
                    .to_string(),
                token_type: "bearer".to_string(),
                expires_in: 7200,
                refresh_token: "76ba4c5c75c96f6087f58a4de10be6c00b29ea1ddc3b2022ee2016d1363e3a7c"
                    .to_string(),
                scope: "public".to_string(),
                created_at: 1487889741
            }
        );

        m.assert();
    }

    #[tokio::test]
    async fn oauth_revoke_token() {
        let m = mock("POST", "/oauth/revoke", "CLIENT_ID")
            .with_status(200)
            .match_body(Matcher::JsonString(
                fs::read_to_string("mock_data/oauth_revoke_token_req.json").unwrap(),
            ))
            .with_body("{}")
            .create();

        TraktApi::with_url(&mockito::server_url(), "CLIENT_ID", Some("CLIENT_SECRET"))
            .oauth_revoke_token("TOKEN")
            .await
            .unwrap();

        m.assert();
    }

    #[tokio::test]
    async fn oauth_device_code() {
        let m = mock("POST", "/oauth/device/code", "CLIENT_ID")
            .with_status(200)
            .match_body(Matcher::JsonString(
                "{\"client_id\": \"CLIENT_ID\"}".to_owned(),
            ))
            .with_body_from_file("mock_data/oauth_device_code.json")
            .create();

        let s = TraktApi::with_url(&mockito::server_url(), "CLIENT_ID", Some("CLIENT_SECRET"))
            .oauth_device_code()
            .await
            .unwrap();

        assert_eq!(
            s,
            AuthenticationDevices {
                device_code: "d9c126a7706328d808914cfd1e40274b6e009f684b1aca271b9b3f90b3630d64"
                    .to_string(),
                user_code: "5055CC52".to_string(),
                verification_url: "https://trakt.tv/activate".to_string(),
                expires_in: 600,
                interval: 5
            }
        );

        m.assert();
    }

    #[tokio::test]
    async fn oauth_device_token() {
        let m = mock("POST", "/oauth/device/token", "CLIENT_ID")
            .with_status(200)
            .match_body(Matcher::JsonString(
                fs::read_to_string("mock_data/oauth_device_token_req.json").unwrap(),
            ))
            .with_body_from_file("mock_data/oauth_get_token.json")
            .create();

        let s = TraktApi::with_url(&mockito::server_url(), "CLIENT_ID", Some("CLIENT_SECRET"))
            .oauth_device_token("fd0847dbb559752d932dd3c1ac34ff98d27b11fe2fea5a864f44740cd7919ad0")
            .await
            .unwrap();

        assert_eq!(
            s,
            AuthenticationTokenResponse {
                access_token: "dbaf9757982a9e738f05d249b7b5b4a266b3a139049317c4909f2f263572c781"
                    .to_string(),
                token_type: "bearer".to_string(),
                expires_in: 7200,
                refresh_token: "76ba4c5c75c96f6087f58a4de10be6c00b29ea1ddc3b2022ee2016d1363e3a7c"
                    .to_string(),
                scope: "public".to_string(),
                created_at: 1487889741
            }
        );

        m.assert();
    }
}
