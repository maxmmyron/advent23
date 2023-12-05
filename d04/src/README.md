# Day 2

## Part 1: Problem

_a scratchcard is composed of a series of winning numbers and chosen numbers. count the matches between winning and chosen numbers, subtract one, and raise that number to the second power to get the score of a card. for example:_

```
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
```

_There are four matches, meaning the score is (4-1)^2 = 16_

_find the sum of a pile of scratchcards._

## Part 1: Train of thought

alright, this hopefully shouldn't be too bad (especially compared to yesterday's problem).

here's the plan (for each line):

1. read in the file, and parse the winning numbers and chosen numbers into their own vectors.
2. filter the chosen numbers vector to only include numbers that are also in the winning numbers vector.
3. if the chosen numbers vector is empty, skip to the next line.
4. otherwise, subtract 1 from the length of the chosen numbers vector, add 1 bit-shifted left by that number to the sum.

## Part 1: Implementation

i'm going to start skipping over the file-reading stuff, since it's been the same for a few days. You can find the code for that in my posts for [day 1](https://mmyron.com/posts/advent23_1), [day 2](https://mmyron.com/posts/advent23_2), and [day 3](https://mmyron.com/posts/advent23_3).

we'll start by setting an overall sum variable to 0:

```rust
let mut sum = 0;
```

for each line, we're going to parse the winning numbers from chosen numbers. a scratchcard has the following format:

```
card <n>: winning numbers | chosen numbers
```

we'll split the line on the `|` character, which will give us the first half of the line, and a string of chosen numbers:

```rust
for line in input {
  if line.len() == 0 {
    continue;
  }

  // format: "card <n>: <winning numbers>"
  let winners = line.split("|").collect::<Vec<&str>>()[0].trim();

  // format: "<chosen numbers>"
  let mut chosen = line.split("|").collect::<Vec<&str>>()[1].trim();
}
```

note that `chosen` is a mutable variable, because we're going to be modifying it in a bit.

to get the winning numbers, we'll split the first half of the line on the `:` character, and take the second half of that split. then, we'll split along the whitespace and collect the result into a vector. we'll keep these as strings for now:

```rust
let winners = line.split("|").collect::<Vec<&str>>()[0].trim()  // "card <n>: <winning numbers>"
  .split(":").collect::<Vec<&str>>()[1].trim()                  // "<winning numbers>"
  .split(" ").collect::<Vec<&str>>();                           // ["<num>", "<num>", ...]
```

we'll do the same for the chosen numbers, minus the intermediate step:

```rust
let mut chosen = line.split("|").collect::<Vec<&str>>()[1].trim()   // "<chosen numbers>"
  .split(" ").collect::<Vec<&str>>();                           // ["<num>", "<num>", ...]
```

because we only care about the chosen numbers that appear in the winning numbers, we'll filter the chosen numbers vector to only include numbers that exist in the `winners` vector. we can use the `retain` method to do this, as it will _keep_ any elements that match our predicate, and remove any that don't (i find this to be ever-so slightly more readable than `filter`):

```rust
chosen.retain(|choice| winners.contains(choice) && choice.len() > 0);
```

we're filtering the `chosen` vector on two conditions:

1. the `winners` vector contains the number
2. the number is not an empty string

the problem's input file is _very_ nicely formatted, and each number column is right-aligned. however, this means we're splitting two spaces in a row at times, which will occasionally give us an empty string. we don't want to count empty strings as matches, so we'll filter them out.

our `chosen` vector now only contains numbers that are also in the `winners` vector. if the vector is empty, we know there were no matches, so we can skip to the next line:

```rust
if chosen.len() == 0 {
  continue;
}
```

however, if there was a match, then we can calculate the score!

the first number match is worth 1 point, with every subsequent math doubling the score. this is very reminiscent of binary counting:

```
128 64 32 16 8 4 2 1
  0  0  0  0 0 0 0 0
```

if the `chosen` array contains matches, then we'll start with a score of 1, bit-shift it left by the number of matches minus 1, and add that to the sum:

```rust
if chosen.len() == 0 {
  continue;
}
sum += 1 << (chosen.len() - 1);
```

and that's actually the entire solution! here's the full code:

```rust
let binding = std::fs::read_to_string("input.txt").unwrap();
let input = binding.split("\n").collect::<Vec<&str>>();

let mut sum = 0;

for line in input {
  if line.len() == 0 {
    continue;
  }

  let winners = line.split("|").collect::<Vec<&str>>()[0]
    .trim()
    .split(":")
    .collect::<Vec<&str>>()[1]
    .trim()
    .split(" ")
    .collect::<Vec<&str>>();
  let mut chosen = line.split("|").collect::<Vec<&str>>()[1]
    .trim()
    .split(" ")
    .collect::<Vec<&str>>();

  chosen.retain(|choice| winners.contains(choice) && choice.len() > 0);

  if chosen.len() == 0 {
    continue;
  }
  sum += 1 << (chosen.len() - 1);
}

println!("{}", sum);
```

## Part 2: Problem

\_the score of a scratchcard determines how many copes it makes. if card `n` creates `m` copies, then there will be a copy of card `n+1`, card `n+2`, and so on, up to card `n+m`. each of these duplicates are scored the same way.

## Part 2: Train of thought

something we can keep in mind is that a card only generate duplicates for cards that come after it. also, duplicates and originals are treated the same, so we can effectively run the same code when determining how many duplicates a card generates as we did when calculating the score.

we'll need to keep track of how many duplicates previous cards have made, but our code can otherwise remain pretty unchanged.

## Part 2: Implementation

we'll define a variable called `copies` to track the number of cards we have for each line. we'll also initialize this variable to have `input.len() - 1` elements all set to 1.

_(NOTE: we're subtracting 1 from the length of the input because the last line is empty)_

```rust
let mut copies: Vec<i32> = Vec::new();

for _ in 0..input.len() - 1 {
  copies.push(1);
}
```

i'm going to switch the for loop we had before to an enumerated for loop, so we can also extract the index of the line we're on for free. this will allow us to calculate which cards get duplicates:

```rust
for (idx, line) in input.iter().enumerate() {
  // ...
}
```

for each line, we'll calculate the winning number vector, chosen number vector, and filter the `chosen` vector the same way we did to part 1.

the number of chosen numbers that match the winning numbers represents how many duplicates we are generating.

if card `X` has two matches, then card `X+1` and `X+2` will get duplicates.

however, if card `X` itself has, say, 3 duplicates (so there's four total `X` cards), then card `X+1` will have 4 duplicates, and card `X+2` will have 4 duplicates.

by the time we're calculating the number of duplicates to delegate out based on the matches on card `X`, we already know how many duplicates card `X` has of itself. we can add the number of duplicates card `X` has to the number of duplicates card `X+1` has, and to the number of duplicates card `X+2` has, and so on.

we'll start by making a loop over the number of matches:

```rust
for j in 1..chosen.len()+1 {
  // ...
}
```

we're iterating from 1 to the number of matches + 1, just to keep the internal logic a little cleaner.

then, we'll index into the `copies` given the current index (from the enumerated for loop) and `j`, and add the number of duplicates that the current card has.

```rust
for j in 1..chosen.len() + 1 {
  copies[idx + j] += copies[idx];
}
```

over time, this will build out our vector with the total number of duplicates each card has (including itself).

we'll finally calculate the sum by iterating over the `copies` vector and adding each element to the sum:

```rust
for copy in copies {
  sum += copy;
}
```

luckily, the changes were pretty minimal. here's the full code:

```rust
let binding = std::fs::read_to_string("input.txt").unwrap();
let input = binding.split("\n").collect::<Vec<&str>>();

let mut sum = 0;

let mut copies: Vec<i32> = Vec::new();

for _ in 0..input.len() - 1 {
  copies.push(1);
}

for (idx, line) in input.iter().enumerate() {
  if line.len() == 0 {
    continue;
  }

  let winners = line.split("|").collect::<Vec<&str>>()[0]
    .trim()
    .split(":")
    .collect::<Vec<&str>>()[1]
    .trim()
    .split(" ")
    .collect::<Vec<&str>>();
  let mut chosen = line.split("|").collect::<Vec<&str>>()[1]
    .trim()
    .split(" ")
    .collect::<Vec<&str>>();

  chosen.retain(|choice| winners.contains(choice) && choice.len() > 0);

  if chosen.len() == 0 {
    continue;
  }

  for j in 1..chosen.len() + 1 {
    copies[idx + j] += copies[idx];
  }
}

for copy_count in copies {
  sum += copy_count;
}

println!("{}", sum);
```
