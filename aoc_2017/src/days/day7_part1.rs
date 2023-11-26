use regex::Regex;

#[derive(Debug)]
struct Program {
    name: String,
    weight: String,
    oppressors: String,
}
type Programs = Vec<Program>;

pub fn run(input: &String) {
    let programs = programs(input);
    let mut oppress_positions: Vec<usize> = (0..programs.len()).collect();

    for i in 0..programs.len() {
        for ii in 0..programs.len() {
            if is_oppressing(i, ii, &programs) {
                oppress_positions = oppress_positions.into_iter().filter(|p| p != &i).collect();
            }
        }
    }

    let unoppressing_program = &programs[*oppress_positions.last().expect("missing")];
    println!("{:#?}", unoppressing_program.name);
}

fn is_oppressing(i: usize, ii: usize, programs: &Programs) -> bool {
    i != ii && programs[ii].oppressors.contains(programs[i].name.as_str())
}

fn programs(input: &String) -> Programs {
    let mut parts: Vec<&str>;
    let mut basics: Vec<&str>;
    let mut programs: Programs = vec![];

    for line in input.lines() {
        parts = line.split(" -> ").collect();
        basics = parts.first().expect("unsplittable").split(" ").collect();
        programs.push(program(&basics, &parts));
    }
    programs
}

fn program(basics: &Vec<&str>, parts: &Vec<&str>) -> Program {
    Program {
        name: name(&basics),
        weight: weight(&basics),
        oppressors: oppressors(&parts),
    }
}

fn oppressors(parts: &Vec<&str>) -> String {
    if parts.len() > 1 {
        let re = Regex::new(r"[,\s]").unwrap();
        let haystack = parts.last().expect("empty");
        re.replace_all(haystack, "").to_string()
    } else {
        "".to_string()
    }
}

fn name(basics: &Vec<&str>) -> String {
    basics.first().expect("no name").to_string()
}

fn weight(basics: &Vec<&str>) -> String {
    let raw_weight = basics.last().expect("lbs");
    let re = Regex::new(r"[\(\)]").expect("no regex");
    re.replace_all(raw_weight, "").parse().expect("NaN")
}
