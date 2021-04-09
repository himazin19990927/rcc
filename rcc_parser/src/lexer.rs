use nom::{
    character::complete::{char, digit1},
    combinator::map_res,
    IResult,
};
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

#[cfg(test)]
mod tests {
    use super::Lexer;

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
}