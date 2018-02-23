use std::fs::File;
use std::fs::metadata;
use std::fs::Metadata;
use std::io::prelude::*;

pub struct ProgramBuffer {
    contents: String,
    position: u64,
}

impl ProgramBuffer {
    pub fn new() -> ProgramBuffer {
        ProgramBuffer { contents: String::new(), position: 0 }
    }
    
    pub fn from(filepath: &str) -> ProgramBuffer {
        ProgramBuffer { contents: file_to_str(filepath), position: 0 }
    }
}

pub fn file_to_str(file_path: &str) -> String {
    let mut f = File::open(file_path).expect("i can't find that file...");
    let mut file_contents = String::with_capacity(metadata(file_path)
                                                  .expect("failed to load file metadata...").len() as usize);
    f.read_to_string(&mut file_contents)
        .expect("i can't read that file...");

    file_contents
}
