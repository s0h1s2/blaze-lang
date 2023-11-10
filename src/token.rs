#[derive(Debug, PartialEq)]
pub struct Location {
    file_name: String,
    line: u64,
    col: u64,
}

impl Location {
    pub fn new(file_name: String, line: u64, col: u64) -> Self {
        return Location {
            file_name,
            line,
            col,
        };
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Integer(u64),
    Float(f64),
    Identifier(String),
    OpenParan,
    CloseParan,
    OpenBrace,
    CloseBrace,
    Plus,
    Minus,
    Star,
    Slash,
    EOF,
}
#[derive(Debug, PartialEq)]
pub struct Token {
    loc: Location,
    kind: TokenKind,
}
impl Token {
    pub fn new(loc: Location, kind: TokenKind) -> Self {
        return Token { loc, kind };
    }
    pub fn get_kind(&self) -> &TokenKind {
        return &self.kind;
    }
}
