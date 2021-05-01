pub mod ast;

use ast::*;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

pub fn parse_expr(input: &str) -> Expr {
    grammar::ExprParser::new().parse(input).unwrap()
}

#[cfg(test)]
mod tests {
    use super::parse_expr;
    use crate::ast::*;

    fn test_expr(src: &str, expect: Expr) {
        assert_eq!(expect, parse_expr(src));
    }

    #[test]
    fn parse_num() {
        test_expr("0", Expr::Integer(0));
        test_expr("10", Expr::Integer(10));
        test_expr("01", Expr::Integer(1));
        test_expr("(1)", Expr::Integer(1));
    }
}
