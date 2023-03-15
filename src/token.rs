#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    EoF,

    // Identifiers + literals
    Identifier, // add, foobar, x, y, ...
    Int,        // 1343456

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LowerThan,
    GraterThan,

    Equal,
    NotEqual,

    // Delimiters
    Comma,
    SemiColon,

    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }

    pub fn lookup_identifier(identifier: &str) -> TokenType {
        match identifier {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            _ => TokenType::Identifier,
        }
    }
}
