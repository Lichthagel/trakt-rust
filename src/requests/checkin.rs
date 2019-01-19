use crate::{
    error::{Error, Result},
    models::{Checkin, CheckinResponse},
    TraktApi,
};

impl TraktApi {
    pub fn checkin(&self, checkin: Checkin, access_token: String) -> Result<CheckinResponse> {
        self.auth_post(
            api_url!(("checkin")),
            checkin.to_json_string()?,
            access_token,
        )
    }

    pub fn checkout(&self, access_token: String) -> Result<()> {
        match self
            .client
            .delete(&api_url!(("checkin")))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("trakt-api-version", 2)
            .header("trakt-api-key", self.client_id.as_str())
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
}
