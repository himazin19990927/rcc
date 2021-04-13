use rcc_parser::{lexer::Lexer, parser::Parser};
use std::{env, iter::Peekable, str::Chars};

fn main() {
    // let mut chars = "int a".chars();
    // let res = read_keyword(&mut chars, "int");
    // println!("result = {:?}, chars = {:?}", res, chars);
}

#[allow(dead_code)]
fn parse_expr() {
    let args: Vec<_> = env::args().collect();
    let src = &args[1];
    let expr = Parser::new(Lexer::new(src.as_str())).expr();
    println!("{:?}", expr);
}

// #[allow(dead_code)]
// fn read_keyword(mut src: &Chars, keyword: &str) -> Option<String> {
//     if keyword.len() > src.as_str().len() {
//         return None;
//     }

//     let (first, last) = src.as_str().split_at(keyword.len());

//     match last.chars().next() {
//         Some(c) => {
//             if !c.is_alphanumeric() && c != '_' {
//                 return None;
//             }
//         }
//         None => return None,
//     }

//     let _ = src.take(keyword.chars().count());
//     return Some(keyword.to_string());
// }
