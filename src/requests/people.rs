use crate::{
    error::Error,
    models::Person,
    TraktApi
};

impl TraktApi {
    pub fn person(&self, id: String) -> Result<Person, Error> {
        self.get(api_url!(("people", id)))
    }
}