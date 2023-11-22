pub fn run(input: &String) {
    let mut valid: u32 = 0;

    for line in input.lines() {
        if is_valid_passphrase(line) {
            valid += 1;
        }
    }

    println!("{}", valid);
}

fn is_valid_passphrase(line: &str) -> bool {
    let mut words: Vec<String> = vec![];
    let mut chars: Vec<char>;
    let mut sorted_word: String;

    for word in line.split(" ").into_iter() {
        chars = word.chars().collect();
        chars.sort_by(|a, b| a.cmp(b));
        sorted_word = chars.into_iter().collect();

        if words.contains(&sorted_word) {
            return false;
        } else {
            words.push(sorted_word);
        }
    }

    return true;
}
