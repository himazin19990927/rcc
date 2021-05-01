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
        test_expr("0", Expr::Int(0));
        test_expr("10", Expr::Int(10));
        test_expr("01", Expr::Int(1));
        test_expr("(1)", Expr::Int(1));
    }

    #[test]
    fn parse_bool() {
        test_expr("true", Expr::Bool(true));
        test_expr("false", Expr::Bool(false));
    }

    #[test]
    fn parse_ident() {
        test_expr("x", Expr::Ident("x".to_string()));
        test_expr("hoge", Expr::Ident("hoge".to_string()));
    }

    #[test]
    fn parse_unary() {
        test_expr("-1", Expr::Unary(UnOp::Neg, Box::new(Expr::Int(1))));
    }

    #[test]
    fn parse_binary() {
        test_expr(
            "1+2",
            Expr::Binary(BinOp::Add, Box::new(Expr::Int(1)), Box::new(Expr::Int(2))),
        );
        test_expr(
            "1-2",
            Expr::Binary(BinOp::Sub, Box::new(Expr::Int(1)), Box::new(Expr::Int(2))),
        );
        test_expr(
            "1*2",
            Expr::Binary(BinOp::Mul, Box::new(Expr::Int(1)), Box::new(Expr::Int(2))),
        );
        test_expr(
            "1/2",
            Expr::Binary(BinOp::Div, Box::new(Expr::Int(1)), Box::new(Expr::Int(2))),
        );
        test_expr(
            "1+2+3",
            Expr::Binary(
                BinOp::Add,
                Box::new(Expr::Binary(
                    BinOp::Add,
                    Box::new(Expr::Int(1)),
                    Box::new(Expr::Int(2)),
                )),
                Box::new(Expr::Int(3)),
            ),
        );
        test_expr(
            "1*2+3",
            Expr::Binary(
                BinOp::Add,
                Box::new(Expr::Binary(
                    BinOp::Mul,
                    Box::new(Expr::Int(1)),
                    Box::new(Expr::Int(2)),
                )),
                Box::new(Expr::Int(3)),
            ),
        );
    }

    #[test]
    fn parse_paren() {
        test_expr("(1)", Expr::Int(1));
        test_expr(
            "(1+2)",
            Expr::Binary(BinOp::Add, Box::new(Expr::Int(1)), Box::new(Expr::Int(2))),
        );
        test_expr(
            "1*(2+3)",
            Expr::Binary(
                BinOp::Mul,
                Box::new(Expr::Int(1)),
                Box::new(Expr::Binary(
                    BinOp::Add,
                    Box::new(Expr::Int(2)),
                    Box::new(Expr::Int(3)),
                )),
            ),
        );
        test_expr(
            "(1+2)*(3/(4+5))",
            Expr::Binary(
                BinOp::Mul,
                Box::new(Expr::Binary(
                    BinOp::Add,
                    Box::new(Expr::Int(1)),
                    Box::new(Expr::Int(2)),
                )),
                Box::new(Expr::Binary(
                    BinOp::Div,
                    Box::new(Expr::Int(3)),
                    Box::new(Expr::Binary(
                        BinOp::Add,
                        Box::new(Expr::Int(4)),
                        Box::new(Expr::Int(5)),
                    )),
                )),
            ),
        );
    }

    #[test]
    fn parse_relational() {
        test_expr(
            "1==2",
            Expr::Binary(BinOp::Eq, Box::new(Expr::Int(1)), Box::new(Expr::Int(2))),
        );
        test_expr(
            "1<2",
            Expr::Binary(BinOp::Lt, Box::new(Expr::Int(1)), Box::new(Expr::Int(2))),
        );
        test_expr(
            "1<=2",
            Expr::Binary(BinOp::Le, Box::new(Expr::Int(1)), Box::new(Expr::Int(2))),
        );
        test_expr(
            "1!=2",
            Expr::Binary(BinOp::Ne, Box::new(Expr::Int(1)), Box::new(Expr::Int(2))),
        );
        test_expr(
            "1>2",
            Expr::Binary(BinOp::Lt, Box::new(Expr::Int(2)), Box::new(Expr::Int(1))),
        );
        test_expr(
            "1>=2",
            Expr::Binary(BinOp::Le, Box::new(Expr::Int(2)), Box::new(Expr::Int(1))),
        );
    }

    #[test]
    fn parse_logical_binary() {
        test_expr(
            "true&&false",
            Expr::Binary(
                BinOp::And,
                Box::new(Expr::Bool(true)),
                Box::new(Expr::Bool(false)),
            ),
        );

        test_expr(
            "true&&false&&true",
            Expr::Binary(
                BinOp::And,
                Box::new(Expr::Binary(
                    BinOp::And,
                    Box::new(Expr::Bool(true)),
                    Box::new(Expr::Bool(false)),
                )),
                Box::new(Expr::Bool(true)),
            ),
        );

        test_expr(
            "true||false",
            Expr::Binary(
                BinOp::Or,
                Box::new(Expr::Bool(true)),
                Box::new(Expr::Bool(false)),
            ),
        );

        test_expr(
            "true||false||true",
            Expr::Binary(
                BinOp::Or,
                Box::new(Expr::Binary(
                    BinOp::Or,
                    Box::new(Expr::Bool(true)),
                    Box::new(Expr::Bool(false)),
                )),
                Box::new(Expr::Bool(true)),
            ),
        );

        test_expr(
            "true||false&&true",
            Expr::Binary(
                BinOp::Or,
                Box::new(Expr::Bool(true)),
                Box::new(Expr::Binary(
                    BinOp::And,
                    Box::new(Expr::Bool(false)),
                    Box::new(Expr::Bool(true)),
                )),
            ),
        );
    }
}
