use crate::token::{Location, Token, TokenKind};
use phf::phf_map;
pub struct Lexer<'a> {
    source: Vec<char>,
    file_name: &'a str,
    current_pos: usize,
    col: u64,
    current_token: Option<Token>,
    line: u64,
}
static KEYWORDS: phf::Map<&'static str, TokenKind> = phf_map! {
    "if" => TokenKind::If,
    "else" => TokenKind::Else,
    "let" => TokenKind::Let,
    "fn" => TokenKind::Fn,
    "while" => TokenKind::While,
};
impl<'a> Lexer<'a> {
    pub fn new(file_name: &'a str, source: &str) -> Lexer<'a> {
        return Lexer {
            source: source.chars().collect(),
            file_name,
            current_pos: 0,
            line: 0,
            col: 1,
            current_token: None,
        };
    }
    fn get_char(&self) -> Option<&char> {
        return self.source.get(self.current_pos);
    }
    fn is_end(&self) -> bool {
        return self.get_char().is_none();
    }
    fn advance(&mut self) {
        self.current_pos += 1;
        self.col += 1;
    }
    fn make_token(&mut self, kind: TokenKind) -> Token {
        let token = Token::new(
            Location::new(self.file_name.to_string(), self.line, self.col),
            kind,
        );
        self.current_token = Some(token.clone());
        return token;
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
                self.col = 0;
                self.advance();
                continue;
            }
            break;
        }
    }
    fn test_ident(c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }
    fn parse_ident_or_keyword(&mut self) -> TokenKind {
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
        if let Some(kind) = KEYWORDS.get(&ident) {
            return kind.clone();
        }
        return TokenKind::Identifier(ident);
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
    pub fn peek_token(&self) -> Option<&Token> {
        return self.current_token.as_ref();
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        self.consume_newline();
        let char_op = self.get_char();
        if self.is_end() {
            return self.make_token(TokenKind::EOF);
        }
        let char = char_op.unwrap();
        // TODO: write a macro to single token or double token to avoid repetition.
        match char {
            '0'..='9' => {
                let result = self.parse_integer();
                return self.make_token(TokenKind::Integer(result));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let kind = self.parse_ident_or_keyword();
                return self.make_token(kind);
            }
            '+' => {
                let token = self.make_token(TokenKind::Plus);
                self.advance();
                return token;
            }
            '-' => {
                let token = self.make_token(TokenKind::Minus);
                self.advance();
                return token;
            }
            '*' => {
                let token = self.make_token(TokenKind::Star);
                self.advance();
                return token;
            }
            '/' => {
                let token = self.make_token(TokenKind::Slash);
                self.advance();
                return token;
            }
            '!' => {
                self.advance();
                if *self.get_char().unwrap() == '=' {
                    self.advance();
                    return self.make_token(TokenKind::NotEqual);
                }
                let token = self.make_token(TokenKind::Bang);
                return token;
            }
            '=' => {
                self.advance();
                if *self.get_char().unwrap() == '=' {
                    let token = self.make_token(TokenKind::Equal);
                    self.advance();
                    return token;
                }
                let token = self.make_token(TokenKind::Assign);
                return token;
            }
            ':' => {
                self.advance();
                return self.make_token(TokenKind::Colon);
            }
            '<' => {
                self.advance();
                if *self.get_char().unwrap() == '=' {
                    self.advance();
                    return self.make_token(TokenKind::Lethan);
                }
                let token = self.make_token(TokenKind::Lthan);
                return token;
            }
            '>' => {
                self.advance();
                if *self.get_char().unwrap() == '=' {
                    self.advance();
                    return self.make_token(TokenKind::Gethan);
                }
                let token = self.make_token(TokenKind::Gthan);
                return token;
            }

            '(' => {
                let token = self.make_token(TokenKind::OpenParan);
                self.advance();
                return token;
            }
            ')' => {
                let token = self.make_token(TokenKind::CloseParan);
                self.advance();
                return token;
            }
            '{' => {
                let token = self.make_token(TokenKind::OpenBrace);
                self.advance();
                return token;
            }
            '}' => {
                let token = self.make_token(TokenKind::CloseBrace);
                self.advance();
                return token;
            }
            _ => unreachable!("Unhandled character:'{}' or unreachable", char),
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
        let source = "abc def my_var _var SomeType _SomeType";
        let mut lex = init_lexer(source);
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
    #[test]
    fn test_operators() {
        let mut lex = init_lexer("+-*/! = == != < > <= >= :");
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Plus);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Minus);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Star);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Slash);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Bang);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Assign);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Equal);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::NotEqual);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Lthan);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Gthan);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Lethan);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Gethan);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Colon);
    }
    #[test]
    fn test_grouping() {
        let mut lex = init_lexer("(){}");
        assert_eq!(lex.next_token().get_kind(), &TokenKind::OpenParan);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::CloseParan);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::OpenBrace);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::CloseBrace);
    }
    #[test]
    fn test_tokens_togther() {
        let mut lex = init_lexer("1*5+a");
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Integer(1));
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Star);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Integer(5));
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Plus);
        assert_eq!(
            lex.next_token().get_kind(),
            &TokenKind::Identifier("a".to_string())
        );
    }
    #[test]
    fn test_keywords() {
        let mut lex = init_lexer("let if else while  fn");
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Let);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::If);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Else);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::While);
        assert_eq!(lex.next_token().get_kind(), &TokenKind::Fn);
    }
    #[test]
    fn test_peek_token() {
        let mut lex = init_lexer("let");
        lex.next_token();
        assert_eq!(lex.peek_token().unwrap().get_kind(), &TokenKind::Let);
        assert_eq!(lex.peek_token().unwrap().get_kind(), &TokenKind::Let);
        lex.next_token();
        assert_eq!(lex.peek_token().unwrap().get_kind(), &TokenKind::EOF);
        assert_eq!(lex.peek_token().unwrap().get_kind(), &TokenKind::EOF);
    }
}
