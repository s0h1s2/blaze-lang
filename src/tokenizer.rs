use crate::token::{Location, Token, TokenKind};
pub struct Lexer<'a> {
    source: Vec<char>,
    file_name: &'a str,
    current_pos: usize,
    line: u64,
}
impl<'a> Lexer<'a> {
    pub fn new(file_name: &'a str, source: &str) -> Lexer<'a> {
        return Lexer {
            source: source.chars().collect(),
            file_name,
            current_pos: 0,
            line: 0,
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
    fn consume_newline(&mut self) {
        while !self.is_end() {
            let ch = *self.get_char().unwrap();
            if ch == '\n' {
                self.line += 1;
                self.advance();
                continue;
            }
            break;
        }
    }
    fn test_ident(c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }
    fn parse_ident(&mut self) -> String {
        let mut ident = String::new();
        while !self.is_end() {
            let ch = *self.get_char().unwrap();
            if Lexer::test_ident(ch) == true {
                ident.push(ch);
                self.advance();
                continue;
            }
            break;
        }
        return ident;
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
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.consume_newline();
        let char_op = self.get_char();
        if self.is_end() {
            return Token::new(
                Location::new(String::from(self.file_name), self.line, 1),
                TokenKind::EOF,
            );
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
            'a'..='z' | 'A'..='Z' | '_' => {
                let result = self.parse_ident();
                return Token::new(
                    Location::new(String::from("main.bl"), 1, 1),
                    TokenKind::Identifier(result),
                );
            }
            _ => unreachable!(),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::token::{Location, Token, TokenKind};
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
        let mut lex = init_lexer("     \t \t \t      ");
        assert_eq!(*lex.next_token().get_kind(), TokenKind::EOF);
    }
    #[test]
    fn test_newline() {
        let mut lex = init_lexer("\n\n\n");
        assert_eq!(
            lex.next_token(),
            Token::new(Location::new("main.bl".to_string(), 3, 1), TokenKind::EOF)
        );
    }
    #[test]
    fn test_newline_whitespace() {
        let mut lex = init_lexer("  \t       \n\n\n");
        assert_eq!(
            lex.next_token(),
            Token::new(Location::new("main.bl".to_string(), 3, 1), TokenKind::EOF)
        );
    }
    #[test]
    fn test_identifier() {
        let mut lex = init_lexer("abc def my_var _var SomeType _SomeType");
        assert_eq!(
            lex.next_token().get_kind(),
            &TokenKind::Identifier("abc".to_string())
        );
        assert_eq!(
            lex.next_token().get_kind(),
            &TokenKind::Identifier("def".to_string())
        );
        assert_eq!(
            lex.next_token().get_kind(),
            &TokenKind::Identifier("my_var".to_string())
        );
        assert_eq!(
            lex.next_token().get_kind(),
            &TokenKind::Identifier("_var".to_string())
        );
        assert_eq!(
            lex.next_token().get_kind(),
            &TokenKind::Identifier("SomeType".to_string())
        );
        assert_eq!(
            lex.next_token().get_kind(),
            &TokenKind::Identifier("_SomeType".to_string())
        );
    }
}
