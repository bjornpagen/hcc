use std::fs::File;
use std::io::prelude::*;

pub fn file_to_str(file_path: &str) -> String {
    let mut f = File::open(file_path).expect("i can't find that file...");
    let mut file_contents = String::new();
    f.read_to_string(&mut file_contents)
        .expect("i can't read that file...");

    file_contents
}
