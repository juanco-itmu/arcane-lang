use tower_lsp::lsp_types::*;

// We need to duplicate some core logic here since we can't easily share
// between binaries. In a real project, you'd use a library crate.

#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    Stel, As, Anders, Terwyl, Druk, Waar, Vals,
    Number(f64), Identifier(String),
    Plus, Minus, Star, Slash,
    Equal, EqualEqual, Bang, BangEqual,
    Less, LessEqual, Greater, GreaterEqual,
    And, Or,
    LeftParen, RightParen, LeftBrace, RightBrace,
    Newline, Eof,
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u32,
    start_col: u32,
    end_col: u32,
}

struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
    col: u32,
    start_col: u32,
}

impl Lexer {
    fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
            col: 0,
            start_col: 0,
        }
    }

    fn scan_tokens(&mut self) -> (Vec<Token>, Vec<Diagnostic>) {
        let mut diagnostics = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            self.start_col = self.col;
            if let Err(e) = self.scan_token() {
                diagnostics.push(e);
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            line: self.line,
            start_col: self.col,
            end_col: self.col,
        });

        (self.tokens.clone(), diagnostics)
    }

    fn scan_token(&mut self) -> std::result::Result<(), Diagnostic> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '=' => {
                let token = if self.match_char('=') {
                    TokenType::EqualEqual
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
                    return Err(self.make_diagnostic("Onverwagte karakter '&'. Bedoel jy '&&'?"));
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.add_token(TokenType::Or);
                } else {
                    return Err(self.make_diagnostic("Onverwagte karakter '|'. Bedoel jy '||'?"));
                }
            }
            '\n' => {
                self.add_token(TokenType::Newline);
                self.line += 1;
                self.col = 0;
            }
            ' ' | '\r' | '\t' => {}
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    return Err(self.make_diagnostic(&format!("Onverwagte karakter '{}'", c)));
                }
            }
        }

        Ok(())
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        let value: f64 = lexeme.parse().unwrap_or(0.0);
        self.add_token(TokenType::Number(value));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        let token_type = match lexeme.as_str() {
            "stel" => TokenType::Stel,
            "as" => TokenType::As,
            "anders" => TokenType::Anders,
            "terwyl" => TokenType::Terwyl,
            "druk" => TokenType::Druk,
            "waar" => TokenType::Waar,
            "vals" => TokenType::Vals,
            _ => TokenType::Identifier(lexeme.clone()),
        };
        self.add_token(token_type);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        self.col += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() { '\0' } else { self.source[self.current] }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { '\0' } else { self.source[self.current + 1] }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            self.col += 1;
            true
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token {
            token_type,
            lexeme,
            line: self.line,
            start_col: self.start_col,
            end_col: self.col,
        });
    }

    fn make_diagnostic(&self, message: &str) -> Diagnostic {
        Diagnostic {
            range: Range {
                start: Position { line: self.line, character: self.start_col },
                end: Position { line: self.line, character: self.col },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("arcane".to_string()),
            message: message.to_string(),
            ..Default::default()
        }
    }
}

// Simple parser for diagnostics
fn parse_for_diagnostics(tokens: &[Token]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let mut i = 0;
    let mut paren_stack: Vec<&Token> = Vec::new();
    let mut brace_stack: Vec<&Token> = Vec::new();

    // Track declared variables for undefined variable detection
    let mut declared_vars: std::collections::HashSet<String> = std::collections::HashSet::new();

    // First pass: collect all declared variables
    let mut j = 0;
    while j < tokens.len() {
        if matches!(tokens[j].token_type, TokenType::Stel) {
            if j + 1 < tokens.len() {
                if let TokenType::Identifier(name) = &tokens[j + 1].token_type {
                    declared_vars.insert(name.clone());
                }
            }
        }
        j += 1;
    }

    while i < tokens.len() {
        let token = &tokens[i];

        match &token.token_type {
            TokenType::LeftParen => paren_stack.push(token),
            TokenType::RightParen => {
                if paren_stack.pop().is_none() {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: token.line, character: token.start_col },
                            end: Position { line: token.line, character: token.end_col },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("arcane".to_string()),
                        message: "Ongepaarde ')' - geen ooreenstemmende '(' gevind".to_string(),
                        ..Default::default()
                    });
                }
            }
            TokenType::LeftBrace => brace_stack.push(token),
            TokenType::RightBrace => {
                if brace_stack.pop().is_none() {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: token.line, character: token.start_col },
                            end: Position { line: token.line, character: token.end_col },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("arcane".to_string()),
                        message: "Ongepaarde '}' - geen ooreenstemmende '{' gevind".to_string(),
                        ..Default::default()
                    });
                }
            }
            TokenType::Stel => {
                // Check for: stel <identifier> = <expr>
                if i + 1 < tokens.len() {
                    if !matches!(tokens[i + 1].token_type, TokenType::Identifier(_)) {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: token.line, character: token.start_col },
                                end: Position { line: token.line, character: token.end_col },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("arcane".to_string()),
                            message: "Verwag veranderlike naam na 'stel'".to_string(),
                            ..Default::default()
                        });
                    } else if i + 2 < tokens.len() && !matches!(tokens[i + 2].token_type, TokenType::Equal) {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: tokens[i + 1].line, character: tokens[i + 1].end_col },
                                end: Position { line: tokens[i + 1].line, character: tokens[i + 1].end_col + 1 },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("arcane".to_string()),
                            message: "Verwag '=' na veranderlike naam".to_string(),
                            ..Default::default()
                        });
                    }
                }
            }
            TokenType::As | TokenType::Terwyl => {
                // Check for: as/terwyl (condition) {
                if i + 1 < tokens.len() && !matches!(tokens[i + 1].token_type, TokenType::LeftParen) {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: token.line, character: token.end_col },
                            end: Position { line: token.line, character: token.end_col + 1 },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("arcane".to_string()),
                        message: format!("Verwag '(' na '{}'", token.lexeme),
                        ..Default::default()
                    });
                }
            }
            TokenType::Druk => {
                // Check for: druk(expr)
                if i + 1 < tokens.len() && !matches!(tokens[i + 1].token_type, TokenType::LeftParen) {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: token.line, character: token.end_col },
                            end: Position { line: token.line, character: token.end_col + 1 },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("arcane".to_string()),
                        message: "Verwag '(' na 'druk'".to_string(),
                        ..Default::default()
                    });
                }
            }
            TokenType::Identifier(name) => {
                // Check if this identifier is used as a variable (not being declared)
                // Skip if previous token is 'stel' (it's a declaration)
                let is_declaration = i > 0 && matches!(tokens[i - 1].token_type, TokenType::Stel);

                // Check if this is a function call (followed by '(')
                let is_function_call = i + 1 < tokens.len()
                    && matches!(tokens[i + 1].token_type, TokenType::LeftParen);

                if is_function_call {
                    // Only 'druk' is a valid function
                    if name != "druk" {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: token.line, character: token.start_col },
                                end: Position { line: token.line, character: token.end_col },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("arcane".to_string()),
                            message: format!("Onbekende funksie: '{}'. Bedoel jy 'druk'?", name),
                            ..Default::default()
                        });
                    }
                } else if !is_declaration && !declared_vars.contains(name) {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: token.line, character: token.start_col },
                            end: Position { line: token.line, character: token.end_col },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("arcane".to_string()),
                        message: format!("Ongedefinieerde veranderlike: '{}'", name),
                        ..Default::default()
                    });
                }
            }
            _ => {}
        }

        i += 1;
    }

    // Report unclosed brackets
    for token in paren_stack {
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position { line: token.line, character: token.start_col },
                end: Position { line: token.line, character: token.end_col },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("arcane".to_string()),
            message: "Ongeslote '(' - verwag ')'".to_string(),
            ..Default::default()
        });
    }

    for token in brace_stack {
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position { line: token.line, character: token.start_col },
                end: Position { line: token.line, character: token.end_col },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("arcane".to_string()),
            message: "Ongeslote '{' - verwag '}'".to_string(),
            ..Default::default()
        });
    }

    diagnostics
}

pub fn analyze_document(text: &str) -> Vec<Diagnostic> {
    let mut lexer = Lexer::new(text);
    let (tokens, mut diagnostics) = lexer.scan_tokens();

    let parse_diagnostics = parse_for_diagnostics(&tokens);
    diagnostics.extend(parse_diagnostics);

    diagnostics
}

pub fn get_hover_info(text: &str, position: Position) -> Option<Hover> {
    let mut lexer = Lexer::new(text);
    let (tokens, _) = lexer.scan_tokens();

    // Find the token at the position
    for token in tokens {
        if token.line == position.line
            && position.character >= token.start_col
            && position.character < token.end_col
        {
            let info = match &token.token_type {
                TokenType::Stel => Some((
                    "**stel** (sleutelwoord)\n\nVerklaar 'n nuwe veranderlike.\n\n```arcane\nstel x = 10\n```",
                    "Declare a new variable"
                )),
                TokenType::As => Some((
                    "**as** (sleutelwoord)\n\nVoorwaardelike stelling (if statement).\n\n```arcane\nas (x > 5) {\n    druk(x)\n}\n```",
                    "Conditional statement (if)"
                )),
                TokenType::Anders => Some((
                    "**anders** (sleutelwoord)\n\nAlternatiewe tak van 'as' stelling.\n\n```arcane\nas (x > 5) {\n    druk(\"groot\")\n} anders {\n    druk(\"klein\")\n}\n```",
                    "Else branch"
                )),
                TokenType::Terwyl => Some((
                    "**terwyl** (sleutelwoord)\n\nHerhaal terwyl voorwaarde waar is.\n\n```arcane\nterwyl (x > 0) {\n    druk(x)\n    stel x = x - 1\n}\n```",
                    "While loop"
                )),
                TokenType::Druk => Some((
                    "**druk** (funksie)\n\nDruk 'n waarde na die konsole.\n\n```arcane\ndruk(42)\ndruk(waar)\n```",
                    "Print to console"
                )),
                TokenType::Waar => Some((
                    "**waar** (boolean)\n\nBoolean waarde vir 'waar' (true).",
                    "Boolean true"
                )),
                TokenType::Vals => Some((
                    "**vals** (boolean)\n\nBoolean waarde vir 'vals' (false).",
                    "Boolean false"
                )),
                _ => None,
            };

            if let Some((afrikaans, english)) = info {
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!("{}\n\n---\n*{}*", afrikaans, english),
                    }),
                    range: Some(Range {
                        start: Position { line: token.line, character: token.start_col },
                        end: Position { line: token.line, character: token.end_col },
                    }),
                });
            }
        }
    }

    None
}

pub fn get_completions(text: &str, position: Position) -> Vec<CompletionItem> {
    let mut completions = vec![
        CompletionItem {
            label: "stel".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Verklaar veranderlike".to_string()),
            insert_text: Some("stel ${1:naam} = ${0:waarde}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "druk".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some("Druk na konsole".to_string()),
            insert_text: Some("druk(${0})".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "as".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("As-stelling (if)".to_string()),
            insert_text: Some("as (${1:voorwaarde}) {\n\t${0}\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "anders".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Anders-tak (else)".to_string()),
            insert_text: Some("anders {\n\t${0}\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "terwyl".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Terwyl-lus (while)".to_string()),
            insert_text: Some("terwyl (${1:voorwaarde}) {\n\t${0}\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "waar".to_string(),
            kind: Some(CompletionItemKind::CONSTANT),
            detail: Some("Boolean waar (true)".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "vals".to_string(),
            kind: Some(CompletionItemKind::CONSTANT),
            detail: Some("Boolean vals (false)".to_string()),
            ..Default::default()
        },
    ];

    // Extract variable names from the document
    let mut lexer = Lexer::new(text);
    let (tokens, _) = lexer.scan_tokens();

    let mut seen_vars = std::collections::HashSet::new();
    let mut i = 0;
    while i < tokens.len() {
        if matches!(tokens[i].token_type, TokenType::Stel) {
            if i + 1 < tokens.len() {
                if let TokenType::Identifier(name) = &tokens[i + 1].token_type {
                    if !seen_vars.contains(name) {
                        seen_vars.insert(name.clone());
                        completions.push(CompletionItem {
                            label: name.clone(),
                            kind: Some(CompletionItemKind::VARIABLE),
                            detail: Some("Veranderlike".to_string()),
                            ..Default::default()
                        });
                    }
                }
            }
        }
        i += 1;
    }

    completions
}
