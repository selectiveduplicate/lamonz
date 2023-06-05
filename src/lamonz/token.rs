#[derive(Debug, Eq, PartialEq)]
/// The supported tokens in the Monkey programming language.
pub(crate) enum TokenType {
    Assign,
    Plus,
    Minus,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Comma,
    Semicolon,
    Ident,
    Let,
    Int,
    Function,
    Invalid,
    Eof,
}

#[derive(Debug)]
/// Represents a distinct token in the language.
/// The token is identified by its `TokenType` and its string form.
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) token_literal: String,
}

#[cfg(test)]
mod lexer_tests {
    use super::*;
    use crate::lamonz::lexer::*;

    #[test]
    fn parse_symbols() {
        let input = "=+(){},;";

        let test_vec: Vec<Token> = vec![
            Token {
                token_type: TokenType::Assign,
                token_literal: String::from("="),
            },
            Token {
                token_type: TokenType::Plus,
                token_literal: String::from("+"),
            },
            Token {
                token_type: TokenType::Lparen,
                token_literal: String::from("("),
            },
            Token {
                token_type: TokenType::Rparen,
                token_literal: String::from(")"),
            },
            Token {
                token_type: TokenType::Lbrace,
                token_literal: String::from("{"),
            },
            Token {
                token_type: TokenType::Rbrace,
                token_literal: String::from("}"),
            },
            Token {
                token_type: TokenType::Comma,
                token_literal: String::from(","),
            },
            Token {
                token_type: TokenType::Semicolon,
                token_literal: String::from(";"),
            },
            Token {
                token_type: TokenType::Eof,
                token_literal: String::from(""),
            }
        ];

        let mut lexer = Lexer::new(input);

        for expected_token in test_vec {
            let tok = lexer.next_token().unwrap();
            assert!(tok.token_literal == expected_token.token_literal);
            assert!(tok.token_type == expected_token.token_type);
        }
    }

    #[test]
    fn parse_ident_and_ints() {
        let input = "let five = 5;";

        let test_vec: Vec<Token> = vec![
            Token {
                token_type: TokenType::Let,
                token_literal: String::from("let"),
            },
            Token {
                token_type: TokenType::Ident,
                token_literal: String::from("five"),
            },
            Token {
                token_type: TokenType::Assign,
                token_literal: String::from("="),
            },
            Token {
                token_type: TokenType::Int,
                token_literal: String::from("5"),
            },
            Token {
                token_type: TokenType::Semicolon,
                token_literal: String::from(";"),
            },
            Token {
                token_type: TokenType::Eof,
                token_literal: String::from(""),
            },
        ];

        let mut lexer = Lexer::new(input);

        for expected_token in test_vec {
            let tok = lexer.next_token().unwrap();
            assert_eq!(tok.token_type, expected_token.token_type);
        }
    }
}
