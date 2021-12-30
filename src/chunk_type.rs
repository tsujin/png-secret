use std::str::FromStr;
use std::fmt;
use crate::{Error, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    ancillary_bit: u8,
    private_bit: u8,
    reserved_bit: u8,
    safe_to_copy_bit: u8,
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.byte_str)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {

        let converted_string = String::from(std::str::from_utf8(&value).unwrap());
        let chunk = ChunkType {
            ancillary_bit: value[0],
            private_bit: value[1],
            reserved_bit: value[2],
            safe_to_copy_bit: value[3],
        };

        Ok(chunk)
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self> {
        if string.chars().all(char::is_alphabetic) {
            let chunk = ChunkType {
                byte_str: String::from(string),
            };
            
            Ok(chunk)
        } else {
            Err("Chunk must be alphabetic")?
        }
    }
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.byte_str.as_bytes().try_into().unwrap()
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    fn is_safe_to_copy(&self) -> bool {
        let ch = self.byte_str.chars().nth(3).unwrap();
        ch.is_lowercase()
    }

    fn is_public(&self) -> bool {
        let ch = self.byte_str.chars().nth(1).unwrap();
        ch.is_uppercase()
    }

    fn is_critical(&self) -> bool {
        let ch = self.byte_str.chars().nth(0).unwrap();
        ch.is_uppercase()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        let ch = self.byte_str.chars().nth(2).unwrap();
        ch.is_uppercase() && ch.is_alphabetic()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}