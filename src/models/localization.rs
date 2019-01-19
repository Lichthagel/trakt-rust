#[derive(Debug, Serialize, Deserialize)]
pub struct Alias {
    title: String,
    country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    name: String,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Country {
    name: String,
    code: String,
}
