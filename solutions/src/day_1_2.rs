use std::io::BufRead;
use std::iter::zip;

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    let mut lists = vec![vec![], vec![]];
    
    for line_result in file.lines() {
        let line = line_result?;
        for (i, location) in line.split("   ").enumerate() {
            lists[i].push(location.parse::<i32>().unwrap());
        }
    }

    lists[0].sort();
    lists[1].sort();

    let mut sum = 0;

    for (i, j) in zip(&lists[0], &lists[1]) {
        let distance = (i - j).abs();
        println!("Distance: {}", distance);
        sum += distance;
    }

    println!("Sum of distances: {}", sum);


    Ok(())
}