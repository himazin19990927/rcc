use core::panic;

use crate::ast::{BinOp, Expr, Stmt, UnOp};
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

    fn peek_kind(&self) -> TokenKind {
        self.cur.kind
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

    fn expect_ident(&mut self) -> String {
        if self.cur.kind != TokenKind::Identifier {
            panic!(
                "expect {:?}, but got {:?}",
                TokenKind::Identifier,
                self.cur.kind
            );
        }

        let ident = self.cur.literal.clone();
        self.next_token();

        return ident;
    }

    pub fn program(&mut self) -> Vec<Stmt> {
        let mut program = Vec::new();
        while self.peek_kind() != TokenKind::EOF {
            program.push(self.stmt());
        }

        return program;
    }

    pub fn stmt(&mut self) -> Stmt {
        if self.consume(TokenKind::Int) {
            let ident = self.expect_ident();
            self.expect(TokenKind::Eq);
            let rhs = self.expr();
            self.expect(TokenKind::Semi);

            return Stmt::Declaration(Expr::Ident(ident), rhs);
        }

        let ident = self.expect_ident();
        self.expect(TokenKind::Eq);
        let rhs = self.expr();
        self.expect(TokenKind::Semi);
        return Stmt::Assign(Expr::Ident(ident), rhs);
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

        match self.peek_kind() {
            TokenKind::Identifier => {
                let ident = self.expect_ident();
                Expr::Ident(ident)
            }
            TokenKind::Num => {
                let num = self.expect_number();
                Expr::Integer(num)
            }
            _ => panic!(
                "Expected {:?} or {:?} but got {:?}",
                TokenKind::Identifier,
                TokenKind::Num,
                self.peek_kind()
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{BinOp, Expr, Stmt, UnOp};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn test_expr(src: &str, expect: Expr) {
        assert_eq!(expect, Parser::new(Lexer::new(src)).expr());
    }

    fn test_stmt(src: &str, expect: Stmt) {
        assert_eq!(expect, Parser::new(Lexer::new(src)).stmt());
    }

    fn test_program(src: &str, expect: Vec<Stmt>) {
        assert_eq!(expect, Parser::new(Lexer::new(src)).program());
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

    #[test]
    fn parse_ident() {
        test_expr("val", Expr::Ident("val".to_string()));
        test_expr(
            "a+1",
            Expr::Binary(
                BinOp::Add,
                Box::new(Expr::Ident("a".to_string())),
                Box::new(Expr::Integer(1)),
            ),
        );
    }

    #[test]
    fn parse_stmt() {
        test_stmt(
            "int a = 0;",
            Stmt::Declaration(Expr::Ident("a".to_string()), Expr::Integer(0)),
        );
        test_stmt(
            "a = 0;",
            Stmt::Assign(Expr::Ident("a".to_string()), Expr::Integer(0)),
        );
    }

    #[test]
    fn parse_program() {
        test_program(
            "int a = 0; a = a + 1;",
            vec![
                Stmt::Declaration(Expr::Ident("a".to_string()), Expr::Integer(0)),
                Stmt::Assign(
                    Expr::Ident("a".to_string()),
                    Expr::Binary(
                        BinOp::Add,
                        Box::new(Expr::Ident("a".to_string())),
                        Box::new(Expr::Integer(1)),
                    ),
                ),
            ],
        )
    }
}
