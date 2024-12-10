use std::{collections::HashMap, io::{BufRead, Lines}, ops::Add};

/// # Position
///
/// Struct representing a position as (Row, Column) in 2D space. Axis are swapped as files are read row-wise.
/// 
/// Implements Addition and Substraction as well as Multiplication with numbers.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position (isize, isize);

/// # TopMap
/// 
/// Topographical map
struct TopMap {
    map: HashMap<Position, isize>
}

impl Add for Position {
    type Output = Self;
    
    fn add(self, pos: Position) -> Self {
        Self(self.0 + pos.0, self.1 + pos.1)
    }
}

// Directions
const UP: Position = Position(-1, 0);
const RIGHT: Position = Position(0, 1);
const DOWN: Position = Position(1, 0);
const LEFT: Position = Position(0, -1);
const DIRECTIONS: [Position; 4] =
    [UP, RIGHT, DOWN, LEFT];

impl TopMap {
    /// Parses plan from reader
    fn new(lines_reader: &mut Lines<Box<dyn BufRead>>) -> Self {
        let mut map: HashMap<Position, isize> = HashMap::new();

        for (row, line) in lines_reader.map(|l| l.unwrap()).enumerate() {
            for (col, c) in line.chars().enumerate() {
                let pos = Position(row as isize, col as isize);
                map.insert(pos, c.to_digit(10).unwrap() as isize);
            }
        }

        Self {
            map
        }
    }

    fn get_height_at(&self, pos: Position) -> Option<isize> {
        self.map.get(&pos).copied()
    }

    fn is_valid_pos (&self, pos: Position) -> bool {
        self.get_height_at(pos).is_some()
    }

    fn iter_starts(&self) -> impl Iterator<Item = &Position> {
        self.map.iter().filter(|(_, &height)|  height == 0)
            .map(|(k, _)| k)
    }

    fn iter_neighbours(&self, pos: Position) -> impl Iterator<Item = Position> + use<'_> {
        DIRECTIONS.iter().map(move |&d| d + pos)
            .filter(|&p| self.is_valid_pos(p))
    }

    /// Get vector of reachable trailheads
    fn walk_trail(&self, start: Position) -> Vec<Position> {
        match self.get_height_at(start) {
            None => {
                vec![]
            },
            Some(9) => {
                // Return itself if target height is reached
                vec![start]
            }
            Some(height) => {
                // Filter neighbours for suitable next steps
                let next_steps = self.iter_neighbours(start)
                    .filter(|&neighbour| self.get_height_at(neighbour).unwrap() == height + 1);
                // find trails from suitable next steps
                next_steps.flat_map(|next| self.walk_trail(next)).collect()
            }
        }
    }
}

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = file.lines();
    let map = TopMap::new(&mut lines);
    let trailhead_scores = map.iter_starts()
        .map(|&start|
            // Do not deduplicate to get number of paths
            map.walk_trail(start).iter().count()
        );

    let sum: usize = trailhead_scores.sum();

    println!("Sum of trailhead scores is {}", sum);

    Ok(())
}
