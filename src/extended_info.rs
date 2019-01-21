pub trait ExtendedInfo {
    fn none(self) -> Self;
    fn full(self) -> Self;
}

pub trait ExtendedInfoMetadata {
    fn none(self) -> Self;
    fn metadata(self) -> Self;
    fn full(self) -> Self;
}