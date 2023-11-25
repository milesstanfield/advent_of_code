pub fn run(input: &String) {
    let cycles = cycles(&input);
    let repeated_cycle_pos = repeated_cycle_pos(&cycles);
    println!("{:?}", (cycles.len() - 1) - repeated_cycle_pos);
}

fn repeated_cycle_pos(cycles: &Vec<String>) -> usize {
    let repeated_cycle = cycles.last().expect("missing");
    let mut cycle_pos: usize = 0;
    for (i, cycle) in cycles.iter().enumerate() {
        cycle_pos = i;
        if repeated_cycle == cycle {
            break;
        }
    }
    cycle_pos
}

fn cycles(input: &String) -> Vec<String> {
    let mut banks: Vec<usize> = input
        .split(" ")
        .map(|str| str.parse().expect("NaN"))
        .collect();

    let mut cycles: Vec<String> = vec![];

    loop {
        cycles.push(joined_banks(&banks));
        banks = cycle(banks);
        if cycles.contains(&joined_banks(&banks)) {
            break;
        }
    }
    cycles.push(joined_banks(&banks));
    cycles
}

fn joined_banks(banks: &Vec<usize>) -> String {
    let strings: Vec<String> = banks.iter().map(|int| int.to_string()).collect();
    strings.join(",")
}

fn cycle(mut banks: Vec<usize>) -> Vec<usize> {
    let redistribute_pos = redistribute_pos(&banks);
    let mut distribute = banks[redistribute_pos];
    let mut pos = redistribute_pos;
    banks[redistribute_pos] = 0;

    while distribute > 0 {
        if pos < banks.len() - 1 {
            pos += 1;
        } else {
            pos = 0;
        }
        banks[pos] += 1;
        distribute -= 1;
    }

    banks
}

fn redistribute_pos(banks: &Vec<usize>) -> usize {
    let mut pos: usize = 0;

    for (i, bank) in banks.iter().enumerate() {
        if i != 0 && bank > &banks[pos] {
            pos = i;
        }
    }

    pos
}
