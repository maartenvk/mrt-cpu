pub mod error;
pub mod token;

use std::path::Path;

use error::CompilationError;

pub fn compile(input_stream: &[u8], file_path: &Path) -> Result<Box<Vec<u8>>, CompilationError> {
    let output_stream = Box::<Vec<u8>>::default();

    // lexical analysis
    let tokens = token::tokenize(input_stream, file_path);
    if let Err(error) = tokens {
        return Err(CompilationError::Tokenizer(error));
    }

    let tokens = *tokens.unwrap();
    for token in tokens {
        println!("Collected token: {:?}", token);
    }

    // parsing

    return Ok(output_stream);
}

pub fn compile_file(input_path: &Path, output_path: &Path) -> Result<(), CompilationError> {
    let input_bytes = std::fs::read(input_path);
    if input_bytes.is_err() {
        return Err(CompilationError::UnableToReadFromInputFile(
            input_path.as_os_str().to_os_string(),
        ));
    }

    let bytes = input_bytes.unwrap();
    let output_bytes = *compile(&bytes, input_path)?;

    let output_result = std::fs::write(output_path, output_bytes);
    if output_result.is_err() {
        return Err(CompilationError::UnableToWriteToOutputFile(
            output_path.as_os_str().to_os_string(),
        ));
    }

    return Ok(());
}
