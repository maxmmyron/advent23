use regex::Regex;

fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let num_re = Regex::new(r"(\d+)").unwrap();

    let times = num_re
        .captures_iter(input[0])
        .map(|x| x[1].parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let best = num_re
        .captures_iter(input[1])
        .map(|x| x[1].parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let mut mult = 1;

    for i in 0..times.len() {
        let t = times[i];
        let b = best[i];

        let rt = (t.powf(2.) - 4. * b).sqrt();

        let high = (t + rt) / 2.;
        let low = (t - rt) / 2.;

        let diff = high.ceil() as i64 - low.floor() as i64 - 1;

        mult *= diff;
    }

    println!("{}", mult);
}
