fn main() {
    let curr_time = std::time::Instant::now();

    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let mut map_sections: Vec<Vec<&str>> = Vec::new();
    let mut curr_section: Vec<&str> = Vec::new();

    // parse seeds
    let seeds = input[0].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    // parse lines into section
    for line in input[1..input.len() - 1].iter() {
        if line.len() == 1 {
            if !curr_section.is_empty() {
                map_sections.push(curr_section.clone());
                curr_section.clear();
            }
        } else {
            curr_section.push(line.trim());
        }
    }

    if !curr_section.is_empty() {
        map_sections.push(curr_section);
    }

    let mut source_dest_maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();

    // construct mappings
    for section in map_sections {
        let mut source_dest_map: Vec<(i64, i64, i64)> = Vec::new();

        for map_str in section[1..].iter() {
            let tuple = map_str
                .split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            source_dest_map.push((tuple[0], tuple[1], tuple[2]));
        }

        source_dest_maps.push(source_dest_map);
    }

    let mut min = i64::MAX;

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

    let elapsed_time = curr_time.elapsed();
    println!("\n{}", min);
    println!(
        "Elapsed: {}us",
        elapsed_time.as_secs() * 1_000_000 + elapsed_time.subsec_micros() as u64
    );
}
