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
        let result = interpret(String::from("2*3"));
        assert_eq!(result, 6);
    }

    #[test]
    fn it_divides_digits() {
        let result = interpret(String::from("4/2"));
        assert_eq!(result, 2);
    }

    #[test]
    fn it_handles_complex_operations() {
        let result = interpret(String::from("5 * 2 / 2 + 5 * 2"));
        assert_eq!(result, 15);
    }

    #[test]
    fn it_skips_whitespace() {
        let result = interpret(String::from("5     +      6"));
        assert_eq!(result, 11);
    }

    #[test]
    fn it_handles_parens() {
        let mut result = interpret(String::from("7 - (3 - 1)"));
        assert_eq!(result, 5);

        result = interpret(String::from("7 + (((3+2)))"));
        assert_eq!(result, 12);
    }

    fn interpret(string: String) -> i32 {
        let lexer = Lexer::new(&string);
        let mut int = Interpreter::new(lexer);
        int.expr()
    }
}
