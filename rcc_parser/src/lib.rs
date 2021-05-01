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

    #[test]
    fn parse_unary() {
        test_expr("-1", Expr::Unary(UnOp::Neg, Box::new(Expr::Integer(1))));
    }

    #[test]
    fn parse_binary() {
        test_expr(
            "1+2",
            Expr::Binary(
                BinOp::Add,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Integer(2)),
            ),
        );
        test_expr(
            "1-2",
            Expr::Binary(
                BinOp::Sub,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Integer(2)),
            ),
        );
        test_expr(
            "1*2",
            Expr::Binary(
                BinOp::Mul,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Integer(2)),
            ),
        );
        test_expr(
            "1/2",
            Expr::Binary(
                BinOp::Div,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Integer(2)),
            ),
        );
        test_expr(
            "1+2+3",
            Expr::Binary(
                BinOp::Add,
                Box::new(Expr::Binary(
                    BinOp::Add,
                    Box::new(Expr::Integer(1)),
                    Box::new(Expr::Integer(2)),
                )),
                Box::new(Expr::Integer(3)),
            ),
        );
        test_expr(
            "1*2+3",
            Expr::Binary(
                BinOp::Add,
                Box::new(Expr::Binary(
                    BinOp::Mul,
                    Box::new(Expr::Integer(1)),
                    Box::new(Expr::Integer(2)),
                )),
                Box::new(Expr::Integer(3)),
            ),
        );
    }

    #[test]
    fn parse_paren() {
        test_expr("(1)", Expr::Integer(1));
        test_expr(
            "(1+2)",
            Expr::Binary(
                BinOp::Add,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Integer(2)),
            ),
        );
        test_expr(
            "1*(2+3)",
            Expr::Binary(
                BinOp::Mul,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Binary(
                    BinOp::Add,
                    Box::new(Expr::Integer(2)),
                    Box::new(Expr::Integer(3)),
                )),
            ),
        );
        test_expr(
            "(1+2)*(3/(4+5))",
            Expr::Binary(
                BinOp::Mul,
                Box::new(Expr::Binary(
                    BinOp::Add,
                    Box::new(Expr::Integer(1)),
                    Box::new(Expr::Integer(2)),
                )),
                Box::new(Expr::Binary(
                    BinOp::Div,
                    Box::new(Expr::Integer(3)),
                    Box::new(Expr::Binary(
                        BinOp::Add,
                        Box::new(Expr::Integer(4)),
                        Box::new(Expr::Integer(5)),
                    )),
                )),
            ),
        );
    }
}
