#[derive(Debug, Serialize, Deserialize)]
pub struct Authentication {
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: Option<String>,
    grant_type: Option<String>
}