//! interpret is an interpreter for the Lox programming language.

#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unused_variables,
    unstable_features,
    unused_import_braces,
    missing_docs
)]

use clap::Parser;
use std::io::{self, Write};

mod expr;
mod parser;
mod scanner;
mod token;

use scanner::Scanner;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pretty: bool,

    #[arg(short, long)]
    file: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if let Some(file_path) = args.file {
        let file_content = std::fs::read_to_string(file_path)?;
        run(&file_content);
    } else {
        loop {
            let mut input = String::new();
            print!("> ");
            io::stdout().flush()?;

            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.eq_ignore_ascii_case("exit") {
                break;
            }

            run(input);
        }
    }

    Ok(())
}

fn run(input: &str) {
    let mut scanner = Scanner::new(input.to_string());
    let tokens = scanner.scan_tokens();
    let mut p = parser::Parser::new(tokens.clone());

    let ast = p.parse();
    let evaluator = expr::Evaluation::new(ast);
    evaluator.interpret();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_simple() {
        let input = "123 + 456";
        let mut scanner = Scanner::new(input.to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].ttype, token::TokenType::Number);
        if let Some(token::LiteralValue::Number(n)) = tokens[0].literal {
            assert_eq!(n, 123.0);
        }
        assert_eq!(tokens[1].ttype, token::TokenType::Plus);
        assert_eq!(tokens[2].ttype, token::TokenType::Number);
        if let Some(token::LiteralValue::Number(n)) = tokens[2].literal {
            assert_eq!(n, 456.0);
        }
        assert_eq!(tokens[3].ttype, token::TokenType::EOF);
    }
}
