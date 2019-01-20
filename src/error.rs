use std::result;

#[derive(Debug)]
pub enum Error {
    Response(reqwest::Response),
    Connection(reqwest::Error),
    Serde(serde_json::Error),
    NoneError,
    ClientSecretNeeded,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Connection(e)
    }
}

impl From<reqwest::Response> for Error {
    fn from(res: reqwest::Response) -> Self {
        Error::Response(res)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

pub type Result<T> = result::Result<T, Error>;
