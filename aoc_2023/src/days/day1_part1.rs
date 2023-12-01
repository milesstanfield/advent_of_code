pub fn run(input: &String) {
    let mut values: usize = 0;
    let mut num_chars: Vec<char>;
    let mut first_char: &char;
    let mut last_char: &char;
    let mut value: usize;

    for line in input.lines() {
        num_chars = line.chars().filter(|char| char.is_digit(10)).collect();
        first_char = num_chars.first().expect("empty");
        last_char = num_chars.last().expect("empty");
        value = format!("{}{}", first_char, last_char).parse().expect("NaN");
        values += value;
    }

    println!("{:?}", values)
}
