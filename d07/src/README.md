# Day 7

## Part 1: Problem

_A hand is five cards, and each hand has an associated bid._

_The rank of a card is the best construction possible given any pairs:_

| rank            | weight | example |
| --------------- | ------ | ------- |
| high card       | 1      | TJ567   |
| pair            | 2      | 55T67   |
| two pair        | 3      | 2233A   |
| three of a kind | 4      | 999J4   |
| full house      | 5      | 55522   |
| four of a kind  | 6      | 22226   |
| five of a kind  | 7      | KKKKK   |

_Any two cards can be compare to one another with the following two rules:_

1. _if the ranks are not the same, the card with the higher rank is better._
2. _if the ranks are the same, compare the nth card in one hand to the nth card in another. the deck with the first higher-rank card is better._

_given a list of cards:_

```
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
```

_we can order them like so:_

```
32T3K 765
KTJJT 220
KK677 28
T55J5 684
QQQJA 483
```

given these cards and their bids, we can calculate an overall score by summing the product of the bid and the rank:

$$
765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5 = 6440
$$

## Part 1: Train of thought

each rank has two unique properties that we can use to our advantage:

1. the pair count
2. the overall unique card count

this pair/unique count is unique for each rank, so we can just count the pairs and unique cards in each rank, and then assign its rank based on that.

after we assign each card a rank, we can just loop through and sort them based on the two rules describe din the problem.

## Part 1: Implementation

we'll start by defining a few vectors and map to hold:

- an array containing the order of cards (such that the index of the card + 1 is its rank)
- a map that maps a hand's (pair count, unique count) to its rank

```rust
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
```

we'll also define a vector of tuples that will hold a card, its bid, and its rank:

```rust
let mut hands: Vec<(&str, i32, i32)> = Vec::new();
```

for each line in the input, we'll split it into its hand and bid components:

```rust
for line in input {
  // break if EOF
  if line.len() == 0 {
    continue;
  }

  let hand = line.split(' ').collect::<Vec<&str>>()[0].trim();
  let bid_str = line.split(' ').collect::<Vec<&str>>()[1].trim()

  let bid = bid_str.parse::<i32>().unwrap();
}
```

cards are grouped into pairs if they are the same card, so we can count the number of unique card by building out some Sets (or hashmaps, in this case). We'll make a set of the number of unique cards, and a set of the number of pairs:

```rust
let mut uniques: HashMap<char, i32> = std::collections::HashMap::new();
let mut pairs = std::collections::HashMap::new();
```

for each card in a set, we run the following algorithm:

- if the card is not in the `uniques` set, add it
- if the card _is_ in the `uniques` set, then we know we have at least one other card of the same rank. we'll add it to the `pairs` set.

```rust
for card in hand.chars() {
    if uniques.contains_key(&card) {
        pairs.insert(card, 1);
        continue;
    } else {
        uniques.insert(card, 1);
    }
}
```

now that we have the number of unique cards and pairs, we can look up the rank of the hand in the `rank_map`:

```rust
let hand_rank = rank_map.get(&(pairs.len(), uniques.len())).unwrap();
```

and finally add a tuple composed of the hand, its bid, and its rank to the `hands` vector:

```rust
hands.push((hand, bid, *hand_rank));
```

after we've finished constructing the `hands` vector, we can sort it using the two rules described in the problem:

- if the ranks are not the same, the card with the higher rank is better.
- if the ranks are the same, compare the nth card in one hand to the nth card in another. the deck with the first higher-rank card is better.

```rust
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
```

finally, we can just sum the product of the bid and the rank for each hand:

```rust
let mut sum: i64 = 0;
for (idx, hand_tuple) in hands.iter().enumerate() {
  sum += (idx as i64 + 1) * hand_tuple.1 as i64;
}

println!("{}", sum);
```

here's the full code:

```rust
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
    sum += (idx as i64 + 1) * hand_tuple.1 as i64;
  }

  println!("{}", sum);
}
```

## Part 2: Problem

_the "J" card now describes a joker card, not a jack. when calculating the rank, the J cards are used to construct the best possible hand rank. When comparing two cards to one another, the J card ranks lowest (even lower than 2)._

## Part 2: Train of thought

for this, we only need to change a few things. we need to update our card_arr to account for the new value of a joker, and we need to add some logic to our rank calculation to account for the joker.

Given the hand `2JJT3`, we can remove the jokers and calculate the rank of the hand as if it were `2T3`. In this case, it's a high card. When we add the two jokers back, the best possible hand would involve the two jokers and the `T` (i.e. a single card), so the new rank of the hand with the jokers is a three of a kind.

we can build out a messy, but functional, algorithm to calculate the rank of a hand with jokers. if a hand--stripped of its jokers--has a match in the `rank_map`, then we can just use that rank (that map is built assuming no jokers). If it doesn't exist, we can just calculate the new rank based on the `joker_count` and the number of existing pairs.

if a hand does _not_ have a match in the `rank_map`, then it has one or more joker cards. we can use this branching logic to calculate the rank of the hand:

```rust
if pairs.len() == 0 {
  if joker_count == 5 || joker_count == 4 {
    hands.push((hand, bid, 6));
  } else if joker_count == 3 {
    hands.push((hand, bid, 5));
  } else if joker_count == 2 {
    hands.push((hand, bid, 3));
  } else if joker_count == 1 {
    hands.push((hand, bid, 1));
  } else {
    hands.push((hand, bid, 0));
  }
} else if pairs.len() == 1 {
  if joker_count == 3 {
    hands.push((hand, bid, 6));
  } else if joker_count == 2 {
    hands.push((hand, bid, 5));
  } else if joker_count == 1 {
    hands.push((hand, bid, 3));
  }
} else if pairs.len() == 2 {
  if joker_count == 1 {
    hands.push((hand, bid, 4));
  }
}
```

definitely not pretty, but it works.

## Part 2: Implementation

we'll rewrite our `card_arr` to account for the new value of a joker:

```rust
let card_arr = vec![
  'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];
```

when we iterate over a line, we'll store the number of jokers in the hand, and we won't factor that card into the initial rank calculation:

```rust
let mut joker_count = 0;

for card in hand.chars() {
  // if the card is a joker, add to joker count and skip
  if card == 'J' {
    joker_count += 1;
    continue;
  }

  if uniques.contains_key(&card) {
    pairs.insert(card, 1);
    continue;
  }

  uniques.insert(card, 1);
}
```

after we have the number of unique cards, pairs, and jokers, we can calculate the actual rank of the hand. we'll use rusts `match` statement to do this:

```rust
match rank_map.get(&(pairs.len(), uniques.len())) {
  Some(hand_rank) => hands.push((hand, bid, *hand_rank)),
  None => {
    if pairs.len() == 0 {
      if joker_count == 5 || joker_count == 4 {
        hands.push((hand, bid, 6));
      } else if joker_count == 3 {
        hands.push((hand, bid, 5));
      } else if joker_count == 2 {
        hands.push((hand, bid, 3));
      } else if joker_count == 1 {
        hands.push((hand, bid, 1));
      } else {
        hands.push((hand, bid, 0));
      }
    } else if pairs.len() == 1 {
      if joker_count == 3 {
        hands.push((hand, bid, 6));
      } else if joker_count == 2 {
        hands.push((hand, bid, 5));
      } else if joker_count == 1 {
        hands.push((hand, bid, 3));
      }
    } else if pairs.len() == 2 {
      if joker_count == 1 {
        hands.push((hand, bid, 4));
      }
    }
  }
}
```

we can use our original sorting algorithm to sort the hands, and keep the summing algorithm as well.

in fact, those are the only things we had to change. here's the full code:

```rust
use std::collections::HashMap;

fn main() {
  let binding = std::fs::read_to_string("input.txt").unwrap();
  let input = binding.split("\n").collect::<Vec<&str>>();

  let card_arr = vec![
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
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
    let mut joker_count = 0;

    for card in hand.chars() {
      // if the card is a joker, add to joker count and skip
      if card == 'J' {
        joker_count += 1;
        continue;
      }
      if uniques.contains_key(&card) {
        pairs.insert(card, 1);
        continue;
      } else {
        uniques.insert(card, 1);
      }
    }

    match rank_map.get(&(pairs.len(), uniques.len())) {
      Some(hand_rank) => hands.push((hand, bid, *hand_rank)),
      None => {
        if pairs.len() == 0 {
          if joker_count == 5 || joker_count == 4 {
            hands.push((hand, bid, 6));
          } else if joker_count == 3 {
            hands.push((hand, bid, 5));
          } else if joker_count == 2 {
            hands.push((hand, bid, 3));
          } else if joker_count == 1 {
            hands.push((hand, bid, 1));
          } else {
            hands.push((hand, bid, 0));
          }
        } else if pairs.len() == 1 {
          if joker_count == 3 {
            hands.push((hand, bid, 6));
          } else if joker_count == 2 {
            hands.push((hand, bid, 5));
          } else if joker_count == 1 {
            hands.push((hand, bid, 3));
          }
        } else if pairs.len() == 2 {
          if joker_count == 1 {
            hands.push((hand, bid, 4));
          }
        }
      }
    }
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
    sum += (idx as i64 + 1) * hand_tuple.1 as i64;
  }

  println!("{}", sum);
}
```

if i spent more time on this i probably could have improved the algorithm for calculating the rank of a hand with jokers, but it works for now (and i have finals to study for).
