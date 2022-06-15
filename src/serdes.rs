use crate::{FromNoun, IntoNoun};

/// Jamming a type involves two steps:
/// (1) Convert the implementing type into a noun.
/// (2) Serialize the noun from (1).
///
/// If the implementing type also implements Noun, then (1) is trivial.
pub trait Jam: IntoNoun + Sized {
    type Error;

    fn jam(self) -> Result<Vec<u8>, <Self as Jam>::Error>;
}

/// Cueing a type involves two steps:
/// (1) Deserialize the jammed value into a noun.
/// (2) Convert the noun from (1) into the implementing type.
///
/// If the implementing type also implements Noun, then (2) is trivial.
pub trait Cue: FromNoun + Sized {
    type Error;

    fn cue(jammed_val: Vec<u8>) -> Result<Self, <Self as Cue>::Error>;
}
