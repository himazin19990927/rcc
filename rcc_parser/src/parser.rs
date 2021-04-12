use crate::ast::{BinOp, Expr, UnOp};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer) -> Parser {
        let cur = lexer.next_token();

        Parser {
            lexer: lexer,
            cur: cur,
        }
    }

    fn next_token(&mut self) {
        self.cur = self.lexer.next_token();
    }

    fn consume(&mut self, kind: TokenKind) -> bool {
        if self.cur.kind == kind {
            self.next_token();
            return true;
        }

        return false;
    }

    fn expect(&mut self, kind: TokenKind) {
        if self.cur.kind != kind {
            panic!("expect {:?}, but got {:?}", kind, self.cur.kind)
        }

        self.next_token();
    }

    fn expect_number(&mut self) -> u32 {
        if self.cur.kind != TokenKind::Num {
            panic!("expect {:?}, but got {:?}", TokenKind::Num, self.cur.kind);
        }

        let num = self.cur.literal.parse().unwrap();
        self.next_token();

        return num;
    }

    pub fn expr(&mut self) -> Expr {
        return self.equality();
    }

    pub fn equality(&mut self) -> Expr {
        let left = self.relational();

        if self.consume(TokenKind::EqEq) {
            let right = self.relational();
            return Expr::Binary(BinOp::Eq, Box::new(left), Box::new(right));
        }

        if self.consume(TokenKind::Ne) {
            let right = self.relational();
            return Expr::Binary(BinOp::Ne, Box::new(left), Box::new(right));
        }

        return left;
    }

    pub fn relational(&mut self) -> Expr {
        let left = self.add();

        if self.consume(TokenKind::Lt) {
            let right = self.add();
            return Expr::Binary(BinOp::Lt, Box::new(left), Box::new(right));
        }

        if self.consume(TokenKind::Le) {
            let right = self.add();
            return Expr::Binary(BinOp::Le, Box::new(left), Box::new(right));
        }

        if self.consume(TokenKind::Gt) {
            let right = self.add();
            return Expr::Binary(BinOp::Lt, Box::new(right), Box::new(left));
        }

        if self.consume(TokenKind::Ge) {
            let right = self.add();
            return Expr::Binary(BinOp::Le, Box::new(right), Box::new(left));
        }

        return left;
    }

    pub fn add(&mut self) -> Expr {
        let left = self.mul();

        if self.consume(TokenKind::Plus) {
            let right = self.add();
            return Expr::Binary(BinOp::Add, Box::new(left), Box::new(right));
        }

        if self.consume(TokenKind::Minus) {
            let right = self.add();
            return Expr::Binary(BinOp::Sub, Box::new(left), Box::new(right));
        }

        return left;
    }

    pub fn mul(&mut self) -> Expr {
        let left = self.unary();

        if self.consume(TokenKind::Star) {
            let right = self.mul();
            return Expr::Binary(BinOp::Mul, Box::new(left), Box::new(right));
        }

        if self.consume(TokenKind::Slash) {
            let right = self.mul();
            return Expr::Binary(BinOp::Div, Box::new(left), Box::new(right));
        }

        return left;
    }

    pub fn unary(&mut self) -> Expr {
        if self.consume(TokenKind::Plus) {
            return self.primary();
        }

        if self.consume(TokenKind::Minus) {
            let right = self.primary();
            return Expr::Unary(UnOp::Neg, Box::new(right));
        }

        return self.primary();
    }

    pub fn primary(&mut self) -> Expr {
        if self.consume(TokenKind::OpenParen) {
            let expr = self.expr();
            self.expect(TokenKind::CloseParen);
            return expr;
        }

        let num = self.expect_number();

        Expr::Integer(num)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{BinOp, Expr, UnOp};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn test_expr(src: &str, expect: Expr) {
        assert_eq!(expect, Parser::new(Lexer::new(src)).expr());
    }

    #[test]
    fn parse_num() {
        test_expr("0", Expr::Integer(0));
        test_expr("10", Expr::Integer(10));
        test_expr("01", Expr::Integer(1));
    }

    #[test]
    fn parse_unary() {
        test_expr("-1", Expr::Unary(UnOp::Neg, Box::new(Expr::Integer(1))));
        test_expr(
            "-1+2",
            Expr::Binary(
                BinOp::Add,
                Box::new(Expr::Unary(UnOp::Neg, Box::new(Expr::Integer(1)))),
                Box::new(Expr::Integer(2)),
            ),
        );
        test_expr(
            "1*-2",
            Expr::Binary(
                BinOp::Mul,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Unary(UnOp::Neg, Box::new(Expr::Integer(2)))),
            ),
        );
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
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Binary(
                    BinOp::Add,
                    Box::new(Expr::Integer(2)),
                    Box::new(Expr::Integer(3)),
                )),
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

    #[test]
    fn parse_relational() {
        test_expr(
            "1==2",
            Expr::Binary(
                BinOp::Eq,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Integer(2)),
            ),
        );
        test_expr(
            "1<2",
            Expr::Binary(
                BinOp::Lt,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Integer(2)),
            ),
        );
        test_expr(
            "1<=2",
            Expr::Binary(
                BinOp::Le,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Integer(2)),
            ),
        );
        test_expr(
            "1!=2",
            Expr::Binary(
                BinOp::Ne,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Integer(2)),
            ),
        );
        test_expr(
            "1>2",
            Expr::Binary(
                BinOp::Lt,
                Box::new(Expr::Integer(2)),
                Box::new(Expr::Integer(1)),
            ),
        );
        test_expr(
            "1>=2",
            Expr::Binary(
                BinOp::Le,
                Box::new(Expr::Integer(2)),
                Box::new(Expr::Integer(1)),
            ),
        );
    }
}
