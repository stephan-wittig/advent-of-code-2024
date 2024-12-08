use core::fmt;
use std::{collections::HashMap, fmt::Display, io::{BufRead, Lines}, ops::Add};

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    // Read lines
    let mut lines = file.lines();
    let patrol_prototype = Patrol::new(&mut lines);
    let mut possible_obstructions: HashMap<Position, bool> = HashMap::new();
    // Mark starting position an impossible
    possible_obstructions.insert(patrol_prototype._guard_pos, false);

    // For all positions in path place obstancle and test if it's a loop
    for (pos, _) in patrol_prototype.clone() {
        match possible_obstructions.get(&pos) {
            Some(_) => {
                // Skip if position was already tested
                continue;
            }
            None => {
                let mut possible_loop = patrol_prototype.clone();
                possible_loop.place_obstacle(pos);
                possible_obstructions.insert(pos, possible_loop.is_loop());
            }
        }
    }
    
    println!("{} patrols are loops", possible_obstructions.into_values().filter(|v| *v).count());

    Ok(())
}

#[derive(Clone)]
struct Patrol {
    _fields: HashMap<Position, char>,
    /// Stores Positions and directions
    _visited: HashMap<Position, HashMap<Position, bool>>,
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

        // Gets map directions -> visited or initialises new map
        let dir_map = self._visited.entry(self._guard_pos).or_insert(HashMap::new());
        let was_visited = dir_map.insert(self._guard_dir, true);
        return was_visited.unwrap_or(false);
    }

    /// Note this destroy the object
    fn is_loop(&mut self) -> bool {
        self.any(|(_, starts_loop)| starts_loop)
    }

    fn place_obstacle(&mut self, pos: Position) -> () {
        match self.get_char_at(self._guard_pos) {
            None => {
                panic!("Position is invalid");
            },
            Some('#') => {
                panic!("Position is already obstructed");
            },
            _ => ()
        }

        self._fields.insert(pos, '#');
    }
}

impl Iterator for Patrol {
    type Item =  (Position, bool);

    /// Advances guard to the next position. Returns whether new position starts a loop
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