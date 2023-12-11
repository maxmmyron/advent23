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

    let mut relevant: Vec<Vec<usize>> = vec![];

    //find first "S" in binding
    let mut i = 0;
    let mut j = 0;

    for (index, row) in input.iter().enumerate() {
        relevant.push(vec![]);
        for (index2, col) in row.iter().enumerate() {
            if col == &"S" {
                i = index;
                j = index2;
                relevant[index].push(j);
            }
        }
    }

    // 0: N, 1: E, 2: S, 3: W
    let mut direction = 4;
    let mut s_vert = false;

    // WARNING: this is a hacky solution and does not account for cases where S
    // exists at the edge of the map
    if input[i - 1][j] == "|" || input[i - 1][j] == "F" || input[i - 1][j] == "7" {
        direction = 0;
        if input[i + 1][j] == "|" || input[i + 1][j] == "J" || input[i + 1][j] == "L" {
            s_vert = true;
        }
    } else if input[i][j + 1] == "-" || input[i][j + 1] == "J" || input[i][j + 1] == "7" {
        direction = 1;
        if input[i + 1][j] == "|" || input[i + 1][j] == "J" || input[i + 1][j] == "L" {
            s_vert = true;
        }
    } else if input[i + 1][j] == "|" || input[i + 1][j] == "L" || input[i + 1][j] == "J" {
        direction = 2;
        if input[i - 1][j] == "|" || input[i - 1][j] == "J" || input[i - 1][j] == "L" {
            s_vert = true;
        }
    } else if input[i][j - 1] == "-" || input[i][j - 1] == "F" || input[i][j - 1] == "L" {
        direction = 3;
        if input[i + 1][j] == "|" || input[i + 1][j] == "J" || input[i + 1][j] == "L" {
            s_vert = true;
        }
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
        relevant[i].push((j));

        if input[i][j] == "S" {
            break;
        }

        if input[i][j] != "|" && input[i][j] != "-" {
            direction = mover.get(input[i][j]).unwrap()[direction];
        }
    }

    let mut area = 0;
    for idx in 0..input.len() {
        let mut within = false;
        for jdx in 0..input[idx].len() {
            if relevant[idx].contains(&jdx) {
                let pipe = input[idx][jdx];
                if pipe == "|" || pipe == "7" || pipe == "F" || (pipe == "S" && s_vert) {
                    within = !within;
                }
            } else {
                if within {
                    area += 1;
                }
            }
        }
    }

    println!("furthest point is {}; area is {}", moves / 2, area);
}
