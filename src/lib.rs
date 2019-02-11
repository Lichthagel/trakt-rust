//! # trakt-rust
//!
//! A api wrapper in Rust for trakt.tv
//!
//! ## Installation
//!
//! Not yet released but you may add
//! ```toml
//! trakt = { git = "https://github.com/Lichthagel/trakt-rust" }
//! ```
//! to your dependencies
//!
//! ## Examples
//!
//! TODO
//!
//! ## License
//!
//! [MIT][license]
//!
//! [license]: https://github.com/Lichthagel/trakt-rust/blob/master/LICENSE

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod macros;
pub mod error;
pub mod extended_info;
pub mod filters;
pub mod models;
pub mod pagination;
pub mod requests;
pub mod selectors;

use crate::{
    error::{Error, Result},
    models::{Certifications, CertificationsType, Country, Genre, Language, MediaType, Network},
};
use reqwest::{IntoUrl, Method, RequestBuilder};
use serde::de::DeserializeOwned;

/// The main struct which contains all requests
#[derive(Debug, Clone)]
pub struct TraktApi {
    client: reqwest::Client,
    client_id: String,
    client_secret: Option<String>,
}

/// Generic functions and simple requests
impl TraktApi {
    /// Creates a new client client ID is needed client secret is optional if you need authorization
    pub fn new(client_id: String, client_secret: Option<String>) -> TraktApi {
        TraktApi {
            client: reqwest::Client::new(),
            client_id,
            client_secret,
        }
    }

    /// Generates a [reqwest::RequestBuilder] with the necessary headers
    ///
    /// [reqwest::RequestBuilder]: ../reqwest/struct.RequestBuilder.html
    fn builder(&self, method: Method, url: impl IntoUrl) -> RequestBuilder {
        self.client
            .request(method, url)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
    }

    /// Executes a [reqwest::RequestBuilder] and parses the [reqwest::Response]
    ///
    /// [reqwest::RequestBuilder]: ../reqwest/struct.RequestBuilder.html
    /// [reqwest::Response]: ../reqwest/struct.Response.html
    fn execute<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T> {
        match self.client.execute(request.build()?) {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(serde_json::from_reader(res).unwrap())
                } else {
                    Err(Error::from(res))
                }
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    /// A generic function which makes a GET request to the given url and receives a deserialized Object
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    fn get<T: DeserializeOwned>(&self, url: String) -> Result<T> {
        self._get(&url)
    }

    /// A generic function which makes a GET request to the given url and receives a deserialized Object
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    fn _get<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        match self
            .client
            .get(url)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(serde_json::from_reader(res).unwrap())
                } else {
                    Err(Error::from(res))
                }
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    /// A generic function which makes an authorized GET request to the given url and receives a deserialized Object
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    fn auth_get<T: DeserializeOwned>(&self, url: String, access_token: &str) -> Result<T> {
        self._auth_get(&url, access_token)
    }

    /// A generic function which makes an authorized GET request to the given url and receives a deserialized Object
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    fn _auth_get<T: DeserializeOwned>(&self, url: &str, access_token: &str) -> Result<T> {
        match self
            .client
            .get(url)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .bearer_auth(access_token)
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(serde_json::from_reader(res).unwrap())
                } else {
                    Err(Error::from(res))
                }
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    /// A generic function which makes a POST request to the given url and receives a deserialized Object
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    fn post<T: DeserializeOwned>(&self, url: String, body: String) -> Result<T> {
        self._post(&url, body)
    }

    /// A generic function which makes a POST request to the given url and receives a deserialized Object
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    fn _post<T: DeserializeOwned>(&self, url: &str, body: String) -> Result<T> {
        match self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .body(body)
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(serde_json::from_reader(res).unwrap())
                } else {
                    Err(Error::from(res))
                }
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    /// A generic function which makes an authorized POST request to the given url and receives a deserialized Object
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    fn auth_post<T: DeserializeOwned>(
        &self,
        url: String,
        body: String,
        access_token: &str,
    ) -> Result<T> {
        self._auth_post(&url, body, access_token)
    }

    /// A generic function which makes an authorized POST request to the given url and receives a deserialized Object
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    fn _auth_post<T: DeserializeOwned>(
        &self,
        url: &str,
        body: String,
        access_token: &str,
    ) -> Result<T> {
        match self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .bearer_auth(access_token)
            .body(body)
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(serde_json::from_reader(res).unwrap())
                } else {
                    Err(Error::from(res))
                }
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    /// A generic function which makes an authorized POST request to the given url and receives nothing
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    fn auth_post_no_body(&self, url: String, body: String, access_token: &str) -> Result<()> {
        self._auth_post_no_body(&url, body, access_token)
    }

    /// A generic function which makes an authorized POST request to the given url and receives nothing
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    fn _auth_post_no_body(&self, url: &str, body: String, access_token: &str) -> Result<()> {
        match self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .bearer_auth(access_token)
            .body(body)
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(())
                } else {
                    Err(Error::from(res))
                }
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    /// A generic function which makes an authorized PUT request to the given url and receives a deserialized object
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    fn auth_put<T: DeserializeOwned>(
        &self,
        url: String,
        body: String,
        access_token: &str,
    ) -> Result<T> {
        self._auth_put(&url, body, access_token)
    }

    /// A generic function which makes an authorized PUT request to the given url and receives a deserialized object
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    fn _auth_put<T: DeserializeOwned>(
        &self,
        url: &str,
        body: String,
        access_token: &str,
    ) -> Result<T> {
        match self
            .client
            .put(url)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .bearer_auth(access_token)
            .body(body)
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(serde_json::from_reader(res).unwrap())
                } else {
                    Err(Error::from(res))
                }
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    /// A generic function which makes an authorized DELETE request to the given url and receives nothing
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    fn auth_delete(&self, url: String, access_token: &str) -> Result<()> {
        self._auth_delete(&url, access_token)
    }

    /// A generic function which makes an authorized DELETE request to the given url and receives nothing
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    fn _auth_delete(&self, url: &str, access_token: &str) -> Result<()> {
        match self
            .client
            .delete(url)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .bearer_auth(access_token)
            .send()
        {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(())
                } else {
                    Err(Error::from(res))
                }
            }
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Get a Vec of all certifications, including names, slugs, and descriptions. [Trakt API][more]
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    /// [more]: https://trakt.docs.apiary.io/#reference/certifications/list/get-certifications
    pub fn certifications(&self, ct: CertificationsType) -> Result<Certifications> {
        self.get(api_url!(("certifications", ct.to_string())))
    }

    /// Get a Vec of all countries, including names and codes. [Trakt API][more]
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    /// [more]: https://trakt.docs.apiary.io/#reference/countries/list/get-countries
    pub fn countries(&self, media_type: MediaType) -> Result<Vec<Country>> {
        self.get(api_url!(("countries", media_type.to_string())))
    }

    /// Get a Vec of all genres, including names and slugs. [Trakt API][more]
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    /// [more]: https://trakt.docs.apiary.io/#reference/genres/get-genres
    pub fn genres(&self, media_type: MediaType) -> Result<Vec<Genre>> {
        self.get(api_url!(("genres", media_type.to_string())))
    }

    /// Get a Vec of all languages, including names and codes. [Trakt API][more]
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    /// [more]: https://trakt.docs.apiary.io/#reference/genres/list/get-languages
    pub fn languages(&self, media_type: MediaType) -> Result<Vec<Language>> {
        self.get(api_url!(("languages", media_type.to_string())))
    }

    /// Get a Vec of all TV networks, including the name. [Trakt API][more]
    ///
    /// # Errors
    ///
    /// Returns [Error::Response] if the response contains an unsuccessful status code
    ///
    /// Returns [Error::Connection] if the connection failed
    ///
    /// Returns [Error::Serde] if the response could not be deserialized
    ///
    /// [Error::Response]: error/enum.Error.html#variant.Response
    /// [Error::Connection]: error/enum.Error.html#variant.Connection
    /// [Error::Serde]: error/enum.Error.html#variant.Serde
    /// [more]: https://trakt.docs.apiary.io/#reference/networks/list/get-networks
    pub fn networks(&self) -> Result<Vec<Network>> {
        self.get(api_url!(("networks")))
    }
}

impl PartialEq for TraktApi {
    fn eq(&self, other: &TraktApi) -> bool {
        self.client_id == other.client_id && self.client_secret == other.client_secret
    }

    fn ne(&self, other: &TraktApi) -> bool {
        self.client_id != other.client_id || self.client_secret != other.client_secret
    }
}

#[cfg(test)]
mod tests {
    use crate::TraktApi;

    #[test]
    fn new_trakt_api() {
        assert_eq!(
            TraktApi {
                client: reqwest::Client::new(),
                client_id: String::from("abc"),
                client_secret: Some(String::from("def")),
            },
            TraktApi::new(String::from("abc"), Some(String::from("def")))
        );
    }
}
