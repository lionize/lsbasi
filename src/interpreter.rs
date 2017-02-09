pub use token::{Token, TokenType};
pub use lexer::Lexer;

static OPERATORS: &'static [TokenType] =
    &[TokenType::Add, TokenType::Subtract, TokenType::Multiply, TokenType::Divide];

pub struct Interpreter<'a> {
    current_token: Option<Token>,
    lexer: Lexer<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(lexer: Lexer<'a>) -> Interpreter<'a> {
        let mut int = Interpreter {
            lexer: lexer,
            current_token: None,
        };
        int.current_token = int.lexer.get_next_token();
        int
    }

    fn error(&self) -> ! {
        panic!("Invalid syntax")
    }

    // #
    // # Parser / Interpreter code
    // #

    /// compare the current token type with the passed token
    /// type and if they match then "eat" the current token
    /// and assign the next token to self.current_token,
    /// otherwise raise an exception.
    fn eat(&mut self, t: TokenType) {
        let current_token: Token = self.current_token.clone().unwrap();
        if current_token.kind == t {
            self.current_token = self.lexer.get_next_token();
        } else {
            self.error();
        }
    }

    /// Returns an Integer token value.
    fn factor(&mut self) -> i32 {
        let token = self.current_token.clone().unwrap();
        self.eat(TokenType::Integer);
        token.value.parse::<i32>().unwrap()
    }

    /// Arithmetic expression parser/interpreter.
    ///
    /// expr   : factor ((MUL | DIV) factor)*
    /// factor : INTEGER
    pub fn expr(&mut self) -> i32 {
        let mut result = self.factor();
        let operators = &[TokenType::Multiply, TokenType::Divide];
        while operators.iter().any(|t| *t == self.current_token.clone().unwrap().kind) {
            let token = self.current_token.clone().unwrap();
            match token.kind {
                t @ TokenType::Multiply => {
                    self.eat(t);
                    result = result * self.factor();
                }

                t @ TokenType::Divide => {
                    self.eat(t);
                    result = result / self.factor();
                }

                _ => {
                    break;
                }
            };
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use interpreter::Interpreter;

    //#[test]
    //fn it_adds_digits() {
    //let string = String::from("11+1");
    //let mut interpreter = Interpreter::new(&string);
    //let result = interpreter.expr();

    //assert_eq!(result, 12);
    //}

    //#[test]
    //fn it_subtracts_digits() {
    //let string = String::from("10-2");
    //let mut interpreter = Interpreter::new(&string);
    //let result = interpreter.expr();

    //assert_eq!(result, 8);
    //}

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

    //#[test]
    //fn it_handles_whitespace() {
    //let string = String::from("2   +    2");
    //let mut interpreter = Interpreter::new(&string);
    //let result = interpreter.expr();

    //assert_eq!(result, 4);
    //}
}
