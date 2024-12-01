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

    lists.iter_mut().for_each(|l| l.sort());

    let sum: i32 = zip(&lists[0], &lists[1]).map(|(i, j)| (i - j).abs()).sum();

    println!("Sum of distances: {}", sum);

    Ok(())
}