extern crate reqwest;
extern crate serde;
#[cfg(test)]
#[macro_use]
extern crate serde_json;

use reqwest::Request;
use reqwest::RequestBuilder;
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;

pub type Result<T> = std::result::Result<T, reqwest::Error>;

/// Main struct
#[derive(Debug, Clone)]
pub struct TraktApi<'a> {
    base_url: &'a str,
    client: Client,
    client_id: &'a str,
    client_secret: Option<&'a str>,
}

impl<'a> TraktApi<'a> {
    /// Creates a new client with a given base url. Client ID is needed. Client secret is optional, if you need authorization
    pub fn with_url(
        base_url: &'a str,
        client_id: &'a str,
        client_secret: Option<&'a str>,
    ) -> TraktApi<'a> {
        TraktApi {
            base_url,
            client: Client::new(),
            client_id,
            client_secret,
        }
    }

    /// Creates a new client. Client ID is needed. Client secret is optional, if you need authorization
    pub fn new(client_id: &'a str, client_secret: Option<&'a str>) -> TraktApi<'a> {
        Self::with_url("https://api.trakt.tv", client_id, client_secret)
    }

    /// Creates a new client in the staging environment. Client ID is needed. Client secret is optional, if you need authorization
    pub fn staging(client_id: &'a str, client_secret: Option<&'a str>) -> TraktApi<'a> {
        Self::with_url("https://api-staging.trakt.tv", client_id, client_secret)
    }

    /// Generates a [reqwest::RequestBuilder] with the necessary headers
    ///
    /// [reqwest::RequestBuilder]: ../reqwest/struct.RequestBuilder.html
    fn builder(&self, method: Method, url: &str) -> RequestBuilder {
        self.client
            .request(method, &format!("{}{}", self.base_url, url))
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id)
    }

    /// Executes a [reqwest::Request] and directly parses the [reqwest::Response]
    ///
    /// [reqwest::Request]: ../reqwest/struct.Request.html
    /// [reqwest::Response]: ../reqwest/struct.Response.html
    async fn execute<T: DeserializeOwned>(&self, request: Request) -> Result<T> {
        self.client.execute(request).await?.json::<T>().await
    }

    /// A generic function which makes a GET request to the given url and receives a deserialized Object
    async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        self.builder(Method::GET, url)
            .send()
            .await?
            .json::<T>()
            .await
    }

    /// A generic function which makes an authorized GET request to the given url and receives a deserialized Object
    async fn get_auth<T: DeserializeOwned>(&self, url: &str, access_token: &str) -> Result<T> {
        self.builder(Method::GET, url)
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<T>()
            .await
    }

    /// A generic function which makes a POST request to the given url and receives a deserialized Object
    async fn post<T: DeserializeOwned>(&self, url: &str, body: String) -> Result<T> {
        self.builder(Method::POST, url)
            .body(body)
            .send()
            .await?
            .json::<T>()
            .await
    }

    /// A generic function which makes an authorized POST request to the given url and receives a deserialized Object
    async fn post_auth<T: DeserializeOwned>(
        &self,
        url: &str,
        body: String,
        access_token: &str,
    ) -> Result<T> {
        self.builder(Method::POST, url)
            .bearer_auth(access_token)
            .body(body)
            .send()
            .await?
            .json::<T>()
            .await
    }

    /// A generic function which makes an authorized DELETE request to the given url and receives nothing
    async fn delete_auth(&self, url: &str, access_token: &str) -> Result<()> {
        self.builder(Method::DELETE, url)
            .bearer_auth(access_token)
            .send()
            .await
            .map(|_| ())
    }
}

impl PartialEq for TraktApi<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.base_url == other.base_url
            && self.client_id == other.client_id
            && self.client_secret == other.client_secret
    }
}

#[cfg(test)]
mod tests {

    use crate::TraktApi;
    use mockito::Mock;
    use reqwest::Client;
    use serde_json::Value;

    pub fn mock(method: &str, path: &str, client_id: &str) -> Mock {
        mockito::mock(method, path)
            .match_header("trakt-api-version", "2")
            .match_header("trakt-api-key", client_id)
            .match_header("Content-Type", "application/json")
    }

    pub fn mock_auth(method: &str, path: &str, client_id: &str, access_token: &str) -> Mock {
        mock(method, path, client_id)
            .match_header("Authorization", &*format!("Bearer {}", access_token))
    }

    #[test]
    fn new_trakt_api() {
        assert_eq!(
            TraktApi {
                base_url: "https://api.trakt.tv",
                client: Client::new(),
                client_id: "abc",
                client_secret: Some("def"),
            },
            TraktApi::new("abc", Some("def"))
        );
    }

    #[test]
    fn staging_trakt_api() {
        assert_eq!(
            TraktApi {
                base_url: "https://api-staging.trakt.tv",
                client: Client::new(),
                client_id: "abc",
                client_secret: Some("def"),
            },
            TraktApi::staging("abc", Some("def"))
        );
    }

    #[tokio::test]
    async fn get() {
        let _m = mock("GET", "/abc", "loremipsum")
            .with_status(200)
            .with_body("{\"dolor\":\"sitamet\"}")
            .create();

        let s = TraktApi::with_url(&mockito::server_url(), "loremipsum", None)
            .get::<Value>("/abc")
            .await
            .unwrap();

        assert_eq!(s, json!({"dolor":"sitamet"}))
    }

    #[tokio::test]
    async fn get_auth() {
        let _m = mock_auth("GET", "/abc", "loremipsum", "mario")
            .with_status(200)
            .with_body("{\"dolor\":\"sitamet\"}")
            .create();

        let s = TraktApi::with_url(&mockito::server_url(), "loremipsum", None)
            .get_auth::<Value>("/abc", "mario")
            .await
            .unwrap();

        assert_eq!(s, json!({"dolor":"sitamet"}))
    }

    #[tokio::test]
    async fn post() {
        let _m = mock("POST", "/abc", "loremipsum")
            .with_status(200)
            .with_body("{\"dolor\":\"sitamet\"}")
            .match_body("abc")
            .create();

        let s = TraktApi::with_url(&mockito::server_url(), "loremipsum", None)
            .post::<Value>("/abc", "abc".to_string())
            .await
            .unwrap();

        assert_eq!(s, json!({"dolor":"sitamet"}))
    }

    #[tokio::test]
    async fn post_auth() {
        let _m = mock_auth("POST", "/abc", "loremipsum", "mario")
            .with_status(200)
            .with_body("{\"dolor\":\"sitamet\"}")
            .match_body("abc")
            .create();

        let s = TraktApi::with_url(&mockito::server_url(), "loremipsum", None)
            .post_auth::<Value>("/abc", "abc".to_string(), "mario")
            .await
            .unwrap();

        assert_eq!(s, json!({"dolor":"sitamet"}))
    }

    #[tokio::test]
    async fn delete_auth() {
        let _m = mock_auth("DELETE", "/abc", "loremipsum", "mario")
            .with_status(200)
            .create();

        TraktApi::with_url(&mockito::server_url(), "loremipsum", None)
            .delete_auth("/abc", "mario")
            .await
            .unwrap();
    }
}
