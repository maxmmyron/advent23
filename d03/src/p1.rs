fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let mut nums: Vec<Vec<(i32, (i32, i32))>> = Vec::new();
    let mut symbols: Vec<Vec<(i32, i32)>> = Vec::new();

    fn is_symbol(c: char) -> bool {
        c == '*'
            || c == '%'
            || c == '-'
            || c == '='
            || c == '#'
            || c == '@'
            || c == '$'
            || c == '/'
            || c == '+'
            || c == '&'
    }

    let mut sum = 0;

    for i in 0..input.len() {
        nums.push(Vec::new());
        symbols.push(Vec::new());

        let line = input[i];
        if line.len() == 0 {
            break;
        }

        let mut num = 0;
        let mut num_scale = 1;
        for j in (0..line.len()).rev() {
            let parsed_symbol = is_symbol(line.chars().nth(j).unwrap());
            let parsed_num = line.chars().nth(j).unwrap().to_digit(10);

            if parsed_num.is_some() {
                num += parsed_num.unwrap() as i32 * num_scale;
                num_scale *= 10;
            } else {
                if parsed_symbol {
                    symbols[i].push((i as i32, j as i32));
                }
                // num != 0, so we just finished parsing a number
                if num != 0 {
                    nums[i].push((num, (i as i32, (j as i32) + 1)));
                    num = 0;
                    num_scale = 1;
                }
            }

            if j == 0 && num != 0 {
                nums[i].push((num, (i as i32, (j as i32))));
            }
        }
    }

    for j in 0..nums.len() {
        let mut relevant_symbols = Vec::new();
        for symbol in symbols[j].iter() {
            relevant_symbols.push(symbol);
        }
        if j != 0 {
            for symbol in symbols[j - 1].iter() {
                relevant_symbols.push(symbol);
            }
        }
        if j != nums.len() - 1 {
            for symbol in symbols[j + 1].iter() {
                relevant_symbols.push(symbol);
            }
        }

        for num_pair in &nums[j] {
            let num = num_pair.0;
            let min_pos = (num_pair.1 .0 - 1, num_pair.1 .1 - 1);
            let max_pos = (
                num_pair.1 .0 + 1,
                num_pair.1 .1 + num.to_string().len() as i32,
            );

            let mut ok = false;
            for sym_pos in relevant_symbols.iter() {
                if sym_pos.0 <= max_pos.0 && sym_pos.0 >= min_pos.0 {
                    if sym_pos.1 <= max_pos.1 && sym_pos.1 >= min_pos.1 {
                        ok = true;
                        break;
                    }
                }
            }

            if ok {
                sum += num;
            }
        }
    }

    print!("{} ", sum);
}
