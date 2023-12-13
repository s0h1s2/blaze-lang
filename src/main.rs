use blaze::{sourcetext::SourceText, token::TokenKind, tokenizer::Lexer};
use colored::Colorize;
use std::{env, fs, process};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let program_name = &args[0];
        println!("Usage:{} input-file", program_name);
        println!("No file provided to process");
        process::exit(1);
    }
    let file_path = &args[1];
    let file_source = fs::read_to_string(file_path).expect("Unable to read file.");
    let source_text = SourceText::new(&file_source);
    let mut lex = Lexer::new("main.fs", &file_source);
    let tokens = lex.get_tokens();
    for token in &tokens {
        println!(
            "{:?}:{}",
            token,
            source_text.get_literal(token.get_span().0, token.get_span().1)
        );
        if token.get_kind() == &TokenKind::Bad {
            let (line_no, col, line) = source_text.get_line_by_pos(token.get_span().0).unwrap();
            let spaces = " ".repeat(col + line_no.to_string().len());
            let result = format!(
                "{}|{}\n{}{}\n{}{} {}",
                line_no.to_string().bright_yellow(),
                line,
                spaces,
                "^".bright_purple(),
                spaces,
                "|-->".bright_purple(),
                "Expected type but got ':'".bright_purple()
            );
            println!("{}", result);
        }
    }
}
