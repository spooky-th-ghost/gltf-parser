pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

pub fn slice_to_u32(slice: &[u8]) -> Result<u32> {
    let bytes = <[u8; 4]>::try_from(&slice[0..4])?;
    let value = u32::from_le_bytes(bytes);
    Ok(value)
}
