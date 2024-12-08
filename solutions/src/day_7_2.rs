use std::{io::BufRead, iter::zip};
use regex::Regex;
use itertools::{repeat_n, Itertools};

#[derive(Clone)]
enum Operation {
    ADD,
    MUL,
    CON
}

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    // Read lines
    let sum: i64 = file.lines().map(|r| r.unwrap())
    .map(Equation::new)
    .filter(|eq| eq.is_valid())
    .map(|eq| eq.result)
    .sum();
    
    println!("The sum of valid equations is {}", sum);

    Ok(())
}

struct Equation {
    result: i64,
    factors: Vec<i64>
}

impl Equation {
    /// Parses equation from string
    fn new(str: String) -> Self {
        let eq_re = Regex::new(r"(\d*): ([\d ]*)").unwrap();
        let (_, [result, factors]) = eq_re.captures(&str).unwrap().extract();

        Self {
            result: result.parse::<i64>().unwrap(),
            factors: factors.split(" ").map(|s| s.parse::<i64>().unwrap()).collect()
        }
    }

    fn is_valid(&self) -> bool {
        let len_operators = self.factors.len() - 1;
        let mut possible_operators = repeat_n(
            vec![Operation::ADD, Operation::MUL, Operation::CON], len_operators
        ).multi_cartesian_product();

        possible_operators.any(|o| self.check_operators(o))
    }

    fn check_operators(&self, operators: Vec<Operation>) -> bool {
        let mut result = self.factors[0];
        for (o, n) in  zip(operators, self.factors[1..].iter()) {
            match o {
                Operation::ADD => {
                    result += n;
                }
                Operation::MUL => {
                    result *= n;
                }
                Operation::CON => {
                    result = [result.to_string(), n.to_string()].concat().parse::<i64>().unwrap();
                }
            }

            if result > self.result {
                return false;
            }
        }

        return result == self.result;
    }
}