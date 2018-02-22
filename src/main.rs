// main for hcc
use std::fs::File;

mod file;
mod lex;

fn main() {
    println!("{}",file::file_to_str("lexer.rs"));
    
}

