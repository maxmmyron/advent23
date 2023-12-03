fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let mut sum = 0;

    let patterns = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in input {
        if line.len() == 0 {
            break;
        }

        let mut curr_find_idx = line.len();
        let mut tens: usize = 0;
        let mut ones: usize = 0;

        let mut curr_rfind_idx = 0;

        for i in 0..patterns.len() + 1 {
            if i == patterns.len() {
                let find_idx = line.find(char::is_numeric);
                let rfind_idx = line.rfind(char::is_numeric);

                if find_idx.is_some() {
                    if find_idx.unwrap() <= curr_find_idx {
                        curr_find_idx = find_idx.unwrap();
                        let str_val = line.chars().nth(curr_find_idx).unwrap();
                        let int_val = str_val.to_string().parse::<usize>().unwrap();
                        tens = int_val * 10;
                    }
                }

                if rfind_idx.is_some() {
                    if rfind_idx.unwrap() >= curr_rfind_idx {
                        curr_rfind_idx = rfind_idx.unwrap();
                        let str_val = line.chars().nth(curr_rfind_idx).unwrap();
                        let int_val = str_val.to_string().parse::<usize>().unwrap();
                        ones = int_val;
                    }
                }

                break;
            }

            let find_idx = line.find(patterns[i]);
            let rfind_idx = line.rfind(patterns[i]);

            if find_idx.is_some() {
                if find_idx.unwrap() <= curr_find_idx {
                    curr_find_idx = find_idx.unwrap();
                    tens = (i + 1) * 10;
                }
            }

            if rfind_idx.is_some() {
                if rfind_idx.unwrap() >= curr_rfind_idx {
                    curr_rfind_idx = rfind_idx.unwrap();
                    ones = i + 1;
                }
            }
        }

        sum += tens + ones;
    }

    println!("{}", sum);
}
