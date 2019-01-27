use std::fmt;
use std::fmt::Display;
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

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::Response(e) => format!("Response ( {} )", e.status()),
                Error::Connection(e) => format!("Connection ( {} )", e),
                Error::Serde(e) => format!("Serde ( {} )", e),
                Error::NoneError => format!("NoneError"),
                Error::ClientSecretNeeded => format!("ClientSecretNeeded"),
            }
        )
    }
}

pub type Result<T> = result::Result<T, Error>;
