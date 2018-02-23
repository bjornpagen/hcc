// main for hcc

mod load;
mod lex;

fn main() {
    println!("{}",load::file_to_str("lex.rs"));
}

