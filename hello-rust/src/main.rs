use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdo = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdo.lock());
    say(&message, width, &mut writer).unwrap();
}
