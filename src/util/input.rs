use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use super::error_handling::exit_with_error_message;

pub fn lines(input_file_path: String) -> impl Iterator<Item = String> {
    let file = File::open(input_file_path).unwrap_or_else(exit_with_error_message);
    let lines = BufReader::new(file).lines()
        .map(|l| l.unwrap_or_else(exit_with_error_message));
    lines
}