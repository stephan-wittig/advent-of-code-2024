use std::io::BufRead;
use regex::Regex;


pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    // Matches mul statements and captures factors
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    
    // Read lines and unwrap
    let sum: i32 = file.lines().map(|l| l.unwrap())
        // In each line, capture all matches
        .map(|l| mul_re.captures_iter(&l)
            // For each capture, extract, parse and multiply factors
            .map(|c| {
                let (_, factors_str) = c.extract::<2>();
                let  factors = factors_str.map(|f| f.parse::<i32>().unwrap());
                return factors[0] * factors[1];
            }).sum::<i32>()
        ).sum();

    println!("Sum of products is {}", sum);

    Ok(())
}