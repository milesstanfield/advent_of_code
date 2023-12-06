struct Race {
    time: i32,
    dist: i32,
}

pub fn run(_: &String) -> usize {
    let mut mult = 1;

    for race in races() {
        mult = mult * winning_holds(race).len();
    }

    println!("{:?}", mult);
    mult
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

fn races() -> Vec<Race> {
    vec![
        Race { time: 7, dist: 9 },
        Race { time: 15, dist: 40 },
        Race {
            time: 30,
            dist: 200,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(run(&"".to_string()), 288);
    }
}
