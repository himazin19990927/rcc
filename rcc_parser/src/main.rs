use rcc_parser::{lexer::Lexer, parser::Parser};
use std::env::{self, args};

fn main() {
    let src = args().nth(1).unwrap();
    let stmt = Parser::new(Lexer::new(src.as_str())).stmt();
    println!("{:?}", stmt);
}

#[allow(dead_code)]
fn parse_expr() {
    let args: Vec<_> = env::args().collect();
    let src = &args[1];
    let expr = Parser::new(Lexer::new(src.as_str())).expr();
    println!("{:?}", expr);
}
