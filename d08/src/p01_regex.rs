use regex::Regex;
use std::collections::HashMap;

fn parse_line(line: &str, regex: regex::Regex) -> (&str, &str, &str) {
    let Some(caps) = regex.captures(line) else {
        panic!("Invalid node: {}", line);
    };

    let label = caps.name("label").unwrap().as_str();
    let left = caps.name("left").unwrap().as_str();
    let right = caps.name("right").unwrap().as_str();

    return (label, left, right);
}

fn main() {
    let time = std::time::Instant::now();
    let node_re = Regex::new(r"(?<label>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();

    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let instructions = input[0].trim().split("").collect::<Vec<&str>>();
    // remove first and last instruction
    let instructions = instructions[1..(instructions.len() - 1)].to_vec();

    let mut nodes: HashMap<&str, usize> = HashMap::new();

    for idx in 2..(input.len() - 1) {
        let line = input[idx].trim();

        let (label, left, right) = parse_line(line, node_re.clone());

        nodes.insert(label, idx);
    }

    let mut count = 0;
    let mut current = "AAA";
    let mut current_index = nodes.get(current).unwrap();

    loop {
        let instruction = instructions[count % instructions.len()];
        let line = input[*current_index].trim();

        let (label, left, right) = parse_line(line, node_re.clone());

        // println!("CURRENT LINE: {} -> ({} {})", line, left, right);

        if label == "ZZZ" {
            // println!("Found ZZZ at index: {}", current_index);
            break;
        }

        count += 1;
        if instruction == "L" {
            // println!("moving left from {} to {}", current, left);
            current = left;
            current_index = nodes.get(current).unwrap();
        } else if instruction == "R" {
            // println!("moving right from {} to {}", current, right);
            current = right;
            current_index = nodes.get(current).unwrap();
        }

        // println!("");
    }

    println!("Time: {}ms", time.elapsed().as_millis());
    println!("Count: {}", count);
}
