use crate::util::{slice_to_u32, Result};

pub struct Header {
    pub magic: u32,
    pub version: u32,
    pub length: u32,
}

impl Header {
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        let magic: u32 = slice_to_u32(&slice[0..4])?;
        let version: u32 = slice_to_u32(&slice[4..8])?;
        let length: u32 = slice_to_u32(&slice[8..12])?;
        Ok(Self {
            magic,
            version,
            length,
        })
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Magic: {:#X}\nVersion: {:?}\nLength: {:?} bytes",
            self.magic, self.version, self.length
        )
    }
}
