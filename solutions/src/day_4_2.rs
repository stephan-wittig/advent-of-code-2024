use std::{char, io::BufRead};

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    // Read lines and unwrap
    let lines: Vec<String> = file.lines().map(|l| l.unwrap()).collect();
    let dimensions = (lines.len(), lines[0].chars().count());

    let test_x = |pos: (usize, usize), lines: Vec<String>| -> bool {
        // Return early if char is not A
        if lines[pos.0].chars().nth(pos.1).unwrap() != 'A' {
            return false;
        }
        
        // Return early if x exceeds bounds
        if !(pos.0 >= 1 &&
            pos.1 >= 1 &&
            pos.0 < dimensions.0 - 1 &&
            pos.1 < dimensions.1 - 1)
        {
            return false;
        }

        // Check arms of X, starting at top right, clockwise
        let tl = lines[pos.0 - 1].chars().nth(pos.1 - 1).unwrap();
        let tr = lines[pos.0 - 1].chars().nth(pos.1 + 1).unwrap();
        let br = lines[pos.0 + 1].chars().nth(pos.1 + 1).unwrap();
        let bl = lines[pos.0 + 1].chars().nth(pos.1 - 1).unwrap();

        return is_x(tl, tr, br, bl).unwrap_or_default();
    };

    let mut sum: i32 = 0;

    // Go through all lines and chars and count how many matches FIND start there
    for (y, l) in lines.iter().enumerate() {
        for (x, _) in l.chars().enumerate() {
            if test_x((y, x), lines.clone()) {
                sum += 1;
            }
        }
    }

    println!("Sum of X-MAS {}", sum);

    Ok(())
}

fn get_opposite_arm(c: char) -> Option<char> {
    match c {
        'M' => Some('S'),
        'S' => Some('M'),
        _ => None
    }
}

fn is_x(tl: char, tr: char, br: char, bl: char) -> Option<bool> {
    return Some(get_opposite_arm(tl)? == br && get_opposite_arm(bl)? == tr);
}