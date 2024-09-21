use crate::token::{Token, TokenType};

mod expression {
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
}

use expression::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    pub fn parse(&mut self) -> Expr {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_comparison();
        while matches!(
            self.current().ttype,
            TokenType::EqualEqual | TokenType::BangEqual
        ) {
            let operator = self.current().clone();
            self.advance();

            let right = self.parse_comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();

        while matches!(
            self.current().ttype,
            TokenType::Less | TokenType::LessEqual | TokenType::Greater | TokenType::GreaterEqual
        ) {
            let operator = self.current().clone();
            self.advance();

            let right = self.parse_term();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();

        while matches!(self.current().ttype, TokenType::Plus | TokenType::Minus) {
            let operator = self.current().clone();
            self.advance();

            let right = self.parse_factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_unary();

        while matches!(self.current().ttype, TokenType::Slash | TokenType::Star) {
            let operator = self.current().clone();
            self.advance();

            let right = self.parse_unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn parse_unary(&mut self) -> Expr {
        match self.current().ttype {
            TokenType::Minus | TokenType::Bang => {
                self.advance();

                let operator = self.current().clone();
                Expr::Unary {
                    op: operator,
                    right: Box::new(self.parse_unary()),
                }
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current().ttype {
            TokenType::Number
            | TokenType::String
            | TokenType::True
            | TokenType::False
            | TokenType::Nil => {
                let value = self.current().literal.clone();
                self.advance();

                Expr::Literal { value }
            }

            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression();

                Expr::Grouped {
                    expr: Box::new(expr),
                }
            }

            _ => todo!(),
        }
    }

    // TODO: this is an operation that should never fail. Return EOF token, or something...
    fn current(&self) -> &Token {
        &self.tokens[self.cursor]
    }

    fn advance(&mut self) {
        self.cursor += 1
    }
}
