pub fn run(input: &String) {
    let chars: Vec<char> = input.chars().collect();
    let jump = input.len() / 2;
    let last_index = input.len() - 1;
    let mut sum: u32 = 0;

    for (i, char) in chars.iter().enumerate() {
        if char == &chars[comparable_index(i, jump, last_index)] {
            sum += char.to_digit(10).expect("NaN");
        }
    }

    println!("{}", sum);
}

fn comparable_index(i: usize, jump: usize, last_index: usize) -> usize {
    if last_index >= i + jump {
        i + jump
    } else {
        i + jump - (last_index + 1)
    }
}
