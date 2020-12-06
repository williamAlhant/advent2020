use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use super::error_handling::{Error, Result};

pub fn lines(input_file_path: String) -> Result<impl Iterator<Item = Result<String>>> {
    let file = File::open(input_file_path).map_err(|source| Error::OpenFile { source })?;
    let lines = BufReader::new(file).lines()
        .map(|l| l.map_err(|source| Error::ReadLines { source }));
    Ok(lines)
}

pub fn lines_from_file_passed_as_argument() -> Result<impl Iterator<Item = Result<String>>> {
    let mut args = env::args();
    if args.len() < 2 {
        return Err(Error::MissingInputPathArgument);
    }
    let input_path = args.nth(1).unwrap();
    lines(input_path)
}
