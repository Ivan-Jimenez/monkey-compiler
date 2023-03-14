use crate::token::*;

pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        let mut token = Token::new(TokenType::EOF, '\0'.to_string());
        self.skip_white_space();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch);
                    token = Token::new(TokenType::EQ, literal);
                } else {
                    token = Token::new(TokenType::ASSIGN, self.ch.to_string());
                }
            }
            '+' => token = Token::new(TokenType::PLUS, self.ch.to_string()),
            '-' => token = Token::new(TokenType::MINUS, self.ch.to_string()),
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch);
                    token = Token::new(TokenType::NOT_EQ, literal)
                } else {
                    token = Token::new(TokenType::BANG, self.ch.to_string())
                }
            }
            '/' => token = Token::new(TokenType::SLASH, self.ch.to_string()),
            '*' => token = Token::new(TokenType::ASTERISK, self.ch.to_string()),
            '<' => token = Token::new(TokenType::LT, self.ch.to_string()),
            '>' => token = Token::new(TokenType::GT, self.ch.to_string()),
            ';' => token = Token::new(TokenType::SEMICOLON, self.ch.to_string()),
            ',' => token = Token::new(TokenType::COMMA, self.ch.to_string()),
            '{' => token = Token::new(TokenType::LBRACE, self.ch.to_string()),
            '}' => token = Token::new(TokenType::RBRACE, self.ch.to_string()),
            '(' => token = Token::new(TokenType::LPAREN, self.ch.to_string()),
            ')' => token = Token::new(TokenType::RPAREN, self.ch.to_string()),
            '\0' => token = Token::new(TokenType::EOF, "".to_string()),
            _ => {
                if self.is_letter(self.ch) {
                    token.literal = self.read_identifier();
                    token.token_type = Token::lookup_identifier(&token.literal);
                    return token;
                } else if self.is_digit(self.ch) {
                    token.token_type = TokenType::INT;
                    token.literal = self.read_number();
                    return token;
                } else {
                    token = Token::new(TokenType::ILLEGAL, self.ch.to_string())
                }
            }
        }
        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position..].chars().next().unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_white_space(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn peek_char(&mut self) -> char {
        return if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position..].chars().next().unwrap()
        };
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.is_digit(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn is_letter(&self, ch: char) -> bool {
        return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z';
    }

    fn is_digit(&self, ch: char) -> bool {
        return '0' <= ch && ch <= '9';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test {
        expected_type: TokenType,
        expected_literal: String,
    }

    #[test]
    fn test_next_token() {
        let input = "let five = 5; \
        let ten = 10;\
        \
        let add = fn(x, y) {\
          x + y;\
        };\
        \
        let result = add(five, ten);\
        !-/*5;\
        5 < 10 > 5;\
        \
        if (5 < 10) {\
          return true;\
        } else {\
          return false;\
        }\
        \
        10 == 10;\
        10 != 9;
        "
        .to_string();

        let tests = [
            Test {
                expected_type: TokenType::LET,
                expected_literal: "let".to_string(),
            },
            Test {
                expected_type: TokenType::IDENTIFIER,
                expected_literal: "five".to_string(),
            },
            Test {
                expected_type: TokenType::ASSIGN,
                expected_literal: "=".to_string(),
            },
            Test {
                expected_type: TokenType::INT,
                expected_literal: "5".to_string(),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::LET,
                expected_literal: "let".to_string(),
            },
            Test {
                expected_type: TokenType::IDENTIFIER,
                expected_literal: "ten".to_string(),
            },
            Test {
                expected_type: TokenType::ASSIGN,
                expected_literal: "=".to_string(),
            },
            Test {
                expected_type: TokenType::INT,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::LET,
                expected_literal: "let".to_string(),
            },
            Test {
                expected_type: TokenType::IDENTIFIER,
                expected_literal: "add".to_string(),
            },
            Test {
                expected_type: TokenType::ASSIGN,
                expected_literal: "=".to_string(),
            },
            Test {
                expected_type: TokenType::FUNCTION,
                expected_literal: "fn".to_string(),
            },
            Test {
                expected_type: TokenType::LPAREN,
                expected_literal: "(".to_string(),
            },
            Test {
                expected_type: TokenType::IDENTIFIER,
                expected_literal: "x".to_string(),
            },
            Test {
                expected_type: TokenType::COMMA,
                expected_literal: ",".to_string(),
            },
            Test {
                expected_type: TokenType::IDENTIFIER,
                expected_literal: "y".to_string(),
            },
            Test {
                expected_type: TokenType::RPAREN,
                expected_literal: ")".to_string(),
            },
            Test {
                expected_type: TokenType::LBRACE,
                expected_literal: "{".to_string(),
            },
            Test {
                expected_type: TokenType::IDENTIFIER,
                expected_literal: "x".to_string(),
            },
            Test {
                expected_type: TokenType::PLUS,
                expected_literal: "+".to_string(),
            },
            Test {
                expected_type: TokenType::IDENTIFIER,
                expected_literal: "y".to_string(),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::RBRACE,
                expected_literal: "}".to_string(),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::LET,
                expected_literal: "let".to_string(),
            },
            Test {
                expected_type: TokenType::IDENTIFIER,
                expected_literal: "result".to_string(),
            },
            Test {
                expected_type: TokenType::ASSIGN,
                expected_literal: "=".to_string(),
            },
            Test {
                expected_type: TokenType::IDENTIFIER,
                expected_literal: "add".to_string(),
            },
            Test {
                expected_type: TokenType::LPAREN,
                expected_literal: "(".to_string(),
            },
            Test {
                expected_type: TokenType::IDENTIFIER,
                expected_literal: "five".to_string(),
            },
            Test {
                expected_type: TokenType::COMMA,
                expected_literal: ",".to_string(),
            },
            Test {
                expected_type: TokenType::IDENTIFIER,
                expected_literal: "ten".to_string(),
            },
            Test {
                expected_type: TokenType::RPAREN,
                expected_literal: ")".to_string(),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::BANG,
                expected_literal: "!".to_string(),
            },
            Test {
                expected_type: TokenType::MINUS,
                expected_literal: "-".to_string(),
            },
            Test {
                expected_type: TokenType::SLASH,
                expected_literal: "/".to_string(),
            },
            Test {
                expected_type: TokenType::ASTERISK,
                expected_literal: "*".to_string(),
            },
            Test {
                expected_type: TokenType::INT,
                expected_literal: "5".to_string(),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::INT,
                expected_literal: "5".to_string(),
            },
            Test {
                expected_type: TokenType::LT,
                expected_literal: "<".to_string(),
            },
            Test {
                expected_type: TokenType::INT,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::GT,
                expected_literal: ">".to_string(),
            },
            Test {
                expected_type: TokenType::INT,
                expected_literal: "5".to_string(),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::IF,
                expected_literal: "if".to_string(),
            },
            Test {
                expected_type: TokenType::LPAREN,
                expected_literal: "(".to_string(),
            },
            Test {
                expected_type: TokenType::INT,
                expected_literal: "5".to_string(),
            },
            Test {
                expected_type: TokenType::LT,
                expected_literal: "<".to_string(),
            },
            Test {
                expected_type: TokenType::INT,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::RPAREN,
                expected_literal: ")".to_string(),
            },
            Test {
                expected_type: TokenType::LBRACE,
                expected_literal: "{".to_string(),
            },
            Test {
                expected_type: TokenType::RETURN,
                expected_literal: "return".to_string(),
            },
            Test {
                expected_type: TokenType::TRUE,
                expected_literal: "true".to_string(),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::RBRACE,
                expected_literal: "}".to_string(),
            },
            Test {
                expected_type: TokenType::ELSE,
                expected_literal: "else".to_string(),
            },
            Test {
                expected_type: TokenType::LBRACE,
                expected_literal: "{".to_string(),
            },
            Test {
                expected_type: TokenType::RETURN,
                expected_literal: "return".to_string(),
            },
            Test {
                expected_type: TokenType::FALSE,
                expected_literal: "false".to_string(),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::RBRACE,
                expected_literal: "}".to_string(),
            },
            Test {
                expected_type: TokenType::INT,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::EQ,
                expected_literal: "==".to_string(),
            },
            Test {
                expected_type: TokenType::INT,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::INT,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::NOT_EQ,
                expected_literal: "!=".to_string(),
            },
            Test {
                expected_type: TokenType::INT,
                expected_literal: "9".to_string(),
            },
            Test {
                expected_type: TokenType::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::EOF,
                expected_literal: "".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for expected_token in tests.iter() {
            let token = lexer.next_token();
            assert_eq!(token.token_type, expected_token.expected_type);
            assert_eq!(token.literal, expected_token.expected_literal);
        }
    }
}
