use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Literal(Literal),
    Variable(String),
    Grouping(Box<Expr>),
    Assign {
        name: String,
        value: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    VarDecl {
        name: String,
        initializer: Expr,
    },
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}
