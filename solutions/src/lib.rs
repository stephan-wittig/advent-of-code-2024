use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Enumerate;
use std::ops::{Add, Mul, Sub};

/// Opens files. Defaults to puzzle of the day
pub fn open(filename: &str, day: u16) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        // Uses default file if no other provided
        "-" => Ok(Box::new(BufReader::new(File::open(format!("../puzzles/day_{}.txt", day))?))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

/// # Position
///
/// Struct representing a position as (Row, Column) in 2D space. Axis are swapped as files are read row-wise.
/// 
/// Implements Addition and Substraction as well as Multiplication with numbers.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position (isize, isize);

impl Add for Position {
    type Output = Self;
    
    fn add(self, pos: Position) -> Self {
        Self(self.0 + pos.0, self.1 + pos.1)
    }
}

impl Mul<isize> for Position {
    type Output = Self;

    fn mul(self, n: isize) -> Self::Output {
        Self(self.0 * n, self.1 * n)
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, pos: Self) -> Self::Output {
        Self(self.0 - pos.0, self.1 - pos.1)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Directions

/// Can be added to a Position to move up
const UP: Position = Position(-1, 0);
/// Can be added to a Position to move righ
const RIGHT: Position = Position(0, 1);
/// Can be added to a Position to move down
const DOWN: Position = Position(1, 0);
/// Can be added to a Position to move left
const LEFT: Position = Position(0, -1);

/// # Plane
/// 
/// Struct representing a 2D space
struct Plane<'a> {
    map: HashMap<Position, char>,
    iter: PlaneIterable<'a>
}

impl<'a> Plane<'a> {
    fn read_file(lines_reader: Lines<Box<dyn BufRead>>) -> Self {
        let mut map: HashMap<Position, char> = HashMap::new();
        Self {
            map,
            iter: PlaneIterable::read_file(lines_reader, &mut map)
        }
    }

    fn read_file_complete(lines_reader: Lines<Box<dyn BufRead>>) -> Self {
        let new_self = Self::read_file(lines_reader);
        println!("Read {} fields", new_self.count());
        return new_self;
    }
}

struct PlaneIterable<'a> {
    lines_reader: Enumerate<std::vec::IntoIter<String>>,
    chars_reader: Enumerate<std::vec::IntoIter<char>>,
    row_idx: usize,
    parent_map: &'a mut HashMap<Position, char>
}

impl<'a> PlaneIterable<'a> {
    fn read_file(lines_reader: Lines<Box<dyn BufRead>>, parent: &'a mut HashMap<Position, char>) -> Self {
        let lines = lines_reader.map(|l| l.unwrap()).collect::<Vec<String>>();
        let mut unw_lines = lines.into_iter().enumerate();
        let (row, first_line) = match unw_lines.next() {
            None => {
                panic!("File reader is empty");
            }
            Some(r) => r
        };
        Self {
            parent_map: parent,
            lines_reader: unw_lines,
            chars_reader: first_line.chars().collect::<Vec<char>>().into_iter().enumerate(),
            row_idx: row
        }
    }
}

impl Iterator for PlaneIterable<'_> {
    type Item = (Position, char);

    /// Reads next field and enters it in map
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars_reader.next() {
                Some((col, c)) => {
                    let new_pos = Position(self.row_idx as isize, col as isize);
                    self.parent_map.insert(new_pos, c);
                    return Some((new_pos, c));
                }
                None => {
                    // Go to next line if None is read
                    // Lines without chars are skipped
                    match self.lines_reader.next() {
                        None => {
                            // If last line, return None
                            return None;
                        }
                        Some((row, line)) => {
                            // Set new chars_reader and let loop handle next char read
                            self.row_idx = row;
                            self.chars_reader = line.chars().collect::<Vec<char>>().into_iter().enumerate();
                        }
                    }
                }
            }
        }
    }
}