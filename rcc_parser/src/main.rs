use rcc_parser::{lexer::Lexer, parser::Parser};
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let src = &args[1];
    let expr = Parser::new(Lexer::new(src.as_str())).expr();
    println!("{:?}", expr);
}
