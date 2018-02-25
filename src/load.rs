// functions to load files

use std::fs::File;
use std::fs::{metadata,Metadata};
use std::io;
use std::io::prelude::*;

pub fn file_to_bytes(file_path: &str) -> Vec<char> {
    let mut f = File::open(file_path).expect("i can't find that file...");
    let file_contents: Vec<char> = f.chars().enumerate().collect();
    file_contents
}
