#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod macros;
pub mod error;
pub mod models;
mod requests;

use crate::{
    error::{Error, Result},
    models::{Certifications, CertificationsType, Country, Genre, Language, MediaType, Network},
};
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct TraktApi {
    client: reqwest::Client,
    client_id: String,
    client_secret: Option<String>,
}

impl TraktApi {
    pub fn new(client_id: String, client_secret: Option<String>) -> TraktApi {
        TraktApi {
            client: reqwest::Client::new(),
            client_id,
            client_secret,
        }
    }

    fn get<T: DeserializeOwned>(&self, url: String) -> Result<T> {
        match self
            .client
            .get(&url)
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

    fn auth_get<T: DeserializeOwned>(&self, url: String, access_token: String) -> Result<T> {
        match self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .header("Authorization", format!("Bearer {}", access_token))
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

    fn post<T: DeserializeOwned>(&self, url: String, body: String) -> Result<T> {
        match self
            .client
            .post(&url)
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

    fn auth_post<T: DeserializeOwned>(
        &self,
        url: String,
        body: String,
        access_token: String,
    ) -> Result<T> {
        match self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .header("Authorization", format!("Bearer {}", access_token))
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

    fn auth_post_no_body(
        &self,
        url: String,
        body: String,
        access_token: String,
    ) -> Result<()> {
        match self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .header("Authorization", format!("Bearer {}", access_token))
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

    fn auth_put<T: DeserializeOwned>(
        &self,
        url: String,
        body: String,
        access_token: String,
    ) -> Result<T> {
        match self
            .client
            .put(&url)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .header("Authorization", format!("Bearer {}", access_token))
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

    fn auth_delete(&self, url: String, access_token: String) -> Result<()> {
        match self
            .client
            .delete(&url)
            .header("Content-Type", "application/json")
            .header("trakt-api-version", "2")
            .header("trakt-api-key", self.client_id.as_str())
            .header("Authorization", format!("Bearer {}", access_token))
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

    pub fn certifications(&self, ct: CertificationsType) -> Result<Certifications> {
        self.get(api_url!(("certifications", ct.to_string())))
    }

    pub fn countries(&self, media_type: MediaType) -> Result<Vec<Country>> {
        self.get(api_url!(("countries", media_type.to_string())))
    }

    pub fn genres(&self, media_type: MediaType) -> Result<Vec<Genre>> {
        self.get(api_url!(("genres", media_type.to_string())))
    }

    pub fn languages(&self, media_type: MediaType) -> Result<Vec<Language>> {
        self.get(api_url!(("languages", media_type.to_string())))
    }

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
