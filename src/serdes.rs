use crate::atom::Atom;

#[derive(Debug)]
pub enum Error {
    AtomConstruction,
    ByteAlignment,
    CacheMiss,
    InvalidBackref,
    InvalidLen,
    InvalidTag,
}

/// A specialized `Result` type for serialization/deserialization operations that return
/// `serdes::Error` on error.
pub type Result<T> = std::result::Result<T, Error>;

pub trait Jam {
    /// Serialize a noun into a bitstream represented as an atom.
    fn jam(self) -> Atom;
}

pub trait Cue: Sized {
    /// Deserialize a bitstream represented as an atom into a noun.
    fn cue(jammed_noun: Atom) -> Result<Self>;
}
