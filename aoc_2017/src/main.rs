use days::day1_part2::run;
use std::{collections::VecDeque, env};

pub mod days;

fn main() {
    let args: VecDeque<String> = env::args().skip(1).collect();

    match args.get(0) {
        Some(arg1) => run(arg1),
        None => panic!(r#"missing arg. usage: cargo run "<some-arg>""#),
    }
}
