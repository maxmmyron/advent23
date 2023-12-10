fn main() {
    let time = std::time::Instant::now();
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let mut sum = 0;
    for line in input {
        if line.len() == 0 {
            continue;
        }

        // splitting line and creating vector takes ~6-12us
        let split_line = line.trim().split(" ").collect::<Vec<&str>>();
        let mut dp: Vec<Vec<Option<i32>>> = vec![vec![None; split_line.len()]; split_line.len()];

        for i in 0..split_line.len() {
            dp[0][i] = Some(split_line[i].parse::<i32>().unwrap());
        }

        // indexing via arr[n]: nth derivative of function
        // indexing via arr[n][m]: value of f^n(m)

        for deriv in 1..split_line.len() {
            for val in (deriv..split_line.len()).rev() {
                let diff = Some(dp[deriv - 1][val].unwrap() - dp[deriv - 1][val - 1].unwrap());
                dp[deriv][val] = diff;
            }
        }

        let mut next = 0;

        for i in 0..split_line.len() {
            if dp[i][dp[i].len() - 1].is_none() {
                break;
            }

            next += dp[i][dp[i].len() - 1].unwrap();
        }

        sum += next;
    }

    println!("Sum: {}", sum);
    println!("Time: {}us", time.elapsed().as_micros());
}
