struct Race {
    time: i32,
    dist: i32,
}

pub fn run(_: &String) -> usize {
    let answer = winning_holds(race()).len();
    println!("{:?}", answer);
    answer
}

fn winning_holds(race: Race) -> Vec<i32> {
    let mut holds: Vec<i32> = vec![];
    let mut _moved: i32 = 0;

    for hold in 1..race.time {
        _moved = hold * (race.time - hold);
        if _moved > race.dist {
            holds.push(hold);
        }
    }

    holds
}

fn race() -> Race {
    let time = 71530;
    let dist = 940200;
    Race { time, dist }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(run(&"".to_string()), 71503);
    }
}
