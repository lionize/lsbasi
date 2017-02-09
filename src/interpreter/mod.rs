use std::str;
use std::iter;
mod token;

pub use self::token::{Token, TokenType};

pub struct Interpreter<'a> {
    current: Option<Token>,
    input: iter::Peekable<str::Chars<'a>>,
}

impl<'a> Interpreter<'a> {
    pub fn new(input: &'a String) -> Interpreter<'a> {
        Interpreter {
            current: None,
            input: input.chars().peekable(),
        }
    }

    fn get_next_token(&mut self) -> Option<Token> {
        if self.eof() {
            let token = Token {
                kind: TokenType::EOF,
                value: "".to_string(),
            };
            return Some(token);
        }

        let current_char = self.input.next();

        let token = match current_char {
            Some('+') => {
                Token {
                    kind: TokenType::Plus,
                    value: '+'.to_string(),
                }
            }

            Some(c) if c.is_digit(10) => {
                Token {
                    kind: TokenType::Integer,
                    value: c.to_string(),
                }
            }

            _ => Token::eof(),
        };
        return Some(token);
    }

    fn eat(&mut self, t: TokenType) {
        let current = self.current.clone().unwrap();
        if current.kind == t {
            self.current = self.get_next_token();
        } else {
            self.error();
        }
    }

    pub fn expr(&mut self) -> i32 {
        self.current = self.get_next_token();

        let left = self.current.clone();
        self.eat(TokenType::Integer);

        let op = self.current.clone();
        self.eat(TokenType::Plus);

        let right = self.current.clone();
        self.eat(TokenType::Integer);

        left.unwrap().value.parse::<i32>().unwrap() + right.unwrap().value.parse::<i32>().unwrap()
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn eof(&mut self) -> bool {
        self.peek().is_none()
    }

    fn error(&self) {
        panic!("We have reached an error")
    }

    fn skip_whitespace(&mut self) {
        while self.current.clone().unwrap().value == " ".to_string() {
            self.get_next_token();
        }
    }
}

#[cfg(test)]
mod tests {
    use interpreter::Interpreter;

    #[test]
    fn it_adds_digits() {
        let string = String::from("1+1");
        let mut interpreter = Interpreter::new(&string);
        let result = interpreter.expr();

        assert_eq!(result, 2);
    }
}
