pub fn run(input: &String) {
    let mut valid: u32 = 0;

    for line in input.lines() {
        if is_valid_passphrase(line) {
            println!("valid line {}", line);
            valid += 1;
        }
    }

    println!("{}", valid);
}

fn is_valid_passphrase(line: &str) -> bool {
    let mut words: Vec<String> = vec![];

    for word in line.split(" ").into_iter() {
        if words.contains(&word.to_string()) {
            return false;
        } else {
            words.push(word.to_string());
        }
    }

    return true;
}
