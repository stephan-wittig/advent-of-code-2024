use std::{collections::VecDeque, io::BufRead};
use itertools::repeat_n;

/// Block can either be a file id (usite) or Empty
#[derive (Clone)]
enum Block {
    File(usize),
    Empty
}

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    let mut expanded_fs: VecDeque<Block> = VecDeque::new();
    
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
                let mut file: VecDeque<Block> = repeat_n(
                    Block::File(file_index),
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
                let mut file: VecDeque<Block> = repeat_n(
                    Block::Empty,
                    c.to_string().parse::<usize>().unwrap()
                ).collect();
                expanded_fs.append(&mut file);
            }
        }
    }

    let mut cleaned_fs: Vec<usize> = vec![];

    while let Some(next_block) = expanded_fs.pop_front() {
        match next_block {
            Block::Empty => {
                if let Some(last_non_space) = get_last_non_space(&mut expanded_fs) {
                    cleaned_fs.push(last_non_space);
                } else {
                    break; // End loop if empty
                }
            }
            Block::File(file) => {
                cleaned_fs.push(file);
            }
        }
    }

    let checksum = cleaned_fs.iter().enumerate().fold( 0,
         | acc, (i, n)| 
        acc + i * n.to_string().parse::<usize>().unwrap()
    );

    println!("\nChecksum: {}", checksum);

    Ok(())
}

/// Get last chat that is not space ('.'). Drops all spaces at the end
fn get_last_non_space(vd: &mut VecDeque<Block>) -> Option<usize> {
    loop {
        let block = vd.pop_back();
        match block {
            Some(Block::Empty) => {
                continue;
            },
            None => { // None means the string is empty
                return None; 
            },
            Some(Block::File(file)) => {
                return Some(file);
            }
        }
    }
}
