//! Noun serialization and deserialization.
//!
//! # Serialization
//!
//! [Jam] is a bitwise encoding of a noun. There are three different entities that may appear in
//! the encoding: atoms, cells, and backreferences. Each entity is identified by a unique sequence
//! of one or two tag bits: an atom's tag is `0b0`, a cell's tag is `0b01`, and a backreference's
//! tag is `0b11`. Encoding begins with the noun, which must be either an atom or a cell.
//!
//! If the noun is an atom, the tag `0b0` is written to the bitstream, followed by the encoded
//! length of the atom. The length is encoded by writing `N` low bits to the bitstream, where `N`
//! is the number of bits required to represent the length, followed by a single high bit, followed
//! by the bits of the length (from least significant to most significant) *with the most
//! significant high bit of the length omitted*. Then, the bits of the atom itself (from least
//! significant to most significant) are written to the bitstream.
//!
//! The atom `19`, for example, serializes ("jams") to `2480`, or `0b100110110000`, which breaks
//! into the tag bits, length bits, and atom bits as follows:
//! ```text
//!  10011      011000      0
//! |_____|    |______|    |_|
//!  atom       length     tag
//! ```
//!
//! If the noun is a cell, the tag `0b01` is written to the bitstream, followed by the encoded
//! head and then the encoded tail.
//!
//! The cell `[0 19]`, for example, serializes ("jams") to `39_689`, or `0b1001101100001001`, which
//! breaks into the tag bits, head bits, and tail bits as follows:
//! ```text
//!  10011         011000         0         1         0         01
//! |_____|       |______|       |_|       |_|       |_|       |__|
//!  tail        tail length  tail tag   head len  head tag  cell tag
//! ```
//! **Note**: The atom `0` serializes to `0b1` because it has length zero.
//!
//!
//! The above description of how atoms and cells are encoded ignored backreferences. A
//! backreference is encoded into the bitstream when a noun that has already been encoded in the
//! bitstream appears again during encoding. A backreference is simply an index into a prior part
//! of the bitstream at which the encoding of the first occurence of the noun in question was
//! encoded. A backreference is encoded just like an atom, except the atom tag `0b0` is replaced
//! with the backreference tag `0b11`. However, if the noun in question is an atom and the encoded
//! atom requires fewer bits than the corresponding backreference, then the atom is encoded into the
//! bitstream rather than the backreference.
//!
//! The cell `[1 1]`, for example, does not have any backreferences in its encoding because `1`
//! requires fewer bits to encode than the backreference that would replace the second occurence of
//! `1` in the bitstream. The cell `[10_000 10_000]`, which does have a backreference in its
//! encoding, serializes ("jams") to `4_952_983_169`, or `0b100100111001110001000011010000001`,
//! which breaks down as follows (notice how `tail` is a backreference, which decodes into the
//! index `2`, which is the start of the encoding of the head):
//! ```text
//!   +---------------------------------------------------------------------> idx 2
//!   |                                                                         |
//!  10         0100         11         10011100010000         11010000         0         01
//! |__|       |____|       |__|       |______________|       |________|       |_|       |__|
//! tail      tail length  tail tag          head             head length    head tag   cell tag
//! ```
//!
//! # Deserialization
//! [Cue] is a bitwise decoding of a jammed noun. It's simply the inverse of the jam encoding
//! described above.
//!
//! [Jam]: https://developers.urbit.org/reference/hoon/stdlib/2p#jam
//! [Cue]: https://developers.urbit.org/reference/hoon/stdlib/2p#cue

use crate::{atom::Atom, marker::Nounish};
use std::{
    fmt::{self, Display, Formatter},
    result,
};

/// Errors that occur when serializing/deserializing.
#[derive(Debug)]
pub enum Error {
    /// Building up an atom with [`atom::Builder`](crate::atom::Builder) failed.
    AtomBuilding,
    /// A key lookup in the cache failed.
    CacheMiss,
    /// A corrupt backreference was encountered.
    InvalidBackref,
    /// A corrupt length encoding was encountered.
    InvalidLen,
    /// A corrupt tag was encountered.
    InvalidTag,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> result::Result<(), fmt::Error> {
        match self {
            Self::AtomBuilding => write!(f, "building an atom a bit at a time failed"),
            Self::CacheMiss => write!(
                f,
                "a key that was expected to be in the cache was missing from the cache"
            ),
            Self::InvalidBackref => write!(f, "encountered an invalid backreference"),
            Self::InvalidLen => write!(f, "encountered an invalid length"),
            Self::InvalidTag => write!(f, "encountered an invalid tag"),
        }
    }
}

/// A specialized [`Result`] type for serialization/deserialization operations that return
/// [`serdes::Error`] on error.
///
/// [`serdes::Error`]: [`Error`]
pub type Result<T> = std::result::Result<T, Error>;

/// Serialize a noun type into a bitstream.
#[doc(alias("serialize", "serialization"))]
pub trait Jam: Nounish {
    /// Serializes ("jams") a noun, returning the resulting bitstream as an atom.
    #[doc(alias("serialize", "serialization"))]
    fn jam(self) -> Atom;
}

/// Deserialize a bitstream into a noun type.
#[doc(alias("deserialize", "deserialization"))]
pub trait Cue: Nounish + Sized {
    /// Deserializes ("cues") a jammed noun (a bitstream represented as an atom), returning the
    /// resulting noun type.
    #[doc(alias("deserialize", "deserialization"))]
    fn cue(jammed_noun: Atom) -> Result<Self>;
}
