use regex::Regex;
use std::collections::HashMap;

type LineIter = usize;
type CharIter = usize;
type Position = (LineIter, CharIter);

// note: i hated doing this puzzle and this is some of the shitest code ive ever written.
// i also have no desire to refactor it or improve it. im washing my hands of it. it worked.
// thats all i care about rn

pub fn run(input: &String) -> usize {
    let mut sum: usize = 0;
    let lines: Vec<&str> = input.lines().into_iter().collect();

    let mut map: HashMap<Position, Vec<String>> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().into_iter().collect();
        let mut num_chars: Vec<char> = vec![];
        let mut is_star_adjacent = false;
        let mut star_positions: Vec<Position> = vec![];

        for (ii, &char) in chars.iter().enumerate() {
            if i == 10 && ii == 55 {
                println!("{}", chars[ii]);
            }

            if is_star_adjacent && char.is_numeric() {
                num_chars.push(char);

                if ii == chars.len() - 1 {
                    // at end
                    for x in 0..star_positions.len() {
                        let string: String = num_chars.iter().collect();
                        if let Some(vec) = map.get(&star_positions[x]) {
                            let mut newv = vec.clone();
                            newv.push(string);
                            map.insert(star_positions[x], newv);
                        } else {
                            map.insert(star_positions[x], vec![string]);
                        }
                    }
                    star_positions = vec![];
                    num_chars = vec![];
                    is_star_adjacent = false;
                }
            } else if char.is_numeric() {
                let mut adj_string: String = "".into();
                let mut tmp: String = "".into();

                if ii != 0 {
                    // left
                    tmp = chars[ii - 1].to_string();
                    adj_string.push_str(&tmp);
                    if tmp == "*" {
                        star_positions.push((i, ii - 1));
                    }
                }

                if ii != chars.len() - 1 {
                    // right
                    tmp = chars[ii + 1].to_string();
                    adj_string.push_str(&tmp);
                    if tmp == "*" {
                        star_positions.push((i, ii + 1));
                    }
                }

                if i != 0 {
                    // top
                    tmp = lines[i - 1].chars().collect::<Vec<char>>()[ii].to_string();
                    adj_string.push_str(&tmp);
                    if tmp == "*" {
                        star_positions.push((i - 1, ii));
                    }
                }

                if i != lines.len() - 1 {
                    // bot
                    tmp = lines[i + 1].chars().collect::<Vec<char>>()[ii].to_string();
                    adj_string.push_str(&tmp);
                    if tmp == "*" {
                        star_positions.push((i + 1, ii));
                    }
                }

                if i != 0 && ii != chars.len() - 1 {
                    // top right
                    tmp = lines[i - 1].chars().collect::<Vec<char>>()[ii + 1].to_string();
                    adj_string.push_str(&tmp);
                    if tmp == "*" {
                        star_positions.push((i - 1, ii + 1));
                    }
                }

                if i != 0 && ii != 0 {
                    // top left
                    tmp = lines[i - 1].chars().collect::<Vec<char>>()[ii - 1].to_string();
                    adj_string.push_str(&tmp);
                    if tmp == "*" {
                        star_positions.push((i - 1, ii - 1));
                    }
                }

                if i != lines.len() - 1 && ii != chars.len() - 1 {
                    // bottom right
                    tmp = lines[i + 1].chars().collect::<Vec<char>>()[ii + 1].to_string();
                    adj_string.push_str(&tmp);
                    if tmp == "*" {
                        star_positions.push((i + 1, ii + 1));
                    }
                }

                if i != lines.len() - 1 && ii != 0 {
                    // bottom left
                    tmp = lines[i + 1].chars().collect::<Vec<char>>()[ii - 1].to_string();
                    adj_string.push_str(&tmp);
                    if tmp == "*" {
                        star_positions.push((i + 1, ii - 1));
                    }
                }
                is_star_adjacent = Regex::new(r"[\*]").unwrap().is_match(&adj_string);

                num_chars.push(char);

                if is_star_adjacent && ii == chars.len() - 1 {
                    for x in 0..star_positions.len() {
                        let string: String = num_chars.iter().collect();
                        if let Some(vec) = map.get(&star_positions[x]) {
                            let mut newv = vec.clone();
                            newv.push(string);
                            map.insert(star_positions[x], newv);
                        } else {
                            map.insert(star_positions[x], vec![string]);
                        }
                    }
                    star_positions = vec![];
                    num_chars = vec![];
                    is_star_adjacent = false;
                }
            } else {
                if is_star_adjacent {
                    for x in 0..star_positions.len() {
                        let string: String = num_chars.iter().collect();
                        if let Some(vec) = map.get(&star_positions[x]) {
                            let mut newv = vec.clone();
                            newv.push(string);
                            map.insert(star_positions[x], newv);
                        } else {
                            map.insert(star_positions[x], vec![string]);
                        }
                    }
                    star_positions = vec![];
                    num_chars = vec![];
                }
                num_chars = vec![];
                is_star_adjacent = false;
            }
        }
    }

    println!("{:#?}", map);

    for (x, y) in map {
        if y.len() == 2 {
            // println!("{:?} {:?}", x, y);
            sum += y[0].parse::<usize>().unwrap() * y[1].parse::<usize>().unwrap();
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
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .into();
        assert_eq!(run(&input), 467835);
    }
}
