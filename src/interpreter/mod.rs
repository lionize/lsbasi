use std::str;
use std::iter;
use std::io;
use std::io::Write;
mod token;

pub use self::token::{Token, TokenType};

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

    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_some() && self.current_char.unwrap() == ' ' {
            self.advance();
        }
    }

    fn integer(&mut self) -> String {
        let mut result = String::from("");
        while self.current_char.is_some() && self.current_char.unwrap().is_digit(10) {
            result.push(self.current_char.unwrap());
            self.advance();
        }
        result
    }

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
        };

        Some(Token::eof())
    }

    fn eat(&mut self, t: TokenType) {
        let current_token = self.current_token.clone().unwrap();
        if current_token.kind == t {
            self.current_token = self.get_next_token();
        } else {
            self.error();
        }
    }

    pub fn expr(&mut self) -> i32 {
        self.current_token = self.get_next_token();

        let left = self.current_token.clone();
        self.eat(TokenType::Integer);

        let op = self.current_token.clone();
        let kind = op.clone().unwrap().kind;
        if kind == TokenType::Add {
            self.eat(TokenType::Add);
        } else if kind == TokenType::Subtract {
            self.eat(TokenType::Subtract);
        } else if kind == TokenType::Multiply {
            self.eat(TokenType::Multiply);
        } else {
            self.eat(TokenType::Divide);
        }

        let right = self.current_token.clone();
        self.eat(TokenType::Integer);

        if kind == TokenType::Add {
            left.unwrap().value.parse::<i32>().unwrap() + right.unwrap().value.parse::<i32>().unwrap()
        } else if kind == TokenType::Subtract {
            left.unwrap().value.parse::<i32>().unwrap() - right.unwrap().value.parse::<i32>().unwrap()
        } else if kind == TokenType::Multiply {
            left.unwrap().value.parse::<i32>().unwrap() * right.unwrap().value.parse::<i32>().unwrap()
        } else {
            left.unwrap().value.parse::<i32>().unwrap() / right.unwrap().value.parse::<i32>().unwrap()
        }
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

    #[test]
    fn it_multiplies_digits() {
        let string = String::from("2*3");
        let mut interpreter = Interpreter::new(&string);
        let result = interpreter.expr();

        assert_eq!(result, 6);
    }

    #[test]
    fn it_handles_whitespace() {
        let string = String::from("2   +    2");
        let mut interpreter = Interpreter::new(&string);
        let result = interpreter.expr();

        assert_eq!(result, 4);
    }
}
