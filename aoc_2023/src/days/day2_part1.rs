use std::slice::Split;

use regex::Regex;

pub fn run(input: &String) -> usize {
    let mut ids_sum: usize = 0;
    let mut ids: Vec<usize> = vec![];
    let mut id: usize;

    // let mut id: usize;

    let mut red_set_max: usize = 12;
    let mut green_set_max: usize = 13;
    let mut blue_set_max: usize = 14;

    for (i, line) in input.lines().enumerate() {
        id = i + 1;
        let regex = Regex::new(r".*: ").unwrap();
        let sets = regex.replace_all(line, "").to_string();

        'set_loop: for set in sets.split(";").into_iter() {
            for data in set.split(",") {
                let data_splits: Vec<&str> = data.trim().split(" ").collect();
                let color = data_splits.last().expect("color missing");
                let count: usize = data_splits.first().unwrap().parse().expect("NaN");

                if (color == &"red" && count > red_set_max)
                    || (color == &"green" && count > green_set_max)
                    || (color == &"blue" && count > blue_set_max)
                {
                    ids = ids.into_iter().filter(|&set_id| set_id != id).collect();
                    break 'set_loop;
                } else {
                    ids.push(i + 1);
                }
            }
        }
    }

    ids.sort_unstable();
    ids.dedup();
    ids_sum = ids.iter().sum();
    println!("{:?}", ids_sum);
    ids_sum
}

#[cfg(test)]
mod tests {

    //     #[test]
    //     fn it_works() {
    //         let input: String = "\
    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    //             .into();
    //         assert_eq!(crate::days::day2_part1::run(&input), 8);
    //     }
}
