use core::fmt;
use std::{collections::HashMap, fmt::Display, io::{BufRead, Lines}};
use std::ops::{Add, Mul, Sub};
use itertools::Itertools;

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


pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = file.lines();

    let plan = Plan::new(&mut lines);
    let antinodes = plan.get_antinodes();

    // println!("Antinodes: {:#?}", antinodes);
    
    println!("There are {} antinodes", antinodes.len());

    Ok(())
}

struct Plan {
    fields: HashMap<Position, char>,
    frequencies: HashMap<char, Vec<Position>>
}

impl Plan {
    /// Parses plan from reader
    fn new(lines_reader: &mut Lines<Box<dyn BufRead>>) -> Self {
        let mut fields: HashMap<Position, char> = HashMap::new();
        let mut frequencies: HashMap<char, Vec<Position>> = HashMap::new();

        for (row, line) in lines_reader.map(|l| l.unwrap()).enumerate() {
            for (col, c) in line.chars().enumerate() {
                let pos = Position(row as isize, col as isize);
                fields.insert(pos, c);
                if c != '.' {
                    let antennas = frequencies.entry(c).or_insert(vec![]);
                    //println!("Antennas read: {:?}", antennas);
                    antennas.push(pos);
                }
            }
        }

        Self {
            fields,
            frequencies
        }
    }

    /// Returns char at pos or None if position is outside map
    fn get_char_at(&self, pos: Position) -> Option<char> {
        self.fields.get(&pos).copied()
    }

    fn is_valid_pos (&self, pos: Position) -> bool {
        self.get_char_at(pos).is_some()
    }

    fn get_antinodes (&self) -> Vec<Position> {
        let mut antinodes: Vec<Position>  = vec![];

        // println!("Frequencies: {:#?}", self.frequencies.len());

        for (frequency, antennas) in self.frequencies.iter() {
            //println!("Frequency: {:?} {:?}", frequency, antennas);
            for antenna_pair in antennas.iter().combinations(2) {
                // println!("Checking combination: {:?}", antennas);
                let [&first, &second] = antenna_pair[..2] else {
                    panic!("Couldn't match two antennas");
                };
                let distance = second - first;
                // Check first possible spot
                let possible_antinodes =  vec![
                    second + distance,
                    first - distance
                ];

                for possible_antinode in possible_antinodes {
                    if self.is_valid_pos(possible_antinode) {
                        //println!("Combination is valid!");
                        antinodes.push(possible_antinode);
                    }
                }
            }
        }

        return antinodes.into_iter().unique().collect::<Vec<Position>>();
    }
}
