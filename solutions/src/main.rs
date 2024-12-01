use clap::Parser;

mod day_1_1;

#[derive(Parser)]
#[command(name = "aoc-2024")]
#[command(version, about, long_about = None)]
struct Cli {
    day: Option<String>,
    part: Option<String>,
    #[arg(short, long, default_value = "-")]
    file: String,
}

fn main() {
    let cli = Cli::parse();
    let filename = cli.file;

    if cli.day.as_deref() == Some("1") {
        match aoc2024::open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let _ = day_1_1::run(file);
            }
        }
    }
}