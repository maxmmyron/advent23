fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let mut nums: Vec<Vec<(i32, (i32, i32))>> = Vec::new();
    let mut gears: Vec<Vec<(i32, i32)>> = Vec::new();

    fn is_gear(c: char) -> bool {
        c == '*'
    }

    let mut sum = 0;

    // generate nums and gears
    for i in 0..input.len() {
        nums.push(Vec::new());
        gears.push(Vec::new());

        let line = input[i];
        if line.len() == 0 {
            break;
        }

        let mut num = 0;
        let mut num_scale = 1;
        for j in (0..line.len()).rev() {
            let parsed_gear = is_gear(line.chars().nth(j).unwrap());
            let parsed_num = line.chars().nth(j).unwrap().to_digit(10);

            if parsed_num.is_some() {
                num += parsed_num.unwrap() as i32 * num_scale;
                num_scale *= 10;
            } else {
                if parsed_gear {
                    gears[i].push((i as i32, j as i32));
                }
                // num != 0, so we just finished parsing a number
                if num != 0 {
                    nums[i].push((num, (i as i32, (j as i32) + 1)));
                    num = 0;
                    num_scale = 1;
                }
            }

            if j == 0 {
                if num != 0 {
                    nums[i].push((num, (i as i32, (j as i32))));
                }
            }
        }
    }

    for j in 0..gears.len() {
        let mut relevant_nums = Vec::new();
        for num in nums[j].iter() {
            relevant_nums.push(num);
        }
        if j != 0 {
            for num in nums[j - 1].iter() {
                relevant_nums.push(num);
            }
        }
        if j != nums.len() - 1 {
            for num in nums[j + 1].iter() {
                relevant_nums.push(num);
            }
        }

        for gear in &gears[j] {
            let mut adjacent_count = Vec::new();
            for num_pair in &relevant_nums {
                let num = num_pair.0;
                let min_pos = (num_pair.1 .0 - 1, num_pair.1 .1 - 1);
                let max_pos = (
                    num_pair.1 .0 + 1,
                    num_pair.1 .1 + num.to_string().len() as i32,
                );

                if gear.0 <= max_pos.0 && gear.0 >= min_pos.0 {
                    if gear.1 <= max_pos.1 && gear.1 >= min_pos.1 {
                        adjacent_count.push(num);
                    }
                }
            }

            if adjacent_count.len() == 2 {
                sum += adjacent_count[0] * adjacent_count[1];
            }
        }
    }

    print!("{}", sum);
}
