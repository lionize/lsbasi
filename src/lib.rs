pub mod token;
pub mod interpreter;
pub mod lexer;

#[cfg(test)]
mod tests {
    use interpreter::Interpreter;
    use lexer::Lexer;

    #[test]
    fn it_adds_digits() {
        let result = interpret(String::from("1+1+2"));
        assert_eq!(result, 4);
    }

    #[test]
    fn it_subtracts_digits() {
        let result = interpret(String::from("5-1-1"));
        assert_eq!(result, 3);
    }

    #[test]
    fn it_multiplies_digits() {
        let string = String::from("2*3");
        let mut lexer = Lexer::new(&string);
        let mut int = Interpreter::new(lexer);
        let result = int.expr();

        assert_eq!(result, 6);
    }

    #[test]
    fn it_divides_digits() {
        let result = interpret(String::from("4/2"));

        assert_eq!(result, 2);
    }

    fn interpret(string: String) -> i32 {
        let mut lexer = Lexer::new(&string);
        let mut int = Interpreter::new(lexer);
        int.expr()
    }
}
