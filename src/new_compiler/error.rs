use std::ffi::OsString;

#[derive(Debug)]
pub enum CompilationError {
    UnableToReadFromInputFile(OsString),
    UnableToWriteToOutputFile(OsString),
    Tokenizer(TokenizationError),
}

#[derive(Debug)]
pub enum TokenizationError {}
