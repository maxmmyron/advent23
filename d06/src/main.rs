use regex::Regex;

fn main() {
    let now = std::time::Instant::now();
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let num_re = Regex::new(r"(\d+)").unwrap();

    let time = num_re
        .captures_iter(input[0])
        .map(|x| x[1].chars().collect::<Vec<_>>())
        .flatten()
        .collect::<String>()
        .parse::<f64>()
        .unwrap();

    let best = num_re
        .captures_iter(input[1])
        .map(|x| x[1].chars().collect::<Vec<_>>())
        .flatten()
        .collect::<String>()
        .parse::<f64>()
        .unwrap();

    let rt = (time.powf(2.) - 4. * best).sqrt();

    let high = (time + rt) / 2.;
    let low = (time - rt) / 2.;

    let diff = high.ceil() as i64 - low.floor() as i64 - 1;

    let elapsed = now.elapsed();
    println!("{}", diff);
    println!(
        "Elapsed: {}ms",
        elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64
    );
}
