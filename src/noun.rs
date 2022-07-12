use crate::{
    atom::{self, Atom, AtomBuilder},
    cell::Cell,
    serdes::{self, Cue, Jam},
    Rc,
};
use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter},
    mem::drop,
};

#[derive(Clone, Debug, Eq, Hash)]
pub enum Noun {
    Atom(Atom),
    Cell(Cell),
}

impl Cue for Noun {
    fn cue(jammed_noun: Atom) -> serdes::Result<Self> {
        fn decode_len(bits: &mut atom::Iter) -> serdes::Result<u64> {
            let mut len_of_len = 0;
            loop {
                match bits.next() {
                    Some(true) => break,
                    Some(false) => len_of_len += 1,
                    None => return Err(serdes::Error::InvalidLen),
                }
            }

            if len_of_len == 0 {
                Ok(0)
            } else {
                // The most significant bit of the length is implicit because it's always 1.
                let len_bits = len_of_len - 1;
                let mut len: u64 = 1 << len_bits;
                for i in 0..len_bits {
                    match bits.next() {
                        Some(true) => len |= 1 << i,
                        Some(false) => len &= !(1 << i),
                        None => return Err(serdes::Error::InvalidLen),
                    }
                }
                Ok(len)
            }
        }

        fn decode_atom(bits: &mut atom::Iter) -> serdes::Result<Atom> {
            let len = decode_len(bits)?;
            if len == 0 {
                Ok(Atom::from(0u8))
            } else {
                let mut atom_builder = AtomBuilder::new();
                for _ in 0..len {
                    if let Some(bit) = bits.next() {
                        atom_builder.push_bit(bit);
                    } else {
                        return Err(serdes::Error::AtomConstruction);
                    }
                }
                Ok(atom_builder.into_atom())
            }
        }

        fn decode_backref(
            bits: &mut atom::Iter,
            cache: &mut HashMap<u64, Rc<Noun>>,
        ) -> serdes::Result<Rc<Noun>> {
            let atom = decode_atom(bits)?;
            if let Some(idx) = atom.as_u64() {
                if let Some(noun) = cache.get(&idx) {
                    Ok(noun.clone())
                } else {
                    Err(serdes::Error::CacheMiss)
                }
            } else {
                Err(serdes::Error::InvalidBackref)
            }
        }

        fn decode_cell(
            bits: &mut atom::Iter,
            cache: &mut HashMap<u64, Rc<Noun>>,
        ) -> serdes::Result<Cell> {
            let head_pos = bits.pos() as u64;
            let head = decode(bits, cache)?;
            cache.insert(head_pos, head.clone());

            let tail_pos = bits.pos() as u64;
            let tail = decode(bits, cache)?;
            cache.insert(tail_pos, tail.clone());

            Ok(Cell::from([head, tail]))
        }

        fn decode(
            bits: &mut atom::Iter,
            cache: &mut HashMap<u64, Rc<Noun>>,
        ) -> serdes::Result<Rc<Noun>> {
            match bits.next() {
                Some(true) => {
                    match bits.next() {
                        // Back reference tag = 0b11.
                        Some(true) => decode_backref(bits, cache),
                        // Cell tag = 0b01.
                        Some(false) => {
                            let pos = bits.pos() as u64;
                            let cell = decode_cell(bits, cache)?.into_noun_ptr();
                            cache.insert(pos, cell.clone());
                            Ok(cell)
                        }
                        None => return Err(serdes::Error::InvalidTag),
                    }
                }
                // Atom tag = 0b0.
                Some(false) => {
                    let pos = bits.pos() as u64;
                    let atom = decode_atom(bits)?.into_noun_ptr();
                    cache.insert(pos, atom.clone());
                    Ok(atom)
                }
                None => unimplemented!(),
            }
        }

        let mut bits = jammed_noun.iter();
        let mut cache = HashMap::new();
        let noun = decode(&mut bits, &mut cache)?;
        // Dropping the cache guarantees that the top level noun has exactly one reference, which
        // makes it safe to move out of the Rc.
        drop(cache);
        let noun = Rc::try_unwrap(noun).unwrap();
        Ok(noun)
    }
}

impl Jam for Noun {
    fn jam(self) -> Atom {
        fn encode_len(mut len: u64, bits: &mut AtomBuilder) {
            let len_of_len = u64::BITS - len.leading_zeros();
            for _ in 0..len_of_len {
                bits.push_bit(false);
            }
            bits.push_bit(true);
            if len_of_len != 0 {
                // Don't write the most significant bit of the length because it's always 1.
                while len != 1 {
                    bits.push_bit((len & 1) != 0);
                    len >>= 1;
                }
            }
        }

        fn encode_atom(atom: &Atom, bits: &mut AtomBuilder) {
            // Atom tag = 0b0.
            bits.push_bit(false);
            encode_len(atom.bit_len() as u64, bits);
            for bit in atom.iter() {
                bits.push_bit(bit);
            }
        }

        fn encode(noun: Rc<Noun>, bits: &mut AtomBuilder, cache: &mut HashMap<Rc<Noun>, u64>) {
            if let Some(idx) = cache.get(&noun) {
                if let Noun::Atom(ref atom) = *noun {
                    let idx_bit_len = u64::from(u64::BITS - idx.leading_zeros());
                    let atom_bit_len = atom.bit_len() as u64;
                    // Backreferences to atoms are only encoded if they're shorter than the atom it
                    // would reference.
                    if atom_bit_len <= idx_bit_len {
                        encode_atom(atom, bits);
                        return;
                    }
                }
                let idx = Atom::from(*idx);
                // Backreference tag = 0b11.
                bits.push_bit(true);
                bits.push_bit(true);
                encode_len(idx.bit_len() as u64, bits);
                for bit in idx.iter() {
                    bits.push_bit(bit);
                }
                return;
            }

            cache.insert(noun.clone(), bits.pos() as u64);
            match *noun {
                Noun::Atom(ref atom) => encode_atom(atom, bits),
                Noun::Cell(ref cell) => {
                    // Cell tag = 0b01.
                    bits.push_bit(true);
                    bits.push_bit(false);
                    encode(cell.head(), bits, cache);
                    encode(cell.tail(), bits, cache);
                }
            }
        }

        let noun = Rc::new(self);
        let mut bits = AtomBuilder::new();
        let mut cache = HashMap::new();
        encode(noun, &mut bits, &mut cache);
        bits.into_atom()
    }
}

impl Display for Noun {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Atom(atom) => atom.fmt(f),
            Self::Cell(cell) => cell.fmt(f),
        }
    }
}

impl PartialEq for Noun {
    fn eq(&self, other: &Self) -> bool {
        if let (Self::Atom(this), Self::Atom(that)) = (self, other) {
            this == that
        } else if let (Self::Cell(this), Self::Cell(that)) = (self, other) {
            this == that
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cue_atom() {
        // 2 deserializes to 0.
        {
            let jammed_noun = Atom::from(0b10u8);
            let atom = Noun::cue(jammed_noun).expect("cue");
            assert_eq!(atom, Atom::from(0u8).into_noun());
        }

        // 12 deserializes to 1.
        {
            let jammed_noun = Atom::from(0b1100u8);
            let atom = Noun::cue(jammed_noun).expect("cue");
            assert_eq!(atom, Atom::from(1u8).into_noun());
        }

        // 72 deserializes to 2.
        {
            let jammed_noun = Atom::from(0b1001000u8);
            let atom = Noun::cue(jammed_noun).expect("cue");
            assert_eq!(atom, Atom::from(2u8).into_noun());
        }

        // 2480 deserializes to 19.
        {
            let jammed_noun = Atom::from(0b100110110000u16);
            let atom = Noun::cue(jammed_noun).expect("cue");
            assert_eq!(atom, Atom::from(19u8).into_noun());
        }
    }

    #[test]
    fn cue_cell() {
        // 39.689 deserializes into [0 19].
        {
            let jammed_noun = Atom::from(0b1001101100001001u16);
            let cell = Noun::cue(jammed_noun).expect("cue");
            assert_eq!(cell, Cell::from([0u8, 19u8]).into_noun());
        }

        // 817 deserializes to [1 1].
        {
            let jammed_noun = Atom::from(0b1100110001u16);
            let cell = Noun::cue(jammed_noun).expect("cue");
            assert_eq!(cell, Cell::from([1u8, 1u8]).into_noun());
        }

        // 4.952.983.169 deserializes into [10.000 10.000].
        {
            let jammed_noun = Atom::from(0b100100111001110001000011010000001u64);
            let cell = Noun::cue(jammed_noun).expect("cue");
            assert_eq!(cell, Cell::from([10_000u16, 10_000u16]).into_noun());
        }

        // 1.301.217.674.263.809 serializes to [999.999.999 999.999.999].
        {
            let jammed_noun = Atom::from(0b100100111110111001101011001001111111111110100000001u64);
            let cell = Noun::cue(jammed_noun).expect("cue");
            assert_eq!(
                cell,
                Cell::from([999_999_999u32, 999_999_999u32]).into_noun()
            );
        }

        // 635.080.761.093 deserializes into [[107 110] [107 110]].
        {
            let jammed_noun = Atom::from(0b1001001111011101110000110101111100000101u64);
            let cell = Noun::cue(jammed_noun).expect("cue");
            let head = Cell::from([107u8, 110u8]).into_noun_ptr();
            assert_eq!(cell, Cell::from([head.clone(), head]).into_noun());
        }
    }

    #[test]
    fn jam_atom() {
        // 0 serializes to 2.
        {
            let atom = Atom::from(0u8).into_noun();
            assert_eq!(atom.jam(), Atom::from(2u8));
        }

        // 1 serializes to 12.
        {
            let atom = Atom::from(1u8).into_noun();
            assert_eq!(atom.jam(), Atom::from(12u8));
        }

        // 2 serializes to 72.
        {
            let atom = Atom::from(2u8).into_noun();
            assert_eq!(atom.jam(), Atom::from(72u8));
        }

        // 19 serializes to 2480.
        {
            let atom = Atom::from(19u8).into_noun();
            assert_eq!(atom.jam(), Atom::from(2480u16));
        }

        // 581.949.002 serializes to 1.191.831.557.952.
        {
            let atom = Atom::from(581_949_002u32).into_noun();
            assert_eq!(atom.jam(), Atom::from(1_191_831_557_952u64));
        }
    }

    #[test]
    fn jam_cell() {
        // [0 19] serializes into 39.689.
        {
            let cell = Cell::from([0u8, 19u8]).into_noun();
            assert_eq!(cell.jam(), Atom::from(39_689u16));
        }

        // [1 1] serializes to 817.
        {
            let cell = Cell::from([1u8, 1u8]).into_noun();
            assert_eq!(cell.jam(), Atom::from(817u16));
        }

        // [222 444 888] serializes to 250.038.217.192.960.129.
        {
            let cell = Cell::from([222u16, 444u16, 888u16]).into_noun();
            assert_eq!(cell.jam(), Atom::from(250_038_217_192_960_129u64));
        }

        // [0 1 2 3 4 5 6 7 8 9 10] serializes to 25.681.224.503.728.653.597.984.370.231.065.
        {
            let cell =
                Cell::from([0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8]).into_noun();
            assert_eq!(
                cell.jam(),
                Atom::from(25_681_224_503_728_653_597_984_370_231_065u128)
            );
        }

        // [99 100 101 102 103 104 0] serializes to 223.372.995.869.285.333.705.242.560.449.
        {
            let cell = Cell::from([99u8, 100u8, 101u8, 102u8, 103u8, 104u8, 0u8]).into_noun();
            assert_eq!(
                cell.jam(),
                Atom::from(223_372_995_869_285_333_705_242_560_449u128)
            );
        }

        // [[222 444 888] [222 444 888]] serializes to 170.479.614.045.978.345.989.
        {
            let head = Cell::from([222u16, 444u16, 888u16]).into_noun_ptr();
            let cell = Cell::from([head.clone(), head]).into_noun();
            assert_eq!(cell.jam(), Atom::from(170_479_614_045_978_345_989u128));
        }

        // [[0 1] [1 2] [2 3] [3 4] 0] serializes to 11.976.248.475.217.237.797.
        {
            let cell = Cell::from([
                Cell::from([0u8, 1u8]).into_noun(),
                Cell::from([1u8, 2u8]).into_noun(),
                Cell::from([2u8, 3u8]).into_noun(),
                Cell::from([3u8, 4u8]).into_noun(),
                Atom::from(0u8).into_noun(),
            ])
            .into_noun();
            assert_eq!(cell.jam(), Atom::from(11_976_248_475_217_237_797u64));
        }

        // [[0 1] [1 2] [2 3] [3 4] [4 5] [5 6] [6 7] [7 8] [8 9] 0] serializes to
        // 7.694.087.033.387.855.647.747.387.855.514.468.399.947.749.137.782.565.
        {
            let cell = Cell::from([
                Cell::from([0u8, 1u8]).into_noun(),
                Cell::from([1u8, 2u8]).into_noun(),
                Cell::from([2u8, 3u8]).into_noun(),
                Cell::from([3u8, 4u8]).into_noun(),
                Cell::from([4u8, 5u8]).into_noun(),
                Cell::from([5u8, 6u8]).into_noun(),
                Cell::from([6u8, 7u8]).into_noun(),
                Cell::from([7u8, 8u8]).into_noun(),
                Cell::from([8u8, 9u8]).into_noun(),
                Atom::from(0u8).into_noun(),
            ])
            .into_noun();
            let jammed_cell = Atom::from(vec![
                37, 23, 35, 11, 137, 46, 52, 102, 97, 226, 22, 46, 118, 97, 227, 23, 62, 4, 11,
                130, 144, 20,
            ]);
            assert_eq!(cell.jam(), jammed_cell);
        }

        // [[0 1] [2 3] [4 5] [6 7] [8 9] [10 11] [12 13] [14 15] [16 17] [18 19] [20 21] 0]
        // serializes to
        // 308.947.677.754.874.070.959.300.747.182.056.036.528.545.493.781.368.831.595.479.491.505.523.344.414.501.
        {
            let cell = Cell::from([
                Cell::from([0u8, 1u8]).into_noun(),
                Cell::from([2u8, 3u8]).into_noun(),
                Cell::from([4u8, 5u8]).into_noun(),
                Cell::from([6u8, 7u8]).into_noun(),
                Cell::from([8u8, 9u8]).into_noun(),
                Cell::from([10u8, 11u8]).into_noun(),
                Cell::from([12u8, 13u8]).into_noun(),
                Cell::from([14u8, 15u8]).into_noun(),
                Cell::from([16u8, 17u8]).into_noun(),
                Cell::from([18u8, 19u8]).into_noun(),
                Cell::from([20u8, 21u8]).into_noun(),
                Atom::from(0u8).into_noun(),
            ])
            .into_noun();
            let jammed_cell = Atom::from(vec![
                37, 23, 18, 93, 152, 184, 133, 141, 95, 16, 132, 100, 65, 20, 178, 5, 97, 72, 23,
                196, 33, 95, 48, 8, 139, 5, 147, 176, 89, 48, 10, 171, 2,
            ]);
            assert_eq!(cell.jam(), jammed_cell);
        }

        // [[%vary 'Origin'] [%vary 'Accept-Encoding']] serializes to
        // 2.923.956.498.268.356.738.336.949.786.175.643.457.788.180.560.108.194.340.456.079.961.920.720.567.301.
        {
            let cell = Cell::from([
                Cell::from(["vary", "Origin"]),
                Cell::from(["vary", "Accept-Encoding"]),
            ])
            .into_noun();
            let jammed_cell = Atom::from(vec![
                5, 124, 187, 48, 185, 60, 224, 123, 146, 75, 59, 75, 115, 55, 19, 224, 29, 52, 54,
                86, 6, 71, 215, 82, 228, 54, 246, 70, 150, 230, 118, 6,
            ]);
            assert_eq!(cell.jam(), jammed_cell);
        }

        // [%x-cached 'HIT'] serializes to 3.419.056.981.361.227.851.413.339.139.505.665.
        {
            let cell = Cell::from(["x-cached", "HIT"]).into_noun();
            let jammed_cell = Atom::from(3_419_056_981_361_227_851_413_339_139_505_665u128);
            assert_eq!(cell.jam(), jammed_cell);
        }

        // [
        //  [%x-cached 'HIT']
        //  [%vary 'Origin']
        //  [%vary 'Accept-Encoding']
        //  0
        // ]
        // serializes to
        // 54.024.941.019.988.598.271.402.968.678.037.641.784.219.129.665.004.500.606.995.034.380.342.041.694.044.032.862.185.742.959.381.716.818.503.067.105.285.
        {
            let cell = Cell::from([
                Cell::from(["x-cached", "HIT"]).into_noun(),
                Cell::from(["vary", "Origin"]).into_noun(),
                Cell::from(["vary", "Accept-Encoding"]).into_noun(),
                Atom::from(0u8).into_noun(),
            ])
            .into_noun();
            let jammed_cell = Atom::from(vec![
                5, 248, 241, 90, 198, 194, 198, 208, 202, 200, 192, 67, 74, 162, 22, 240, 237, 194,
                228, 242, 128, 239, 73, 46, 237, 44, 205, 93, 227, 118, 128, 119, 208, 216, 88, 25,
                28, 93, 75, 145, 219, 216, 27, 89, 154, 219, 89,
            ]);
            assert_eq!(cell.jam(), jammed_cell);
        }

        // TODO: add a header at a time until the failure occurs in a simpler case.

        /*
        // [
        //  [%x-cached 'HIT']
        //  [%vary 'Origin']
        //  [%vary 'Accept-Encoding']
        //  [%connection %keep-alive]
        //  [%content-length '59']
        //  [%content-type 'application/json']
        //  [%date 'Fri, 08 Jul 2022 16:43:50 GMT']
        //  [%server 'nginx/1.14.0 (Ubuntu)']
        //  0
        // ]
        // serializes to
        // 3.043.179.738.672.136.626.575.394.190.673.759.579.727.849.692.505.022.485.010.845.918.679.803.129.305.613.240.481.085.317.833.394.833.266.220.825.965.186.842.937.880.270.236.914.729.803.832.464.269.764.501.574.488.877.790.107.593.782.177.681.982.718.127.995.102.131.872.994.171.509.635.987.438.144.206.782.752.956.394.374.740.266.989.132.241.617.591.666.709.510.062.224.526.419.612.002.666.485.664.696.539.844.896.863.868.458.496.893.612.588.610.229.380.812.024.688.103.535.216.455.403.528.290.497.755.248.780.777.953.932.500.730.437.340.171.838.891.251.943.028.869.039.612.929.246.368.535.415.502.947.874.821
        {
            let cell = Cell::from([
                Cell::from(["x-cached", "HIT"]).into_noun(),
                Cell::from(["vary", "Origin"]).into_noun(),
                Cell::from(["vary", "Accept-Encoding"]).into_noun(),
                Cell::from(["connection", "keep-alive"]).into_noun(),
                Cell::from(["content-length", "59"]).into_noun(),
                Cell::from(["content-type", "application/json"]).into_noun(),
                Cell::from(["date", "Fri, 08 Jul 2022 16:43:50 GMT"]).into_noun(),
                Cell::from(["server", "nginx/1.14.0 (Ubuntu])"]).into_noun(),
                Atom::from(0u8).into_noun(),
            ])
            .into_noun();
            let jammed_cell = Atom::from(vec![
                5, 248, 241, 90, 198, 194, 198, 208, 202, 200, 192, 67, 74, 162, 22, 240, 237, 194,
                228, 242, 128, 239, 73, 46, 237, 44, 205, 93, 227, 118, 128, 119, 208, 216, 88, 25,
                28, 93, 75, 145, 219, 216, 27, 89, 154, 219, 185, 0, 62, 99, 111, 110, 110, 101,
                99, 116, 105, 111, 110, 128, 207, 90, 89, 25, 92, 75, 24, 91, 154, 93, 185, 0, 190,
                99, 111, 110, 116, 101, 110, 116, 45, 108, 101, 110, 103, 116, 104, 208, 53, 121,
                1, 252, 198, 222, 220, 232, 202, 220, 232, 90, 232, 242, 224, 202, 0, 255, 48, 56,
                56, 182, 180, 177, 48, 186, 180, 55, 183, 23, 181, 185, 55, 119, 1, 159, 44, 140,
                174, 12, 224, 217, 72, 46, 141, 5, 4, 6, 7, 68, 169, 142, 13, 68, 6, 70, 70, 6, 36,
                198, 70, 135, 102, 70, 167, 6, 6, 228, 168, 137, 90, 128, 111, 174, 76, 206, 174,
                76, 14, 160, 201, 237, 44, 205, 13, 239, 37, 198, 37, 134, 198, 5, 6, 4, 165, 74,
                172, 206, 141, 174, 46, 21,
            ]);
            assert_eq!(cell.jam(), jammed_cell);
        }
        */
    }
}
