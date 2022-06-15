use crate::IntoNoun;

pub trait Atom: IntoNoun + Sized {
    type Error;

    fn as_vec(&self) -> Vec<u8>;

    fn as_u64(&self) -> Result<u64, <Self as Atom>::Error>;
}
