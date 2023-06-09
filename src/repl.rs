use crate::lexer::Lexer;
use crate::token::TokenType;
use std::io::{self, BufRead, Write};

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

            if tok.token_type == TokenType::EoF {
                break;
            }

            writeln!(output, "{:?}", tok).unwrap();
        }
    }
}
