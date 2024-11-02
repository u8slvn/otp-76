use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn write_file(file_path: &str, contents: &str) -> Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(contents.as_bytes())
}
