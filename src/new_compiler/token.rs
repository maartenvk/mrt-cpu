use std::path::Path;

use crate::new_compiler::error::TokenizationError;

use super::error::{Position, TokenTypeConversionError};

#[derive(Debug)]
pub struct Token {
    position: Position,
    data: Vec<char>,
    ttype: TokenType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Unknown,
    Symbol,
    Number,
    Whitespace,
    Comment,
}

impl TokenType {
    pub fn convert_from(c: char, token: &Token) -> Result<TokenType, TokenTypeConversionError> {
        // Comment ends after new line
        if token.ttype == TokenType::Comment {
            return Ok(if c != '\n' {
                TokenType::Comment
            } else {
                TokenType::Whitespace
            });
        }

        // Check for hexadecimal notation
        if token.ttype == TokenType::Number && token.data == ['0'] && c == 'x' || c == 'X' {
            return Ok(TokenType::Number);
        }

        let ttype = match c {
            'a'..='z' | 'A'..='Z' => TokenType::Symbol,
            '0'..='9' => {
                if token.ttype == TokenType::Symbol {
                    TokenType::Symbol
                } else {
                    TokenType::Number
                }
            }
            ' ' | '\t' | '\n' | '\r' => TokenType::Whitespace,
            '#' => TokenType::Comment,
            _ => TokenType::Unknown,
        };

        if ttype == TokenType::Unknown {
            return Err(TokenTypeConversionError::UnknownCharacter(c));
        }

        return Ok(ttype);
    }
}

impl Token {
    pub fn new(position: Position) -> Self {
        return Token {
            position,
            data: vec![],
            ttype: TokenType::Unknown,
        };
    }

    pub fn take(&mut self, c: char) -> Result<(), TokenTypeConversionError> {
        let ttype = TokenType::convert_from(c, self);
        if let Err(error) = ttype {
            return Err(error);
        }

        let ttype = ttype.unwrap();

        // The token currently has no type set yet
        if self.ttype == TokenType::Unknown {
            self.ttype = ttype;
        }

        if self.ttype != ttype {
            return Err(TokenTypeConversionError::IncompatibleTypes(
                ttype, self.ttype,
            ));
        }

        self.data.push(c);
        return Ok(());
    }

    pub fn ttype(&self) -> TokenType {
        return self.ttype;
    }
}

pub fn tokenize(
    input_stream: &[u8],
    file_path: &Path,
) -> Result<Box<Vec<Token>>, TokenizationError> {
    let mut tokens = vec![];

    let chars = (*input_stream)
        .iter()
        .map(|byte| *byte as char)
        .collect::<Vec<char>>();

    let mut pos = Position::new(file_path);
    let mut current_token = Token::new(pos.clone());
    for c in chars {
        if c == '\n' {
            pos.next_line();
        } else {
            pos.next_char();
        }

        if let Err(error) = current_token.take(c) {
            if matches!(error, TokenTypeConversionError::IncompatibleTypes(..)) {
                tokens.push(current_token);
                current_token = Token::new(pos.clone());
                _ = current_token.take(c);
            } else {
                return Err(TokenizationError::TokenTypeConversion(pos, error));
            }
        }
    }

    tokens.push(current_token);
    return Ok(Box::new(tokens));
}
