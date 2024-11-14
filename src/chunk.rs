use crate::util::{slice_to_u32, Result};

pub struct Chunk {
    pub length: u32,
    pub chunk_type: ChunkType,
    pub data: Vec<u8>,
}

pub enum ChunkType {
    Json,
    Binary,
    Ignore,
}

#[derive(Debug)]
pub struct ChunkError;

impl std::fmt::Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chunk Type cannot be parsed")
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkError;
    fn try_from(value: [u8; 4]) -> std::result::Result<Self, Self::Error> {
        let value = u32::from_le_bytes(value);
        match value {
            0x4E4F534A => Ok(ChunkType::Json),
            0x004E4942 => Ok(ChunkType::Binary),
            _ => Ok(ChunkType::Ignore),
        }
    }
}
