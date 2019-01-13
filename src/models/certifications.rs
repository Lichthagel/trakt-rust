#[derive(Debug, Serialize, Deserialize)]
pub struct Certifications {
    us: Vec<Certification>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Certification {
    name: String,
    slug: String,
    description: String
}

pub enum CertificationsType {
    MOVIES,
    SHOWS
}

impl ToString for CertificationsType {
    fn to_string(&self) -> String {
        match self {
            CertificationsType::MOVIES => String::from("movies"),
            CertificationsType::SHOWS => String::from("shows")
        }
    }
}