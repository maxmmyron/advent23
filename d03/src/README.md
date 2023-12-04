# Day 3

## Part 1

### Problem

_a 2D representation of an engine is composed of periods, number values, and symbols. a valid number is a number in the engine schematic that also has a symbol adjacent to it. for example:_

```
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```

_467 is valid, but 114 isn't. find the sum of all valid numbers in the engine schematic._

### Train of thought

i wouldn't call this trivial by any means, but it's definitely not as hard as you'd think. there are two primary things we need to do here:

1. parse numbers from each line
2. check if each number is valid.

there are two approaches i could go about solving this:

1. parse the numbers from each line, and check every surrounding cell for a symbol.
2. compact the overall schematic into two arrays: one of `(number, position)` tuples, and one of `(symbol, position)` tuples. then, check if each number has a symbol adjacent to it.

i think that the second approach would probably be better, so i'm going to go with that.

a few years ago, i built out a [game engine in TypeScript](https://github.com/maxmmyron/slce). it wasn't very good, but i do remember implementing collision detection. this problem reminds me a lot of that! if we "pad" the physical space a number takes up with a 1-unit border, we get something adjacent to a box:

```
......
.8483.
......
```

we can use [AABB collision detection](https://en.wikipedia.org/wiki/Minimum_bounding_box#Axis-aligned_minimum_bounding_box) to figure out whether a symbol is adjacent to a number. we need to know the upper-left and lower-right coordinates, or bounds, of the box surrounding the number. we also need to know the position of the symbol.

given the position of the starting digit, it's pretty trivial to get the top-left and lower-right bounds of this. we can subtract 1 from the `x` and `y` coordinates of the starting digit to get the top-left bound, and add the length of the number to `x` and 1 to `y` to get the lower-right bound.

we can think of our asterisk as a point, since it's just a single character. it's "adjacent" to a number if it's within the bounds of the number's padded box:

```
............ ..........._.. <--- not adjacent
....8483.... .....8483.....
........_... <--- adjacent ..............

```

given a digit, if any nearby symbol fails the AABB test (i.e. it's within the bounds of the number's padded box), then the number is adjacent to a symbol, and we'll add it to the list.

### Implementation

let's start off by noting that there are several types of symbols we can encounter. we'll quickly create a small function to check if a character is one of these symbols:

```rust
fn is_symbol(c: char) -> bool {
  c == '*'
    || c == '%'
    || c == '-'
    || c == '='
    || c == '#'
    || c == '@'
    || c == '$'
    || c == '/'
    || c == '+'
    || c == '&'
}
```

we'll also need to keep track of the numbers, symbols, and their positions. we'll use a vector of vectors for this so we can easily iterate over each line:

```rust
let mut nums: Vec<Vec<(i32, (i32, i32))>> = Vec::new();
let mut symbols: Vec<Vec<(i32, i32)>> = Vec::new();
```

we don't need to know exactly what symbol is what, so we'll just keep the `symbols` vector as a vector of `(x, y)` tuples.

we'll also need to keep track of the sum of all valid numbers:

```rust
let mut sum = 0;
```

now, we can start parsing the input. for each line, we'll add new vectors to our `nums` and `symbols` vectors, and begin iterating over each character in the line:

```rust
for i in 0..input.len() {
  nums.push(Vec::new());
  symbols.push(Vec::new());

  let line = input[i];
  if line.len() == 0 {
    break;
  }

  for j in (0..line.len()).rev() {

  }
}
```

note how we're iterating over the characters in the line in reverse. i'm doing this to make it easier to parse numbers, since we can just multiply the digit by a power of 10 to get the correct value. likewise, we won't need to track the start coordinate of the number; we're working in reverse, so the start coordinate is just the last number before a non-digit character.

at this step we'll add two mutable variables:

1. `num`, which will be the number we're currently parsing
2. `num_scale`, which will be the power of 10 we multiply the digit by. we'll start this at 1, and multiply it by 10 every time we parse another digit.

we have to check our character to see if its a symbol or a number:

```rust
let mut num = 0;
let mut num_scale = 1;
for j in (0..line.len()).rev() {
  let parsed_symbol = is_symbol(line.chars().nth(j).unwrap());
  let parsed_num = line.chars().nth(j).unwrap().to_digit(10);

  // ...
}
```

if it's a number, we'll multiply it by `num_scale` and add it to `num`. then, we'll multiply `num_scale` by 10:

```rust
let mut num = 0;
let mut num_scale = 1;
for j in (0..line.len()).rev() {
  let parsed_symbol = is_symbol(line.chars().nth(j).unwrap());
  let parsed_num = line.chars().nth(j).unwrap().to_digit(10);

  if parsed_num.is_some() {
    num += parsed_num.unwrap() as i32 * num_scale;
    num_scale *= 10;
  }
}
```

if it isn't a number, that means the current character is either a symbol, or a non-symbol. it also means that we _may_ have finished parsing a number. we need to check for both the case where we've successfully parsed a symbol, and the case where we've finished parsing a number. these cases are not mutually exclusive, so we'll use an `if` statement for both:

```rust
let mut num = 0;
let mut num_scale = 1;
for j in (0..line.len()).rev() {
  let parsed_symbol = is_symbol(line.chars().nth(j).unwrap());
  let parsed_num = line.chars().nth(j).unwrap().to_digit(10);

  if parsed_num.is_some() {
    num += parsed_num.unwrap() as i32 * num_scale;
    num_scale *= 10;
  } else {
    if parsed_symbol {
      // we've parsed a symbol
    }
    if num != 0 {
      // we've finished parsing a number
    }
  }
}
```

if we've parsed a symbol, we'll add its position to the `symbols` vector:

```rust
if parsed_symbol {
  symbols[i].push((i as i32, j as i32));
}
```

in day 1 i mentioned not casting `usize` variables to `i32` since we lose precision. here, we're doing exactly that because the benefit we get down the line is worth the precision loss (which in this case isn't even an issue).

if we've finished parsing a number, we'll add it, and its position, to the `nums` vector; and we'll reset `num` and `num_scale`:

```rust
if num != 0 {
  nums[i].push((num, (i as i32, (j as i32) + 1)));
  num = 0;
  num_scale = 1;
}
```

so, here's the full snippet for parsing the input:

```rust
for i in 0..input.len() {
  nums.push(Vec::new());
  symbols.push(Vec::new());

  let line = input[i];
  if line.len() == 0 {
    break;
  }

  let mut num = 0;
  let mut num_scale = 1;
  for j in (0..line.len()).rev() {
    let parsed_symbol = is_symbol(line.chars().nth(j).unwrap());
    let parsed_num = line.chars().nth(j).unwrap().to_digit(10);

    if parsed_num.is_some() {
      num += parsed_num.unwrap() as i32 * num_scale;
      num_scale *= 10;
    } else {
      if parsed_symbol {
        symbols[i].push((i as i32, j as i32));
      }
      if num != 0 {
        nums[i].push((num, (i as i32, (j as i32) + 1)));
        num = 0;
        num_scale = 1;
      }
    }
  }
}
```

now, we can start checking if each number is adjacent to a symbol. we'll start by iterating over each line.

```rust
for j in 0..nums.len() {
  // ...
}
```

for a symbol to be adjacent to a number, it can only be on the same line, or the line above or below it. we'll create a vector of all the relevant symbols by pushing the symbols on the current line, the line above it, and the line below it to the vector:

```rust
for j in 0..nums.len() {
  let mut relevant_symbols = Vec::new();
  for symbol in symbols[j].iter() {
    relevant_symbols.push(symbol);
  }
  if j != 0 {
    for symbol in symbols[j - 1].iter() {
      relevant_symbols.push(symbol);
    }
  }
  if j != nums.len() - 1 {
    for symbol in symbols[j + 1].iter() {
      relevant_symbols.push(symbol);
    }
  }

  // ...
}
```

then, we can iterate over each number/position pair in the current line (`j`). we'll run a loop and extract some important information about the number and its bounding box.

in the train of thought, i mentioned that we'd use AABB collision detection, and here we're going to define the bounds of the box.

`min_pos` is just the position we originally stored, with 1 subtracted from the `x` and `y` components. `max_pos` is the position with the length of the number added to the `x` component, and 1 added to the `y` component:

```rust
for num_pair in &nums[j] {
  let num = num_pair.0;
  let min_pos = (num_pair.1 .0 - 1, num_pair.1 .1 - 1);
  let max_pos = (
    num_pair.1 .0 + 1,
    num_pair.1 .1 + num.to_string().len() as i32,
  );
}
```

now, we can iterate over the relevant symbols, and check if they're within the bounds of the number's padded box. a symbol is within the bounds of the box if its position is simultaneously less than or equal to the `max_pos` and greater than or equal to the `min_pos`. we'll also track if we've found a symbol, so we can break out of the loop early:

```rust
// ...
let mut ok = false;
for sym_pos in relevant_symbols.iter() {
  if sym_pos.0 <= max_pos.0 && sym_pos.0 >= min_pos.0 {
    if sym_pos.1 <= max_pos.1 && sym_pos.1 >= min_pos.1 {
      ok = true;
      break;
    }
  }
}
// ...
```

then, we add the number to the sum if `ok` is true:

```rust
if ok {
  sum += num;
}
```

and that's actually it! i found the parsing to be the most difficult part of this problem, since it requires a fair bit of thought and planning (and i'm still getting quite used to rust's relative lack of string indexing). here's the full code for part 1:

```rust
fn main() {
  let binding = std::fs::read_to_string("input.txt").unwrap();
  let input = binding.split("\n").collect::<Vec<&str>>();

  let mut nums: Vec<Vec<(i32, (i32, i32))>> = Vec::new();
  let mut symbols: Vec<Vec<(i32, i32)>> = Vec::new();

  fn is_symbol(c: char) -> bool {
    c == '*'
      || c == '%'
      || c == '-'
      || c == '='
      || c == '#'
      || c == '@'
      || c == '$'
      || c == '/'
      || c == '+'
      || c == '&'
  }

  let mut sum = 0;

    // generate nums and symbols
  for i in 0..input.len() {
    nums.push(Vec::new());
    symbols.push(Vec::new());

    let line = input[i];
    if line.len() == 0 {
      break;
    }

    let mut num = 0;
    let mut num_scale = 1;
    for j in (0..line.len()).rev() {
      let parsed_symbol = is_symbol(line.chars().nth(j).unwrap());
      let parsed_num = line.chars().nth(j).unwrap().to_digit(10);

      if parsed_num.is_some() {
        num += parsed_num.unwrap() as i32 * num_scale;
        num_scale *= 10;
      } else {
        if parsed_symbol {
          symbols[i].push((i as i32, j as i32));
        }
        // num != 0, so we just finished parsing a number
        if num != 0 {
          nums[i].push((num, (i as i32, (j as i32) + 1)));
          num = 0;
          num_scale = 1;
        }
      }

      if j == 0 {
        if num != 0 {
          nums[i].push((num, (i as i32, (j as i32))));
        }
      }
    }
  }

  for j in 0..nums.len() {
    let mut relevant_symbols = Vec::new();
    for symbol in symbols[j].iter() {
      relevant_symbols.push(symbol);
    }
    if j != 0 {
      for symbol in symbols[j - 1].iter() {
        relevant_symbols.push(symbol);
      }
    }
    if j != nums.len() - 1 {
      for symbol in symbols[j + 1].iter() {
        relevant_symbols.push(symbol);
      }
    }

    for num_pair in &nums[j] {
      let num = num_pair.0;
      let min_pos = (num_pair.1 .0 - 1, num_pair.1 .1 - 1);
      let max_pos = (
        num_pair.1 .0 + 1,
        num_pair.1 .1 + num.to_string().len() as i32,
      );

      let mut ok = false;
      for sym_pos in relevant_symbols.iter() {
        if sym_pos.0 <= max_pos.0 && sym_pos.0 >= min_pos.0 {
          if sym_pos.1 <= max_pos.1 && sym_pos.1 >= min_pos.1 {
            ok = true;
            break;
          }
        }
      }

      if ok {
        sum += num;
      }
    }
  }

  print!("{} ", sum);
}

```

## Part 2

### Problem

_a **gear** is an asterisk (`*`) symbol that is adjacent to exactly two numbers. the **gear ratio** is the product of the two numbers a gear is adjacent to. find the sum of all gear ratios._

### Train of thought

luckily, this is pretty similar to the last problem, and our idea of mapping numbers and symbols to their positions set us up for some success! we might need to swap around some nested for-loops, but otherwise we can keep most of our code from part 1.

### Implementation

because we're only working with "gears" here, we can modify our `is_symbol()` function to only care about the `*` character;

```rust
fn is_gear(c: char) -> bool {
  c == '*'
}
```

and we'll update our symbol position vector accordingly:

```rust
let mut gears: Vec<Vec<(i32, i32)>> = Vec::new();

// ...

let parsed_gear = is_gear(line.chars().nth(j).unwrap());

// ...
```

our code for building out our vectors are otherwise the same.

in the first part, our focus was on the validity of a number. we checked against all of the surrounding symbols for a given number to see if it was valid. in this part, our focus shifts to the validity of a gear. we're going to swap around our nested for-loops so we can keep focus on a single gear, since we want to count the adjacent numbers:

```rust
for j in 0..gears.len() {
  // ...

  for gear in &gears[j] {
    // ...
    for num_pair in &relevant_nums {
      // ...
    }

    // ...
  }
}
```

we'll build out the same "relevant" vector, but we'll work with our number/positions pairs, instead of our symbol positions:

```rust
for j in 0..gears.len() {
  let mut relevant_nums = Vec::new();
  for num in nums[j].iter() {
      relevant_nums.push(num);
  }
  if j != 0 {
      for num in nums[j - 1].iter() {
          relevant_nums.push(num);
      }
  }
  if j != nums.len() - 1 {
      for num in nums[j + 1].iter() {
          relevant_nums.push(num);
      }
  }

  for gear in &gears[j] {
    // ...
    for num_pair in &relevant_nums {
      // ...
    }

    // ...
  }
}
```

in the first part, we checked for general number validity based on the present of one or more symbols. here, because we're checking if a gear has a specific number of adjacent numbers, we'll track them using a vector:

```rust
for gear in &gears[j] {
  let mut adjacent_count = Vec::new();

  for num_pair in &relevant_nums {
    // ...
  }

  // ...
}
```

for every relevant num pair, we can run the exact same AABB padded collision test. if it matches, we just add that number to the adjacent count vector:

```rust
for gear in &gears[j] {
  let mut adjacent_count = Vec::new();

  for num_pair in &relevant_nums {
    let num = num_pair.0;
    let min_pos = (num_pair.1 .0 - 1, num_pair.1 .1 - 1);
    let max_pos = (
        num_pair.1 .0 + 1,
        num_pair.1 .1 + num.to_string().len() as i32,
    );

    if gear.0 <= max_pos.0 && gear.0 >= min_pos.0 {
        if gear.1 <= max_pos.1 && gear.1 >= min_pos.1 {
            adjacent_count.push(num);
        }
    }
  }

  // ...
}
```

then, we finally just check if the length of the vector is exactly 2. if it is, we add the product of those two vector elements to our sum:

```rust
for gear in &gears[j] {
  let mut adjacent_count = Vec::new();

  for num_pair in &relevant_nums {
    let num = num_pair.0;
    let min_pos = (num_pair.1 .0 - 1, num_pair.1 .1 - 1);
    let max_pos = (
        num_pair.1 .0 + 1,
        num_pair.1 .1 + num.to_string().len() as i32,
    );

    if gear.0 <= max_pos.0 && gear.0 >= min_pos.0 {
        if gear.1 <= max_pos.1 && gear.1 >= min_pos.1 {
            adjacent_count.push(num);
        }
    }
  }

  if adjacent_count.len() == 2 {
    sum += adjacent_count[0] * adjacent_count[1];
  }
}
```

luckily part 2 really wasn't that bad! part 1 was definitely frustrating at first, but it really just comes down to figuring out exactly what data structure you want to boil down the input to.

here's the full code:

```rust
fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input = binding.split("\n").collect::<Vec<&str>>();

    let mut nums: Vec<Vec<(i32, (i32, i32))>> = Vec::new();
    let mut gears: Vec<Vec<(i32, i32)>> = Vec::new();

    fn is_gear(c: char) -> bool {
        c == '*'
    }

    let mut sum = 0;

    // generate nums and gears
    for i in 0..input.len() {
        nums.push(Vec::new());
        gears.push(Vec::new());

        let line = input[i];
        if line.len() == 0 {
            break;
        }

        let mut num = 0;
        let mut num_scale = 1;
        for j in (0..line.len()).rev() {
            let parsed_gear = is_gear(line.chars().nth(j).unwrap());
            let parsed_num = line.chars().nth(j).unwrap().to_digit(10);

            if parsed_num.is_some() {
                num += parsed_num.unwrap() as i32 * num_scale;
                num_scale *= 10;
            } else {
                if parsed_gear {
                    gears[i].push((i as i32, j as i32));
                }
                // num != 0, so we just finished parsing a number
                if num != 0 {
                    nums[i].push((num, (i as i32, (j as i32) + 1)));
                    num = 0;
                    num_scale = 1;
                }
            }

            if j == 0 {
                if num != 0 {
                    nums[i].push((num, (i as i32, (j as i32))));
                }
            }
        }
    }

    for j in 0..gears.len() {
        let mut relevant_nums = Vec::new();
        for num in nums[j].iter() {
            relevant_nums.push(num);
        }
        if j != 0 {
            for num in nums[j - 1].iter() {
                relevant_nums.push(num);
            }
        }
        if j != nums.len() - 1 {
            for num in nums[j + 1].iter() {
                relevant_nums.push(num);
            }
        }

        for gear in &gears[j] {
            let mut adjacent_count = Vec::new();
            for num_pair in &relevant_nums {
                let num = num_pair.0;
                let min_pos = (num_pair.1 .0 - 1, num_pair.1 .1 - 1);
                let max_pos = (
                    num_pair.1 .0 + 1,
                    num_pair.1 .1 + num.to_string().len() as i32,
                );

                if gear.0 <= max_pos.0 && gear.0 >= min_pos.0 {
                    if gear.1 <= max_pos.1 && gear.1 >= min_pos.1 {
                        adjacent_count.push(num);
                    }
                }
            }

            if adjacent_count.len() == 2 {
                sum += adjacent_count[0] * adjacent_count[1];
            }
        }
    }

    print!("{}", sum);
}
```
