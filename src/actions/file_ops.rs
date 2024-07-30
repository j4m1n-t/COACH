use std::fs;
use std::path::Path;

pub fn copy_file<P: AsRef<Path>>(src: P, dest: P) -> std::io::Result<()> {
    fs::copy(src, dest)?;
    Ok(())
}