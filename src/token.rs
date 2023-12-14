#[derive(Debug, PartialEq, Clone)]

pub struct Span {
    file_name: String,
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(file_name: String, start: usize, end: usize) -> Self {
        Span {
            file_name,
            start,
            end,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Integer,
    Float,
    Identifier,
    OpenParan,
    CloseParan,
    OpenBrace,
    CloseBrace,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    NotEqual,
    Lthan,
    Gthan,
    Lethan,
    Gethan,
    Bang,
    Assign,
    Colon,
    KeywordsBegin,
    Let,
    If,
    Else,
    While,
    Fn,
    KeywordsEnd,
    Bad,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    loc: Span,
    kind: TokenKind,
}
impl Token {
    pub fn new(loc: Span, kind: TokenKind) -> Self {
        Token { loc, kind }
    }
    pub fn get_kind(&self) -> &TokenKind {
        &self.kind
    }
    pub fn get_span(&self) -> (usize, usize) {
        (self.loc.start, self.loc.end)
    }
}
