fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let mut sum = 0;

    for game in input {
        if game.len() == 0 {
            break;
        }

        let ID = game.split(":").collect::<Vec<&str>>()[0]
            .split(" ")
            .collect::<Vec<&str>>()[1]
            .trim()
            .parse::<i32>()
            .unwrap();

        let grabs = game.split(":").collect::<Vec<&str>>()[1]
            .trim()
            .split(|c| c == ';' || c == ',')
            .collect::<Vec<&str>>();

        let mut min_r = 0;
        let mut min_g = 0;
        let mut min_b = 0;

        for grab in grabs {
            let amount = grab.trim().split(" ").collect::<Vec<&str>>()[0]
                .parse::<i32>()
                .unwrap();
            let color = grab.trim().split(" ").collect::<Vec<&str>>()[1];

            if color == "red" && amount > min_r {
                min_r = amount;
            } else if color == "green" && amount > min_g {
                min_g = amount;
            } else if color == "blue" && amount > min_b {
                min_b = amount;
            }
        }

        sum += min_r * min_g * min_b;
    }

    println!("{}", sum);
}
