const MARKER: char = '%';

pub fn run(input: &String) -> usize {
    let mut values: usize = 0;
    let mut num_chars: Vec<&str>;
    let mut first_char: &str;
    let mut last_char: &str;
    let mut value: usize;

    let marker_wrapped_number_words = marker_wrapped_number_words(input);
    let marker_wrapped_number_chars = marker_wrapped_number_chars(marker_wrapped_number_words);
    let marker_wrapped_num_string: String = marker_wrapped_number_chars.iter().collect();

    for line in marker_wrapped_num_string.lines() {
        num_chars = line.split(MARKER).filter(|c| !c.is_empty()).collect();
        first_char = num_chars.first().expect("empty");
        last_char = num_chars.last().expect("empty");
        value = format!("{}{}", first_char, last_char).parse().expect("NaN");
        values += value;
    }

    println!("{:?}", values);
    values
}

fn marker_wrapped_number_chars(marker_wrapped_number_words: String) -> Vec<char> {
    let mut is_translated_wrapping = false;
    let mut chars: Vec<char> = vec![];

    for char in marker_wrapped_number_words.chars() {
        if char == MARKER {
            is_translated_wrapping = !is_translated_wrapping;
            chars.push(char);
        } else if char.is_numeric() {
            if is_translated_wrapping {
                chars.push(char);
            } else {
                chars.splice(chars.len().., [MARKER, char, MARKER]);
            }
        } else if char.is_whitespace() {
            chars.push(char);
        }
    }

    chars
}

fn marker_wrapped_number_words(input: &String) -> String {
    let translations = [
        (0, "zero"),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
    ];

    let input_chars: Vec<char> = input.chars().into_iter().collect();
    let mut output: String = "".into();

    for (i, input_char) in input_chars.iter().enumerate() {
        if let Some(translation) = translations
            .iter()
            .find(|t| String::from_iter(input_chars[i..].into_iter()).starts_with(t.1))
        {
            output.push_str(
                format!("{}{}{}", MARKER, translation.0.to_string().as_str(), MARKER).as_str(),
            );
        } else {
            output.push_str(input_char.to_string().as_str());
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::days::day1_part2::run;

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
        assert_eq!(run(&input), 281);
    }
}
