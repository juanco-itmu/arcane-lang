use crate::ast::{Expr, Literal, Stmt};
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            self.skip_newlines();
            if !self.is_at_end() {
                statements.push(self.declaration()?);
            }
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.check(&TokenType::Stel) {
            self.advance();
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume_identifier("Verwag veranderlike naam.")?;
        self.consume(&TokenType::Equal, "Verwag '=' na veranderlike naam.")?;
        let initializer = self.expression()?;
        self.consume_newline_or_eof()?;
        Ok(Stmt::VarDecl { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.check(&TokenType::Druk) {
            self.advance();
            self.print_statement()
        } else if self.check(&TokenType::As) {
            self.advance();
            self.if_statement()
        } else if self.check(&TokenType::Terwyl) {
            self.advance();
            self.while_statement()
        } else if self.check(&TokenType::LeftBrace) {
            self.advance();
            Ok(Stmt::Block(self.block()?))
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        self.consume(&TokenType::LeftParen, "Verwag '(' na 'druk'.")?;
        let value = self.expression()?;
        self.consume(&TokenType::RightParen, "Verwag ')' na uitdrukking.")?;
        self.consume_newline_or_eof()?;
        Ok(Stmt::Print(value))
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(&TokenType::LeftParen, "Verwag '(' na 'as'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Verwag ')' na voorwaarde.")?;
        self.skip_newlines();

        self.consume(&TokenType::LeftBrace, "Verwag '{' na 'as' voorwaarde.")?;
        let then_branch = Stmt::Block(self.block()?);
        self.skip_newlines();

        let else_branch = if self.check(&TokenType::Anders) {
            self.advance();
            self.skip_newlines();
            self.consume(&TokenType::LeftBrace, "Verwag '{' na 'anders'.")?;
            Some(Box::new(Stmt::Block(self.block()?)))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.consume(&TokenType::LeftParen, "Verwag '(' na 'terwyl'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Verwag ')' na voorwaarde.")?;
        self.skip_newlines();

        self.consume(&TokenType::LeftBrace, "Verwag '{' na 'terwyl' voorwaarde.")?;
        let body = Stmt::Block(self.block()?);

        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        self.skip_newlines();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
            self.skip_newlines();
        }

        self.consume(&TokenType::RightBrace, "Verwag '}' na blok.")?;
        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume_newline_or_eof()?;
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.or()?;

        if self.check(&TokenType::Equal) {
            self.advance();
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }

            return Err("Ongeldige toewysing teiken.".to_string());
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;

        while self.check(&TokenType::Or) {
            let operator = self.advance().clone();
            let right = self.and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;

        while self.check(&TokenType::And) {
            let operator = self.advance().clone();
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.check(&TokenType::EqualEqual) || self.check(&TokenType::BangEqual) {
            let operator = self.advance().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        while self.check(&TokenType::Less)
            || self.check(&TokenType::LessEqual)
            || self.check(&TokenType::Greater)
            || self.check(&TokenType::GreaterEqual)
        {
            let operator = self.advance().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.check(&TokenType::Plus) || self.check(&TokenType::Minus) {
            let operator = self.advance().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.check(&TokenType::Star) || self.check(&TokenType::Slash) {
            let operator = self.advance().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.check(&TokenType::Bang) || self.check(&TokenType::Minus) {
            let operator = self.advance().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.check(&TokenType::Waar) {
            self.advance();
            return Ok(Expr::Literal(Literal::Boolean(true)));
        }

        if self.check(&TokenType::Vals) {
            self.advance();
            return Ok(Expr::Literal(Literal::Boolean(false)));
        }

        if let TokenType::Number(n) = &self.peek().token_type {
            let value = *n;
            self.advance();
            return Ok(Expr::Literal(Literal::Number(value)));
        }

        if let TokenType::Identifier(name) = &self.peek().token_type {
            let name = name.clone();
            self.advance();
            return Ok(Expr::Variable(name));
        }

        if self.check(&TokenType::LeftParen) {
            self.advance();
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Verwag ')' na uitdrukking.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(format!(
            "Verwag uitdrukking op lyn {}.",
            self.peek().line
        ))
    }

    // Helper methods

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(format!("{} (lyn {})", message, self.peek().line))
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<String, String> {
        if let TokenType::Identifier(name) = &self.peek().token_type {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(format!("{} (lyn {})", message, self.peek().line))
        }
    }

    fn consume_newline_or_eof(&mut self) -> Result<(), String> {
        if self.check(&TokenType::Newline) {
            self.advance();
            Ok(())
        } else if self.is_at_end() || self.check(&TokenType::RightBrace) {
            Ok(())
        } else {
            Err(format!(
                "Verwag nuwe lyn na stelling. (lyn {})",
                self.peek().line
            ))
        }
    }

    fn skip_newlines(&mut self) {
        while self.check(&TokenType::Newline) {
            self.advance();
        }
    }
}
