use std::collections::HashMap;

use regex::Regex;

pub fn run(input: &String) -> usize {
    let mut sum: usize = 0;
    let lines: Vec<&str> = input.lines().into_iter().collect();
    let mut map: HashMap<(usize, usize), Vec<String>> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().into_iter().collect();
        let mut num_chars: Vec<char> = vec![];
        let mut is_star_adjacent = false;

        for (ii, &char) in chars.iter().enumerate() {
            if is_star_adjacent && char.is_numeric() {
                num_chars.push(char);

                if ii == chars.len() - 1 {
                    // at end
                    sum += num_chars
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .expect("NaN");
                }
            } else if char.is_numeric() {
                let mut adj_string: String = "".into();

                if ii != 0 {
                    // left
                    adj_string.push_str(&chars[ii - 1].to_string());
                }

                if ii != chars.len() - 1 {
                    // right
                    adj_string.push_str(&chars[ii + 1].to_string());
                }

                if i != 0 {
                    // top
                    adj_string
                        .push_str(&lines[i - 1].chars().collect::<Vec<char>>()[ii].to_string());
                }

                if i != lines.len() - 1 {
                    // bot
                    adj_string
                        .push_str(&lines[i + 1].chars().collect::<Vec<char>>()[ii].to_string());
                }

                if i != 0 && ii != chars.len() - 1 {
                    // top right
                    adj_string
                        .push_str(&lines[i - 1].chars().collect::<Vec<char>>()[ii + 1].to_string());
                }

                if i != 0 && ii != 0 {
                    // top left
                    adj_string
                        .push_str(&lines[i - 1].chars().collect::<Vec<char>>()[ii - 1].to_string());
                }

                if i != lines.len() - 1 && ii != chars.len() - 1 {
                    // bottom right
                    adj_string
                        .push_str(&lines[i + 1].chars().collect::<Vec<char>>()[ii + 1].to_string());
                }

                if i != lines.len() - 1 && ii != 0 {
                    // bottom left

                    adj_string
                        .push_str(&lines[i + 1].chars().collect::<Vec<char>>()[ii - 1].to_string());
                }
                is_star_adjacent = Regex::new(r"[\*]").unwrap().is_match(&adj_string);

                num_chars.push(char);

                if is_star_adjacent && ii == chars.len() - 1 {
                    // at end
                    sum += num_chars
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .expect("NaN");
                }
            } else {
                if is_star_adjacent {
                    sum += num_chars
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .expect("NaN");
                }
                num_chars = vec![];
                is_star_adjacent = false;
            }
        }
    }

    println!("{:?}", sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: String = "\
467..114..
...-......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...*.*....
.664.598.."
            .into();
        assert_eq!(run(&input), 467835);
    }
}
