use std::io::BufRead;

const FIND: &str = "XMAS";

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    let find_len = FIND.chars().count();
    // Read lines and unwrap
    let lines: Vec<String> = file.lines().map(|l| l.unwrap()).collect();
    let dimensions = (lines.len(), lines[0].chars().count());

    let test_direction = |pos: (usize, usize), dir: (i32, i32), lines: Vec<String>| -> bool {
        let end_pos: (i32, i32) = (pos.0 as i32 + ((find_len as i32 - 1) * dir.0), pos.1 as i32 + (find_len as i32 - 1) * dir.1);
        // Return early if word exceed bounds
        println!("End position: {:?}", end_pos);
        if !(end_pos.0 >= 0 &&
            end_pos.1 >= 0 &&
            end_pos.0 < dimensions.0 as i32 &&
            end_pos.1 < dimensions.1 as i32)
        {
            return false;
        }

        // Skip first letter because it was already checked
        let find_iter = FIND.char_indices().skip(1);

        for (i, c) in find_iter {
            let indices: (i32, i32) = (pos.0 as i32 + dir.0 * i as i32, pos.1 as i32 + dir.1 * i as i32);

            if lines[indices.0 as usize].chars().nth(indices.1 as usize).unwrap() != c {
                return false;
            }
        }

        return true;
    };


    let get_matches = |pos: (usize, usize), lines: Vec<String>| -> i32 {
        println!("Checking pos {:?}", pos);
        // Valid directions
        let directions = vec![
            // 'Normal': Top-to-bottom and Left-to-right
            (1, 0),
            (0, 1),
            // Reverse of normal
            (-1, 0),
            (0, -1),
            // Diagonal
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1)
        ];
        // Multiple matches might start at the same point
        let mut matches: i32 = 0;

        if lines[pos.0].chars().nth(pos.1).unwrap() != FIND.chars().nth(0).unwrap() {
            return matches;
        }

        for direction in directions {
            if test_direction(pos, direction, lines.clone()) {
                matches += 1;
            }
        }

        return matches;
    };

    let mut sum: i32 = 0;

    // Go through all lines and chars and count how many matches FIND start there
    for (y, l) in lines.iter().enumerate() {
        for (x, _) in l.chars().enumerate() {
            sum += get_matches((y, x), lines.clone());
        }
    }

    println!("Sum of XMAS {}", sum);

    Ok(())
}