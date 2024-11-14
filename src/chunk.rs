use crate::util::{slice_to_u32, Result};

pub struct Chunk {
    pub length: u32,
    pub chunk_type: ChunkType,
    pub data: Vec<u8>,
}

impl Chunk {
    pub fn from_slice(mut starting_index: usize, slice: &[u8]) -> Result<(Chunk, usize)> {
        let mut ending_index: usize = starting_index + 4;
        let length = slice_to_u32(&slice[starting_index..ending_index])?;
        starting_index += 4;
        ending_index = starting_index + 4;

        let type_bytes = <[u8; 4]>::try_from(&slice[starting_index..ending_index])?;
        let chunk_type: ChunkType = ChunkType::try_from(type_bytes)?;

        starting_index += 4;
        ending_index = starting_index + length as usize;

        let data = &slice[starting_index..ending_index].to_vec();

        Ok((
            Chunk {
                length,
                chunk_type,
                data: data.to_owned(),
            },
            ending_index,
        ))
    }
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

impl std::error::Error for ChunkError {}

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
