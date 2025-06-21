use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // The pattern to search for
    #[arg(short, long)]
    pattern: String,

    // File to search or '-' for stdin
    #[arg(short, long, default_value = "-")]
    input: String,
}

fn process_lines<T: BufRead>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args = Args::parse();
    let pattern = Regex::new(&args.pattern).unwrap();
    let input = args.input;

    if input == "-" {
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        process_lines(reader, pattern);
        return;
    }

    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);
    process_lines(reader, pattern)

}
