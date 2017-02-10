use std::str;

pub use token::{Token, TokenType};

pub struct Lexer<'a> {
    current_char: Option<char>,
    input: str::Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Lexer<'a> {
        let mut lex = Lexer {
            current_char: None,
            input: input.chars(),
        };
        lex.current_char = lex.input.next();
        lex
    }

    fn error(&self) -> ! {
        panic!("We have reached an error")
    }

    /// Advances the iterator and sets the `current_char` variable.
    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    /// Advances the iterator to skip whitespace.
    fn skip_whitespace(&mut self) {
        while self.current_char.is_some() && self.current_char.unwrap() == ' ' {
            self.advance();
        }
    }

    /// Returns a (multi-digit) integer consumed from the input.
    fn integer(&mut self) -> String {
        let mut result = String::from("");
        while self.current_char.is_some() && self.current_char.unwrap().is_digit(10) {
            result.push(self.current_char.unwrap());
            self.advance();
        }
        result
    }

    /// Lexical analyzer (also known as scanner or tokenizer)
    ///
    /// This method is responsible for breaking a sentence
    /// apart into tokens, one token at a time.
    pub fn get_next_token(&mut self) -> Option<Token> {
        while self.current_char.is_some() {
            match self.current_char {
                Some(' ') => {
                    self.skip_whitespace();
                    continue;
                }

                Some(c) if c.is_digit(10) => {
                    let tok = Token {
                        kind: TokenType::Integer,
                        value: self.integer(),
                    };
                    return Some(tok);
                }

                Some('+') => {
                    self.advance();
                    let tok = Token {
                        kind: TokenType::Add,
                        value: "+".to_string(),
                    };
                    return Some(tok);
                }

                Some('-') => {
                    self.advance();
                    let tok = Token {
                        kind: TokenType::Subtract,
                        value: "-".to_string(),
                    };
                    return Some(tok);
                }

                Some('*') => {
                    self.advance();
                    let tok = Token {
                        kind: TokenType::Multiply,
                        value: "*".to_string(),
                    };
                    return Some(tok);
                }

                Some('/') => {
                    self.advance();
                    let tok = Token {
                        kind: TokenType::Divide,
                        value: "/".to_string(),
                    };
                    return Some(tok);
                }

                _ => {
                    self.error();
                }
            };
        }

        Some(Token::eof())
    }
}
