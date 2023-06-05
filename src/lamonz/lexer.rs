use std::{iter::Peekable, str::Chars};
use crate::lamonz::token::*;

#[derive(Debug)]
pub(crate) struct Lexer<'a> {
    input_iter: Peekable<Chars<'a>>, 
}

impl<'a> Lexer<'a> {
    /// Initializes a new lexer with the given input.
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            input_iter: input.chars().peekable(),
        }
    }

    /// Advances the iterator on the input.
    fn advance(&mut self) {
        self.input_iter.next();
    }

    /// Produces the next token.
    pub(crate) fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        match self.input_iter.peek() {
            Some('=') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Assign,
                    token_literal: "=".to_string(),
                })
            }
            Some('+') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Plus,
                    token_literal: "+".to_string(),
                })
            }
            Some('-') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Minus,
                    token_literal: "-".to_string(),
                })
            }
            Some('(') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Lparen,
                    token_literal: "(".to_string(),
                })
            }
            Some(')') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Rparen,
                    token_literal: ")".to_string(),
                })
            }
            Some('{') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Lbrace,
                    token_literal: "{".to_string(),
                })
            }
            Some('}') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Rbrace,
                    token_literal: "}".to_string(),
                })
            }
            Some(';') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Semicolon,
                    token_literal: ";".to_string(),
                })
            }
            Some(',') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Comma,
                    token_literal: ",".to_string(),
                })
            }
            Some(ch) => {
                if ch.is_ascii_alphabetic() {
                    let ident: String = self.parse_identifier();
                    let tok_type = Self::lookup_identifier(&ident).unwrap_or(TokenType::Ident);
                    Some(Token {
                        token_type: tok_type,
                        token_literal: ident,
                    })
                } else if ch.is_ascii_digit() {
                    let number = self.parse_number();
                    Some(Token {
                        token_type: TokenType::Int,
                        token_literal: number,
                    })
                } else {
                    Some(Token {
                        token_type: TokenType::Invalid,
                        token_literal: "".to_string(),
                    })
                }
            }
            _ => {
                Some(Token {
                token_type: TokenType::Eof,
                token_literal: "".to_string(),
            })},
        }
    }

    /// Consumes whitespace in the input stream.
    fn skip_whitespace(&mut self) {
        if let Some(next_ch) = self.input_iter.peek() {
            if next_ch.is_ascii_whitespace() {
                self.advance();
            }
        }
    }

    /// Parses an identifier string from the input.
    fn parse_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.input_iter.peek() {
            if ch.is_ascii_alphabetic() {
                ident.push(*ch);
                self.input_iter.next();
            } else {
                break;
            }
        }
        ident
    }

    /// Parses a number as a string from the input.
    fn parse_number(&mut self) -> String {
        let mut number = String::new();
        while let Some(ch) = self.input_iter.peek() {
            if ch.is_ascii_digit() {
                number.push(*ch);
                self.input_iter.next();
            } else {
                break;
            }
        }
        number
    }

    /// Checks if the identifier string is a keyword.
    /// Returns the corresponding `TokenType` if it finds the keyword.
    fn lookup_identifier(ident: &str) -> Option<TokenType> {
        match ident {
            "let" => Some(TokenType::Let),
            "fn" => Some(TokenType::Function),
            _ => None,
        }
    }
}
