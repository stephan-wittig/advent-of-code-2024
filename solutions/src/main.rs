use clap::Parser;

mod day_1_1;
mod day_1_2;
mod day_2_1;
mod day_2_2;
mod day_3_1;
mod day_3_2;
mod day_4_1;
mod day_4_2;
mod day_5_1;
mod day_5_2;
mod day_6_1;
mod day_6_2;
mod day_7_1;
mod day_7_2;
mod day_8_1;
mod day_8_2;
mod day_9_1;
mod day_9_2;
mod day_10_1;
mod day_10_2;
mod day_11_1;
mod day_11_2;

#[derive(Parser)]
#[command(name = "aoc-2024")]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    day: u16,
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    part: u16,
    #[arg(short, long, default_value = "-")]
    file: String,
}

fn main() {
    let cli = Cli::parse();
    let file = match aoc2024::open(&cli.file, cli.day) {
        Err(err) => panic!("Failed to read input: {}", err),
        Ok(file) => file
    };

    match cli.day {
        1 => match cli.part {
            1 => {
                let _ = day_1_1::run(file);
            }
            2 => {
                let _ = day_1_2::run(file);
            }
            _ => println!("Part {} not implemented, yet", cli.part)
        }
        2 => match cli.part {
            1 => {
                let _ = day_2_1::run(file);
            }
            2 => {
                let _ = day_2_2::run(file);
            }
            _ => println!("Part {} not implemented, yet", cli.part)
        }
        3 => match cli.part {
            1 => {
                let _ = day_3_1::run(file);
            }
            2 => {
                let _ = day_3_2::run(file);
            }
            _ => println!("Part {} not implemented, yet", cli.part)
        }
        4 => match cli.part {
            1 => {
                let _ = day_4_1::run(file);
            }
            2 => {
                let _ = day_4_2::run(file);
            }
            _ => println!("Part {} not implemented, yet", cli.part)
        }
        5 => match cli.part {
            1 => {
                let _ = day_5_1::run(file);
            }
            2 => {
                let _ = day_5_2::run(file);
            }
            _ => println!("Part {} not implemented, yet", cli.part)
        }
        6 => match cli.part {
            1 => {
                let _ = day_6_1::run(file);
            }
            2 => {
                let _ = day_6_2::run(file);
            }
            _ => println!("Part {} not implemented, yet", cli.part)
        }
        7 => match cli.part {
            1 => {
                let _ = day_7_1::run(file);
            }
            2 => {
                let _ = day_7_2::run(file);
            }
            _ => println!("Part {} not implemented, yet", cli.part)
        }
        8 => match cli.part {
            1 => {
                let _ = day_8_1::run(file);
            }
            2 => {
                let _ = day_8_2::run(file);
            }
            _ => println!("Part {} not implemented, yet", cli.part)
        }
        9 => match cli.part {
            1 => {
                let _ = day_9_1::run(file);
            }
            2 => {
                let _ = day_9_2::run(file);
            }
            _ => println!("Part {} not implemented, yet", cli.part)
        },
        10 => match cli.part {
            1 => {
                let _ = day_10_1::run(file);
            }
            2 => {
                let _ = day_10_2::run(file);
            }
            _ => println!("Part {} not implemented, yet", cli.part)
        },
        11 => match cli.part {
            1 => {
                let _ = day_11_1::run(file);
            }
            2 => {
                let _ = day_11_2::run(file);
            }
            _ => println!("Part {} not implemented, yet", cli.part)
        }
        _ => println!("Day {} not implemented, yet", cli.day)
    }
}