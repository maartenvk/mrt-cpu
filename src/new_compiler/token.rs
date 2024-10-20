use std::path::Path;

use crate::new_compiler::error::TokenizationError;

pub struct Token {
    offset: usize, // offset in file
    data: Vec<char>,
}

pub fn tokenize(
    input_stream: &[u8],
    file_path: &Path,
) -> Result<Box<Vec<Token>>, TokenizationError> {
    let tokens = Box::<Vec<Token>>::default();

    let chars = (*input_stream)
        .iter()
        .map(|byte| *byte as char)
        .collect::<Vec<char>>();

    for c in chars {
        // do something
    }

    return Ok(tokens);
}
