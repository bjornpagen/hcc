// main for hcc

#![feature(io)]

extern crate regex;

use std::env;

mod load;
mod preprocessing;

fn main() {
    let args: Vec<String> = env::args().collect();
}

