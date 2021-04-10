use std::{str::Chars, u32};

#[derive(Debug, PartialEq)]
pub enum Token {
    Num(u32),
    Plus,
    Minus,
    Star,
    Slash,
    EOF,
    Unknown,
}

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
            return Token::EOF;
        }

        let token = match self.ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            c => {
                if c.is_digit(10) {
                    return Token::Num(self.read_number().unwrap());
                }

                unimplemented!();
            }
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

    pub fn read_number(&mut self) -> Option<u32> {
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

        match num_str.parse() {
            Ok(num) => Some(num),
            Err(_) => None,
        }
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
    use super::{Lexer, Token};

    #[test]
    fn read_num() {
        assert_eq!(Some(1), Lexer::new("01").read_number());
        assert_eq!(Some(100), Lexer::new("100").read_number());
        assert_eq!(Some(1), Lexer::new("1+1").read_number());
        assert_eq!(None, Lexer::new("a1").read_number());
    }

    fn lexer_test(src: &str, expected: &Vec<Token>) {
        let mut lexer = Lexer::new(src);

        let mut tokens = Vec::new();
        while tokens.last() != Some(&Token::EOF) {
            println!("{:?}", lexer);
            tokens.push(lexer.next_token());
        }

        assert_eq!(expected, &tokens);
    }

    #[test]
    fn tokenze_num() {
        lexer_test("100", &vec![Token::Num(100), Token::EOF]);

        lexer_test(
            "1+2",
            &vec![Token::Num(1), Token::Plus, Token::Num(2), Token::EOF],
        );
        lexer_test(
            "1-2",
            &vec![Token::Num(1), Token::Minus, Token::Num(2), Token::EOF],
        );
        lexer_test(
            "1*2",
            &vec![Token::Num(1), Token::Star, Token::Num(2), Token::EOF],
        );
        lexer_test(
            "1/2",
            &vec![Token::Num(1), Token::Slash, Token::Num(2), Token::EOF],
        );

        lexer_test(
            " 1 + 2",
            &vec![Token::Num(1), Token::Plus, Token::Num(2), Token::EOF],
        );
        lexer_test(
            " 1 + 2 ",
            &vec![Token::Num(1), Token::Plus, Token::Num(2), Token::EOF],
        );
    }
}
