use std::{collections::HashMap, io::{BufRead, Lines}};

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    // Read lines
    let mut lines = file.lines();

    let rules = Rules::new(&mut lines);

    let sum: i32 = lines.map(|l| Update::new(l.unwrap()))
        .filter(|update| !update.is_valid(&rules))
        .map(|mut update| update.fix_order(&rules).middle_page())
        .sum();
    
    println!("The sum of the middle page numbers of valid updates is {}", sum);

    Ok(())
}

struct Rules {
    _rules_map: HashMap<i32, Vec<i32>>
}

impl Rules {
    /*
     * Parses all instructions from reader. Advances reader until start of updates
     */
    fn new(lines_reader: &mut Lines<Box<dyn BufRead>>) -> Self {
        let mut _rules_map: HashMap<i32, Vec<i32>>= HashMap::new();

        for line in lines_reader {
            let line = line.unwrap_or("".to_string());
            if line == "" {
                // Stop reading if empty line is found
                break;
            }

            let [predecessor, successor] =
                line.split("|").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>()[..] 
            else {
                panic!("Pattern of rule cannot match");
            };

            let successors =_rules_map.entry(predecessor).or_insert(vec![]);
            successors.push(successor);
        }

        Self {
            _rules_map
        }
    }

    fn successors_of(&self, predecessor: i32) -> Option<&Vec<i32>> {
        let successors = self._rules_map.get(&predecessor);
        successors
    }
}

struct Update {
    _pages: Vec<i32>
}

impl Update {
    fn new(update_line: String) -> Self {
        Self {
            _pages: update_line.split(",").map(|s| s.parse::<i32>().unwrap()).collect()
        }
    }

    fn middle_page(&self) -> i32 {
        if self._pages.len() % 2 != 1 {
            panic!("Number of updated pages is even. There's no middle page!");
        }
        self._pages[(self._pages.len() - 1) / 2]
    }

    fn find_invalid_pair(&self, rules: &Rules) -> Option<(usize, usize)> {
        let pages_iter = self._pages.iter().enumerate().rev();
        // Note this iterates in reverse to check if predecessors should be successors
        for (i, page) in pages_iter {
            match rules.successors_of(*page) {
                None => continue, // No need to check if there are no rules
                Some(sucessors) => {
                    // Loop through actual predecessors
                    let predecessors = &self._pages[..i];
                    for (j, predecessor) in predecessors.iter().enumerate() {
                        if sucessors.contains(predecessor) {
                            // One of the predecessors should be a successor
                            return Some((i, j));
                        }
                    }
                }
            }
        }

        return None;
    }

    fn is_valid(&self, rules: &Rules) -> bool {
        self.find_invalid_pair(rules).is_none()
    }

    fn fix_order(&mut self, rules: &Rules) -> &Self {
        while let Some(invalid_pair) = self.find_invalid_pair(rules) {
            let tmp = self._pages[invalid_pair.0];
            self._pages[invalid_pair.0] = self._pages[invalid_pair.1];
            self._pages[invalid_pair.1] = tmp;
        }

        self
    }
}