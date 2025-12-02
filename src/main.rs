use std::env;
use std::fs;
use std::time::Instant;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u8 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);

    println!("Running Day {:02}", day);

    let cwd = env::current_dir().unwrap();
    let input_path = cwd.join("data").join(format!("day-{}.txt", day));

    let raw_input = match fs::read_to_string(&input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading input file: {}", input_path.display());
            eprintln!("   Reason: {}", e);
            return;
        }
    };

    let input = raw_input.trim();

    let start = Instant::now();
    match day {
        1 => days::day01::run(&input),
        2 => days::day02::run(&input),
        _ => eprintln!("Day {} not implemented yet!", day),
    }

    let duration = start.elapsed();
    println!("⏱️  Time: {:?}", duration);
}
