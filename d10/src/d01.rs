use std::collections::HashMap;

fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>()[0..binding.split("\n").count() - 1]
        .iter()
        .map(|x| x.trim().split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
        .iter()
        .map(|x| x[1..x.len() - 1].to_vec())
        .collect::<Vec<Vec<&str>>>();

    //find first "S" in binding
    let mut i = 0;
    let mut j = 0;

    for (index, row) in input.iter().enumerate() {
        for (index2, col) in row.iter().enumerate() {
            if col == &"S" {
                i = index;
                j = index2;
            }
        }
    }

    // 0: N, 1: E, 2: S, 3: W
    let mut direction = 4;

    if input[i - 1][j] == "|" || input[i - 1][j] == "F" || input[i - 1][j] == "7" {
        direction = 0;
    } else if input[i][j + 1] == "-" || input[i][j + 1] == "J" || input[i][j + 1] == "7" {
        direction = 1;
    } else if input[i + 1][j] == "|" || input[i + 1][j] == "L" || input[i + 1][j] == "J" {
        direction = 2;
    } else if input[i][j - 1] == "-" || input[i][j - 1] == "F" || input[i][j - 1] == "L" {
        direction = 3;
    }

    let mut mover = HashMap::new();
    mover.insert("F", [1, 4, 4, 2]);
    mover.insert("7", [3, 2, 4, 4]);
    mover.insert("L", [4, 4, 1, 0]);
    mover.insert("J", [4, 0, 3, 4]);

    let mut moves = 0;
    loop {
        match direction {
            0 => i -= 1,
            1 => j += 1,
            2 => i += 1,
            3 => j -= 1,
            _ => panic!("invalid direction"),
        }

        moves += 1;

        if input[i][j] == "S" {
            break;
        }

        if input[i][j] != "|" && input[i][j] != "-" {
            direction = mover.get(input[i][j]).unwrap()[direction];
        }
    }

    println!("furthest point is {} steps away", moves / 2);
}
