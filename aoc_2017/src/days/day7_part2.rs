use regex::Regex;

#[derive(Debug)]
struct Program {
    name: String,
    weight: usize,
    oppressors: String,
    oppressing: String,
    oppressors_weight: usize,
    total_weight: usize,
}

type Programs = Vec<Program>;

pub fn run(input: &String) {
    let programs = programs(input);

    for pa in &programs {
        for pb in &programs {
            if !pa.oppressing.is_empty() && !pb.oppressing.is_empty() {
                if pa.name != pb.name
                    && pa.oppressing == pb.oppressing
                    && pa.total_weight != pb.total_weight
                {
                    println!("{:?}, {:?}", pa, pb);
                }
            }
        }
    }
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

    for i in 0..programs.len() {
        if !programs[i].oppressors.is_empty() {
            programs[i].oppressors_weight = oppressors_weight(&programs[i].oppressors, &programs);
        }
        programs[i].oppressing = oppressing(i, &programs);
    }

    for i in 0..programs.len() {
        programs[i].total_weight = programs[i].weight + programs[i].oppressors_weight;
    }

    programs
}

fn oppressing(i: usize, programs: &Programs) -> String {
    let mut string: String = "".into();

    for ii in 0..programs.len() {
        if i != ii && programs[ii].oppressors.contains(programs[i].name.as_str()) {
            string = format!("{} {}", string, programs[ii].name);
        }
    }
    string.trim().into()
}

fn index_by_name(programs: &Programs, name: &str) -> usize {
    for (i, program) in programs.iter().enumerate() {
        if program.name == name {
            return i;
        }
    }
    panic!("{:?} not found in {:?}", name, programs);
}

fn oppressors_weight(oppressors: &str, programs: &Programs) -> usize {
    let mut oppressors_weight = 0;
    let mut weight: usize;
    let mut oppressor_index: usize;

    for oppressor_name in oppressors.split(" ") {
        oppressor_index = index_by_name(&programs, &oppressor_name);
        weight = programs[oppressor_index].weight;
        oppressors_weight += weight;
    }
    oppressors_weight
}

fn program(basics: &Vec<&str>, parts: &Vec<&str>) -> Program {
    Program {
        name: basics.first().expect("no name").to_string(),
        weight: weight(&basics),
        oppressing: "".into(),
        oppressors: oppressors(&parts),
        oppressors_weight: 0,
        total_weight: 0,
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

fn weight(basics: &Vec<&str>) -> usize {
    let raw_weight = basics.last().expect("lbs");
    let re = Regex::new(r"[\(\)]").expect("no regex");
    re.replace_all(raw_weight, "").parse().expect("NaN")
}
