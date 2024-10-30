#[cfg(test)]
mod tests {
    use std::path::Path;

    use mrt_cpu::new_compiler::{
        error::{Position, TokenTypeConversionError, TokenizationError},
        token::{tokenize, Token, TokenType},
    };

    fn create_mock_position() -> Position {
        let path = Path::new("");
        let position = Position::new(path);

        return position;
    }

    #[test]
    fn compiler_can_tokenize_comment() {
        let mut token = Token::new(create_mock_position());

        for c in "# a comment 03121 # # ##".chars() {
            assert!(token.take(c).is_ok());
        }

        assert_eq!(token.ttype(), TokenType::Comment);
    }

    #[test]
    fn compiler_can_tokenize_symbol() {
        let mut token = Token::new(create_mock_position());

        for c in "abcde0139".chars() {
            assert!(token.take(c).is_ok());
        }

        assert_eq!(token.ttype(), TokenType::Symbol);
    }

    #[test]
    fn compiler_can_tokenize_number() {
        let mut token = Token::new(create_mock_position());

        for c in "38129".chars() {
            assert!(token.take(c).is_ok());
        }

        assert_eq!(token.ttype(), TokenType::Number);
    }

    #[test]
    fn compiler_can_tokenize_number_and_symbol() {
        let mut token = Token::new(create_mock_position());

        for c in "123".chars() {
            assert!(token.take(c).is_ok());
        }

        let result = token.take('a');
        assert!(result.is_err());

        let error = result.err().unwrap();
        assert_eq!(
            error,
            TokenTypeConversionError::IncompatibleTypes(TokenType::Symbol, TokenType::Number)
        );

        assert_eq!(token.ttype(), TokenType::Number);
    }

    #[test]
    fn compiler_errors_on_tokenizing_unknown_character() {
        let mut token = Token::new(create_mock_position());

        let result = token.take('_'); // '_' is not a recognized character at the moment
        assert!(result.is_err());

        let error = result.err().unwrap();
        assert_eq!(error, TokenTypeConversionError::UnknownCharacter('_'));
        assert_eq!(token.ttype(), TokenType::Unknown); // Not yet set
    }

    #[test]
    fn compiler_can_tokenize_hexadecimal_number() {
        let mut token = Token::new(create_mock_position());

        for c in "0x38129".chars() {
            assert!(token.take(c).is_ok());
        }

        assert_eq!(token.ttype(), TokenType::Number);
    }

    #[test]
    fn compiler_position_is_correct() {
        let position = create_mock_position();
        let (line, column) = position.get_line_info();

        assert_eq!(line, 0);
        assert_eq!(column, 0);
    }

    #[test]
    fn compiler_position_new_line_resets_line_offset() {
        let mut position = create_mock_position();

        for _ in 0..5 {
            position.next_char();
        }

        let (_, column) = position.get_line_info();
        assert_eq!(column, 5);

        position.next_line();

        let (line, column) = position.get_line_info();

        assert_eq!(line, 1);
        assert_eq!(column, 0);
    }

    #[test]
    fn compiler_tokenizer_tokenizes() {
        let path = Path::new("");
        let input_stream = [];

        let result = tokenize(&input_stream, path);
        assert!(result.is_ok());
    }

    #[test]
    fn compiler_tokenizer_tokenizes_comment() {
        let path = Path::new("");
        let input_stream = "# comment".as_bytes();

        let result = tokenize(&input_stream, path);
        assert!(result.is_ok());

        let tokens = result.unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].ttype(), TokenType::Comment);
    }

    #[test]
    fn compiler_tokenizer_tokenizes_comment_until_newline() {
        let path = Path::new("");
        let input_stream = "# comment\n".as_bytes();

        let result = tokenize(&input_stream, path);
        assert!(result.is_ok());

        let tokens = result.unwrap();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].ttype(), TokenType::Comment);
        assert_eq!(tokens[1].ttype(), TokenType::Whitespace); // the newline '\n'
    }

    #[test]
    fn compiler_tokenizer_tokenizes_number_and_symbol() {
        let path = Path::new("");
        let input_stream = "123abc".as_bytes();

        // should tokenize as Number { 123 }, Symbol { abc }
        let result = tokenize(&input_stream, path);
        assert!(result.is_ok());

        let tokens = result.unwrap();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].ttype(), TokenType::Number);
        assert_eq!(tokens[1].ttype(), TokenType::Symbol);
    }

    #[test]
    fn compiler_fails_tokenizing_and_shows_correct_position() {
        let path = Path::new("");
        let input_stream = "123_".as_bytes();

        // should tokenize as Number { 123 }, with an error at '_'
        let result = tokenize(&input_stream, path);
        assert!(result.is_err());

        let error = result.err().unwrap();

        if let TokenizationError::TokenTypeConversion(position, conversion_error) = error {
            let (line, column) = position.get_line_info();
            assert_eq!(line, 0);
            assert_eq!(column, 4);

            if let TokenTypeConversionError::UnknownCharacter(c) = conversion_error {
                assert_eq!(c, '_');
            } else {
                panic!();
            }
        } else {
            panic!()
        };
    }
}
