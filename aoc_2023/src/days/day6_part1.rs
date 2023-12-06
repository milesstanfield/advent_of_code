struct Race {
    time: i32,
    dist: i32,
}

pub fn run(input: &String) -> usize {
    let races = vec![
        Race { time: 7, dist: 9 },
        Race { time: 15, dist: 40 },
        Race {
            time: 30,
            dist: 200,
        },
    ];
    let mut winning_holds: Vec<i32>;
    let mut moved: i32;
    let mut ways_to_win: Vec<i32> = vec![];

    for race in races {
        moved = 0;
        winning_holds = vec![];

        for hold in 1..race.time {
            moved = hold * (race.time - hold);
            if moved > race.dist {
                winning_holds.push(hold);
            }
        }

        ways_to_win.push(winning_holds.len() as i32);
    }

    let mut mult = 0;
    for way_to_win in ways_to_win {
        mult = mult * way_to_win;
    }
    println!("{:?}", mult);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(run(&input), 288);
    }
}
