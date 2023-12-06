struct Race {
    time: i32,
    dist: i32,
}

pub fn run() -> i32 {
    let races = vec![
        Race {
            time: 60,
            dist: 601,
        },
        Race {
            time: 80,
            dist: 1163,
        },
        Race {
            time: 86,
            dist: 1559,
        },
        Race {
            time: 76,
            dist: 1300,
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

    let mut mult = 1;
    for way_to_win in ways_to_win {
        mult = mult * way_to_win;
    }
    println!("{:?}", mult);

    mult
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: String = "\
Time:      7  15   30
Distance:  9  40  200"
            .into();
        assert_eq!(run(), 288);
    }
}
