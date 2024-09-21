use crate::token::{LiteralValue, Token};

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    Grouped {
        expr: Box<Expr>,
    },
    Literal {
        value: Option<LiteralValue>,
    },
}
