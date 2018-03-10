// main for hcc

extern crate hcc;

use hcc::lex::*;
use hcc::io::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let chars = file_to_chars(args[1].as_str());
    let token_list = lex(&chars);

    for tok in token_list {
        println!("{:?}", tok);
    }
}

