use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn open(filename: &str, day: u16) -> MyResult<Box<dyn BufRead>> {
    match filename {
        // Uses default file if no other provided
        "-" => Ok(Box::new(BufReader::new(File::open(format!("../puzzles/day_{}.txt", day))?))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
