// functions to load files

use std::fs::File;
use std::fs::{metadata,Metadata};
use std::io::prelude::*;

pub fn file_to_bytes(file_path: &str) -> Vec<u8> {
    let mut f = File::open(file_path).expect("i can't find that file...");
    let mut file_contents: u8 = Vec::with_capacity(metadata(file_path)
                                                   .expect("failed to load file metadata...").len() as usize);
    f.read_to_end(&mut file_contents)
        .expect("i can't read that file...");

    file_contents
}
