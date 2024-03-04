#![cfg_attr(bench, feature(test))]

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Code(u8);

const INVALID: Code = Code(128);
const SKIP: Code = Code(129);
const PAD: Code = Code(130);

pub trait Alphabet {
    fn decode_table(&self) -> &'static [Code; 256];
    fn encode_table(&self) -> &'static [u8; 64];
}

struct Standard;

impl Alphabet for Standard {
    fn decode_table(&self) -> &'static [Code; 256] {
        &[
            // '\x00'..'\x0f'
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            SKIP,
            SKIP,
            INVALID,
            INVALID,
            SKIP,
            INVALID,
            INVALID,
            // '\x10'..'\x1f'
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            // ' '..'/'
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            Code(62),
            INVALID,
            INVALID,
            INVALID,
            Code(63),
            // '0'..'?'
            Code(52),
            Code(53),
            Code(54),
            Code(55),
            Code(56),
            Code(57),
            Code(58),
            Code(59),
            Code(60),
            Code(61),
            INVALID,
            INVALID,
            INVALID,
            PAD,
            INVALID,
            INVALID,
            // '@'..'O'
            INVALID,
            Code(0),
            Code(1),
            Code(2),
            Code(3),
            Code(4),
            Code(5),
            Code(6),
            Code(7),
            Code(8),
            Code(9),
            Code(10),
            Code(11),
            Code(12),
            Code(13),
            Code(14),
            // 'P'..'_'
            Code(15),
            Code(16),
            Code(17),
            Code(18),
            Code(19),
            Code(20),
            Code(21),
            Code(22),
            Code(23),
            Code(24),
            Code(25),
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            // '`'..'o'
            INVALID,
            Code(26),
            Code(27),
            Code(28),
            Code(29),
            Code(30),
            Code(31),
            Code(32),
            Code(33),
            Code(34),
            Code(35),
            Code(36),
            Code(37),
            Code(38),
            Code(39),
            Code(40),
            // 'p'..'\x7f'
            Code(41),
            Code(42),
            Code(43),
            Code(44),
            Code(45),
            Code(46),
            Code(47),
            Code(48),
            Code(49),
            Code(50),
            Code(51),
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            // '\x80'..'\x8f'
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            // '\x90'..'\x9f'
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            // '\xa0'..'\xaf'
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            // '\xb0'..'\xbf'
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            // '\xc0'..'\xcf'
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            // '\xd0'..'\xdf'
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            // '\xe0'..'\xef'
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            // '\xf0'..'\xff'
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
            INVALID,
        ]
    }

    fn encode_table(&self) -> &'static [u8; 64] {
        &[0u8; 64]
    }
}

/*
struct Pem;

impl Alphabet for Pem {
}
*/

pub const fn decode_len_estimate(input_len: usize) -> usize {
    ((input_len + 3) / 4) * 3
}

pub fn decode(alphabet: impl Alphabet, input: &[u8], output: &mut [u8]) -> Result<usize, Error> {
    let table = alphabet.decode_table();
    let mut quad = Quad::new();
    let mut offs = 0;
    let mut state = State::Data;

    for (i, inp) in input.iter().enumerate() {
        match (state, table[*inp as usize]) {
            (_, SKIP) => continue,
            (State::Data, PAD) => {
                state = State::Pad1;
            }
            (State::Pad1, PAD) => {
                state = State::Pad2;
            }
            (State::Data, Code(v)) => quad.add(v),
            (_, INVALID) | (State::Pad1, _) | (State::Pad2, _) => {
                return Err(Error::InvalidInput { at_offset: i })
            }
        };

        if quad.complete() {
            offs += quad.emit(&mut output[offs..])?;
        }
    }

    let pad = match state {
        State::Data => 0,
        State::Pad1 => 1,
        State::Pad2 => 2,
    };
    offs += quad.emit_final(pad, &mut output[offs..])?;
    Ok(offs)
}

#[derive(Debug, Copy, Clone)]
enum State {
    Data,
    Pad1,
    Pad2,
}

pub fn decode_into_vec(alphabet: impl Alphabet, input: &[u8]) -> Result<Vec<u8>, Error> {
    let mut out = vec![0u8; decode_len_estimate(input.len())];
    let len = decode(alphabet, input, &mut out[..])?;
    out.truncate(len);
    Ok(out)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInput { at_offset: usize },
}

#[derive(Debug)]
struct Quad {
    codes: [u8; 4],
    used: usize,
}

impl Quad {
    fn new() -> Self {
        Self {
            codes: [0u8; 4],
            used: 0,
        }
    }

    fn add(&mut self, v: u8) {
        self.codes[self.used] = v;
        self.used += 1;
    }

    #[inline]
    fn complete(&self) -> bool {
        self.used == 4
    }

    #[inline]
    fn to_triple(&mut self) -> Triple {
        debug_assert!(self.used == 4);
        let a = self.codes[0];
        let b = self.codes[1];
        let c = self.codes[2];
        let d = self.codes[3];
        self.used = 0;
        Triple(
            [
                a << 2 | b >> 4,
                ((b & 0xf) << 4) | (c >> 2),
                (c & 0x3) << 6 | d,
            ],
            3,
        )
    }

    #[inline]
    fn emit_pad(&mut self, out: &mut [u8], pad: usize) -> Result<usize, Error> {
        let len = 3 - pad;
        let t = self.to_triple();
        out[..len].copy_from_slice(&t.as_ref()[..len]);
        Ok(len)
    }

    #[inline]
    fn emit(&mut self, out: &mut [u8]) -> Result<usize, Error> {
        self.emit_pad(out, 0)
    }

    #[inline]
    fn emit_final(&mut self, pad: usize, out: &mut [u8]) -> Result<usize, Error> {
        for _ in 0..pad {
            self.add(0);
        }

        match (self.used, pad) {
            // valid explicit padding
            (4, 2) | (4, 1) | (4, 0) => self.emit_pad(out, pad),

            // one padding implied by length
            (3, 0) => {
                self.add(0);
                self.emit_pad(out, 1)
            }

            // two padding implied by length
            (2, 0) => {
                self.add(0);
                self.add(0);
                self.emit_pad(out, 2)
            }

            // no data, nothing to emit
            (0, _) => Ok(0),

            (_, _) => Err(Error::InvalidInput { at_offset: 0 }),
        }
    }
}

struct Triple([u8; 3], usize);

impl AsRef<[u8]> for Triple {
    fn as_ref(&self) -> &[u8] {
        &self.0[..self.1]
    }
}

#[cfg(bench)]
mod benchmarks;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_decode() {
        assert_eq!(
            &b"hello world\n"[..],
            &decode_into_vec(Standard, b"aGVsbG8gd29ybGQK").unwrap()
        );
        assert_eq!(
            &b"hell"[..],
            &decode_into_vec(Standard, b"aGVsbA==").unwrap()
        );
        assert_eq!(
            &b"hello"[..],
            &decode_into_vec(Standard, b"aGVsbG8=").unwrap()
        );
    }

    #[test]
    fn padding_in_middle() {
        let mut out = [0u8; 32];
        assert_eq!(
            decode(Standard, b"a=VsbA==", &mut out),
            Err(Error::InvalidInput { at_offset: 2 })
        );
    }

    #[test]
    fn unpadded() {
        assert_eq!(decode_into_vec(Standard, b"").unwrap(), vec![]);
        assert_eq!(
            decode_into_vec(Standard, b"a"),
            Err(Error::InvalidInput { at_offset: 0 })
        );
        assert_eq!(decode_into_vec(Standard, b"aa").unwrap(), vec![0x69]);
        assert_eq!(decode_into_vec(Standard, b"aaa").unwrap(), vec![0x69, 0xa6]);
        assert_eq!(
            decode_into_vec(Standard, b"aaaa").unwrap(),
            vec![0x69, 0xa6, 0x9a]
        );
    }
}
