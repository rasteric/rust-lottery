extern crate clap;
extern crate rand;

use clap::*;
use rand::prelude::*;
use std::process;

fn main() {
    let matches = App::new("lottery")
        .version("0.1.0")
        .author("Erich Rast <erich@snafu.de>")
        .about("Get random lottery suggestions.")
        .arg(Arg::with_name("drawings")
             .required(true)
             .index(1)
             .help("the number of drawings, where each value can be drawn only once"))
        .arg(Arg::with_name("upper_value")
             .required(true)
             .index(2)
             .help("the highest number to draw from (inclusive)"))
        .arg(Arg::with_name("lower_value")
             .short("l")
             .long("lower_value")
             .required(false)
             .takes_value(true)
             .help("the lowest number to draw from (inclusive), default=1"))
        .get_matches();

    let upper = value_t!(matches.value_of("upper_value"), u32).unwrap_or_else(|e| e.exit());
    let lower = value_t!(matches.value_of("lower_value"), u32).unwrap_or(1);
    let drawings = value_t!(matches.value_of("drawings"), u32).unwrap_or_else(|e| e.exit());

    // sanity checks
    if lower > upper {
        println! ("The lowest value {} cannot be larger than the highest value {}!", lower, upper);
        process::exit(2);
    }
    if drawings > upper-lower+1 {
        println! ("You cannot draw more unique numbers than there are unique numbers!");
        process::exit(1);
    }
    if drawings == 0 {
        println! ("You requested 0 drawings, good bye and good luck next time!");
        process::exit(3);
    }
    
    // okay, everything fine, now draw the results
    let mut result = draw(drawings,upper,lower);
    result.sort();

    print!("{}", result.get(0).unwrap());
    for i in &result[1..] {
        print! (", {}", i);
    }
    println!();
}

fn draw (times: u32, upper: u32, lower: u32) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    let mut buff;
    
    for _ in 0..times {
        loop {
            buff = draw_one(upper,lower);
            if !result.contains(&buff) {
                break;
            }
        }
        result.push(buff);
    }
    result
}

fn draw_one (upper: u32, lower: u32) -> u32 {
    let num = thread_rng().gen_range(lower, upper+1);
    num
}
