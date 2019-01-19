#[derive(Debug)]
pub enum Error {
    Response(reqwest::Response),
    Connection(reqwest::Error),
    ClientSecretNeeded
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