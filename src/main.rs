// main for hcc

#![feature(io)]

extern crate regex;

#[macro_use]
extern crate lazy_static;

use std::env;

mod load;
mod preprocessing;

fn main() {
    let args: Vec<String> = env::args().collect();

    let chars = load::file_to_chars(args[1].as_str());

    for c in &chars {
        println!("{}", c);
    }
    
    let token_list = preprocessing::lex(&chars);

    for tok in token_list {
        println!("{:?}", tok);
    }
}

