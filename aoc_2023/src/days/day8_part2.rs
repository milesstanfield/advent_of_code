use std::collections::HashMap;

use regex::Regex;

struct Nodes {
    map: HashMap<String, Node>,
}

impl Nodes {
    fn from(lines: Vec<&str>) -> Self {
        let mut map = HashMap::new();
        for line in lines.into_iter().skip(2) {
            let (name, tuple) = Node::parse(line);
            map.insert(name, tuple);
        }
        Nodes { map }
    }
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    fn new(left: &str, right: &str) -> Self {
        Node {
            left: left.into(),
            right: right.into(),
        }
    }

    fn parse(line: &str) -> (String, Node) {
        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
        let (_, [name, left, right]) = re.captures(line).unwrap().extract();
        (name.into(), Node::new(left, right))
    }
}

struct Input;
impl Input {
    fn parse(input: &String) -> (Vec<char>, Nodes) {
        let lines: Vec<&str> = input.lines().collect();
        let instructions = lines.first().unwrap_or(&"".into()).chars().collect();
        let nodes = Nodes::from(lines);
        (instructions, nodes)
    }
}

fn traverse<'a>(
    mut node_name: &'a str,
    mut sum: i32,
    instructions: &Vec<char>,
    nodes: &'a Nodes,
) -> Option<i32> {
    for instruction in instructions {
        let node = nodes.map.get(node_name).unwrap();

        node_name = match instruction {
            'L' => &node.left,
            _ => &node.right,
        };

        sum += 1;

        if node_name == "ZZZ" {
            break;
        }
    }

    if node_name != "ZZZ" {
        traverse(node_name, sum, instructions, nodes)
    } else {
        Some(sum)
    }
}

pub fn run(input: &String) -> i32 {
    let (instructions, nodes) = Input::parse(input);
    match traverse(&mut "AAA", 0, &instructions, &nodes) {
        Some(sum) => dbg!(sum),
        None => 0,
    }
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
