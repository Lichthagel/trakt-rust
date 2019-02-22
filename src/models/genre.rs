/// A [genre]
///
/// [genre]: https://trakt.docs.apiary.io/#reference/genres
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Genre {
    pub name: String,
    pub slug: String,
}
