use core::fmt;
use std::{collections::HashMap, fmt::Display, io::{BufRead, Lines}, ops::Add};

/// Postion as (Row, Column) within the map
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position (isize, isize);

impl Add for Position {
    type Output = Self;
    
    fn add(self, pos: Position) -> Self {
        Self(self.0 + pos.0, self.1 + pos.1)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

const UP: Position = Position(-1, 0);
const RIGHT: Position = Position(0, 1);
const DOWN: Position = Position(1, 0);
const LEFT: Position = Position(0, -1);

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    // Read lines
    let mut lines = file.lines();

    let patrol = Patrol::new(&mut lines);

    let mut count = 0;
    
    for (i, (pos, was_visited)) in patrol.enumerate() {
        println!("Step {} at {}{}", i, pos, if !was_visited {" new!"} else {""});
        if !was_visited {
            count += 1;
        }
    }
    
    println!("The number of visited fields is {}", count);

    Ok(())
}

struct Patrol {
    _fields: HashMap<Position, char>,
    _visited: HashMap<Position, bool>,
    _guard_pos: Position,
    _guard_dir: Position
}

impl Patrol {
    /// Parses map from reader
    fn new(lines_reader: &mut Lines<Box<dyn BufRead>>) -> Self {
        let mut _fields: HashMap<Position, char> = HashMap::new();
        let mut _guard_pos: Option<Position> = None;

        for (row, line) in lines_reader.map(|l| l.unwrap()).enumerate() {
            for (col, c) in line.chars().enumerate() {
                _fields.insert(Position(row as isize, col as isize), c);
                if c == '^' {
                    _guard_pos = Some(Position(row as isize, col as isize));
                }
            }
        }

        Self {
            _fields,
            _visited: HashMap::new(),
            _guard_pos: _guard_pos.unwrap(),
            _guard_dir: UP
        }
    }

    /// Returns char at pos or None if position is outside map
    fn get_char_at(&self, pos: Position) -> Option<char> {
        self._fields.get(&pos).copied()
    }

    fn _turn (&mut self) -> () {
        self._guard_dir = match self._guard_dir {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            _ => {
                panic!("_guard_dir is not a valid direction!");
            }
        };
    }

    /// Turn according to rules until next field is unobstructed. 
    /// This might be an infinite loop if the map is invalid.
    fn turn_until_unobstructed (&mut self) -> () {
        while 
            match self.get_char_at(self._guard_pos + self._guard_dir) {
                Some('#') => {
                    self._turn();
                    true
                }
                _ => false,
            }
        {}
    }

    /// Marks current position as visited and returns whether it was visited before
    fn _visit (&mut self) -> bool {
        match self.get_char_at(self._guard_pos) {
            None => {
                panic!("Current position is invalid");
            },
            Some('#') => {
                panic!("Current position is obstructed");
            },
            _ => ()
        }

        self._visited.insert(self._guard_pos, true).unwrap_or(false)
    }
}

impl Iterator for Patrol {
    type Item =  (Position, bool);

    /// Advances guard to the next position. Returns whether new position is new
    fn next(&mut self) -> Option<(Position, bool)> {
        self.turn_until_unobstructed();
        // Return first position without making a step at start
        if self._visited.is_empty() {
            return Some((self._guard_pos, self._visit()));
        }

        self._guard_pos = self._guard_pos + self._guard_dir;

        match self.get_char_at(self._guard_pos) {
            None => None,
            _ => Some((self._guard_pos, self._visit()))
        }
    }
}