fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let mut sum = 0;

    let mut copies: Vec<i32> = Vec::new();

    for _ in 0..input.len() - 1 {
        copies.push(1);
    }

    for (idx, line) in input.iter().enumerate() {
        if line.len() == 0 {
            continue;
        }

        let winners = line.split("|").collect::<Vec<&str>>()[0]
            .trim()
            .split(":")
            .collect::<Vec<&str>>()[1]
            .trim()
            .split(" ")
            .collect::<Vec<&str>>();
        let mut chosen = line.split("|").collect::<Vec<&str>>()[1]
            .trim()
            .split(" ")
            .collect::<Vec<&str>>();

        chosen.retain(|choice| winners.contains(choice) && choice.len() > 0);

        if chosen.len() == 0 {
            continue;
        }

        for j in 1..chosen.len() + 1 {
            copies[idx + j] += copies[idx];
        }
    }

    for copy_count in copies {
        sum += copy_count;
    }
    println!("{}", sum);
}
