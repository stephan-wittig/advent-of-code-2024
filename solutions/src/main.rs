use clap::Parser;

mod day_1_1;
mod day_1_2;
mod day_2_1;
mod day_2_2;
mod day_3_1;
mod day_3_2;

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
        _ => println!("Day {} not implemented, yet", cli.day)
    }
}