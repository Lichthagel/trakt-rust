#[derive(Debug)]
pub enum Error {
    RESPONSE(reqwest::Response),
    CONNECTION(reqwest::Error)
}