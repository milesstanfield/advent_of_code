use std::collections::HashMap;

use eval::{eval, to_value};
use regex::Regex;

pub fn run(input: &String) -> usize {
    let re = Regex::new(r"^(\w+) ([^\s]+) (\d+) if (\w+) ([^\n]+)").unwrap();
    let mut map: HashMap<String, usize> = HashMap::new();

    for line in input.lines() {
        for (_, [var, instr, instr_val, cond_var, expr]) in
            re.captures_iter(line).map(|c| c.extract())
        {
            if let Ok(expression) = eval(&format!("{} {}", 0, expr).to_string()) {
                if expression.as_bool().expect("missing bool") {
                    println!("yes! {}", line);
                }
            }

            // println!(
            //     "{} {} {} {} {} {}",
            //     var, instr, instr_val, cond_var, cond_oper, cond_val
            // );
        }
    }

    // let captures: Vec<&str> = r.captures_iter(input).collect();

    // println!("{:?}", .all(f));

    // for line in input.lines() {
    //     println!("line {}", line);
    // }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: String = "\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"
            .into();
        assert_eq!(run(&input), 0);
    }
}
