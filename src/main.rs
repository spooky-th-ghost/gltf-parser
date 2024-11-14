mod header;
mod util;

use header::Header;
use util::Result;

fn main() -> Result<()> {
    let data: Vec<u8> = std::fs::read("test.glb")?;
    let header: Header = Header::from_slice(&data)?;
    println!("{}", header);
    Ok(())
}
