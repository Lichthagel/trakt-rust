#[derive(Debug)]
pub enum Error {
    Response(reqwest::Response),
    Connection(reqwest::Error)
}