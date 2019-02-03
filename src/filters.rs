use std::fmt;

pub trait CommonFilters {
    fn query(self, query: &str) -> Self;
    fn year(self, year: u32) -> Self;
    fn genre(self, genre_slug: &str) -> Self;
    fn language(self, language_code: &str) -> Self;
    fn country(self, country_code: &str) -> Self;
    fn runtimes(self, from: u32, to: u32) -> Self;
    fn ratings(self, from: u32, to: u32) -> Self;
}

pub trait MovieFilters {
    fn certification(self, cert_slug: &str) -> Self;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShowStatus {
    #[serde(rename = "returning series")]
    Returning,
    #[serde(rename = "in production")]
    InProduction,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "ended")]
    Ended,
}

impl fmt::Display for ShowStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            ShowStatus::Returning => "returning series",
            ShowStatus::InProduction => "in production",
            ShowStatus::Planned => "planned",
            ShowStatus::Cancelled => "cancelled",
            ShowStatus::Ended => "ended",
        })
    }
}

pub trait ShowFilters {
    fn certification(self, cert_slug: &str) -> Self;
    fn network(self, network_name: &str) -> Self;
    fn status(self, status: ShowStatus) -> Self;
}

pub trait TypeFilter<T> {
    fn item_type(self, item_type: T) -> Self;
}
