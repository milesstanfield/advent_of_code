use std::ops::Range;

use regex::Regex;

pub fn run(input: &String) -> Vec<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let seeds = seeds(lines.first()).sort();
    // [13, 14, 55, 79];

    // 50 98 2    => 50..51  98..99
    // 52 50 48   => 52..(52 + 48)  50..(50 + 48)

    let r1: Range<usize> = 98..(98 + 1);
    let r2: Range<usize> = 50..(50 + 47);

    if r1.contains(&13) || r2.contains(&13) {
        println!("todo")
    } else {
        println!("13 is same");
    }

    if r1.contains(&14) || r2.contains(&14) {
        println!("todo")
    } else {
        println!("14 is same");
    }

    if r1.contains(&55) {
        println!("r1 has 55");
    } else if r2.contains(&55) {
        println!("55 is {}", 55 + (52 - 50));
    } else {
        println!("14 is same");
    }

    if r1.contains(&79) {
        println!("r1 has 79");
    } else if r2.contains(&79) {
        println!("79 is {}", 79 + (52 - 50));
    } else {
        println!("79 is same");
    }

    // if .ranee {}
    // todo parse this
    // let foo = 9..3;
    let soil_ranges: Vec<Range<u32>> =
        vec![50..(50 + 1), 98..(98 + 1), 52..(52 + 47), 50..(50 + 47)];

    println!("seeds {:?}", seeds);
    let mut sum: usize = 0;

    // sum
    vec![]
}

fn seeds(line1: Option<&&str>) -> Vec<u32> {
    Regex::new(r"\D")
        .unwrap()
        .split(line1.unwrap())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect(&format!("{} NaN", s)))
        .collect::<Vec<u32>>()
}

// the maps describe entire ranges of numbers that can be converted.
// Each line within a map contains three numbers:
// the destination range start, the source range start, and the range length

// seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2    => 50..51  98..99
// With this information, you know that seed number 98 corresponds to soil number
// 50 and that seed number 99 corresponds to soil number 51.

// 52 50 48   => 52..(52 + 48)  50..(50 + 48)
// The second line means that the source range starts at 50 and contains 48 values: 50, 51, ..., 96, 97.
// This corresponds to a destination range starting at 52 and also containing 48 values: 52, 53, ..., 98, 99.
// So, seed number 53 corresponds to soil number 55.

// Any source numbers that aren't mapped correspond to the same destination number.
// So, seed number 10 corresponds to soil number 10.

// seed  soil
// 0     0
// 1     1
// ...   ...
// 48    48
// 49    49
// 50    52
// 51    53
// ...   ...
// 96    98
// 97    99
// 98    50
// 99    51

// With this map, you can look up the soil number required for each initial seed number:

// Seed number 79 corresponds to soil number 81.
// Seed number 14 corresponds to soil number 14.
// Seed number 55 corresponds to soil number 57.
// Seed number 13 corresponds to soil number 13.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: String = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            .into();
        let todo: Vec<u32> = vec![81, 14, 57, 13];
        assert_eq!(run(&input), todo);
    }
}
