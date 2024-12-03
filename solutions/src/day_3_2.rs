use std::io::BufRead;
use regex::Regex;


pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_re = Regex::new(r"(?:^|do\(\))(.+?)(?:$|don't\(\))").unwrap();
    
    // Read lines, unwrap and join into one string
    let input: String = file.lines().map(|l| l.unwrap()).collect::<Vec<String>>().concat();
    // Removes 'don't' sequences
    let valid_input: String = do_re.captures_iter(&input).map(|c| c.extract::<1>().1[0]).collect();
    
    println!("Valid string:  {}", valid_input);

    // Find mul instructions in string
    let sum = mul_re.captures_iter(&valid_input)
        // For each capture, extract, parse and multiply factors
        .map(|c| {
            let (_, factors_str) = c.extract::<2>();
            let  factors = factors_str.map(|f| f.parse::<i32>().unwrap());
            return factors[0] * factors[1];
        }).sum::<i32>();

    println!("Sum of products is {}", sum);

    Ok(())
}