#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Certifications {
    pub us: Vec<Certification>,
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Certification {
    pub name: String,
    pub slug: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CertificationsType {
    Movies,
    Shows,
}

impl ToString for CertificationsType {
    fn to_string(&self) -> String {
        match self {
            CertificationsType::Movies => String::from("movies"),
            CertificationsType::Shows => String::from("shows"),
        }
    }
}
