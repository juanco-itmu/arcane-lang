#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords (Afrikaans)
    Stel,       // variable declaration
    As,         // if
    Anders,     // else
    Terwyl,     // while
    Druk,       // print
    Waar,       // true
    Vals,       // false

    // Literals
    Number(f64),
    Identifier(String),

    // Operators
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Equal,          // =
    EqualEqual,     // ==
    Bang,           // !
    BangEqual,      // !=
    Less,           // <
    LessEqual,      // <=
    Greater,        // >
    GreaterEqual,   // >=
    And,            // &&
    Or,             // ||

    // Punctuation
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    Newline,

    // End of file
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}
