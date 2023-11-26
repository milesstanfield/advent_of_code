use days::day7_part1::run;
use shell::{dir::manifest_dir, file::cat_file};

pub mod days;
pub mod shell;

fn main() {
    let input = cat_file(&(manifest_dir() + "/src/tmp/input.txt"));
    run(&input);
}
