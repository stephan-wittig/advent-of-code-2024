use std::io::BufRead;

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    let mut stones: Vec<i64> = file.lines().next().unwrap()?
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    for i in 1..26 {
        stones = stones.iter().flat_map(|s| blink(*s)).collect();
        print!("\rSaw {} stones after blinking {} time(s)", stones.len(), i);
    }

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