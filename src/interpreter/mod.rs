use std::str;
use std::iter;
use std::io;
use std::io::Write;
mod token;

pub use self::token::{Token, TokenType};

static OPERATORS: &'static [TokenType] =
    &[TokenType::Add, TokenType::Subtract, TokenType::Multiply, TokenType::Divide];

pub struct Interpreter<'a> {
    current_token: Option<Token>,
    current_char: Option<char>,
    input: str::Chars<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(input: &'a String) -> Interpreter<'a> {
        let mut int = Interpreter {
            current_token: None,
            current_char: None,
            input: input.chars(),
        };
        int.current_char = int.input.next();
        int
    }

    /// Advances the iterator and sets the `current_char` variable.
    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

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
    fn get_next_token(&mut self) -> Option<Token> {
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

    // #
    // # Parser / Interpreter code
    // #

    /// compare the current token type with the passed token
    /// type and if they match then "eat" the current token
    /// and assign the next token to self.current_token,
    /// otherwise raise an exception.
    fn eat(&mut self, t: TokenType) {
        let current_token = self.current_token.clone().unwrap();
        if current_token.kind == t {
            self.current_token = self.get_next_token();
        } else {
            self.error();
        }
    }

    /// Returns an Integer token value.
    fn term(&mut self) -> i32 {
        let token = self.current_token.clone().unwrap();
        self.eat(TokenType::Integer);
        token.value.parse::<i32>().unwrap()
    }

    /// Arithmetic expression parser/interpreter.
    pub fn expr(&mut self) -> i32 {
        self.current_token = self.get_next_token();

        let mut result = self.term();
        while OPERATORS.iter().any(|t| *t == self.current_token.clone().unwrap().kind) {
            let token = self.current_token.clone().unwrap();
            match token.kind {
                t @ TokenType::Add => {
                    self.eat(t);
                    result = result + self.term();
                }

                t @ TokenType::Subtract => {
                    self.eat(t);
                    result = result - self.term();
                }

                _ => {
                    break;
                }
            };
        }
        result
    }

    fn error(&self) -> ! {
        panic!("We have reached an error")
    }
}

#[cfg(test)]
mod tests {
    use interpreter::Interpreter;

    #[test]
    fn it_adds_digits() {
        let string = String::from("11+1");
        let mut interpreter = Interpreter::new(&string);
        let result = interpreter.expr();

        assert_eq!(result, 12);
    }

    #[test]
    fn it_subtracts_digits() {
        let string = String::from("10-2");
        let mut interpreter = Interpreter::new(&string);
        let result = interpreter.expr();

        assert_eq!(result, 8);
    }

    //#[test]
    //fn it_multiplies_digits() {
    //let string = String::from("2*3");
    //let mut interpreter = Interpreter::new(&string);
    //let result = interpreter.expr();

    //assert_eq!(result, 6);
    //}

    //#[test]
    //fn it_divides_digits() {
    //let string = String::from("6/2");
    //let mut interpreter = Interpreter::new(&string);
    //let result = interpreter.expr();

    //assert_eq!(result, 3);
    //}

    #[test]
    fn it_handles_whitespace() {
        let string = String::from("2   +    2");
        let mut interpreter = Interpreter::new(&string);
        let result = interpreter.expr();

        assert_eq!(result, 4);
    }
}
