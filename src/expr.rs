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

impl Expr {
    pub fn eval<R>(&self, callback: impl Callback<R>) -> R {
        match self {
            Expr::Binary { left, op, right } => callback.eval_binary(left, op, right),
            Expr::Unary { op, right } => callback.eval_unary(op, right),
            Expr::Literal { value } => callback.eval_literal(value),
            Expr::Grouped { expr } => callback.eval_grouped(expr),
        }
    }
}

pub trait Callback<R> {
    fn eval_binary(self, left: &Expr, op: &Token, right: &Expr) -> R;
    fn eval_unary(self, op: &Token, right: &Expr) -> R;
    fn eval_literal(self, value: &Option<LiteralValue>) -> R;
    fn eval_grouped(self, expr: &Expr) -> R;
}

pub struct Prettify {
    expr: Expr,
}

impl Prettify {
    pub fn new(expr: Expr) -> Self {
        Prettify { expr }
    }
    pub fn pretty_print(&self) {
        println!("{}", self.expr.eval(self));
    }
}

impl Callback<String> for &Prettify {
    fn eval_binary(self, left: &Expr, op: &Token, right: &Expr) -> String {
        format!("({} {} {})", op.lexeme, left.eval(self), right.eval(self))
    }
    fn eval_unary(self, op: &Token, right: &Expr) -> String {
        format!("({} {})", op.lexeme, right.eval(self))
    }
    fn eval_literal(self, value: &Option<LiteralValue>) -> String {
        match value {
            Some(t) => format!("{}", t),
            None => format!("unknown"),
        }
    }
    fn eval_grouped(self, expr: &Expr) -> String {
        format!("(group {})", expr.eval(self))
    }
}
