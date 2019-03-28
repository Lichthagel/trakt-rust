pub mod pagination;
pub mod requests;

use crate::{
    error::Error,
    models::{Certifications, CertificationsType, Country, Genre, Language, MediaType, Network},
};
use futures::Future;
use reqwest::{
    r#async::{Client, Request, RequestBuilder},
    Method,
};
use serde::de::DeserializeOwned;

pub type Result<T> = Box<Future<Item = T, Error = Error> + Send>;

/// The main struct which contains all requests
#[derive(Debug, Clone)]
pub struct TraktApi<'a> {
    base_url: &'a str,
    client: Client,
    client_id: String,
    client_secret: Option<String>,
}

/// Generic functions and simple requests
///
/// # Example
///
/// ```rust,no_run
/// extern crate futures;
/// extern crate tokio_core;
/// extern crate trakt;
///
/// use futures::Future;
/// use trakt::{asyn::TraktApi, pagination::Pagination};
/// use tokio_core::reactor::Core;
///
/// fn fetch() -> impl Future<Item = (), Error = ()> {
///     let api = TraktApi::new(
///         "...".to_owned(),
///         None,
///     );
///
///     let access_token = "";
///
///     api.user_settings(access_token)
///         .map(|res| {
///             dbg!(res);
///         })
///         .map_err(|e| {
///             panic!(e);
///         })
/// }
///
/// fn main() {
///     let mut core = Core::new().unwrap();
///     core.run(fetch());
/// }
/// ```
impl<'a> TraktApi<'a> {
    /// Creates a new client. Client ID is needed. Client secret is optional, if you need authorization
    pub fn new(client_id: String, client_secret: Option<String>) -> TraktApi<'a> {
        Self::with_url("https://api.trakt.tv", client_id, client_secret)
    }

    /// Creates a new client in the staging environment. Client ID is needed. Client secret is optional, if you need authorization
    pub fn staging(client_id: String, client_secret: Option<String>) -> TraktApi<'a> {
        Self::with_url("https://api-staging.trakt.tv", client_id, client_secret)
    }

    /// Creates a new client with a given base url. Client ID is needed. Client secret is optional, if you need authorization
    pub fn with_url(
        base_url: &'a str,
        client_id: String,
        client_secret: Option<String>,
    ) -> TraktApi<'a> {
        TraktApi {
            base_url,
            client: Client::new(),
            client_id,
            client_secret,
        }
    }

    /// Generates a [reqwest::RequestBuilder] with the necessary headers
    ///
    /// [reqwest::RequestBuilder]: ../reqwest/struct.RequestBuilder.html
    fn builder(&self, method: Method, url: String) -> RequestBuilder {
        self.client
            .request(method, &format!("{}{}", self.base_url, url))
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
    }

    /// Executes a [reqwest::Request] and parses the [reqwest::Response]
    ///
    /// [reqwest::Request]: ../reqwest/struct.Request.html
    /// [reqwest::Response]: ../reqwest/struct.Response.html
    fn execute<T: DeserializeOwned + Send + 'static>(&self, request: Request) -> Result<T> {
        Box::new(
            self.client
                .execute(request)
                .and_then(|mut res| res.json())
                .map_err(Error::from),
        )
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
    fn get<T: DeserializeOwned + Send + 'static>(&self, url: String) -> Result<T> {
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
    fn _get<T: DeserializeOwned + Send + 'static>(&self, url: &str) -> Result<T> {
        Box::new(
            self.client
                .get(&format!("{}{}", self.base_url, url))
                .header("Content-Type", "application/json")
                .header("trakt-api-version", "2")
                .header("trakt-api-key", self.client_id.as_str())
                .send()
                .and_then(|mut res| res.json())
                .map_err(Error::from),
        )
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
    fn auth_get<T: DeserializeOwned + Send + 'static>(
        &self,
        url: String,
        access_token: &str,
    ) -> Result<T> {
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
    fn _auth_get<T: DeserializeOwned + Send + 'static>(
        &self,
        url: &str,
        access_token: &str,
    ) -> Result<T> {
        Box::new(
            self.client
                .get(&format!("{}{}", self.base_url, url))
                .header("Content-Type", "application/json")
                .header("trakt-api-version", "2")
                .header("trakt-api-key", self.client_id.as_str())
                .header("Authorization", format!("Bearer {}", access_token))
                .send()
                .and_then(|mut res| res.json())
                .map_err(Error::from),
        )
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
    fn post<T: DeserializeOwned + Send + 'static>(&self, url: String, body: String) -> Result<T> {
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
    fn _post<T: DeserializeOwned + Send + 'static>(&self, url: &str, body: String) -> Result<T> {
        Box::new(
            self.client
                .post(&format!("{}{}", self.base_url, url))
                .header("Content-Type", "application/json")
                .header("trakt-api-version", "2")
                .header("trakt-api-key", self.client_id.as_str())
                .body(body)
                .send()
                .and_then(|mut res| res.json())
                .map_err(Error::from),
        )
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
    fn auth_post<T: DeserializeOwned + Send + 'static>(
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
    fn _auth_post<T: DeserializeOwned + Send + 'static>(
        &self,
        url: &str,
        body: String,
        access_token: &str,
    ) -> Result<T> {
        Box::new(
            self.client
                .post(&format!("{}{}", self.base_url, url))
                .header("Content-Type", "application/json")
                .header("trakt-api-version", "2")
                .header("trakt-api-key", self.client_id.as_str())
                .header("Authorization", format!("Bearer {}", access_token))
                .body(body)
                .send()
                .and_then(|mut res| res.json())
                .map_err(Error::from),
        )
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
        Box::new(
            self.client
                .post(&format!("{}{}", self.base_url, url))
                .header("Content-Type", "application/json")
                .header("trakt-api-version", "2")
                .header("trakt-api-key", self.client_id.as_str())
                .header("Authorization", format!("Bearer {}", access_token))
                .body(body)
                .send()
                .and_then(|mut res| res.json())
                .map_err(Error::from),
        )
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
    fn auth_put<T: DeserializeOwned + Send + 'static>(
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
    fn _auth_put<T: DeserializeOwned + Send + 'static>(
        &self,
        url: &str,
        body: String,
        access_token: &str,
    ) -> Result<T> {
        Box::new(
            self.client
                .put(&format!("{}{}", self.base_url, url))
                .header("Content-Type", "application/json")
                .header("trakt-api-version", "2")
                .header("trakt-api-key", self.client_id.as_str())
                .header("Authorization", format!("Bearer {}", access_token))
                .body(body)
                .send()
                .and_then(|mut res| res.json())
                .map_err(Error::from),
        )
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
        Box::new(
            self.client
                .delete(&format!("{}{}", self.base_url, url))
                .header("Content-Type", "application/json")
                .header("trakt-api-version", "2")
                .header("trakt-api-key", self.client_id.as_str())
                .header("Authorization", format!("Bearer {}", access_token))
                .send()
                .map(|_| ())
                .map_err(Error::from),
        )
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

    #[cfg(feature = "sync")]
    pub fn into_sync(self) -> crate::TraktApi<'a> {
        crate::TraktApi::with_url(self.base_url, self.client_id, self.client_secret)
    }
}

impl<'a> PartialEq for TraktApi<'a> {
    fn eq(&self, other: &TraktApi) -> bool {
        self.client_id == other.client_id
            && self.client_secret == other.client_secret
            && self.base_url == other.base_url
    }
}

#[cfg(test)]
mod tests {
    use crate::{asyn::TraktApi, error::Error, models::*, tests::mock};
    use futures::future::Future;
    use tokio_core::reactor::Core;

    #[test]
    fn new_trakt_api() {
        assert_eq!(
            TraktApi {
                base_url: "https://api.trakt.tv",
                client: reqwest::r#async::Client::new(),
                client_id: String::from("abc"),
                client_secret: Some(String::from("def")),
            },
            TraktApi::new(String::from("abc"), Some(String::from("def")))
        );
    }

    #[test]
    fn staging_trakt_api() {
        assert_eq!(
            TraktApi {
                base_url: "https://api-staging.trakt.tv",
                client: reqwest::r#async::Client::new(),
                client_id: String::from("abc"),
                client_secret: Some(String::from("def")),
            },
            TraktApi::staging(String::from("abc"), Some(String::from("def")))
        );
    }

    #[test]
    fn certifications() -> Result<(), Error> {
        let m = mock("GET", "/certifications/movies", "...")
            .with_status(200)
            .with_body_from_file("mock_data/certifications_movies.json")
            .create();
        let mut core = Core::new().unwrap();

        let fut = TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .certifications(CertificationsType::Movies)
            .map(|res| {
                assert_eq!(
                    res,
                    Certifications {
                        us: vec![
                            Certification {
                                name: "G".to_owned(),
                                slug: "g".to_owned(),
                                description: "All Ages".to_owned()
                            },
                            Certification {
                                name: "PG".to_owned(),
                                slug: "pg".to_owned(),
                                description: "Parental Guidance Suggested".to_owned()
                            },
                            Certification {
                                name: "PG-13".to_owned(),
                                slug: "pg-13".to_owned(),
                                description: "Parents Strongly Cautioned - Ages 13+ Recommended"
                                    .to_owned()
                            },
                            Certification {
                                name: "R".to_owned(),
                                slug: "r".to_owned(),
                                description: "Mature Audiences - Ages 17+ Recommended".to_owned()
                            },
                            Certification {
                                name: "Not Rated".to_owned(),
                                slug: "nr".to_owned(),
                                description: "Not Rated".to_owned()
                            }
                        ]
                    }
                )
            })
            .then(|res| {
                m.assert();
                res
            });

        core.run(fut)
    }

    #[test]
    fn countries() -> Result<(), Error> {
        let m = mock("GET", "/countries/movies", "...")
            .with_status(200)
            .with_body_from_file("mock_data/countries_movies.json")
            .create();
        let mut core = Core::new().unwrap();

        let fut = TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .countries(MediaType::Movies)
            .map(|res| {
                assert!(res.contains(&Country {
                    name: "Greece".to_owned(),
                    code: "gr".to_owned()
                }));
                assert!(res.contains(&Country {
                    name: "Zambia".to_owned(),
                    code: "zm".to_owned()
                }));
            })
            .then(|res| {
                m.assert();
                res
            });

        core.run(fut)
    }

    #[test]
    fn genres() -> Result<(), Error> {
        let m = mock("GET", "/genres/movies", "...")
            .with_status(200)
            .with_body_from_file("mock_data/genres_movies.json")
            .create();
        let mut core = Core::new().unwrap();

        let fut = TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .genres(MediaType::Movies)
            .map(|res| {
                assert!(res.contains(&Genre {
                    name: "Animation".to_owned(),
                    slug: "animation".to_owned()
                }));
                assert!(res.contains(&Genre {
                    name: "Superhero".to_owned(),
                    slug: "superhero".to_owned()
                }));
            })
            .then(|res| {
                m.assert();
                res
            });

        core.run(fut)
    }

    #[test]
    fn languages() -> Result<(), Error> {
        let m = mock("GET", "/languages/movies", "...")
            .with_status(200)
            .with_body_from_file("mock_data/languages_movies.json")
            .create();
        let mut core = Core::new().unwrap();

        let fut = TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .languages(MediaType::Movies)
            .map(|res| {
                assert!(res.contains(&Language {
                    name: "English".to_owned(),
                    code: "en".to_owned()
                }));
                assert!(res.contains(&Language {
                    name: "Fulah".to_owned(),
                    code: "ff".to_owned()
                }));
            })
            .then(|res| {
                m.assert();
                res
            });

        core.run(fut)
    }

    #[test]
    fn networks() -> Result<(), Error> {
        let m = mock("GET", "/networks", "...")
            .with_status(200)
            .with_body_from_file("mock_data/networks.json")
            .create();
        let mut core = Core::new().unwrap();

        let fut = TraktApi::with_url(&mockito::server_url(), "...".to_owned(), None)
            .networks()
            .map(|res| {
                assert!(res.contains(&Network {
                    name: "AT-X".to_owned()
                }));
                assert!(res.contains(&Network {
                    name: "Apple Music".to_owned()
                }));
            })
            .then(|res| {
                m.assert();
                res
            });

        core.run(fut)
    }
}
