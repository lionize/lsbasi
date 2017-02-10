pub use token::{Token, TokenType};
pub use lexer::Lexer;

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

    fn current_token(&self) -> Token {
        self.current_token.clone().unwrap()
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

    fn term(&mut self) -> i32 {
        // term : factor ((MUL | DIV) factor)*
        let mut result = self.factor();

        let operators = &[TokenType::Multiply, TokenType::Divide];
        while operators.iter().any(|t| *t == self.current_token().kind) {
            let token = self.current_token();
            result = match token.kind {
                t @ TokenType::Multiply => {
                    self.eat(t);
                    result * self.factor()
                }

                t @ TokenType::Divide => {
                    self.eat(t);
                    result / self.factor()
                }

                _ => {
                    break;
                }
            };
        }
        result
    }

    /// Arithmetic expression parser/interpreter.
    ///
    /// expr   : term ((PLUS|MINUS) term)*
    /// term   : factor ((MUL | DIV) factor)*
    /// factor : INTEGER
    pub fn expr(&mut self) -> i32 {
        let mut result = self.term();

        let operators = &[TokenType::Add, TokenType::Subtract];
        while operators.iter().any(|t| *t == self.current_token.clone().unwrap().kind) {
            let token = self.current_token.clone().unwrap();
            result = match token.kind {
                t @ TokenType::Add => {
                    self.eat(t);
                    result + self.term()
                }

                t @ TokenType::Subtract => {
                    self.eat(t);
                    result - self.term()
                }

                _ => {
                    break;
                }
            };
        }
        result
    }
}
