/// Trait allowing to request extended info. [More]
///
/// [More]: https://trakt.docs.apiary.io/#introduction/extended-info
pub trait ExtendedInfo {
    fn none(self) -> Self;
    fn full(self) -> Self;
}

/// Trait allowing to request extended info and metadata. [More]
///
/// [More]: https://trakt.docs.apiary.io/#introduction/extended-info
pub trait ExtendedInfoMetadata: ExtendedInfo {
    fn metadata(self) -> Self;
}