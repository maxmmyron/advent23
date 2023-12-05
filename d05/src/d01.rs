use regex::Regex;

fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let seed_re = Regex::new(r"(\d+)").unwrap();

    // parse seed
    let seeds = seed_re
        .captures_iter(input[0])
        .map(|x| x[1].parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    // parse lines into sections
    let mut source_dest_maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    let mut section = String::new();
    let map_re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

    for line in input[2..input.len()].iter() {
        if line.len() > 1 {
            section += line;
        } else {
            let mut source_dest_map: Vec<(i64, i64, i64)> = Vec::new();

            for (_, [source, dest, range]) in
                map_re.captures_iter(section.as_str()).map(|x| x.extract())
            {
                source_dest_map.push((
                    source.parse::<i64>().unwrap(),
                    dest.parse::<i64>().unwrap(),
                    range.parse::<i64>().unwrap(),
                ));
            }

            source_dest_maps.push(source_dest_map);
            section = String::new();
        }
    }

    let mut min = i64::MAX;

    // map seeds to destinations and store the minimum
    for seed in seeds {
        let mut curr_dest = seed;

        for source_dest_map in &source_dest_maps {
            for map in source_dest_map {
                let source_start = map.1;
                let source_end = map.1 + map.2 - 1;
                let dest_start = map.0;

                if (curr_dest >= source_start) && (curr_dest <= source_end) {
                    curr_dest = dest_start + (curr_dest - source_start);
                    break;
                }
            }
        }

        if curr_dest < min {
            min = curr_dest;
        }
    }

    println!("\n{}", min);
}
