struct Authenctication {
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: Option<String>,
    grant_type: Option<String>
}