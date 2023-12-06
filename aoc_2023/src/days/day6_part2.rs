struct Race {
    time: usize,
    dist: usize,
}

pub fn run(_: &String) -> usize {
    let races = vec![Race {
        time: 60808676,
        dist: 601116315591300,
    }];
    let mut winning_holds: Vec<usize>;
    let mut moved: usize;
    let mut ways_to_win: Vec<usize> = vec![];

    for race in races {
        moved = 0;
        winning_holds = vec![];

        for hold in 1..race.time {
            moved = hold * (race.time - hold);
            if moved > race.dist {
                winning_holds.push(hold);
            }
        }

        ways_to_win.push(winning_holds.len() as usize);
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
        assert_eq!(run(&"".to_string()), 71503);
    }
}
