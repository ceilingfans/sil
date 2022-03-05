use std::fs::File;
use std::io::{BufReader, Read, Result as IoResult};
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P) -> IoResult<String> {
    let f = File::open(path)?;
    let mut buf_reader = BufReader::new(f);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}
