use std::{iter::Peekable, str::Chars};

use crate::lamonz::token::*;

#[derive(Debug)]
pub(crate) struct Lexer<'a> {
    input_iter: Peekable<Chars<'a>>, //position: usize,
                                     //read_position: usize,
                                     //ch: u8,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            input_iter: input.chars().peekable(),
        }
    }

    pub(crate) fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        match self.input_iter.peek() {
            Some('=') => {
                self.input_iter.next();
                Some(Token {
                    token_type: TokenType::Assign,
                    token_literal: "=".to_string(),
                })
            }
            Some('+') => {
                self.input_iter.next();
                Some(Token {
                    token_type: TokenType::Plus,
                    token_literal: "+".to_string(),
                })
            }
            Some('-') => {
                self.input_iter.next();
                Some(Token {
                    token_type: TokenType::Minus,
                    token_literal: "-".to_string(),
                })
            }
            Some('(') => {
                self.input_iter.next();
                Some(Token {
                    token_type: TokenType::Lparen,
                    token_literal: "(".to_string(),
                })
            }
            Some(')') => {
                self.input_iter.next();
                Some(Token {
                    token_type: TokenType::Rparen,
                    token_literal: ")".to_string(),
                })
            }
            Some('{') => {
                self.input_iter.next();
                Some(Token {
                    token_type: TokenType::Lbrace,
                    token_literal: "{".to_string(),
                })
            }
            Some('}') => {
                self.input_iter.next();
                Some(Token {
                    token_type: TokenType::Rbrace,
                    token_literal: "}".to_string(),
                })
            }
            Some(';') => {
                self.input_iter.next();
                Some(Token {
                    token_type: TokenType::Semicolon,
                    token_literal: ";".to_string(),
                })
            }
            Some(',') => {
                self.input_iter.next();
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
                dbg!(self.input_iter.peek());
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
                self.input_iter.next();
            }
        }
    }

    /// Parses an identifier string from the input.
    fn parse_identifier(&mut self) -> String {
        self.input_iter
            .by_ref()
            .take_while(|ch| ch.is_ascii_alphabetic())
            .collect()
    }

    /// Parses a number
    fn parse_number(&mut self) -> String {
        let number = self.input_iter
            .by_ref()
            .take_while(|ch| ch.is_ascii_digit())
            .collect();
        number
    }

    /// Looks up a keyword from an identifier string.
    fn lookup_identifier(ident: &str) -> Option<TokenType> {
        if ident.eq("let") {
            Some(TokenType::Let)
        } else if ident.eq("fn") {
            Some(TokenType::Function)
        } else {
            return None;
        }
    }
}
