use crate::ast::{BinOp, Expr};
use crate::lexer::{Lexer, Token, TokenKind};

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

    #[allow(dead_code)]
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
        let left = self.mul();

        if self.consume(TokenKind::Plus) {
            let right = self.expr();
            return Expr::Binary(BinOp::Add, Box::new(left), Box::new(right));
        }

        if self.consume(TokenKind::Minus) {
            let right = self.expr();
            return Expr::Binary(BinOp::Sub, Box::new(left), Box::new(right));
        }

        return left;
    }

    pub fn mul(&mut self) -> Expr {
        let left = self.primary();

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

    pub fn primary(&mut self) -> Expr {
        let num = self.expect_number();

        Expr::Integer(num)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{BinOp, Expr};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn parse_num() {
        assert_eq!(Expr::Integer(0), Parser::new(Lexer::new("0")).expr());
        assert_eq!(Expr::Integer(10), Parser::new(Lexer::new("10")).expr());
        assert_eq!(Expr::Integer(1), Parser::new(Lexer::new("01")).expr());
    }

    #[test]
    fn parse_binary() {
        assert_eq!(
            Expr::Binary(
                BinOp::Add,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Integer(1))
            ),
            Parser::new(Lexer::new("1+1")).expr(),
        );

        assert_eq!(
            Expr::Binary(
                BinOp::Sub,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Integer(1))
            ),
            Parser::new(Lexer::new("1-1")).expr(),
        );

        assert_eq!(
            Expr::Binary(
                BinOp::Mul,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Integer(1))
            ),
            Parser::new(Lexer::new("1*1")).expr(),
        );

        assert_eq!(
            Expr::Binary(
                BinOp::Div,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Integer(1))
            ),
            Parser::new(Lexer::new("1/1")).expr(),
        );

        assert_eq!(
            Expr::Binary(
                BinOp::Add,
                Box::new(Expr::Integer(1)),
                Box::new(Expr::Binary(
                    BinOp::Add,
                    Box::new(Expr::Integer(2)),
                    Box::new(Expr::Integer(3)),
                ))
            ),
            Parser::new(Lexer::new("1+2+3")).expr(),
        );

        assert_eq!(
            Expr::Binary(
                BinOp::Add,
                Box::new(Expr::Binary(
                    BinOp::Mul,
                    Box::new(Expr::Integer(1)),
                    Box::new(Expr::Integer(2)),
                )),
                Box::new(Expr::Integer(3)),
            ),
            Parser::new(Lexer::new("1*2+3")).expr(),
        );
    }
}
