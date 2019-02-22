/// The response of getting certifications.
/// [API docs]
///
/// [API docs]: https://trakt.docs.apiary.io/#reference/certifications
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Certifications {
    pub us: Vec<Certification>,
}

/// A certification.
/// [API docs]
///
/// [API docs]: https://trakt.docs.apiary.io/#reference/certifications
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Certification {
    pub name: String,
    pub slug: String,
    pub description: String,
}

/// For requesting certifications for shows or for movies
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
