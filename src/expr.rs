use crate::token::{LiteralValue, Token, TokenType};

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
    pub fn accept<R, V: Visitor<R>>(&self, visitor: V) -> R {
        match self {
            Expr::Binary { left, op, right } => visitor.visit_binary(left, op, right),
            Expr::Unary { op, right } => visitor.visit_unary(op, right),
            Expr::Literal { value } => visitor.visit_literal(value),
            Expr::Grouped { expr } => visitor.visit_grouped(expr),
        }
    }
}

pub trait Visitor<R> {
    fn visit_binary(self, left: &Expr, op: &Token, right: &Expr) -> R;
    fn visit_unary(self, op: &Token, right: &Expr) -> R;
    fn visit_literal(self, value: &Option<LiteralValue>) -> R;
    fn visit_grouped(self, expr: &Expr) -> R;
}

pub struct Prettify {
    expr: Expr,
}

impl Prettify {
    pub fn new(expr: Expr) -> Self {
        Prettify { expr }
    }
    pub fn pretty_print(&self) {
        println!("{}", self.expr.accept(self));
    }
}

impl Visitor<String> for &Prettify {
    fn visit_binary(self, left: &Expr, op: &Token, right: &Expr) -> String {
        format!(
            "({} {} {})",
            op.lexeme,
            left.accept(self),
            right.accept(self)
        )
    }
    fn visit_unary(self, op: &Token, right: &Expr) -> String {
        format!("({} {})", op.lexeme, right.accept(self))
    }
    fn visit_literal(self, value: &Option<LiteralValue>) -> String {
        match value {
            Some(t) => format!("{}", t),
            None => format!("unknown"),
        }
    }
    fn visit_grouped(self, expr: &Expr) -> String {
        format!("(group {})", expr.accept(self))
    }
}

pub struct Evaluation {
    expr: Expr,
}

impl Evaluation {
    pub fn new(expr: Expr) -> Self {
        Evaluation { expr }
    }

    pub fn interpret(&self) {
        let value = self.expr.accept(self);
        println!("{}", value);
    }
}

impl Visitor<LiteralValue> for &Evaluation {
    fn visit_literal(self, value: &Option<LiteralValue>) -> LiteralValue {
        match value {
            Some(t) => t.clone(),
            None => LiteralValue::Nil,
        }
    }

    fn visit_binary(self, left: &Expr, op: &Token, right: &Expr) -> LiteralValue {
        let left = left.accept(self);
        let right = right.accept(self);

        match op.ttype {
            TokenType::Minus => match (left, right) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => LiteralValue::Number(l - r),
                _ => panic!("operands must be numbers"),
            },
            TokenType::Plus => match (left, right) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => LiteralValue::Number(l + r),
                (LiteralValue::String(l), LiteralValue::String(r)) => {
                    LiteralValue::String(format!("{}{}", l, r))
                }
                _ => panic!("operands must be numbers or strings"),
            },
            TokenType::Star => match (left, right) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => LiteralValue::Number(l * r),
                _ => panic!("operands must be numbers"),
            },
            TokenType::Slash => match (left, right) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => LiteralValue::Number(l / r),
                _ => panic!("operands must be numbers"),
            },
            TokenType::EqualEqual => match (left, right) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => LiteralValue::Boolean(l == r),
                (LiteralValue::String(l), LiteralValue::String(r)) => LiteralValue::Boolean(l == r),
                (LiteralValue::Nil, LiteralValue::Nil) => LiteralValue::Boolean(true),
                (LiteralValue::Boolean(l), LiteralValue::Boolean(r)) => {
                    LiteralValue::Boolean(l == r)
                }
                _ => panic!("unknown operands"),
            },
            TokenType::BangEqual => match (left, right) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => LiteralValue::Boolean(l != r),
                (LiteralValue::String(l), LiteralValue::String(r)) => LiteralValue::Boolean(l != r),
                (LiteralValue::Nil, LiteralValue::Nil) => LiteralValue::Boolean(false),
                (LiteralValue::Boolean(l), LiteralValue::Boolean(r)) => {
                    LiteralValue::Boolean(l != r)
                }
                _ => panic!("operands must be numbers or strings"),
            },
            TokenType::Less => match (left, right) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => LiteralValue::Boolean(l < r),
                _ => panic!("operands must be numbers"),
            },
            TokenType::Greater => match (left, right) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => LiteralValue::Boolean(l > r),
                _ => panic!("operands must be numbers"),
            },
            TokenType::LessEqual => match (left, right) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => LiteralValue::Boolean(l <= r),
                _ => panic!("operands must be numbers"),
            },
            TokenType::GreaterEqual => match (left, right) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => LiteralValue::Boolean(l >= r),
                _ => panic!("operands must be numbers"),
            },
            _ => panic!("unknown operator"),
        }
    }

    fn visit_unary(self, op: &Token, right: &Expr) -> LiteralValue {
        let right = right.accept(self);
        match op.ttype {
            TokenType::Bang => match right {
                LiteralValue::Boolean(b) => LiteralValue::Boolean(!b),
                _ => panic!("operand must be a boolean"),
            },
            TokenType::Minus => match right {
                LiteralValue::Number(n) => LiteralValue::Number(-n),
                _ => panic!("operand must be a number"),
            },
            _ => panic!("unknown operator"),
        }
    }

    fn visit_grouped(self, expr: &Expr) -> LiteralValue {
        expr.accept(self)
    }
}
