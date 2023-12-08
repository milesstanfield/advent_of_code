struct Input {
    nodes: Vec<Node>,
    instructions: Vec<char>,
}

struct Node {
    name: String,
    left: String,
    right: String,
}

impl From<Vec<&str>> for Node {
    fn from(lines: Vec<&str>) -> Self {
        // Node { name: (), left: (), right: () }
    }
}

impl From<&String> for Input {
    fn from(input: &String) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let instructions_line = lines.first().unwrap_or(&"".into());
        let instructions = instructions_line.chars().collect();

        // let lines = input.lines();
        // let instructions_line = lines.into_iter().first().unwrap_or(&"".into());
        // let nodes: Vec<&str> = lines.skip(2).collect();

        // Input {
        //     nodes,
        //     instructions: instructions_line.chars().collect(),
        // }

        Input {
            nodes: (),
            instructions,
        }
    }
}

pub fn run(input: &String) -> usize {
    let mut sum: usize = 0;
    for line in Input::from(input).lines {
        println!("line {:?}", line);
    }

    dbg!(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: String = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            .into();
        assert_eq!(run(&input), 6);
    }
}
