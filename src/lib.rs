use std::error;
use std::fmt;

pub trait ToHex {
    fn to_hex(&self) -> String;

    fn write_hex<W: fmt::Write>(&self, w: &mut W) -> fmt::Result {
        w.write_str(&self.to_hex())
    }
}

impl<T: AsRef<[u8]>> ToHex for T {
    fn to_hex(&self) -> String {
        static CHARS: &'static [u8] = b"0123456789abcdef";

        let bytes = self.as_ref();
        let mut v = Vec::with_capacity(bytes.len() * 2);
        for &byte in bytes.iter() {
            v.push(CHARS[(byte >> 4) as usize]);
            v.push(CHARS[(byte & 0xf) as usize]);
        }

        unsafe {
            String::from_utf8_unchecked(v)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FromHexError {
    InvalidHexCharacter {
        c: char,
        index: usize,
    },
    InvalidHexLength,
}

impl error::Error for FromHexError {
    fn description(&self) -> &str {
        match *self {
            FromHexError::InvalidHexCharacter{ .. } => "invalid character",
            FromHexError::InvalidHexLength => "invalid length",
        }
    }
}

impl fmt::Display for FromHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FromHexError::InvalidHexCharacter { c, index } =>
                write!(f, "Invalid character '{}' at position {}", c, index),
            FromHexError::InvalidHexLength =>
                write!(f, "Invalid string length"),
        }
    }
}

pub trait FromHex: Sized {
    type Error;

    fn from_hex(s: &str) -> Result<Self, Self::Error>;
}

impl FromHex for Vec<u8> {
    type Error = FromHexError;

    fn from_hex(s: &str) -> Result<Self, Self::Error> {
        let mut b = Vec::with_capacity(s.len() / 2);
        let mut modulus = 0;
        let mut buf = 08;

        for (idx, byte) in s.bytes().enumerate() {
            buf <<= 4;

            match byte {
                b'A'...b'F' => buf |= byte - b'A' + 10,
                b'a'...b'f' => buf |= byte - b'a' + 10,
                b'0'...b'9' => buf |= byte - b'0',
                b' '|b'\r'|b'\n'|b'\t' => {
                    buf >>= 4;
                    continue
                }
                _ => {
                    let ch = s[idx..].chars().next().unwrap();
                    return Err(FromHexError::InvalidHexCharacter {
                        c: ch,
                        index: idx,
                    })
                }
            }

            modulus += 1;
            if modulus == 2 {
                modulus = 0;
                b.push(buf);
            }
        }

        match modulus {
            0 => Ok(b.into_iter().collect()),
            _ => Err(FromHexError::InvalidHexLength),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{FromHex, ToHex};

    #[test]
    fn test_to_hex() {
        assert_eq!("foobar".to_hex(), "666f6f626172");
    }

    #[test]
    pub fn test_from_hex_okay() {
        assert_eq!(Vec::from_hex("666f6f626172").unwrap(),
                   b"foobar");
        assert_eq!(Vec::from_hex("666F6F626172").unwrap(),
                   b"foobar");
    }
}
