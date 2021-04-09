use nom::{
    // bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map_res,
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Token {
    Num(u32),
    Plus,
    Minus,
    Star,
    Slash,
}

pub struct Lexer<'a> {
    src: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &str) -> Lexer {
        Lexer { src: src }
    }

    /// 字句解析が終了している場合にtrueを返す。
    pub fn is_end(&self) -> bool {
        self.src.len() == 0
    }

    /// 引数として与えられた文字をパースできる場合にtrueを返し、文字列を読み進める。
    /// そうでない場合はfalseを返す。
    pub fn expect_char(&mut self, expect: char) -> bool {
        let result: IResult<&str, char> = char(expect)(self.src);

        match result {
            Ok((remain, _)) => {
                self.src = remain;
                return true;
            }

            Err(_) => {
                return false;
            }
        }
    }

    /// 数をパースできる場合にその数字を返し、文字列を読み進める。
    pub fn read_num(&mut self) -> Option<u32> {
        let from_str = |s: &str| u32::from_str_radix(s, 10);
        let result: IResult<&str, u32> = map_res(digit1, from_str)(self.src);

        match result {
            Ok((remain, result)) => {
                self.src = remain;
                return Some(result);
            }
            Err(_) => {
                return None;
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.is_end() {
            return None;
        }

        if let Some(num) = self.read_num() {
            return Some(Token::Num(num));
        }

        if self.expect_char('+') {
            return Some(Token::Plus);
        } else if self.expect_char('-') {
            return Some(Token::Minus);
        } else if self.expect_char('*') {
            return Some(Token::Star);
        } else if self.expect_char('/') {
            return Some(Token::Slash);
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

    #[test]
    fn test_read_num() {
        let mut lexer = Lexer::new("123abc");
        assert_eq!(Some(123), lexer.read_num());
        assert_eq!(None, lexer.read_num());
    }

    #[test]
    fn test_expect_char() {
        let mut lexer = Lexer::new("ab");
        assert_eq!(true, lexer.expect_char('a'));
        assert_eq!(false, lexer.expect_char('a'));
        assert_eq!(true, lexer.expect_char('b'));
        assert_eq!(false, lexer.expect_char('b'));
    }

    #[test]
    fn test_is_end() {
        let mut lexer = Lexer::new("a");

        assert_eq!(false, lexer.is_end());
        let _ = lexer.expect_char('a');
        assert_eq!(true, lexer.is_end());
    }

    fn assert_eq_lexer(src: &str, expected: &Vec<Token>) {
        let result: Vec<_> = Lexer::new(src).into_iter().collect();
        assert_eq!(*expected, result);
    }

    #[test]
    fn test_lexer() {
        assert_eq_lexer("1+2", &vec![Token::Num(1), Token::Plus, Token::Num(2)]);
        assert_eq_lexer("1-2", &vec![Token::Num(1), Token::Minus, Token::Num(2)]);
        assert_eq_lexer("1*2", &vec![Token::Num(1), Token::Star, Token::Num(2)]);
        assert_eq_lexer("1/2", &vec![Token::Num(1), Token::Slash, Token::Num(2)]);

        assert_eq_lexer("1*2+3", &vec![Token::Num(1), Token::Star, Token::Num(2), Token::Plus, Token::Num(3)]);
    }
}
