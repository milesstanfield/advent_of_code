pub fn run(input: &String) -> usize {
    let mut output: usize = 0;

    println!("{:?}", output);
    output
}

#[cfg(test)]
mod tests {
    use crate::days::day2_part1::run;

    #[test]
    fn it_works() {
        let input: String = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .into();
        assert_eq!(run(&input), 0);
    }
}
