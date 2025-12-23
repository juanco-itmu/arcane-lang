#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords (Afrikaans) - Original
    Stel,       // variable declaration (deprecated, use Laat)
    As,         // if
    Anders,     // else
    Terwyl,     // while
    Druk,       // print
    Waar,       // true
    Vals,       // false

    // Keywords (Afrikaans) - Functional
    Funksie,    // function definition
    Fn,         // lambda/anonymous function
    Gee,        // return
    Laat,       // let (immutable binding)
    Mut,        // mutable marker
    Pas,        // match
    Geval,      // case
    Tipe,       // type definition
    Of,         // or (variant separator)

    // Literals
    Number(f64),
    Str(String),    // string literal
    Identifier(String),

    // Operators
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
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
    LeftBracket,    // [
    RightBracket,   // ]
    Comma,          // ,
    Underscore,     // _ (wildcard pattern)
    Arrow,          // -> (optional, for type annotations)
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
