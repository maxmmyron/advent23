use num;
use std::collections::HashMap;

fn parse_line(line: &str) -> (&str, &str, &str) {
    // XXX = (YYY, ZZZ)
    let label = &line[0..3];
    let left = &line[7..10];
    let right = &line[12..15];

    return (label, left, right);
}

fn main() {
    let time = std::time::Instant::now();

    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let instructions = input[0].trim().split("").collect::<Vec<&str>>();
    // remove first and last instruction
    let instructions = instructions[1..(instructions.len() - 1)].to_vec();

    let mut counts = Vec::new();
    let mut currents = Vec::new();
    let mut nodes: HashMap<&str, usize> = HashMap::new();

    for idx in 2..(input.len() - 1) {
        let line = input[idx].trim();

        let (label, left, right) = parse_line(line);

        if label.ends_with('A') {
            counts.push(0);
            currents.push(label);
        }

        nodes.insert(label, idx);
    }

    for i in 0..currents.len() {
        let mut current = currents[i];

        let mut current_index = nodes.get(current).unwrap();

        loop {
            let instruction = instructions[counts[i] % instructions.len()];
            let line = input[*current_index].trim();

            let (label, left, right) = parse_line(line);

            if label.ends_with('Z') {
                break;
            }

            counts[i] += 1;
            if instruction == "L" {
                current = left;
                current_index = nodes.get(current).unwrap();
            } else if instruction == "R" {
                current = right;
                current_index = nodes.get(current).unwrap();
            }
        }
    }

    let mut lcm = counts[0];
    for i in 1..counts.len() {
        lcm = num::integer::lcm(lcm, counts[i]);
    }

    println!("{}", lcm);
}
