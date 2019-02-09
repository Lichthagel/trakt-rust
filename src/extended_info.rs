/// Trait allowing to request no extended info. [More]
///
/// [More]: https://trakt.docs.apiary.io/#introduction/extended-info
pub trait ExtendedInfoNone: Sized + WithNone {
    fn none(self) -> Self::None;
}

/// Trait allowing to request full extended info. [More]
///
/// [More]: https://trakt.docs.apiary.io/#introduction/extended-info
pub trait ExtendedInfoFull: Sized + WithFull {
    fn full(self) -> Self::Full;
}

/// Trait allowing to request extended info and metadata. [More]
///
/// [More]: https://trakt.docs.apiary.io/#introduction/extended-info
pub trait ExtendedInfoMetadata: Sized + WithMetadata {
    fn metadata(self) -> Self::Metadata;
}

/// Trait allowing to request extended info. [More]
///
/// [More]: https://trakt.docs.apiary.io/#introduction/extended-info
pub trait ExtendedInfo: ExtendedInfoFull + ExtendedInfoNone {}

/// To define a type without extended info
pub trait WithNone {
    type None;
}

/// To define a type with full extended info
pub trait WithFull {
    type Full;
}

/// To define a type with metadata extended info
pub trait WithMetadata {
    type Metadata;
}
