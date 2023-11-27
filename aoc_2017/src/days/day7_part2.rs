use regex::Regex;

#[derive(Debug)]
struct Program {
    name: String,
    weight: usize,
    oppressors: String,
    oppressing_weight: usize,
    total_weight: usize,
}

type Programs = Vec<Program>;

pub fn run(input: &String) {
    let programs = programs(input);

    // let bottom_program_name = "tknk";
    // let oppressing_stats = oppressing_stats(&programs, bottom_program_name);

    for program in programs {
        if !program.oppressors.is_empty() {
            println!("{:?}", program)
        }
    }
}

// fn oppressing_stats(programs: &Programs, bottom_program_name: &str) -> Vec<(usize, usize)> {
//     let mut weight: usize;
//     let mut data: (usize, usize);
//     let mut datas: Vec<(usize, usize)> = vec![];

//     for (i, program) in programs.iter().enumerate() {
//         if !program.oppressors.is_empty() && program.name != bottom_program_name {
//             weight = program.weight.parse().expect("weightless");
//             data = (i, program.oppressing_weight + weight);
//             datas.push(data);
//         }
//     }
//     datas
// }

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
            programs[i].oppressing_weight = oppressing_weight(&programs[i].oppressors, &programs);
        }
    }

    for i in 0..programs.len() {
        programs[i].total_weight = programs[i].weight + programs[i].oppressing_weight;
    }

    programs
}

fn index_by_name(programs: &Programs, name: &str) -> usize {
    for (i, program) in programs.iter().enumerate() {
        if program.name == name {
            return i;
        }
    }
    panic!("{:?} not found in {:?}", name, programs);
}

fn oppressing_weight(oppressors: &str, programs: &Programs) -> usize {
    let mut oppressing_weight = 0;
    let mut weight: usize;
    let mut oppressor_index: usize;

    for oppressor_name in oppressors.split(" ") {
        oppressor_index = index_by_name(&programs, &oppressor_name);
        weight = programs[oppressor_index].weight;
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
