mod lexer;
mod token;

use std::env;
use std::fs;

use lexer::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(args.get(1).expect("Please enter a valid file."))
        .expect("Please enter a valid file.");

    println!("{}", input);

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    println!("{:?}", tokens);
}
