use crate::token::{Token, TokenType};

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(TokenType::Eof, String::new(), self.line));
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '+' => self.add_token(TokenType::Plus),
            '-' => {
                if self.match_char('>') {
                    self.add_token(TokenType::Arrow);
                } else {
                    self.add_token(TokenType::Minus);
                }
            }
            '*' => self.add_token(TokenType::Star),
            '%' => self.add_token(TokenType::Percent),
            '/' => {
                if self.match_char('/') {
                    // Comment - skip until end of line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '"' => self.string()?,
            '=' => {
                let token = if self.match_char('=') {
                    TokenType::EqualEqual
                } else if self.match_char('>') {
                    TokenType::FatArrow  // => for pattern matching
                } else {
                    TokenType::Equal
                };
                self.add_token(token);
            }
            '!' => {
                let token = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token);
            }
            '>' => {
                let token = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token);
            }
            '&' => {
                if self.match_char('&') {
                    self.add_token(TokenType::And);
                } else {
                    return Err(format!("Onverwagte karakter '&' op lyn {}", self.line));
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.add_token(TokenType::Or);
                } else {
                    return Err(format!("Onverwagte karakter '|' op lyn {}", self.line));
                }
            }
            '\n' => {
                self.add_token(TokenType::Newline);
                self.line += 1;
            }
            ' ' | '\r' | '\t' => {}
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    return Err(format!("Onverwagte karakter '{}' op lyn {}", c, self.line));
                }
            }
        }

        Ok(())
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for decimal part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consume the '.'
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let lexeme: String = self.source[self.start..self.current].iter().collect();
        let value: f64 = lexeme.parse().unwrap();
        self.add_token(TokenType::Number(value));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let lexeme: String = self.source[self.start..self.current].iter().collect();
        let token_type = match lexeme.as_str() {
            // Original keywords
            "as" => TokenType::As,
            "anders" => TokenType::Anders,
            "terwyl" => TokenType::Terwyl,
            "druk" => TokenType::Druk,
            "waar" => TokenType::Waar,
            "vals" => TokenType::Vals,
            // Functional keywords
            "funksie" => TokenType::Funksie,
            "fn" => TokenType::Fn,
            "gee" => TokenType::Gee,
            "laat" => TokenType::Laat,
            "pas" => TokenType::Pas,
            "geval" => TokenType::Geval,
            "tipe" => TokenType::Tipe,
            "of" => TokenType::Of,
            // Module keywords
            "laai" => TokenType::Laai,
            "verskaf" => TokenType::Verskaf,
            // Wildcard pattern
            "_" => TokenType::Underscore,
            _ => TokenType::Identifier(lexeme.clone()),
        };

        self.add_token(token_type);
    }

    fn string(&mut self) -> Result<(), String> {
        let start_line = self.line;

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            if self.peek() == '\\' && !self.is_at_end() {
                self.advance(); // consume the backslash
                if !self.is_at_end() {
                    self.advance(); // consume the escaped character
                }
            } else {
                self.advance();
            }
        }

        if self.is_at_end() {
            return Err(format!("Onbeëindigde string op lyn {}", start_line));
        }

        // Consume the closing "
        self.advance();

        // Extract the string value (without quotes)
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();

        // Process escape sequences
        let processed = self.process_escapes(&value)?;
        self.add_token(TokenType::Str(processed));
        Ok(())
    }

    fn process_escapes(&self, s: &str) -> Result<String, String> {
        let mut result = String::new();
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some(other) => {
                        return Err(format!("Ongeldige ontsnappingskarakter: \\{}", other));
                    }
                    None => {
                        return Err("Onverwagte einde van string na \\".to_string());
                    }
                }
            } else {
                result.push(c);
            }
        }

        Ok(result)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(token_type, lexeme, self.line));
    }
}
