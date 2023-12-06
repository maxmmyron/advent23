use regex::Regex;

// NOTE: this will take forever to run!
fn main() {
    let time = std::time::Instant::now();

    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let seed_re = Regex::new(r"(\d+)").unwrap();

    // parse seed
    let mut seeds = seed_re
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
    for idx in (0..seeds.len()).step_by(2) {
        let seed_start = seeds[idx];
        let seed_range = seeds[idx + 1];
        let seed_end = seed_start + seed_range - 1;

        // when we form splits, we'll add them here.
        // after we parse every source_dest_map, we'll mutate the original seed map.
        let mut new_seed_map = Vec::new();

        for source_dest_map in &source_dest_maps {
            for map in source_dest_map {
                let source_start = source_dest_map[0].1;
                let source_range = source_dest_map[0].2;
                let source_end = source_start + source_range - 1;

                if (source_start >= seed_start && source_start <= seed_end) {
                    // split at source start, transform seed map so seed start is at dest start
                    let seed_split_a = (seed_start, source_start - 1);
                    let seed_split_b = (
                        source_dest_map[0].0,
                        source_dest_map[0].0 + source_range - 1,
                    );
                }
                if (source_end <= seed_end && source_end >= seed_start) {
                    // split at source end + 1, transform seed map so seed end is at dest end
                    let seed_split_a = (seed_start, source_end + 1);
                    let seed_split_b = (
                        source_dest_map[0].0,
                        source_dest_map[0].0 + source_range - 1,
                    );
                }
                if source_start < seed_start && source_end > seed_end {
                    // entire seed map lies within mapping region, so we can just shift the seed start to the dest start
                    new_seed_map.push((source_dest_map[0].0, seed_start));
                }
                if {
                  // entire seed map lies outside
                }
            }
        }

        // after going through every map section, we can parse the final seed map and take the lowest even indexed value

        // for seed in start..=end {
        //     let mut curr_dest = seed;

        //     for source_dest_map in &source_dest_maps {
        //         for map in source_dest_map {
        //             let source_start = map.1;
        //             let source_end = map.1 + map.2 - 1;
        //             let dest_start = map.0;

        //             if (curr_dest >= source_start) && (curr_dest <= source_end) {
        //                 curr_dest = dest_start + (curr_dest - source_start);
        //                 break;
        //             }
        //         }
        //     }

        //     if curr_dest < min {
        //         min = curr_dest;
        //     }
        // }
    }

    let elapsed_ms = time.elapsed().as_millis();
    println!("\n{}", min);
    println!("\nElapsed: {} ms", elapsed_ms);
}
