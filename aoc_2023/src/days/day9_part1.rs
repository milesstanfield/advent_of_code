struct StepVecs;
impl StepVecs {
    fn parse(values: Vec<i64>, mut vecs: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        let mut vec: Vec<i64> = vec![];

        for i in 0..values.len() {
            if i != 0 {
                vec.push(values[i - 1].abs_diff(values[i]) as i64);
            }
        }

        vecs.push(vec.clone());

        if vecs.last().unwrap().into_iter().all(|n| n == &0) {
            vecs
        } else {
            Self::parse(vec, vecs)
        }
    }
}

struct History {
    values: Vec<i64>,
}

impl History {
    fn next_value(&self) -> i64 {
        let values = self.values.clone();
        let step_vecs = StepVecs::parse(values.clone(), vec![self.values.clone()]);

        let mut val: i64 = 0;
        for step_vec in step_vecs.into_iter().filter(|x| !x.is_empty()) {
            val += step_vec.last().unwrap();
        }

        println!("next val for values {:?} is val {:?}", values, val);

        val
    }
}

impl From<&str> for History {
    fn from(line: &str) -> Self {
        let values: Vec<i64> = line
            .split(" ")
            .map(|value| value.parse::<i64>().unwrap())
            .collect();
        History { values }
    }
}

struct Histories;
impl Histories {
    fn parse(input: &String) -> Vec<History> {
        input
            .lines()
            .into_iter()
            .map(|line| History::from(line))
            .collect()
    }
}

pub fn run(input: &String) -> i64 {
    let mut sum: i64 = 0;
    for history in Histories::parse(input) {
        sum += history.next_value();
    }
    dbg!(sum)
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
        assert_eq!(run(&input), 114);
    }
}
