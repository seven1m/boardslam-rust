extern crate boardslam;

use std::{env, process};
use boardslam::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Please specify 3 numbers between 1 and 6");
        process::exit(1);
    }
    let n1 = args[1].parse::<u8>().unwrap_or(0);
    let n2 = args[2].parse::<u8>().unwrap_or(0);
    let n3 = args[3].parse::<u8>().unwrap_or(0);
    if n1 == 0 || n2 == 0 || n3 == 0 {
        println!("Please specify 3 numbers between 1 and 6");
        process::exit(1);
    }
    let results = fill_board(n1, n2, n3);
    println!("{}", display(&results));
    let missing = get_missing(&results);
    let missing = missing
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    println!("\nmissing: {}", missing);
}
