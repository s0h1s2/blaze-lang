use blaze::tokenizer::Lexer;
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
    let mut lex = Lexer::new(file_path, &file_source);
    let token = lex.next_token();
    println!("{:?}", token)
}
