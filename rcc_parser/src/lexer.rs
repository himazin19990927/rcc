use crate::token::{Token, TokenKind};
use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'a> {
    src: Chars<'a>,
    terminated: bool,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &str) -> Lexer {
        let mut lexer = Lexer {
            src: src.chars(),
            terminated: false,
            ch: ' ',
        };

        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.terminated {
            return Token::new(TokenKind::EOF, self.ch);
        }

        let token = match self.ch {
            '<' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::new(TokenKind::Le, "<=")
                }
                _ => Token::new(TokenKind::Lt, self.ch),
            },

            '=' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::new(TokenKind::EqEq, "==")
                }
                _ => todo!("Lexical analysis of \"==\" is not implemented."),
            },

            '>' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::new(TokenKind::Ge, ">=")
                }
                _ => Token::new(TokenKind::Gt, self.ch),
            },

            '!' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::new(TokenKind::Ne, "!=")
                }
                _ => todo!("Lexical analysis of \"!\" is not implemented."),
            },

            '+' => Token::new(TokenKind::Plus, self.ch),
            '-' => Token::new(TokenKind::Minus, self.ch),
            '*' => Token::new(TokenKind::Star, self.ch),
            '/' => Token::new(TokenKind::Slash, self.ch),
            '(' => Token::new(TokenKind::OpenParen, self.ch),
            ')' => Token::new(TokenKind::CloseParen, self.ch),

            '0'..='9' => return Token::new(TokenKind::Num, self.read_number().unwrap()),

            _ => match self.read_str() {
                Some(word) => {
                    return match word.as_str() {
                        "int" => Token::new(TokenKind::Int, word),
                        _ => Token::new(TokenKind::Identifier, word),
                    }
                }
                None => panic!("cannot tokenize"),
            },
        };

        self.read_char();
        return token;
    }

    pub fn read_char(&mut self) -> char {
        if self.terminated {
            return self.ch;
        }

        match self.src.next() {
            Some(c) => {
                self.ch = c;
            }
            None => {
                self.ch = '\0';
                self.terminated = true;
            }
        }

        self.ch
    }

    pub fn read_number(&mut self) -> Option<String> {
        if self.terminated || !self.ch.is_digit(10) {
            return None;
        }

        let mut num_str = String::from(self.ch);
        loop {
            let c = self.read_char();
            if c.is_digit(10) {
                num_str.push(c);
            } else {
                break;
            }
        }

        Some(num_str)
    }

    pub fn read_str(&mut self) -> Option<String> {
        let is_letter = |c: char| c.is_ascii_alphanumeric() || c == '_';

        if self.terminated || !is_letter(self.ch) {
            return None;
        }

        let mut word = String::from(self.ch);
        loop {
            let c = self.read_char();
            if is_letter(c) {
                word.push(c);
            } else {
                break;
            }
        }

        Some(word)
    }

    pub fn peek_char(&self) -> char {
        let mut chars = self.src.clone();

        match chars.next() {
            Some(c) => c,
            None => '\0',
        }
    }

    pub fn skip_whitespace(&mut self) {
        while !self.terminated && self.ch.is_whitespace() {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token, TokenKind};

    #[test]
    fn read_num() {
        assert_eq!(Some("01".to_string()), Lexer::new("01").read_number());
        assert_eq!(Some("100".to_string()), Lexer::new("100").read_number());
        assert_eq!(Some("1".to_string()), Lexer::new("1+1").read_number());
        assert_eq!(None, Lexer::new("a1").read_number());
    }

    #[test]
    fn read_str() {
        assert_eq!(Some("int".to_string()), Lexer::new("int a").read_str());
        assert_eq!(
            Some("return".to_string()),
            Lexer::new("return 0").read_str()
        );
    }

    fn test_lexer(src: &str, expected: Vec<Token>) {
        let mut lexer = Lexer::new(src);

        let mut tokens: Vec<Token> = Vec::new();

        loop {
            tokens.push(lexer.next_token());

            if tokens.last().unwrap().kind == TokenKind::EOF {
                break;
            }
        }

        assert_eq!(expected, tokens);
    }

    #[test]
    fn tokenize_num() {
        test_lexer(
            "100",
            vec![
                Token::new(TokenKind::Num, "100"),
                Token::new(TokenKind::EOF, '\0'),
            ],
        );

        test_lexer(
            "1+2",
            vec![
                Token::new(TokenKind::Num, "1"),
                Token::new(TokenKind::Plus, '+'),
                Token::new(TokenKind::Num, "2"),
                Token::new(TokenKind::EOF, "\0"),
            ],
        );
    }

    #[test]
    fn tokenize_paren() {
        test_lexer(
            "(",
            vec![
                Token::new(TokenKind::OpenParen, '('),
                Token::new(TokenKind::EOF, "\0"),
            ],
        );
        test_lexer(
            ")",
            vec![
                Token::new(TokenKind::CloseParen, ')'),
                Token::new(TokenKind::EOF, "\0"),
            ],
        );
        test_lexer(
            "(100+1)",
            vec![
                Token::new(TokenKind::OpenParen, '('),
                Token::new(TokenKind::Num, "100"),
                Token::new(TokenKind::Plus, '+'),
                Token::new(TokenKind::Num, "1"),
                Token::new(TokenKind::CloseParen, ')'),
                Token::new(TokenKind::EOF, "\0"),
            ],
        );
    }

    #[test]
    fn tokenize_relational() {
        test_lexer(
            "1==2",
            vec![
                Token::new(TokenKind::Num, "1"),
                Token::new(TokenKind::EqEq, "=="),
                Token::new(TokenKind::Num, "2"),
                Token::new(TokenKind::EOF, "\0"),
            ],
        );

        test_lexer(
            "1!=2",
            vec![
                Token::new(TokenKind::Num, "1"),
                Token::new(TokenKind::Ne, "!="),
                Token::new(TokenKind::Num, "2"),
                Token::new(TokenKind::EOF, "\0"),
            ],
        );

        test_lexer(
            "1<2",
            vec![
                Token::new(TokenKind::Num, "1"),
                Token::new(TokenKind::Lt, "<"),
                Token::new(TokenKind::Num, "2"),
                Token::new(TokenKind::EOF, "\0"),
            ],
        );

        test_lexer(
            "1<=2",
            vec![
                Token::new(TokenKind::Num, "1"),
                Token::new(TokenKind::Le, "<="),
                Token::new(TokenKind::Num, "2"),
                Token::new(TokenKind::EOF, "\0"),
            ],
        );

        test_lexer(
            "1>2",
            vec![
                Token::new(TokenKind::Num, "1"),
                Token::new(TokenKind::Gt, ">"),
                Token::new(TokenKind::Num, "2"),
                Token::new(TokenKind::EOF, "\0"),
            ],
        );

        test_lexer(
            "1>=2",
            vec![
                Token::new(TokenKind::Num, "1"),
                Token::new(TokenKind::Ge, ">="),
                Token::new(TokenKind::Num, "2"),
                Token::new(TokenKind::EOF, "\0"),
            ],
        );
    }

    #[test]
    fn tokenize_str() {
        test_lexer(
            "int a",
            vec![
                Token::new(TokenKind::Int, "int"),
                Token::new(TokenKind::Identifier, "a"),
                Token::new(TokenKind::EOF, "\0"),
            ],
        )
    }
}
