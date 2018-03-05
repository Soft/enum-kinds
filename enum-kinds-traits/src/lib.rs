#![no_std]

pub trait ToKind {
    type Kind;

    fn kind(&self) -> Self::Kind;
}
