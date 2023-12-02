pub fn run(input: &String) -> usize {
    let mut output: usize = 0;

    println!("{:?}", input);
    output
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input: String = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .into();
        assert_eq!(crate::days::day2_part1::run(&input), 0);
    }
}
