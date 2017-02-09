pub mod token;
pub mod interpreter;
pub mod lexer;

#[cfg(test)]
mod tests {
    use interpreter::Interpreter;
    use lexer::Lexer;

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
        let string = String::from("6/3");
        let mut lexer = Lexer::new(&string);
        let mut int = Interpreter::new(lexer);
        let result = int.expr();

        assert_eq!(result, 2);
    }
}
