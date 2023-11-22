pub fn run(input: &String) {
    let mut max = 0;
    let mut min = 0;
    let mut sum = 0;
    let mut _num = 0;

    for line in input.lines() {
        for (i, str_num) in line.split(" ").into_iter().enumerate() {
            _num = str_num.parse().expect("NaN");

            if i == 0 {
                (min, max) = (_num, _num);
            }

            if _num > max {
                max = _num;
            } else if _num < min {
                min = _num;
            }
        }
        sum += max - min;
    }

    println!("{}", sum);
}
