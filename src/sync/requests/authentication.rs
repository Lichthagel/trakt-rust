use crate::{
    error::Error,
    models::{AuthenticationDevices, AuthenticationTokenResponse},
    Result, TraktApi,
};
use serde_json::json;
use serde_json::Value;

impl<'a> TraktApi<'a> {
    pub fn oauth_authorize(&self, redirect_uri: &str, state: Option<&str>) -> String {
        match state {
            Some(state) => format!(
                "https://trakt.tv{}",
                api_url!(
                    ("oauth/authorize"),
                    ("response_type", "code"),
                    ("client_id", self.client_id),
                    ("redirect_uri", redirect_uri),
                    ("state", state)
                )
            ),
            None => format!(
                "https://trakt.tv{}",
                api_url!(
                    ("oauth/authorize"),
                    ("response_type", "code"),
                    ("client_id", self.client_id),
                    ("redirect_uri", redirect_uri)
                )
            ),
        }
    }

    pub fn oauth_get_token(
        &self,
        code: &str,
        redirect_uri: &str,
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
        refresh_token: &str,
        redirect_uri: &str,
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

    pub fn oauth_revoke_token(&self, token: &str) -> Result<()> {
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
        .map(|_res: Value| ())
    }

    pub fn oauth_device_code(&self) -> Result<AuthenticationDevices> {
        self.post(
            api_url!(("oauth/device/code")),
            json!({"client_id": self.client_id}).to_string(),
        )
    }

    pub fn oauth_device_token(&self, device_code: &str) -> Result<AuthenticationTokenResponse> {
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

#[cfg(test)]
mod tests {
    use crate::{
        models::{AuthenticationDevices, AuthenticationTokenResponse},
        tests::mock,
        TraktApi,
    };
    use mockito::Matcher;
    use std::fs;

    #[test]
    fn oauth_authorize() {
        let c = TraktApi::new("...".to_owned(), None);

        assert_eq!(
            c.oauth_authorize("http://localhost:8080/auth", None),
            "https://trakt.tv/oauth/authorize?response_type=code&client_id=...&redirect_uri=http://localhost:8080/auth".to_owned()
        )
    }

    #[test]
    fn oauth_get_token() {
        let m = mock("POST", "/oauth/token", "CLIENT_ID")
            .with_status(200)
            .match_body(Matcher::JsonString(
                fs::read_to_string("mock_data/oauth_get_token_req.json").unwrap(),
            ))
            .with_body_from_file("mock_data/oauth_get_token.json")
            .create();

        let res = TraktApi::with_url(
            &mockito::server_url(),
            "CLIENT_ID".to_owned(),
            Some("CLIENT_SECRET".to_owned()),
        )
        .oauth_get_token("CODE", "http://localhost:8080/auth")
        .unwrap();

        assert_eq!(
            res,
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

    /*#[test]
    #[should_panic]
    fn oauth_get_token_panic() {
        let _m = mock("POST", "oauth/token")
            .with_status(200)
            .match_body(Matcher::JsonString(fs::read_to_string("mock_data/oauth_get_token_req.json").unwrap()))
            .with_body_from_file("mock_data/oauth_get_token.json")
            .create();

        let fut = TraktApi::with_url(&mockito::server_url(), "CLIENT_ID".to_owned(), None)
            .oauth_get_token("CODE", "http://localhost:8080/auth")
            .map(|_res| {
                ()
            })
            .map_err(|e| {
                println!("{}", e);
                match e {
                    Error::ClientSecretNeeded => panic!("{}", e),
                    _ => (),
                }
            });

        tokio::run(fut);
    }*/

    #[test]
    fn oauth_refresh_token() {
        let m = mock("POST", "/oauth/token", "CLIENT_ID")
            .with_status(200)
            .match_body(Matcher::JsonString(
                fs::read_to_string("mock_data/oauth_refresh_token_req.json").unwrap(),
            ))
            .with_body_from_file("mock_data/oauth_get_token.json")
            .create();

        let res = TraktApi::with_url(
            &mockito::server_url(),
            "CLIENT_ID".to_owned(),
            Some("CLIENT_SECRET".to_owned()),
        )
        .oauth_refresh_token("REFRESH_TOKEN", "http://localhost:8080/auth")
        .unwrap();

        assert_eq!(
            res,
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

    #[test]
    fn oauth_revoke_token() {
        let m = mock("POST", "/oauth/revoke", "CLIENT_ID")
            .with_status(200)
            .match_body(Matcher::JsonString(
                fs::read_to_string("mock_data/oauth_revoke_token_req.json").unwrap(),
            ))
            .with_body("{}")
            .create();

        TraktApi::with_url(
            &mockito::server_url(),
            "CLIENT_ID".to_owned(),
            Some("CLIENT_SECRET".to_owned()),
        )
        .oauth_revoke_token("TOKEN")
        .unwrap();

        m.assert();
    }

    #[test]
    fn oauth_device_code() {
        let m = mock("POST", "/oauth/device/code", "CLIENT_ID")
            .with_status(200)
            .match_body(Matcher::JsonString(
                "{\"client_id\": \"CLIENT_ID\"}".to_owned(),
            ))
            .with_body_from_file("mock_data/oauth_device_code.json")
            .create();

        let res = TraktApi::with_url(
            &mockito::server_url(),
            "CLIENT_ID".to_owned(),
            Some("CLIENT_SECRET".to_owned()),
        )
        .oauth_device_code()
        .unwrap();

        assert_eq!(
            res,
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

    #[test]
    fn oauth_device_token() {
        let m = mock("POST", "/oauth/device/token", "CLIENT_ID")
            .with_status(200)
            .match_body(Matcher::JsonString(
                fs::read_to_string("mock_data/oauth_device_token_req.json").unwrap(),
            ))
            .with_body_from_file("mock_data/oauth_get_token.json")
            .create();

        let res = TraktApi::with_url(
            &mockito::server_url(),
            "CLIENT_ID".to_owned(),
            Some("CLIENT_SECRET".to_owned()),
        )
        .oauth_device_token("fd0847dbb559752d932dd3c1ac34ff98d27b11fe2fea5a864f44740cd7919ad0")
        .unwrap();

        assert_eq!(
            res,
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
