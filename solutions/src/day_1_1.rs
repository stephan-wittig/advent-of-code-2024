use std::io::BufRead;

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    let mut lists = vec![vec![], vec![]];
    
    for line_result in file.lines() {
        let line = line_result?;
        for (i, location) in line.split("   ").enumerate() {
            lists[i].push(location.parse::<i32>().unwrap());
        }
    }

    for list in lists {
        println!("First element: {}", list[0])
    }

    Ok(())
}