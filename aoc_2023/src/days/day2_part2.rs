use regex::Regex;

pub fn run(input: &String) -> usize {
    let sets_regex = Regex::new(r".*: ").unwrap();
    let mut power_sum: usize = 0;
    let mut red_set_max: usize;
    let mut green_set_max: usize;
    let mut blue_set_max: usize;
    let mut sets: String;
    let mut data_splits: Vec<&str>;
    let mut color: &str;
    let mut count: usize;

    for line in input.lines() {
        sets = sets_regex.replace_all(line, "").to_string();
        red_set_max = 0;
        green_set_max = 0;
        blue_set_max = 0;

        for set in sets.split(";").into_iter() {
            for data in set.split(",") {
                data_splits = data.trim().split(" ").collect();
                color = &data_splits.last().expect("color missing");
                count = data_splits.first().unwrap().parse().expect("NaN");

                match color {
                    "red" => red_set_max = count.max(red_set_max),
                    "blue" => blue_set_max = count.max(blue_set_max),
                    _ => green_set_max = count.max(green_set_max),
                }
            }
        }

        power_sum += red_set_max * green_set_max * blue_set_max;
    }

    power_sum
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let input: String = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .into();
        assert_eq!(crate::days::day2_part2::run(&input), 2286);
    }
}
