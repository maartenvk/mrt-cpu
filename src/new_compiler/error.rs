use std::{ffi::OsString, path::Path, rc::Rc};

use crate::new_compiler::token::TokenType;

// Used to describe the accurate position of the compiler in case of an error
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    path: Rc<OsString>, // Rc because it will be cloned often for many tokens in the same file. Since they are in the same file, it is not necessary to reallocate a lot of memory to store the path to the file.
    line_number: usize,
    line_offset: usize,
    actual_offset: usize,
}

impl Position {
    pub fn new(path: &Path) -> Self {
        return Position {
            path: Rc::new(path.as_os_str().to_os_string()),
            line_number: 0,
            line_offset: 0,
            actual_offset: 0,
        };
    }

    pub fn get_line_info(&self) -> (usize, usize) {
        return (self.line_number, self.line_offset);
    }

    pub fn next_line(&mut self) {
        self.line_number += 1;
        self.line_offset = 0;
        self.actual_offset += 1;
    }

    pub fn next_char(&mut self) {
        self.line_offset += 1;
        self.actual_offset += 1;
    }
}

#[derive(Debug)]
pub enum CompilationError {
    UnableToReadFromInputFile(OsString),
    UnableToWriteToOutputFile(OsString),
    Tokenizer(TokenizationError),
}

#[derive(Debug, PartialEq)]
pub enum TokenizationError {
    TokenTypeConversion(Position, TokenTypeConversionError),
}

#[derive(Debug, PartialEq)]
pub enum TokenTypeConversionError {
    IncompatibleTypes(TokenType, TokenType), // received type, current type
    UnknownCharacter(char),
}
