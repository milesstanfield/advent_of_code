use std::{collections::HashMap, ops::Range};

use regex::Regex;

type TranslationMap = HashMap<usize, usize>;
type DataVec = Vec<usize>;
type DataVecs = Vec<DataVec>;

pub fn run(input: &String) -> usize {
    let groups = groups(input);
    let mut nums = line_numbers(groups[0]);

    for group in groups.into_iter().skip(1) {
        let data_vecs = data_vecs(group);
        let translation_map = translation_map(&nums, data_vecs);
        nums = translate(&nums, translation_map);
    }

    nums.sort();
    let lowest = nums.first().unwrap();
    println!("{:?}", lowest);

    *lowest
}

fn groups(input: &String) -> Vec<&str> {
    let re = Regex::new(r"\n\n").unwrap();
    re.split(input).collect()
}

fn data_vecs(group: &str) -> DataVecs {
    let re = Regex::new(r"(.*map:\n)(.*)").unwrap();
    let replacement = re.replace_all(group, "${2}");
    replacement.split("\n").map(|l| line_numbers(l)).collect()
}

fn translate(nums: &Vec<usize>, map: TranslationMap) -> Vec<usize> {
    let mut translated_nums: Vec<usize> = vec![];
    for num in nums {
        if let Some(translation) = map.get(&num) {
            translated_nums.push(*translation);
        } else {
            translated_nums.push(*num);
        }
    }
    translated_nums
}

fn translation_map(nums: &Vec<usize>, data_vecs: DataVecs) -> TranslationMap {
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut range: Range<usize>;

    for data in data_vecs {
        range = data[1]..(data[1] + (data[2] - 1) + 1);

        for num in nums.iter() {
            if range.contains(num) {
                let diff = data[0].abs_diff(data[1]);
                if data[0] >= data[1] {
                    map.insert(*num, num + diff);
                } else {
                    map.insert(*num, num - diff);
                }
            }
        }
    }
    map
}

fn line_numbers(line: &str) -> Vec<usize> {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(line)
        .map(|s| s.as_str().parse().unwrap())
        .collect()
}

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
        assert_eq!(run(&input), 35);
    }
}
