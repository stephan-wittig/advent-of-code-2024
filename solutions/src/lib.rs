use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Opens files. Defaults to puzzle of the day
pub fn open(filename: &str, day: u16) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        // Uses default file if no other provided
        "-" => Ok(Box::new(BufReader::new(File::open(format!("../puzzles/day_{}.txt", day))?))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}