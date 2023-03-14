use std::io::{self, BufRead, Write};
use crate::token::TokenType;
use crate::lexer::Lexer;

const PROMPT: &str = ">> ";

pub fn start(input: &mut dyn BufRead, output: &mut dyn Write) {
    let mut scanner = io::BufReader::new(input).lines();

    loop {
        write!(output, "{}", PROMPT).unwrap();

        let line = match scanner.next() {
            Some(Ok(line)) => line,
            _ => return,
        };

        let mut lexer = Lexer::new(line);

        loop {
            let tok = lexer.next_token();

            if tok.token_type == TokenType::EOF {
                break;
            }

            writeln!(output, "{:?}", tok).unwrap();
        }
    }
}
