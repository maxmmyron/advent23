use std::collections::HashMap;

fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let mut sections: Vec<Vec<&str>> = Vec::new();
    let mut current_section: Vec<&str> = Vec::new();

    // parse seeds
    let seeds = input[0].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for line in input[1..input.len() - 1].iter() {
        if line.len() == 1 {
            if !current_section.is_empty() {
                sections.push(current_section.clone());
                current_section.clear();
            }
        } else {
            current_section.push(line.trim());
        }
    }

    if !current_section.is_empty() {
        sections.push(current_section);
    }

    let mut seed_soil: HashMap<i64, i64> = HashMap::new();
    let mut soil_fert: HashMap<i64, i64> = HashMap::new();
    let mut fert_water: HashMap<i64, i64> = HashMap::new();
    let mut water_light: HashMap<i64, i64> = HashMap::new();
    let mut light_temp: HashMap<i64, i64> = HashMap::new();
    let mut temp_humid: HashMap<i64, i64> = HashMap::new();
    let mut humid_loc: HashMap<i64, i64> = HashMap::new();

    let mut maps = Vec::from([
        &mut seed_soil,
        &mut soil_fert,
        &mut fert_water,
        &mut water_light,
        &mut light_temp,
        &mut temp_humid,
        &mut humid_loc,
    ]);

    for (idx, section) in sections.iter().enumerate() {
        for mapping in section[1..].iter() {
            let mapping = mapping
                .split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            for i in 0..mapping[2] {
                maps[idx].insert(mapping[1] + i, mapping[0] + i);
            }
        }
    }

    let mut min = i64::MAX;
    for seed in seeds {
        let mut dest = seed;
        let mut val_ref;

        print!("{} ", seed);

        for map in &maps {
            val_ref = map.get(&dest);
            if val_ref.is_some() {
                print!("-> {} ", *val_ref.unwrap());
                dest = *val_ref.unwrap();
            }
        }

        if dest < min {
            min = dest;
        }
    }

    println!("\n{}", min);
}
