use crate::ast::{Expr, Literal, Stmt};
use crate::bytecode::{Chunk, OpCode};
use crate::token::TokenType;
use crate::value::Value;

pub struct Compiler {
    chunk: Chunk,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            chunk: Chunk::new(),
        }
    }

    pub fn compile(&mut self, statements: Vec<Stmt>) -> Result<Chunk, String> {
        for stmt in statements {
            self.compile_stmt(stmt)?;
        }
        self.emit(OpCode::Return);
        Ok(self.chunk.clone())
    }

    fn compile_stmt(&mut self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression(expr) => {
                self.compile_expr(expr)?;
                self.emit(OpCode::Pop);
            }
            Stmt::Print(expr) => {
                self.compile_expr(expr)?;
                self.emit(OpCode::Print);
            }
            Stmt::VarDecl { name, initializer } => {
                self.compile_expr(initializer)?;
                self.emit(OpCode::SetVar(name));
            }
            Stmt::Block(statements) => {
                for stmt in statements {
                    self.compile_stmt(stmt)?;
                }
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.compile_expr(condition)?;

                // Jump to else branch if condition is false
                let jump_to_else = self.emit(OpCode::JumpIfFalse(0));
                self.emit(OpCode::Pop); // Pop condition

                self.compile_stmt(*then_branch)?;

                if let Some(else_stmt) = else_branch {
                    // Jump over else branch after then
                    let jump_over_else = self.emit(OpCode::Jump(0));

                    // Patch the jump to else
                    let else_start = self.current_offset();
                    self.chunk.patch_jump(jump_to_else, else_start);
                    self.emit(OpCode::Pop); // Pop condition

                    self.compile_stmt(*else_stmt)?;

                    // Patch jump over else
                    let after_else = self.current_offset();
                    self.chunk.patch_jump(jump_over_else, after_else);
                } else {
                    let after_if = self.current_offset();
                    self.chunk.patch_jump(jump_to_else, after_if);
                    self.emit(OpCode::Pop); // Pop condition
                }
            }
            Stmt::While { condition, body } => {
                let loop_start = self.current_offset();

                self.compile_expr(condition)?;

                let exit_jump = self.emit(OpCode::JumpIfFalse(0));
                self.emit(OpCode::Pop); // Pop condition

                self.compile_stmt(*body)?;

                self.emit(OpCode::Jump(loop_start));

                let after_loop = self.current_offset();
                self.chunk.patch_jump(exit_jump, after_loop);
                self.emit(OpCode::Pop); // Pop condition
            }
        }

        Ok(())
    }

    fn compile_expr(&mut self, expr: Expr) -> Result<(), String> {
        match expr {
            Expr::Literal(lit) => {
                let value = match lit {
                    Literal::Number(n) => Value::Number(n),
                    Literal::Boolean(b) => Value::Boolean(b),
                    Literal::Nil => Value::Nil,
                };
                let idx = self.chunk.add_constant(value);
                self.emit(OpCode::Constant(idx));
            }
            Expr::Variable(name) => {
                self.emit(OpCode::GetVar(name));
            }
            Expr::Assign { name, value } => {
                self.compile_expr(*value)?;
                self.emit(OpCode::SetVar(name));
            }
            Expr::Grouping(inner) => {
                self.compile_expr(*inner)?;
            }
            Expr::Unary { operator, right } => {
                self.compile_expr(*right)?;
                match operator.token_type {
                    TokenType::Minus => self.emit(OpCode::Negate),
                    TokenType::Bang => self.emit(OpCode::Not),
                    _ => return Err("Onbekende unêre operator.".to_string()),
                };
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                // Handle short-circuit operators specially
                match operator.token_type {
                    TokenType::And => {
                        self.compile_expr(*left)?;
                        let jump = self.emit(OpCode::JumpIfFalse(0));
                        self.emit(OpCode::Pop);
                        self.compile_expr(*right)?;
                        let after = self.current_offset();
                        self.chunk.patch_jump(jump, after);
                    }
                    TokenType::Or => {
                        self.compile_expr(*left)?;
                        // If true, skip right side
                        let else_jump = self.emit(OpCode::JumpIfFalse(0));
                        let end_jump = self.emit(OpCode::Jump(0));

                        let else_branch = self.current_offset();
                        self.chunk.patch_jump(else_jump, else_branch);
                        self.emit(OpCode::Pop);
                        self.compile_expr(*right)?;

                        let end = self.current_offset();
                        self.chunk.patch_jump(end_jump, end);
                    }
                    _ => {
                        self.compile_expr(*left)?;
                        self.compile_expr(*right)?;

                        match operator.token_type {
                            TokenType::Plus => self.emit(OpCode::Add),
                            TokenType::Minus => self.emit(OpCode::Subtract),
                            TokenType::Star => self.emit(OpCode::Multiply),
                            TokenType::Slash => self.emit(OpCode::Divide),
                            TokenType::EqualEqual => self.emit(OpCode::Equal),
                            TokenType::BangEqual => self.emit(OpCode::NotEqual),
                            TokenType::Less => self.emit(OpCode::Less),
                            TokenType::LessEqual => self.emit(OpCode::LessEqual),
                            TokenType::Greater => self.emit(OpCode::Greater),
                            TokenType::GreaterEqual => self.emit(OpCode::GreaterEqual),
                            _ => return Err("Onbekende binêre operator.".to_string()),
                        };
                    }
                }
            }
        }

        Ok(())
    }

    fn emit(&mut self, op: OpCode) -> usize {
        self.chunk.write(op)
    }

    fn current_offset(&self) -> usize {
        self.chunk.code.len()
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
