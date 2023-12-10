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
    mut sum: usize,
    instructions: &Vec<char>,
    nodes: &'a Nodes,
) -> usize {
    for instruction in instructions {
        sum += 1;

        let node = nodes.map.get(node_name).unwrap();

        node_name = match instruction {
            'L' => &node.left,
            _ => &node.right,
        };

        if node_name.ends_with('Z') {
            break;
        }
    }

    if node_name.ends_with('Z') {
        sum
    } else {
        traverse(node_name, sum, instructions, nodes)
    }
}

pub fn run(input: &String) -> usize {
    let (instructions, nodes) = Input::parse(input);

    let mut sums: Vec<usize> = vec![];
    for node_name in nodes.map.keys().filter(|k| k.ends_with('A')) {
        sums.push(traverse(node_name, 0, &instructions, &nodes));
    }

    let lcm = sums
        .into_iter()
        .reduce(|a, b| num::integer::lcm(b, a))
        .unwrap();

    dbg!(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: String = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            .into();
        assert_eq!(run(&input), 6);
    }
}
