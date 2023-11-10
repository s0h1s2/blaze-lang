use crate::token::{Location, Token, TokenKind};
pub struct Lexer<'a> {
    source: Vec<char>,
    file_name: &'a str,
    current_pos: usize,
}
impl<'a> Lexer<'a> {
    pub fn new(file_name: &'a str, source: &str) -> Lexer<'a> {
        return Lexer {
            source: source.chars().collect(),
            file_name,
            current_pos: 0,
        };
    }
    fn get_char(&self) -> Option<&char> {
        return self.source.get(self.current_pos);
    }
    fn is_end(&self) -> bool {
        return self.get_char().is_none();
    }
    fn advance(&mut self) {
        self.current_pos += 1
    }
    fn parse_integer(&mut self) -> u64 {
        // let start = self.current_pos;
        let mut result: u64 = 0;
        while !self.is_end() {
            let ch = *self.get_char().unwrap();
            if ch >= '0' && ch <= '9' {
                result = result * 10;
                result = result + (ch as u8 - b'0') as u64;
                self.advance();
                continue;
            }
            break;
        }

        return result;
    }
    fn skip_whitespace(&mut self) {
        while !self.is_end() {
            let ch = self.get_char().unwrap();
            if ch == &' ' || ch == &'\t' {
                self.advance();
                continue;
            }
            break;
        }
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let char_op = self.get_char();
        if self.is_end() {
            return Token::new(Location::new(String::from("main.bl"), 1, 1), TokenKind::EOF);
        }
        let char = char_op.unwrap();
        match char {
            '0'..='9' => {
                let result = self.parse_integer();
                return Token::new(
                    Location::new(String::from("main.bl"), 1, 1),
                    TokenKind::Integer(result),
                );
            }
            _ => unreachable!(),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::token::{Location, TokenKind};
    use crate::tokenizer::Lexer;
    fn get_loc() -> Location {
        return Location::new(String::from("main.bl"), 1, 1);
    }
    fn init_lexer(source: &str) -> Lexer {
        let lexer = Lexer::new("main.bl", source);
        return lexer;
    }
    #[test]
    fn test_integer_token() {
        let mut lex = init_lexer("12 321 1400 133377");
        assert_eq!(*lex.next_token().get_kind(), TokenKind::Integer(12));
        assert_eq!(*lex.next_token().get_kind(), TokenKind::Integer(321));
        assert_eq!(*lex.next_token().get_kind(), TokenKind::Integer(1400));
        assert_eq!(*lex.next_token().get_kind(), TokenKind::Integer(133377));
    }
    #[test]
    fn test_whitespace_and_tab() {
        let mut lex = init_lexer("     \t \t \t");
        assert_eq!(*lex.next_token().get_kind(), TokenKind::EOF);
    }
}
