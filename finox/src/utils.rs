use std::error::Error;
use std::fs::OpenOptions;
use std::{thread, time};

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn read_tickers(filename: impl AsRef<Path>) -> Vec<String> {
    let f = File::open(filename).expect("no such file");
    let buf = BufReader::new(f);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}