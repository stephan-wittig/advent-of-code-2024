use std::io::BufRead;

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    let mut lists = vec![vec![], vec![]];
    
    for line_result in file.lines() {
        let line = line_result?;
        for (i, location) in line.split("   ").enumerate() {
            lists[i].push(location.parse::<i32>().unwrap());
        }
    }

    let sum: i32 = lists[0].iter().map(|i| lists[1].iter().filter(|j| i == *j).count() as i32  * i).sum();

    println!("Sum of products: {}", sum);

    Ok(())
}