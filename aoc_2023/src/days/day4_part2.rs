use regex::Regex;

#[derive(Debug)]
struct Score {
    copies: usize,
    matches: usize,
}
pub fn run(input: &String) -> usize {
    let cards: Vec<&str> = input.lines().collect();
    let mut total: usize = 0;
    let mut scores = scores(cards);

    for copied_score in copied_scores(&mut scores) {
        total += copied_score.copies + 1;
    }

    total
}

fn copied_scores(scores: &mut Vec<Score>) -> &mut Vec<Score> {
    for i in 0..scores.len() {
        for ii in (i + 1)..(i + scores[i].matches + 1) {
            scores[ii].copies += 1 + scores[i].copies;
        }
    }
    scores
}

fn scores(cards: Vec<&str>) -> Vec<Score> {
    let copies = 0;
    let mut scores: Vec<Score> = vec![];
    let mut matches: usize;

    for i in 0..cards.len() {
        matches = card_matches(cards[i]);
        scores.push(Score { copies, matches })
    }
    scores
}

fn card_matches(card: &str) -> usize {
    let re = Regex::new(r"Card .*: (.*) \| (.*)").unwrap();
    let (_, [wins_str, ours_str]) = re.captures(card).unwrap().extract();
    let wins = numbers_from(wins_str);
    let ours = numbers_from(ours_str);
    tally_matches(wins, ours)
}

fn tally_matches(wins: Vec<usize>, ours: Vec<usize>) -> usize {
    let mut matches: usize = 0;
    for our in ours {
        if wins.contains(&our) {
            matches += 1;
        }
    }
    matches
}

fn numbers_from(string: &str) -> Vec<usize> {
    string.split_whitespace().map(|s| to_num(s)).collect()
}

fn to_num(s: &str) -> usize {
    s.parse().expect(&format!("{:?} NaN", s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: String = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .into();
        assert_eq!(run(&input), 30);
    }
}
