pub fn run(input: &String) {
    let chars: Vec<char> = input.chars().collect();
    let mut sum: u32 = 0;
    let mut _is_last_char = false;

    for (i, char) in chars.iter().enumerate() {
        _is_last_char = i == input.len() - 1;

        if (_is_last_char && char == &chars[0]) || (!_is_last_char && char == &chars[i + 1]) {
            sum += char.to_digit(10).expect("NaN");
        }
    }

    println!("{}", sum);
}
