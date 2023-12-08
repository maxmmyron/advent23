use std::collections::HashMap;

fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let card_arr = vec![
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    let mut rank_map = std::collections::HashMap::new();
    rank_map.insert((0, 5), 0);
    rank_map.insert((1, 4), 1);
    rank_map.insert((2, 3), 2);
    rank_map.insert((1, 3), 3);
    rank_map.insert((2, 2), 4);
    rank_map.insert((1, 2), 5);
    rank_map.insert((1, 1), 6);

    let mut hands = Vec::new();

    for line in input {
        if line.len() == 0 {
            continue;
        }

        let hand = line.split(' ').collect::<Vec<&str>>()[0].trim();
        let bid_str = line.split(' ').collect::<Vec<&str>>()[1].trim();

        let bid = bid_str.parse::<i32>().unwrap();

        let mut uniques: HashMap<char, i32> = std::collections::HashMap::new();
        let mut pairs = std::collections::HashMap::new();

        for card in hand.chars() {
            if uniques.contains_key(&card) {
                pairs.insert(card, 1);
                continue;
            } else {
                uniques.insert(card, 1);
            }
        }

        let hand_rank = rank_map.get(&(pairs.len(), uniques.len())).unwrap();
        hands.push((hand, bid, *hand_rank));
    }

    hands.sort_by(|a, b| {
        if a.2.cmp(&b.2) == std::cmp::Ordering::Equal {
            for idx in 0..a.0.len() {
                let a_card = a.0.chars().collect::<Vec<char>>()[idx];
                let b_card = b.0.chars().collect::<Vec<char>>()[idx];

                let a_card_rank = card_arr.iter().position(|&r| r == a_card).unwrap() + 1;
                let b_card_rank = card_arr.iter().position(|&r| r == b_card).unwrap() + 1;

                if a_card_rank > b_card_rank {
                    return std::cmp::Ordering::Greater;
                } else if a_card_rank < b_card_rank {
                    return std::cmp::Ordering::Less;
                }
            }
        }

        return a.2.cmp(&b.2);
    });

    let mut sum: i64 = 0;
    for (idx, hand_tuple) in hands.iter().enumerate() {
        println!("{} {} {}", hand_tuple.0, hand_tuple.1, hand_tuple.2);
        sum += (idx as i64 + 1) * hand_tuple.1 as i64;
    }

    println!("{}", sum);
}
