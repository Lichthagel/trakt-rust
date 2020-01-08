#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    ClientSecretNeeded,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}
