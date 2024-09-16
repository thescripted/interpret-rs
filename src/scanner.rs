use crate::token::{LiteralValue, Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    cursor: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            cursor: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.finished() {
            let c = self.current();
            match c {
                '(' => self.parse_single_char(TokenType::LeftParen, "("),
                ')' => self.parse_single_char(TokenType::RightParen, ")"),
                '{' => self.parse_single_char(TokenType::LeftBrace, "{"),
                '}' => self.parse_single_char(TokenType::RightBrace, "}"),
                ',' => self.parse_single_char(TokenType::Comma, ","),
                '.' => self.parse_single_char(TokenType::Dot, "."),
                '-' => self.parse_single_char(TokenType::Minus, "-"),
                '+' => self.parse_single_char(TokenType::Plus, "+"),
                ';' => self.parse_single_char(TokenType::Semicolon, ";"),
                '*' => self.parse_single_char(TokenType::Star, "*"),
                '!' => self.parse_bang(),
                '=' => self.parse_equal(),
                '>' => self.parse_greater(),
                '<' => self.parse_less(),
                '/' => self.parse_slash(),
                ' ' | '\r' | '\t' => self.advance(), // ignore whitespace
                '\n' => {
                    self.advance();
                    self.line += 1;
                }
                c => {
                    if c.is_digit(10) {
                        self.parse_number();
                    } else if c.is_alphabetic() {
                        self.parse_identifier();
                    } else {
                        eprintln!("error at line {}: Unexpected character '{}'", self.line, c);
                        self.advance();
                    }
                }
            }
        }
        self.add_token(TokenType::EOF, "");

        self.tokens.clone()
    }

    fn parse_single_char(&mut self, ttype: TokenType, lexeme: &str) {
        self.add_token(ttype, lexeme);
        self.advance();
    }

    fn parse_bang(&mut self) {
        if self.peek() == '=' {
            self.add_token(TokenType::BangEqual, "!=");
            self.advance();
        } else {
            self.add_token(TokenType::Bang, "!");
        }
        self.advance()
    }

    fn parse_equal(&mut self) {
        if self.peek() == '=' {
            self.add_token(TokenType::EqualEqual, "==");
            self.advance();
        } else {
            self.add_token(TokenType::Equal, "=");
        }
        self.advance()
    }

    fn parse_greater(&mut self) {
        if self.peek() == '=' {
            self.add_token(TokenType::GreaterEqual, ">=");
            self.advance();
        } else {
            self.add_token(TokenType::Greater, ">");
        }
        self.advance()
    }

    fn parse_less(&mut self) {
        if self.peek() == '=' {
            self.add_token(TokenType::LessEqual, "<=");
            self.advance();
        } else {
            self.add_token(TokenType::Less, "<");
        }
        self.advance()
    }

    fn parse_slash(&mut self) {
        if self.peek() == '/' {
            while self.current() != '\n' && !self.finished() {
                self.advance();
            }
        } else {
            self.add_token(TokenType::Slash, "/");
        }
        self.advance();
    }

    fn parse_number(&mut self) {
        let start = self.cursor;
        while self.current().is_digit(10) {
            self.advance();
        }

        // e.g., matches 3\.1 but not 3\.
        if self.current() == '.' && self.peek().is_digit(10) {
            self.advance();
            while self.current().is_digit(10) {
                self.advance();
            }
        }
        let end = self.cursor;

        let number = self.source[start..end].iter().collect::<String>();
        self.add_token_with_literal(
            TokenType::Number,
            &number,
            Some(LiteralValue::Number(number.parse().unwrap())), // TODO(ben): handle error
        );
    }

    fn parse_identifier(&mut self) {
        let start = self.cursor;
        while self.current().is_alphanumeric() {
            self.advance();
        }
        let end = self.cursor;

        let text = self.source[start..end].iter().collect::<String>();
        let ttype = get_keyword_token_type(&text);
        self.add_token(ttype, &text);
    }
    fn add_token(&mut self, ttype: TokenType, lexeme: &str) {
        self.tokens.push(Token {
            ttype,
            lexeme: lexeme.to_string(),
            literal: None,
            line: self.line,
        });
    }

    fn add_token_with_literal(
        &mut self,
        ttype: TokenType,
        lexeme: &str,
        literal: Option<LiteralValue>,
    ) {
        self.tokens.push(Token {
            ttype,
            lexeme: lexeme.to_string(),
            literal,
            line: self.line,
        });
    }

    fn current(&mut self) -> char {
        if self.finished() {
            return '\0';
        }
        self.source[self.cursor]
    }

    fn advance(&mut self) {
        self.cursor += 1;
    }

    fn peek(&mut self) -> char {
        if self.finished() || self.cursor + 1 >= self.source.len() {
            return '\0';
        }

        self.source[self.cursor + 1]
    }

    fn finished(&mut self) -> bool {
        self.cursor >= self.source.len()
    }
}

fn get_keyword_token_type(text: &str) -> TokenType {
    match text {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "fun" => TokenType::Fun,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier,
    }
}
