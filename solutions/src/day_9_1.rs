use std::{collections::VecDeque, io::BufRead};
use itertools::repeat_n;

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    let mut expanded_fs: VecDeque<char> = VecDeque::new();
    
    // This should be just one line
    let input = file.lines().next().unwrap().unwrap();

    // Expand file system
    let mut input_iter = input.chars();
    let mut file_index: usize = 0;

    loop {
        // First read file
        match input_iter.next() {
            None => {
                break
            }
            Some(c) => {
                // Vector with file_index repeated c times
                let mut file: VecDeque<char> = repeat_n(
                    match file_index.to_string().chars().nth(0) {
                        None => {panic!("Cannot stringify file index")}
                        Some(c) => c
                    },
                    c.to_string().parse::<usize>().unwrap()
                ).collect();
                expanded_fs.append(&mut file);
                file_index += 1;
            }
        }
        // Next read space to next file
        match input_iter.next() {
            None => {
                break;
            }
            Some(c) => {
                // Vector with '.' repeated c times
                let mut file: VecDeque<char> = repeat_n(
                    '.',
                    c.to_string().parse::<usize>().unwrap()
                ).collect();
                expanded_fs.append(&mut file);
            }
        }
    }

    println!("Expanded file system: {:?}", expanded_fs.iter().collect::<String>());

    let mut cleaned_fs: Vec<char> = vec![];

    while let Some(next_char) = expanded_fs.pop_front() {
        if next_char == '.' {
            if let Some(last_non_space) = get_last_non_space(&mut expanded_fs) {
                cleaned_fs.push(last_non_space);
            } else {
                break; // End loop if empty
            }
        } else {
            cleaned_fs.push(next_char);
        }
    }

    println!("Cleaned file system: {:?}", cleaned_fs.iter().collect::<String>());

    let checksum = cleaned_fs.iter().enumerate().fold( 0 as usize,
         | acc, (i, n)| 
        acc + i * n.to_string().parse::<usize>().unwrap()
    );

    println!("Checksum: {}", checksum);

    Ok(())
}

/// Get last chat that is not space ('.'). Drops all spaces at the end
fn get_last_non_space(vd: &mut VecDeque<char>) -> Option<char> {
    loop {
        let c = vd.pop_back();
        match c {
            Some('.') => {
                continue;
            },
            r => {
                return r; // None means the string is empty
            }
        }
    }
}
