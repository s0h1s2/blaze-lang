use std::borrow::BorrowMut;

use crate::token::{Span, Token, TokenKind};
use phf::phf_map;
pub struct Lexer<'a> {
    source: Vec<char>,
    file_name: &'a str,
    current_pos: usize,
    end: usize,
    start: usize,
    current_token: Option<Token>,
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
            start: 0,
            end: 0,
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
    }
    fn make_token(&mut self, kind: TokenKind) -> Token {
        self.end = self.current_pos;
        let token = Token::new(
            Span::new(self.file_name.to_string(), self.start, self.end),
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
        return TokenKind::Identifier;
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
        self.start = self.current_pos;
        let char_op = self.get_char();
        if self.is_end() {
            return self.make_token(TokenKind::EOF);
        }
        let char = char_op.unwrap();
        match char {
            '0'..='9' => {
                self.parse_integer();
                return self.make_token(TokenKind::Integer);
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
                token
            }
            ')' => {
                let token = self.make_token(TokenKind::CloseParan);
                self.advance();
                token
            }
            '{' => {
                let token = self.make_token(TokenKind::OpenBrace);
                self.advance();
                token
            }
            '}' => {
                let token = self.make_token(TokenKind::CloseBrace);
                self.advance();
                token
            }
            _ => {
                let token = self.make_token(TokenKind::Bad);
                self.advance();
                token
            }
        }
    }
    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut token = self.next_token();
        while token.borrow_mut().get_kind() != &TokenKind::EOF {
            tokens.push(token);
            token = self.next_token();
        }
        tokens
    }
}
