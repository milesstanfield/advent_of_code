use regex::Regex;

#[derive(Debug, Clone)]
struct Program {
    name: String,
    weight: String,
    oppressors: String,
    oppressing_weight: usize,
}
type Programs = Vec<Program>;

pub fn run(input: &String) {
    let programs = programs(input);
    let programs = oppressing_weight_programs(programs);

    // let program_weight: usize = program.weight.parse().unwrap();
    // println!("{:?}", sum + program_weight)
}

fn index_by_name(programs: &Programs, name: &str) -> usize {
    for (i, program) in programs.iter().enumerate() {
        if program.name == name {
            return i;
        }
    }
    panic!("not found")
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

fn oppressing_weight_programs(programs: Programs) -> Programs {
    let mut mprograms = programs.clone();

    for i in 0..mprograms.len() {
        mprograms[i].oppressing_weight = oppressing_weight(&mprograms[i].oppressors, &programs);
    }

    mprograms
}

fn oppressing_weight(oppressors: &str, programs: &Programs) -> usize {
    let mut oppressing_weight = 0;
    let mut weight: usize = 0;
    let mut oppressor_index: usize = 0;

    for oppressor_name in oppressors.split(" ") {
        oppressor_index = index_by_name(&programs, &oppressor_name);
        weight = programs[oppressor_index].weight.parse().unwrap();
        oppressing_weight += weight;
    }
    oppressing_weight
}

fn program(basics: &Vec<&str>, parts: &Vec<&str>) -> Program {
    Program {
        name: basics.first().expect("no name").to_string(),
        weight: weight(&basics),
        oppressors: oppressors(&parts),
        oppressing_weight: 0,
    }
}

fn oppressors(parts: &Vec<&str>) -> String {
    if parts.len() > 1 {
        let re = Regex::new(r"[,\s]+").unwrap();
        let haystack = parts.last().expect("empty");
        re.replace_all(haystack, " ").to_string()
    } else {
        "".to_string()
    }
}

fn weight(basics: &Vec<&str>) -> String {
    let raw_weight = basics.last().expect("lbs");
    let re = Regex::new(r"[\(\)]").expect("no regex");
    re.replace_all(raw_weight, "").parse().expect("NaN")
}
