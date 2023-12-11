fn extrapolate(line: &str) -> i32 {
    let mut history: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
    history.reverse();
    let mut sum: i32 = *history.last().unwrap();
    let mut sequences: Vec<i32> = history.windows(2).map(|pair| pair[1] - pair[0]).collect();
    sum += sequences.last().unwrap();

    loop {
        sequences = sequences.windows(2).map(|pair| pair[1] - pair[0]).collect();
        if sequences.iter().all(|seq| *seq == 0) {
            return sum;
        } else {
            sum += sequences.last().unwrap();
        }
    }
}

pub fn run(input: &String) -> i32 {
    let mut total: i32 = 0;

    for line in input.lines() {
        total += extrapolate(line);
    }

    dbg!(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: String = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            .into();
        assert_eq!(run(&input), 2);
    }
}
