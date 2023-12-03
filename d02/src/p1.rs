fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let mut sum = 0;

    let max_r = 12;
    let max_g = 13;
    let max_b = 14;

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

        let mut ok = true;
        for grab in grabs {
            let amount = grab.trim().split(" ").collect::<Vec<&str>>()[0]
                .parse::<i32>()
                .unwrap();
            let color = grab.trim().split(" ").collect::<Vec<&str>>()[1];

            if color == "red" && max_r < amount {
                ok = false;
                break;
            } else if color == "green" && max_g < amount {
                ok = false;
                break;
            } else if color == "blue" && max_b < amount {
                ok = false;
                break;
            }
        }

        if ok {
            sum += ID;
        }
    }

    println!("{}", sum);
}
