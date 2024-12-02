use std::io::BufRead;

pub fn run(file: Box<dyn BufRead>) -> Result<(), Box<dyn std::error::Error>> {
    let mut reports: Vec<Vec<i32>> = vec![];
    
    for line_result in file.lines() {
        let line = line_result?;
        reports.push(line.split(" ").map(|s| s.parse::<i32>().unwrap()) .collect());
    }

    let safe_reports = reports.iter().filter(|report| is_report_safe(report));

    println!("Number of safe reports: {}", safe_reports.count());

    Ok(())
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    return position_issue(report) == None;
}

fn is_report_safish(report: &Vec<i32>) -> bool {
    if is_report_safe(report) {
        return true;
    }

    let issue_pos = position_issue(report).unwrap();

    // try removing first offending element
    if is_report_safe(&remove_pos(report, issue_pos)) {
        return true;
    }

    // try removing second offending element
    if is_report_safe(&remove_pos(report, issue_pos + 1)) {
        return true;
    }

    return false;

}

fn remove_pos(report: &Vec<i32>, position: usize) -> Vec<i32> {
    let mut new_report = report.clone();
    new_report.remove(position);
    return new_report;
} 

// -1 if no issue. Else, index of issue
fn position_issue(report: &Vec<i32>) -> Option<usize> {
    let general_direction = get_general_direction(report);

    return report.windows(2).position(|levels| {
        get_current_direction(levels) != general_direction ||
        get_step_width(levels) > 3
    });
}

fn get_step_width(levels: &[i32]) -> i32 {
    return (levels[1] - levels[0]).abs();
}

// 1 if increasing, -1 if decreasing, 0 if no change
fn get_current_direction(levels: &[i32]) -> i32 {
    let diff = levels[1] - levels[0];
    if diff > 0 {
        return 1;
    } else if diff < 0 {
        return -1;
    } else {
        return 0;
    }
}

// 1 if increasing, -1 if decreasing, 0 if no clear direction
fn get_general_direction(report: &Vec<i32>) -> i32 {
    let mut neg = 0;
    let mut pos = 0;

    report.windows(2).for_each(|levels| {
        match get_current_direction(levels) {
            1 => {
                pos += 1;
            }
            -1 => {
                neg += 0;
            }
            _ => ()
        }
    });

    if pos < 1 {
        return -1;
    } else if neg < 1 {
        return 1;
    } else {
        return 0;
    }
}
