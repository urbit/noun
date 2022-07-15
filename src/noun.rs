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
        fn decode_atom(bits: &mut atom::Iter) -> serdes::Result<Atom> {
            let len = {
                let mut len_of_len = 0;
                loop {
                    match bits.next() {
                        Some(true) => break,
                        Some(false) => len_of_len += 1,
                        None => return Err(serdes::Error::InvalidLen),
                    }
                }

                if len_of_len == 0 {
                    0
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
                    len
                }
            };
            if len == 0 {
                Ok(Atom::from(0u8))
            } else {
                let mut atom_builder = AtomBuilder::new();
                for _ in 0..len {
                    let bit = bits.next().ok_or(serdes::Error::AtomConstruction)?;
                    atom_builder.push_bit(bit);
                }
                Ok(atom_builder.into_atom())
            }
        }

        fn decode(
            bits: &mut atom::Iter,
            cache: &mut HashMap<u64, Rc<Noun>>,
        ) -> serdes::Result<Rc<Noun>> {
            let pos = bits.pos() as u64;
            match bits.next() {
                Some(true) => {
                    match bits.next() {
                        // Back reference tag = 0b11.
                        Some(true) => {
                            let idx = decode_atom(bits)?
                                .as_u64()
                                .ok_or(serdes::Error::InvalidBackref)?;
                            let noun = cache.get(&idx).ok_or(serdes::Error::CacheMiss)?;
                            Ok(noun.clone())
                        }
                        // Cell tag = 0b01.
                        Some(false) => {
                            let pos = bits.pos() as u64;
                            let head = decode(bits, cache)?;
                            cache.insert(pos, head.clone());

                            let pos = bits.pos() as u64;
                            let tail = decode(bits, cache)?;
                            cache.insert(pos, tail.clone());

                            Ok(Cell::from([head, tail]).into_noun_ptr())
                        }
                        None => return Err(serdes::Error::InvalidTag),
                    }
                }
                // Atom tag = 0b0.
                Some(false) => {
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
    fn jam_cue_atom() {
        // 0 serializes to 2.
        {
            let atom = Atom::from(0u8).into_noun();
            let jammed_atom = Atom::from(2u8);
            assert_eq!(atom.clone().jam(), jammed_atom);
            assert_eq!(Noun::cue(jammed_atom).expect("cue"), atom);
        }

        // 1 serializes to 12.
        {
            let atom = Atom::from(1u8).into_noun();
            let jammed_atom = Atom::from(12u8);
            assert_eq!(atom.clone().jam(), jammed_atom);
            assert_eq!(Noun::cue(jammed_atom).expect("cue"), atom);
        }

        // 2 serializes to 72.
        {
            let atom = Atom::from(2u8).into_noun();
            let jammed_atom = Atom::from(72u8);
            assert_eq!(atom.clone().jam(), jammed_atom);
            assert_eq!(Noun::cue(jammed_atom).expect("cue"), atom);
        }

        // 19 serializes to 2480.
        {
            let atom = Atom::from(19u8).into_noun();
            let jammed_atom = Atom::from(2480u16);
            assert_eq!(atom.clone().jam(), jammed_atom);
            assert_eq!(Noun::cue(jammed_atom).expect("cue"), atom);
        }

        // 581.949.002 serializes to 1.191.831.557.952.
        {
            let atom = Atom::from(581_949_002u32).into_noun();
            let jammed_atom = Atom::from(1_191_831_557_952u64);
            assert_eq!(atom.clone().jam(), jammed_atom);
            assert_eq!(Noun::cue(jammed_atom).expect("cue"), atom);
        }
    }

    #[test]
    fn jam_cue_cell() {
        // [0 19] serializes into 39.689.
        {
            let cell = Cell::from([0u8, 19u8]).into_noun();
            let jammed_cell = Atom::from(39_689u16);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        // [1 1] serializes to 817.
        {
            let cell = Cell::from([1u8, 1u8]).into_noun();
            let jammed_cell = Atom::from(817u16);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        // [10.000 10.000] serializes into 4.952.983.169.
        {
            let cell = Cell::from([10_000u16, 10_000u16]).into_noun();
            let jammed_cell = Atom::from(0b100100111001110001000011010000001u64);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        // [999.999.999 999.999.999] serializes to 1.301.217.674.263.809.
        {
            let cell = Cell::from([999_999_999u32, 999_999_999u32]).into_noun();
            let jammed_cell = Atom::from(0b100100111110111001101011001001111111111110100000001u64);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        // [222 444 888] serializes to 250.038.217.192.960.129.
        {
            let cell = Cell::from([222u16, 444u16, 888u16]).into_noun();
            let jammed_cell = Atom::from(250_038_217_192_960_129u64);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        // [[107 110] [107 110]] serializes to 635.080.761.093.
        {
            let head = Cell::from([107u8, 110u8]).into_noun_ptr();
            let cell = Cell::from([head.clone(), head]).into_noun();
            let jammed_cell = Atom::from(0b1001001111011101110000110101111100000101u64);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        // [0 1 2 3 4 5 6 7 8 9 10] serializes to 25.681.224.503.728.653.597.984.370.231.065.
        {
            let cell =
                Cell::from([0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8]).into_noun();

            let jammed_cell = Atom::from(25_681_224_503_728_653_597_984_370_231_065u128);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        // [99 100 101 102 103 104 0] serializes to 223.372.995.869.285.333.705.242.560.449.
        {
            let cell = Cell::from([99u8, 100u8, 101u8, 102u8, 103u8, 104u8, 0u8]).into_noun();
            let jammed_cell = Atom::from(223_372_995_869_285_333_705_242_560_449u128);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        // [[222 444 888] [222 444 888]] serializes to 170.479.614.045.978.345.989.
        {
            let head = Cell::from([222u16, 444u16, 888u16]).into_noun_ptr();
            let cell = Cell::from([head.clone(), head]).into_noun();
            let jammed_cell = Atom::from(170_479_614_045_978_345_989u128);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
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
            let jammed_cell = Atom::from(11_976_248_475_217_237_797u64);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
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
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
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
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
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
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        // [%x-cached 'HIT'] serializes to 3.419.056.981.361.227.851.413.339.139.505.665.
        {
            let cell = Cell::from(["x-cached", "HIT"]).into_noun();
            let jammed_cell = Atom::from(3_419_056_981_361_227_851_413_339_139_505_665u128);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
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
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        // [
        //  [%x-cached 'HIT']
        //  [%vary 'Origin']
        //  [%vary 'Accept-Encoding']
        //  [%connection %keep-alive]
        //  0
        // ]
        // serializes to
        // 337.262.554.346.536.272.809.263.434.776.769.507.747.563.642.163.274.356.590.866.601.381.185.210.047.197.602.041.291.492.349.359.710.146.253.456.410.124.597.539.747.968.329.210.676.233.756.528.621.237.772.236.684.881.982.257.157.
        {
            let cell = Cell::from([
                Cell::from(["x-cached", "HIT"]).into_noun(),
                Cell::from(["vary", "Origin"]).into_noun(),
                Cell::from(["vary", "Accept-Encoding"]).into_noun(),
                Cell::from(["connection", "keep-alive"]).into_noun(),
                Atom::from(0u8).into_noun(),
            ])
            .into_noun();
            let jammed_cell = Atom::from(vec![
                5, 248, 241, 90, 198, 194, 198, 208, 202, 200, 192, 67, 74, 162, 22, 240, 237, 194,
                228, 242, 128, 239, 73, 46, 237, 44, 205, 93, 227, 118, 128, 119, 208, 216, 88, 25,
                28, 93, 75, 145, 219, 216, 27, 89, 154, 219, 185, 0, 62, 99, 111, 110, 110, 101,
                99, 116, 105, 111, 110, 128, 207, 90, 89, 25, 92, 75, 24, 91, 154, 93, 89,
            ]);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        // [
        //  [%x-cached 'HIT']
        //  [%vary 'Origin']
        //  [%vary 'Accept-Encoding']
        //  [%connection %keep-alive]
        //  [%content-length '59']
        //  0
        // ]
        // serializes to
        // 3.990.449.691.910.602.529.066.905.532.683.432.376.481.595.709.178.878.504.454.195.556.485.477.933.263.285.640.666.005.735.466.251.842.927.155.004.960.860.680.532.900.649.099.981.551.377.940.562.846.107.819.739.708.673.550.687.574.046.762.646.573.624.712.055.218.066.331.781.777.383.880.709.
        {
            let cell = Cell::from([
                Cell::from(["x-cached", "HIT"]).into_noun(),
                Cell::from(["vary", "Origin"]).into_noun(),
                Cell::from(["vary", "Accept-Encoding"]).into_noun(),
                Cell::from(["connection", "keep-alive"]).into_noun(),
                Cell::from(["content-length", "59"]).into_noun(),
                Atom::from(0u8).into_noun(),
            ])
            .into_noun();

            let jammed_cell = Atom::from(vec![
                5, 248, 241, 90, 198, 194, 198, 208, 202, 200, 192, 67, 74, 162, 22, 240, 237, 194,
                228, 242, 128, 239, 73, 46, 237, 44, 205, 93, 227, 118, 128, 119, 208, 216, 88, 25,
                28, 93, 75, 145, 219, 216, 27, 89, 154, 219, 185, 0, 62, 99, 111, 110, 110, 101,
                99, 116, 105, 111, 110, 128, 207, 90, 89, 25, 92, 75, 24, 91, 154, 93, 185, 0, 190,
                99, 111, 110, 116, 101, 110, 116, 45, 108, 101, 110, 103, 116, 104, 208, 53, 185,
            ]);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        // [
        //  [%x-cached 'HIT']
        //  [%vary 'Origin']
        //  [%vary 'Accept-Encoding']
        //  [%connection %keep-alive]
        //  [%content-length '59']
        //  [%content-type 'application/json']
        //  0
        // ]
        // serializes to
        // 457.091.532.517.554.390.786.006.469.499.335.612.968.565.672.928.161.381.020.311.663.688.274.027.085.520.617.498.455.482.250.967.104.025.408.758.206.722.076.439.600.215.410.221.147.402.178.087.201.942.131.610.143.956.380.101.670.231.516.755.661.365.800.622.895.086.504.659.989.465.932.175.163.419.593.361.271.135.450.933.952.288.380.655.088.063.544.328.215.173.068.577.627.294.732.347.635.717.
        {
            let cell = Cell::from([
                Cell::from(["x-cached", "HIT"]).into_noun(),
                Cell::from(["vary", "Origin"]).into_noun(),
                Cell::from(["vary", "Accept-Encoding"]).into_noun(),
                Cell::from(["connection", "keep-alive"]).into_noun(),
                Cell::from(["content-length", "59"]).into_noun(),
                Cell::from(["content-type", "application/json"]).into_noun(),
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
                56, 182, 180, 177, 48, 186, 180, 55, 183, 23, 181, 185, 55, 183,
            ]);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        // [
        //  [%x-cached 'HIT']
        //  [%vary 'Origin']
        //  [%vary 'Accept-Encoding']
        //  [%connection %keep-alive]
        //  [%content-length '59']
        //  [%content-type 'application/json']
        //  [%date 'Fri, 08 Jul 2022 16:43:50 GMT']
        //  0
        // ]
        // serializes to
        // 13.511.042.605.182.938.704.141.471.102.024.299.594.955.332.558.307.008.475.497.880.712.859.675.484.538.257.353.584.918.969.851.530.816.463.453.015.799.780.201.687.994.505.320.116.239.292.288.026.317.441.863.373.121.089.158.775.350.462.009.180.764.811.763.710.140.560.982.428.678.690.500.900.830.579.867.363.560.770.528.863.245.324.334.716.215.913.775.596.115.997.818.308.278.262.023.037.514.343.972.996.368.914.301.576.422.996.306.770.335.657.447.377.076.271.378.523.965.990.245.753.334.799.571.283.621.312.517.
        {
            let cell = Cell::from([
                Cell::from(["x-cached", "HIT"]).into_noun(),
                Cell::from(["vary", "Origin"]).into_noun(),
                Cell::from(["vary", "Accept-Encoding"]).into_noun(),
                Cell::from(["connection", "keep-alive"]).into_noun(),
                Cell::from(["content-length", "59"]).into_noun(),
                Cell::from(["content-type", "application/json"]).into_noun(),
                Cell::from(["date", "Fri, 08 Jul 2022 16:43:50 GMT"]).into_noun(),
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
                198, 70, 135, 102, 70, 167, 6, 6, 228, 168, 137, 42,
            ]);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        //  [%server 'nginx/1.14.0 (Ubuntu)'] serializes to
        //  36.625.686.482.471.374.629.531.055.727.019.932.223.514.833.888.924.393.604.670.670.633.971.596.801.
        {
            let cell = Cell::from(["server", "nginx/1.14.0 (Ubuntu)"]).into_noun();
            let jammed_cell = Atom::from(vec![
                1, 190, 185, 50, 57, 187, 50, 57, 128, 38, 183, 179, 52, 55, 188, 151, 24, 151, 24,
                26, 23, 24, 16, 148, 42, 177, 58, 55, 186, 186, 20,
            ]);
            assert_eq!(cell.clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        // (jam [[%x-cached 'HIT'] [%vary 'Origin'] [%vary 'Accept-Encoding'] [%connection %keep-alive] [%content-length '59'] [%content-type 'application/json'] [%date 'Fri, 08 Jul 2022 16:43:50 GMT'] [%server 'nginx/1.14.0 (Ubuntu)'] 0])
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
        // 3.043.179.738.672.136.626.575.394.190.673.759.579.727.849.692.505.022.485.010.845.918.679.803.129.305.613.240.481.085.317.833.394.833.266.220.825.965.186.842.937.880.270.236.914.729.803.832.464.269.764.501.574.488.877.790.107.593.782.177.681.982.718.127.995.102.131.872.994.171.509.635.987.438.144.206.782.752.956.394.374.740.266.989.132.241.617.591.666.709.510.062.224.526.419.612.002.666.485.664.696.539.844.896.863.868.458.496.893.612.588.610.229.380.812.024.688.103.535.216.455.403.528.290.497.755.248.780.777.953.932.500.730.437.340.171.838.891.251.943.028.869.039.612.929.246.368.535.415.502.947.874.821.
        {
            let cell = Cell::from([
                Cell::from(["x-cached", "HIT"]).into_noun(),
                Cell::from(["vary", "Origin"]).into_noun(),
                Cell::from(["vary", "Accept-Encoding"]).into_noun(),
                Cell::from(["connection", "keep-alive"]).into_noun(),
                Cell::from(["content-length", "59"]).into_noun(),
                Cell::from(["content-type", "application/json"]).into_noun(),
                Cell::from(["date", "Fri, 08 Jul 2022 16:43:50 GMT"]).into_noun(),
                Cell::from(["server", "nginx/1.14.0 (Ubuntu)"]).into_noun(),
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
            assert_eq!(cell.clone().clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }

        // [
        //  %request
        //  0
        //  'POST'
        //  'http://eth-mainnet.urbit.org:8545'
        //  [['Content-Type' 'application/json'] 0]
        //  0
        //  78
        //  '[{"params":[],"id":"block number","jsonrpc":"2.0","method":"eth_blockNumber"}]'
        // ]
        // serializes to
        // 534.926.240.328.183.504.043.224.467.158.150.263.359.506.153.835.684.400.054.708.654.784.265.586.779.466.767.311.401.093.568.872.399.514.089.871.794.788.465.339.360.316.141.009.784.521.401.502.584.590.628.538.383.397.474.667.076.686.296.931.914.112.162.585.777.490.924.604.432.397.967.740.482.953.038.069.595.525.949.395.091.512.693.509.388.265.834.094.423.223.982.487.236.123.144.939.682.105.684.811.115.401.159.600.617.316.591.045.520.893.570.145.126.936.115.415.644.005.172.954.075.003.434.319.780.206.191.080.707.020.476.210.689.
        //
        // 534926240328183504043224467158150263359506153835684400054708654784265586779466767311401093568872399514089871794788465339360316141009784521401502584590628538383397474667076686296931914112162585777490924604432397967740482953038069595525949395091512693509388265834094423223982487236123144939682105684811115401159600617316591045520893570145126936115415644005172954075003434319780206191080707020476210689
        {
            let cell = Cell::from([
                Atom::from("request").into_noun(),
                Atom::from(0u8).into_noun(),
                Atom::from("POST").into_noun(),
                Atom::from("http://eth-mainnet.urbit.org:8545").into_noun(),
                Cell::from([
                    Cell::from([Atom::from("Content-Type"), Atom::from("application/json")]).into_noun(),
                    Atom::from(0u8).into_noun(),
                ]).into_noun(),
                Atom::from(0u8).into_noun(),
                Atom::from(78u8).into_noun(),
                Atom::from(r#"[{"params":[],"id":"block number","jsonrpc":"2.0","method":"eth_blockNumber"}]"#).into_noun(),

            ])
            .into_noun();
            let jammed_cell = Atom::from(vec![
                1, 94, 185, 178, 184, 186, 178, 57, 122, 6, 124, 168, 167, 41, 106, 0, 52, 64, 163,
                163, 131, 211, 121, 121, 41, 163, 67, 107, 105, 11, 75, 115, 115, 43, 163, 115,
                169, 147, 19, 75, 163, 115, 121, 147, 59, 211, 193, 169, 161, 169, 43, 128, 223,
                208, 155, 27, 93, 153, 27, 93, 11, 85, 30, 92, 25, 224, 31, 6, 7, 199, 150, 54, 22,
                70, 151, 246, 230, 246, 162, 54, 247, 230, 54, 131, 59, 1, 240, 205, 214, 158, 8,
                92, 152, 92, 88, 219, 156, 136, 206, 86, 23, 139, 72, 26, 153, 136, 142, 136, 24,
                219, 219, 216, 26, 136, 91, 93, 155, 88, 153, 156, 8, 139, 136, 218, 220, 155, 155,
                28, 220, 152, 136, 142, 136, 140, 11, 140, 8, 139, 72, 91, 25, 29, 218, 27, 153,
                136, 142, 72, 25, 29, 218, 151, 24, 219, 219, 216, 154, 83, 93, 155, 88, 153, 156,
                72, 95, 23,
            ]);
            assert_eq!(cell.clone().clone().jam(), jammed_cell);
            assert_eq!(Noun::cue(jammed_cell).expect("cue"), cell);
        }
    }
}
