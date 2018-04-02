// functions to load files

use std::fs::File;
use std::fs::metadata;
use std::io::prelude::*;

pub fn file_to_chars(file_path: &str) -> Vec<char> {
    let f = File::open(file_path).expect("i can't find that file...");
    let metadata = metadata(file_path).expect("i can't find that file...");

    let file_contents: Vec<char> = {
        let mut vec: Vec<char> = Vec::with_capacity(metadata.len() as usize);
        for c in f.chars() {
            vec.push(c.unwrap());
        }
        vec
    };

    file_contents
}

