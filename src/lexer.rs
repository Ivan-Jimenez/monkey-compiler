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
        let mut token = Token::new(TokenType::EoF, '\0'.to_string());
        self.skip_white_space();

        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch);
                    token = Token::new(TokenType::Equal, literal);
                } else {
                    token = Token::new(TokenType::Assign, self.ch.to_string());
                }
            }
            '+' => token = Token::new(TokenType::Plus, self.ch.to_string()),
            '-' => token = Token::new(TokenType::Minus, self.ch.to_string()),
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch);
                    token = Token::new(TokenType::NotEqual, literal)
                } else {
                    token = Token::new(TokenType::Bang, self.ch.to_string())
                }
            }
            '/' => token = Token::new(TokenType::Slash, self.ch.to_string()),
            '*' => token = Token::new(TokenType::Asterisk, self.ch.to_string()),
            '<' => token = Token::new(TokenType::LowerThan, self.ch.to_string()),
            '>' => token = Token::new(TokenType::GraterThan, self.ch.to_string()),
            ';' => token = Token::new(TokenType::SemiColon, self.ch.to_string()),
            ',' => token = Token::new(TokenType::Comma, self.ch.to_string()),
            '{' => token = Token::new(TokenType::LeftBrace, self.ch.to_string()),
            '}' => token = Token::new(TokenType::RightBrace, self.ch.to_string()),
            '(' => token = Token::new(TokenType::LeftParenthesis, self.ch.to_string()),
            ')' => token = Token::new(TokenType::RightParenthesis, self.ch.to_string()),
            '\0' => token = Token::new(TokenType::EoF, "".to_string()),
            _ => {
                if self.is_letter(self.ch) {
                    token.literal = self.read_identifier();
                    token.token_type = Token::lookup_identifier(&token.literal);
                    return token;
                } else if self.ch.is_ascii_digit() {
                    token.token_type = TokenType::Int;
                    token.literal = self.read_number();
                    return token;
                } else {
                    token = Token::new(TokenType::Illegal, self.ch.to_string())
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
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn is_letter(&self, ch: char) -> bool {
        ch.is_ascii_uppercase() || ch.is_ascii_lowercase()
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
                expected_type: TokenType::Let,
                expected_literal: "let".to_string(),
            },
            Test {
                expected_type: TokenType::Identifier,
                expected_literal: "five".to_string(),
            },
            Test {
                expected_type: TokenType::Assign,
                expected_literal: "=".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "5".to_string(),
            },
            Test {
                expected_type: TokenType::SemiColon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::Let,
                expected_literal: "let".to_string(),
            },
            Test {
                expected_type: TokenType::Identifier,
                expected_literal: "ten".to_string(),
            },
            Test {
                expected_type: TokenType::Assign,
                expected_literal: "=".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::SemiColon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::Let,
                expected_literal: "let".to_string(),
            },
            Test {
                expected_type: TokenType::Identifier,
                expected_literal: "add".to_string(),
            },
            Test {
                expected_type: TokenType::Assign,
                expected_literal: "=".to_string(),
            },
            Test {
                expected_type: TokenType::Function,
                expected_literal: "fn".to_string(),
            },
            Test {
                expected_type: TokenType::LeftParenthesis,
                expected_literal: "(".to_string(),
            },
            Test {
                expected_type: TokenType::Identifier,
                expected_literal: "x".to_string(),
            },
            Test {
                expected_type: TokenType::Comma,
                expected_literal: ",".to_string(),
            },
            Test {
                expected_type: TokenType::Identifier,
                expected_literal: "y".to_string(),
            },
            Test {
                expected_type: TokenType::RightParenthesis,
                expected_literal: ")".to_string(),
            },
            Test {
                expected_type: TokenType::LeftBrace,
                expected_literal: "{".to_string(),
            },
            Test {
                expected_type: TokenType::Identifier,
                expected_literal: "x".to_string(),
            },
            Test {
                expected_type: TokenType::Plus,
                expected_literal: "+".to_string(),
            },
            Test {
                expected_type: TokenType::Identifier,
                expected_literal: "y".to_string(),
            },
            Test {
                expected_type: TokenType::SemiColon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::RightBrace,
                expected_literal: "}".to_string(),
            },
            Test {
                expected_type: TokenType::SemiColon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::Let,
                expected_literal: "let".to_string(),
            },
            Test {
                expected_type: TokenType::Identifier,
                expected_literal: "result".to_string(),
            },
            Test {
                expected_type: TokenType::Assign,
                expected_literal: "=".to_string(),
            },
            Test {
                expected_type: TokenType::Identifier,
                expected_literal: "add".to_string(),
            },
            Test {
                expected_type: TokenType::LeftParenthesis,
                expected_literal: "(".to_string(),
            },
            Test {
                expected_type: TokenType::Identifier,
                expected_literal: "five".to_string(),
            },
            Test {
                expected_type: TokenType::Comma,
                expected_literal: ",".to_string(),
            },
            Test {
                expected_type: TokenType::Identifier,
                expected_literal: "ten".to_string(),
            },
            Test {
                expected_type: TokenType::RightParenthesis,
                expected_literal: ")".to_string(),
            },
            Test {
                expected_type: TokenType::SemiColon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::Bang,
                expected_literal: "!".to_string(),
            },
            Test {
                expected_type: TokenType::Minus,
                expected_literal: "-".to_string(),
            },
            Test {
                expected_type: TokenType::Slash,
                expected_literal: "/".to_string(),
            },
            Test {
                expected_type: TokenType::Asterisk,
                expected_literal: "*".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "5".to_string(),
            },
            Test {
                expected_type: TokenType::SemiColon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "5".to_string(),
            },
            Test {
                expected_type: TokenType::LowerThan,
                expected_literal: "<".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::GraterThan,
                expected_literal: ">".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "5".to_string(),
            },
            Test {
                expected_type: TokenType::SemiColon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::If,
                expected_literal: "if".to_string(),
            },
            Test {
                expected_type: TokenType::LeftParenthesis,
                expected_literal: "(".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "5".to_string(),
            },
            Test {
                expected_type: TokenType::LowerThan,
                expected_literal: "<".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::RightParenthesis,
                expected_literal: ")".to_string(),
            },
            Test {
                expected_type: TokenType::LeftBrace,
                expected_literal: "{".to_string(),
            },
            Test {
                expected_type: TokenType::Return,
                expected_literal: "return".to_string(),
            },
            Test {
                expected_type: TokenType::True,
                expected_literal: "true".to_string(),
            },
            Test {
                expected_type: TokenType::SemiColon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::RightBrace,
                expected_literal: "}".to_string(),
            },
            Test {
                expected_type: TokenType::Else,
                expected_literal: "else".to_string(),
            },
            Test {
                expected_type: TokenType::LeftBrace,
                expected_literal: "{".to_string(),
            },
            Test {
                expected_type: TokenType::Return,
                expected_literal: "return".to_string(),
            },
            Test {
                expected_type: TokenType::False,
                expected_literal: "false".to_string(),
            },
            Test {
                expected_type: TokenType::SemiColon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::RightBrace,
                expected_literal: "}".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::Equal,
                expected_literal: "==".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::SemiColon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "10".to_string(),
            },
            Test {
                expected_type: TokenType::NotEqual,
                expected_literal: "!=".to_string(),
            },
            Test {
                expected_type: TokenType::Int,
                expected_literal: "9".to_string(),
            },
            Test {
                expected_type: TokenType::SemiColon,
                expected_literal: ";".to_string(),
            },
            Test {
                expected_type: TokenType::EoF,
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
