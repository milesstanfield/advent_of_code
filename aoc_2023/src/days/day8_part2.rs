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

fn next_name(name: String, instruction: &char, nodes: &Nodes) -> String {
    match instruction {
        'L' => nodes.map.get(name.as_str()).unwrap().left.clone(),
        'R' => nodes.map.get(name.as_str()).unwrap().right.clone(),
        _ => panic!("huh?"),
    }
}

fn traverse<'a>(
    mut node_names: Vec<String>,
    mut sum: i32,
    instructions: &Vec<char>,
    nodes: &Nodes,
) -> i32 {
    for instruction in instructions {
        node_names = node_names
            .into_iter()
            .map(|name| next_name(name, instruction, nodes))
            .collect();

        sum += 1;

        if node_names.clone().into_iter().all(|n| n.ends_with('Z')) {
            break;
        }
    }

    if node_names.clone().into_iter().all(|n| n.ends_with('Z')) {
        sum
    } else {
        traverse(node_names, sum, instructions, nodes)
    }
}

pub fn run(input: &String) -> i32 {
    let (instructions, nodes) = Input::parse(input);

    let node_names: Vec<String> = nodes
        .map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.to_string())
        .collect();

    dbg!(traverse(node_names, 0, &instructions, &nodes))
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
