fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let mut sum = 0;

    for line in input {
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
        sum += 1 << (chosen.len() - 1);
    }

    println!("{}", sum);
}
