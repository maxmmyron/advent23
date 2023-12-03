# Day 2

## Part 2

### Problem

_cubes of three colors are drawn from a bag. a game is represented as a semicolon-separated list of turns, where each turn is a comma-separated list of drawings from the bag. after each turn, the drawn cubes are returned to the bag. for example:_

```
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
```

_given a list of games, sum the IDs of the games that are possible given the bag contains 12 red, 13 green, and 14 blue cubes._

### Train of thought

i'm not going to be surprised if there is some combinatorics in the second part of today. for now thought, this seems pretty straight forward; it's really just string parsing for this problem. because each turn is entirely independent from the others, we can run the following algorithm:

1. parse the string: track the ID and split the game into turns.
2. for each turn, parse it into a mapping between colored cubes and their count.
3. if any one color draw has a count greater than the number of cubes available, the game is not possible. move on to the next game.
4. if all turns are possible, add the game ID to the sum.

### Implementation

implementation is pretty straightforward. we start similarly to day 1, where we get the input from a file:

```rust
let binding = std::fs::read_to_string("input.txt").unwrap();
let input = binding.split("\n").collect::<Vec<&str>>();
```

we also need to define some tracking variables. we have a sum of valid games, and three variables to store the most cubes of any one color that we could draw on a turn:

```rust
let mut sum = 0;

let max_r = 12;
let max_g = 13;
let max_b = 14;
```

for each game, we need to get two things:

1. the ID of the game, since that's what we're summing,
2. the turns of the game, since that's what determines whether a game is valid or not.

any one game has the general format

```
Game <ID>: <draw> <color>, <draw> <color> <draw> <color>; <draw> <color>, <draw> <color>; ...
```

to get the ID, we can split the game on the colon using the `split()` method, and then split the first element of the resulting vector on the space. That will give us a vector of two elements, where the first element is `"Gmae"` and the second element is `<ID>`. finally, we parse the second element, the ID, into an integer using the `parse()` method.

```rust
let ID = game
  // split the game on the colon, giving us
  // ["Game <ID>", "<draw> <color>, <draw> <color> <draw> <color>; <draw> <color>, <draw> <color>; ..."]
  .split(":").collect::<Vec<&str>>()[0]
  // split the first element of the vector on the space, giving us
  // ["Game", "<ID>"]
  .split(" ").collect::<Vec<&str>>()[1]
  // trim any whitespace from the ID, and parse it into an integer.
  .trim().parse::<i32>()
  // unwrap the result of the parse, since we know it will be valid.
  .unwrap();
```

we can run a very similar line of code to get all of the turns in a single array. since we know that the turns are independent, we can store them all in a single array and iterate over them in one go.

a game is comprised of turns, which are separated by semicolons. any turn is comprised of draws, which are separated by commas. we can get all draws in a single array by splitting along both semicolons and commas:

```rust
let grabs = game
  // split the game on the colon, giving us
  // ["Game <ID>", "<draw> <color>, <draw> <color> <draw> <color>; <draw> <color>, <draw> <color>; ..."]
  .split(":").collect::<Vec<&str>>()[1]
  // trim the whitespace from the second element
  .trim()
  // split the string on both semicolons *and* commas
  .split(|c| c == ';' || c == ',')
  // collect the result into a vector so we can iterate over it
  .collect::<Vec<&str>>();
```

we now have an array of grabs, where a grab is comprised of a number and a color. a grab is valid if the number of cubes of that color is less than or equal to the number of cubes available.

a grab has the format `<number> <color>`, so we can split it on the whitespace.

we can iterate over each grab, and further parse the grabs into a color and amount. we do need to track whether a turn is valid, so we'll toss in a boolean to do that:

```rust
let mut ok = true;
for grab in grabs {
  let amount = grab.trim().split(" ").collect::<Vec<&str>>()[0]
    .parse::<i32>()
    .unwrap();
  let color = grab.trim().split(" ").collect::<Vec<&str>>()[1];

  // ...
}
```

we can then check against each color; if the amount of cubes of that color is greater than the number of cubes available, that game is not valid and we can break out of the loop. if we reach the end of the loop, then the game is valid.

```rust
let mut ok = true;
for grab in grabs {
  let amount = grab.trim().split(" ").collect::<Vec<&str>>()[0]
      .parse::<i32>()
      .unwrap();
  let color = grab.trim().split(" ").collect::<Vec<&str>>()[1];

  if color == "red" && max_r < amount {
      ok = false;
      break;
  } else if color == "green" && max_g < amount {
      ok = false;
      break;
  } else if color == "blue" && max_b < amount {
      ok = false;
      break;
  }
}

if ok {
    sum += ID;
}
```

and that's it! here's the full code:

```rust
fn main() {
  let binding = std::fs::read_to_string("input.txt").unwrap();
  let input = binding.split("\n").collect::<Vec<&str>>();

  let mut sum = 0;

  let max_r = 12;
  let max_g = 13;
  let max_b = 14;

  for game in input {
    if game.len() == 0 {
      break;
    }

    let ID = game.split(":").collect::<Vec<&str>>()[0]
      .split(" ")
      .collect::<Vec<&str>>()[1]
      .trim()
      .parse::<i32>()
      .unwrap();

    let grabs = game.split(":").collect::<Vec<&str>>()[1]
      .trim()
      .split(|c| c == ';' || c == ',')
      .collect::<Vec<&str>>();

    let mut ok = true;
    for grab in grabs {
      let amount = grab.trim().split(" ").collect::<Vec<&str>>()[0]
        .parse::<i32>()
        .unwrap();
      let color = grab.trim().split(" ").collect::<Vec<&str>>()[1];

      if color == "red" && max_r < amount {
        ok = false;
        break;
      } else if color == "green" && max_g < amount {
        ok = false;
        break;
      } else if color == "blue" && max_b < amount {
        ok = false;
        break;
      }
    }

    if ok {
      sum += ID;
    }
  }

  println!("{}", sum);
}
```

## Part 2

### Problem

_each game has a minimum number of cubes that must be present for the game to be valid. for example, the game_

```
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
```

_would require a minimum of 4 red, 6 blue, and 2 green cubes._

_the **power** of a game is the product of the minimum number of cubes of each color. what is the sum of all the powers of a series of games?_

### Train of thought

ok, so there weren't any combinatorics. this seems like a pretty straightforward extension of the previous problem, and we can reuse a lot of the code from the previous problem.

### Implementation

the changes necessary to the code are pretty minimal. we can remove the code that handles the `max_r`, `max_g`, and `max_b` variables, since that isn't a part of the problem anymore. likewise, we can remove the code that parses the ID, and the code that checks whether a grab is valid.

we'll add three variables before we loop over each grab: `min_r`, `min_g`, and `min_b`. these will track the minimum number of cubes of each color that we need to draw in order for the game to be valid. for each grab, we'll check if the grabs of a certain color is greater than the minimum number of cubes of that color. if it is, we'll update the minimum number of cubes of that color.

after a game, we'll sum the product of the minimum number of cubes of each color, and add it to the sum.

```rust
let mut min_r = 0;
let mut min_g = 0;
let mut min_b = 0;

for grab in grabs {
  let amount = grab.trim().split(" ").collect::<Vec<&str>>()[0]
    .parse::<i32>()
    .unwrap();
  let color = grab.trim().split(" ").collect::<Vec<&str>>()[1];

  if color == "red" && amount > min_r {
    min_r = amount;
  } else if color == "green" && amount > min_g {
    min_g = amount;
  } else if color == "blue" && amount > min_b {
    min_b = amount;
  }
}

sum += min_r * min_g * min_b;
```

again, here's the full code:

```rust
fn main() {
  let binding = std::fs::read_to_string("input.txt").unwrap();
  let input = binding.split("\n").collect::<Vec<&str>>();

  let mut sum = 0;

  for game in input {
    if game.len() == 0 {
      break;
    }

    let grabs = game.split(":").collect::<Vec<&str>>()[1]
      .trim()
      .split(|c| c == ';' || c == ',')
      .collect::<Vec<&str>>();

    let mut min_r = 0;
    let mut min_g = 0;
    let mut min_b = 0;

    for grab in grabs {
      let amount = grab.trim().split(" ").collect::<Vec<&str>>()[0]
        .parse::<i32>()
        .unwrap();
      let color = grab.trim().split(" ").collect::<Vec<&str>>()[1];

      if color == "red" && amount > min_r {
        min_r = amount;
      } else if color == "green" && amount > min_g {
        min_g = amount;
      } else if color == "blue" && amount > min_b {
        min_b = amount;
      }
    }

    sum += min_r * min_g * min_b;
  }

  println!("{}", sum);
}
```
