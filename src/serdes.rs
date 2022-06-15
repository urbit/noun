/// Jamming a type involves two steps:
/// (1) Convert the implementing type into a noun.
/// (2) Serialize the noun from (1).
///
/// If the implementing type also implements Noun, then (1) is trivial.
pub trait Jam
where
    Self: Sized,
{
    type Error;

    fn jam(self) -> Result<Vec<u8>, Self::Error>;
}

/// Cueing a type involves two steps:
/// (1) Deserialize the jammed value into a noun.
/// (2) Convert the noun from (1) into the implementing type.
///
/// If the implementing type also implements Noun, then (2) is trivial.
pub trait Cue
where
    Self: Sized,
{
    type Error;

    fn cue(jammed_val: Vec<u8>) -> Result<Self, Self::Error>;
}
