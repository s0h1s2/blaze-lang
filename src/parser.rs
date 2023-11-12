use crate::tokenizer::Lexer;

pub struct Parser<'lexer> {
    lex: &'lexer mut Lexer<'lexer>,
}
impl<'lexer> Parser<'lexer> {
    pub fn new(lex: &'lexer mut Lexer<'lexer>) -> Parser {
        return Parser { lex };
    }
    pub fn parse(&mut self) {
        let mut kind = self.lex.next_token();
        println!("{:?}", kind);
        kind = self.lex.next_token();
        println!("{:?}", kind);
        kind = self.lex.next_token();
        println!("{:?}", kind);
        // (filename:col):unable to parse '+'
        // let a:i8=1;
    }
}
