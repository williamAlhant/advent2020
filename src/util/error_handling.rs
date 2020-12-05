#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("No input file was given as argument")]
    MissingInputPathArgument,

    #[error("Failed to open file")]
    OpenFile {
        source: std::io::Error
    },

    #[error("Failed to read lines")]
    ReadLines {
        source: std::io::Error
    }
}

pub type Result<T> = std::result::Result<T, Error>;