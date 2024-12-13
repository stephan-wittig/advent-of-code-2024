use std::{collections::HashMap, io::BufRead};

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    let stones: Vec<i64> = file.lines().next().unwrap()?
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // For each stone number, record how many there are
    let mut stones_map: HashMap<i64, i64> = HashMap::new();

    for stone in stones {
        let stone_count = stones_map.entry(stone).or_insert(0);
        *stone_count = *stone_count + 1;
    }

    for i in 1..76 {
        let mut new_map: HashMap<i64, i64> = HashMap::new();
        for stone_index  in stones_map.clone().keys() {
            let stones_count = stones_map.remove(stone_index).unwrap();
            let new_indices = blink(*stone_index);
            for stone_index in new_indices {
                let new_stones_count = new_map.entry(stone_index).or_insert(0);
                *new_stones_count += stones_count
            }
        }
        print!("\rBlinked {} times", i);
        stones_map = new_map;
    }

    let total_stones_count: i64 = stones_map.values().sum();
    println!("\nCounted {} stones", total_stones_count);


    Ok(())
}

fn blink(stone: i64) -> Vec<i64> {
    if stone == 0 {
        return vec![1];
    }

    let mut stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        return vec![
            stone_str.split_off(stone_str.len()/2).parse::<i64>().unwrap(),
            stone_str.parse::<i64>().unwrap()
        ];
    }

    return vec![stone * 2024];
}