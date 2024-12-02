use std::io::BufRead;

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    let mut reports: Vec<Vec<i32>> = vec![];
    
    for line_result in file.lines() {
        let line = line_result?;
        reports.push(line.split(" ").map(|s| s.parse::<i32>().unwrap()) .collect());
    }

    let safe_reports = reports.iter().filter(|report| {
        report.windows(2).all(|levels| {
            (levels[0] - levels[1]).abs() < 4
        }) && (
            report.windows(2).all(|levels| {
                (levels[0] - levels[1]) < 0
            }) || report.windows(2).all(|levels| {
                (levels[0] - levels[1]) > 0
            })
        )
    });

    println!("Number of safe reports: {}", safe_reports.count());

    Ok(())
}