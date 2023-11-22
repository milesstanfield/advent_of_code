pub fn run(input: &String) {
    let mut sum = 0;
    let mut _line_nums: Vec<usize> = vec![];

    for line in input.lines() {
        _line_nums = line_numbers(&line);

        for (i, num1) in _line_nums.iter().enumerate() {
            for (ii, num2) in _line_nums.iter().enumerate() {
                if i != ii && num1 >= num2 && (num1 % num2 == 0) {
                    sum += num1 / num2;
                }
            }
        }
    }

    println!("{}", sum);
}

fn line_numbers(line: &str) -> Vec<usize> {
    let mut numbers: Vec<usize> = vec![];

    for str_num in line.split(" ").into_iter() {
        numbers.push(str_num.parse().expect("NaN"));
    }

    numbers
}
