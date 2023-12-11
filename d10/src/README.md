# Day 10

## Part 1: Problem

_a map of pipes looks similar to the following:_

```
.....
.S-7.
.|.|.
.L-J.
.....
```

_where `.` is an empty space, `S` is the starting point, `|` and `-` are vertical or horizontal pipes (respectively), and `F`, `7`, `L`, and `J` are corner pipes. Find the number of steps required to get as far away from S as possible._

## Part 1: Train of thought

from the [problem description on the AoC site](), we can assume the pipe is a single loop.

to solve this, we can start by finding S, picking the first pipe that S connects to, and following along the pipe until we hit S again. then, the furthest point from S is the overall distance divided by 2.

we can assign a direction to move in for each pipe, given the direction we've traveled from. for example, if we hit a `F` pipe by traveling upwards, we will start moving to the right. similarly, if we hit `-` pipe, we will continue moving in the same direction.

we can form a small table of this behavior:

| pipe | from ↑ | from → | from ↓ | from ← |
| ---- | ------ | ------ | ------ | ------ |
| `¦`  | ↑      | N/A    | ↓      | N/A    |
| `-`  | N/A    | →      | N/A    | ←      |
| `F`  | →      | N/A    | N/A    | ↓      |
| `7`  | ←      | ↓      | N/A    | N/A    |
| `L`  | N/A    | N/A    | →      | ↑      |
| `J`  | N/A    | ↑      | ←      | N/A    |

The `N/A` values here represent directions we shouldn't be coming from. For example, if we hit a `¦` pipe from the left, we know we've made a mistake, because we should only be coming from the top or bottom.

so, we'll parse the current pipe, and the direction we're coming from, and then look up the next direction in the table.

## Part 1: Implementation

let's parse the input into a 2D array of characters split on empty characters. we'll collect the initial input into a vector of strings, and then map those strings into a vector of characters. splitting on the empty character will also add some extraneous empty strings on the front and end, so we'll remap the vector to remove those.

```rust
let input = binding.split("\n").collect::<Vec<&str>>()[0..binding.split("\n").count() - 1]
  .iter()
  // split each string into a vector of characters
  .map(|x| x.trim().split("").collect::<Vec<&str>>())
  .collect::<Vec<Vec<&str>>>()
  .iter()
  // remove the first and last empty strings
  .map(|x| x[1..x.len() - 1].to_vec())
  .collect::<Vec<Vec<&str>>>();
```

after we map the input, we'll have something similar to:

```
[".", ".", ".", ".", "."]
[".", "S", "-", "7", "."]
[".", "|", ".", "|", "."]
[".", "L", "-", "J", "."]
[".", ".", ".", ".", "."]
```

we need to find the starting point, so we'll just iterate over the input:

```rust
let mut i = 0;
let mut j = 0;

for (index, row) in input.iter().enumerate() {
  for (index2, col) in row.iter().enumerate() {
    if col == &"S" {
      i = index;
      j = index2;
    }
  }
}
```

cool! so now "S" is stored as `(i,j)`. we need to find a starting direction to move in now. we know that "S" is at `input[i][j]`, and that "S" only has two pipes that lead into it. we can just loop through each of the possible directions, and see if the next character is a related pipe.

- if we're looking to the north of "S" (i-1, j), then we are looking for a "|", "F", or "7" pipe.
- if we're looking to the east of "S" (i, j+1), then we are looking for a "-", "J", or "7" pipe.
- if we're looking to the south of "S" (i+1, j), then we are looking for a "|", "L", or "J" pipe.
- if we're looking to the west of "S" (i, j-1), then we are looking for a "-", "F", or "L" pipe.

we'll store the direction as an integer, where 0 is north, 1 is east, 2 is south, and 3 is west.

```rust
// 0: N, 1: E, 2: S, 3: W
let mut direction = 4;

if input[i - 1][j] == "|" || input[i - 1][j] == "F" || input[i - 1][j] == "7" {
  direction = 0;
} else if input[i][j + 1] == "-" || input[i][j + 1] == "J" || input[i][j + 1] == "7" {
  direction = 1;
} else if input[i + 1][j] == "|" || input[i + 1][j] == "L" || input[i + 1][j] == "J" {
  direction = 2;
} else if input[i][j - 1] == "-" || input[i][j - 1] == "F" || input[i][j - 1] == "L" {
  direction = 3;
}
```

let's take a look at the earlier table; i'll reformat it so we use our integer directions:

| pipe | from 0 | from 1 | from 2 | from 3 |
| ---- | ------ | ------ | ------ | ------ |
| `¦`  | 0      | 4      | 2      | 4      |
| `-`  | 4      | 1      | 4      | 3      |
| `F`  | 1      | 4      | 4      | 2      |
| `7`  | 3      | 2      | 4      | 4      |
| `L`  | 4      | 4      | 1      | 0      |
| `J`  | 4      | 0      | 3      | 4      |

for the `N/A` values, we'll just use 4. we can use this table to find the next direction to move in.

real quick: notice the "|" and "-" pipes. these are special cases, because they don't change the direction we're moving in. we'll handle these cases first. we can then simplify our table, since if we encounter a straight pipe we can keep the same direction.

| pipe | from 0 | from 1 | from 2 | from 3 |
| ---- | ------ | ------ | ------ | ------ |
| `F`  | 1      | 4      | 4      | 2      |
| `7`  | 3      | 2      | 4      | 4      |
| `L`  | 4      | 4      | 1      | 0      |
| `J`  | 4      | 0      | 3      | 4      |

we can implement this table as a `HashMap`. let's call it `mover`, since it moves us in a direction and i am shit at naming things:

```rust
let mut mover = HashMap::new();

mover.insert("F", [1, 4, 4, 2]);
mover.insert("7", [3, 2, 4, 4]);
mover.insert("L", [4, 4, 1, 0]);
mover.insert("J", [4, 0, 3, 4]);
```

now we have our initial position, direction, and a map that we can consult to traverse the pipe loop. let's start moving!

we'll initialize a mutable variable to track our moves, and start a generic `loop`:

```rust
let mut moves = 0;

loop {
  // ...
}
```

we'll start by moving in the direction we're currently meant to move in. we can use a `match` statement here:

```rust
match direction {
  0 => i -= 1,
  1 => j += 1,
  2 => i += 1,
  3 => j -= 1,
  _ => panic!("invalid direction"),
}
```

we'll panic if we somehow hit the `4` in our HashMap.

after we update `i` or `j`, we'll increment the move counter, and check if the new pipe we're on is the end (i.e "S"):

```rust
moves += 1;

if input[i][j] == "S" {
  break;
}
```

if we aren't on "S", then we need to update our direction based on the new pipe type. if the pipe type _isn't_ "|" or "-", then we'll consult the `mover` map we constructed earlier. we use the pipe type to retrieve the array of possible directions, and then index into that array using our current direction. this will give us the next direction to move in.

```rust
if input[i][j] != "|" && input[i][j] != "-" {
  direction = mover.get(input[i][j]).unwrap()[direction];
}
```

and then we'll loop back around and move again.

once we finally break the loop by matching an "S", we'll just print the number of moves, divided by 2:

```rust
println!("furthest point is {} steps away", moves / 2);
```

and that's it! here's the full code:

```rust
use std::collections::HashMap;

fn main() {
  let binding = std::fs::read_to_string("input.txt").unwrap();
  let input = binding.split("\n").collect::<Vec<&str>>()[0..binding.split("\n").count() - 1]
    .iter()
    .map(|x| x.trim().split("").collect::<Vec<&str>>())
    .collect::<Vec<Vec<&str>>>()
    .iter()
    .map(|x| x[1..x.len() - 1].to_vec())
    .collect::<Vec<Vec<&str>>>();

  //find first "S" in binding
  let mut i = 0;
  let mut j = 0;

  for (index, row) in input.iter().enumerate() {
    for (index2, col) in row.iter().enumerate() {
      if col == &"S" {
        i = index;
        j = index2;
      }
    }
  }

  // 0: N, 1: E, 2: S, 3: W
  let mut direction = 4;

  if input[i - 1][j] == "|" || input[i - 1][j] == "F" || input[i - 1][j] == "7" {
    direction = 0;
  } else if input[i][j + 1] == "-" || input[i][j + 1] == "J" || input[i][j + 1] == "7" {
    direction = 1;
  } else if input[i + 1][j] == "|" || input[i + 1][j] == "L" || input[i + 1][j] == "J" {
    direction = 2;
  } else if input[i][j - 1] == "-" || input[i][j - 1] == "F" || input[i][j - 1] == "L" {
    direction = 3;
  }

  let mut mover = HashMap::new();
  mover.insert("F", [1, 4, 4, 2]);
  mover.insert("7", [3, 2, 4, 4]);
  mover.insert("L", [4, 4, 1, 0]);
  mover.insert("J", [4, 0, 3, 4]);

  let mut moves = 0;
  loop {
    match direction {
      0 => i -= 1,
      1 => j += 1,
      2 => i += 1,
      3 => j -= 1,
      _ => panic!("invalid direction"),
    }

    moves += 1;

    if input[i][j] == "S" {
      break;
    }

    if input[i][j] != "|" && input[i][j] != "-" {
      direction = mover.get(input[i][j]).unwrap()[direction];
    }
  }

  println!("furthest point is {} steps away", moves / 2);
}
```

## Part 2: Problem

_find the number of cells that are surrounded by the pipe loop. for example:_

```
.....
.S-7.
.|.|.
.L-J.
.....
```

_has 1 cell surrounded by the pipe loop._

## Part 2: Train of thought

this one was a head scratcher for me. i pretty much immediately thought of a flood-fill algorithm, but i knew there had to be an easier way.

imagine the following piece of a pipe:

```
............
..F------7..
..|......|..
..|......|..
..L------J..
............
```

imagine scanning the pipe from left to right, top to bottom. the pipe is a loop, so every time we cross the boundary of one side, we can be certain we'll cross the boundary of the other side. in fact, we can be certain that, over a row, we'll cross the boundary of the pipe an even number of times.

so, if we've crossed an odd number of times, we're inside the pipe. if we're inside the area contained by the pipe, and we scan a character that isn't a part of the network, then we can add that to our interior area!

if we scan from left to right, we can just focus on pipes where we move in a clockwise direction. for example:

```
..F------7..
```

we first scan "F", which rotates our direction 90° clockwise (from north to east). later, we scan "7", which rotates our direction 90° clockwise again (from east to south). likewise:

```
..|------|..
```

if we scan the first "|", we scan a pipe that goes north. we know that later, we'll need to be going south. when we scan the second "|", we'll be going south, which completes the loop. on the other hand:

```
.. L------J..
```

scanning from left to right, we scan "L", which rotates our direction 90° _counter-clockwise_ (from south to east or west to north). likewise, scanning "J" rotates our direction 90° _counter-clockwise_ again (from east to north or west to south). we can actually ignore these pipes, since they don't change our direction in a clockwise manner.

actually, we only need to pay attention to any "|", "F", or "7" pipes. let's overlay the directions on our pipe network:

```
..............
..↑------↓..
..↑......↓..
..↑......↓..
.. L------J..
```

as we move from left to right, we'll cross the boundary of the pipe loop. if we've crossed an odd number of times, and we scan either a pipe not in the network or any empty space, we can add that to our interior area. if we've crossed an even number of times, we can just ignore the pipe. we can also ignore the pipe if it exists in the network.

## Part 2: Implementation

before we find where "S" is, we'll initialize a mutable vector to track which pipes are in our network:

```rust
let mut relevant: Vec<Vec<usize>> = vec![];
```

then, we can find "S", and add its position to our `relevant` vector:

```rust
// ...

for (index, row) in input.iter().enumerate() {
  relevant.push(vec![]);
  for (index2, col) in row.iter().enumerate() {
    if col == &"S" {
      i = index;
      j = index2;
      relevant[index].push(j);
    }
  }
}
```

"S" is a special character, so we need to figure our what type of pipe it actually is. we can do this as we are parsing the initial direction, since that already tells us 1 out of the 2 pipes that connect to "S".

we only need to know if "S" is a "|", "F", or "7" pipe, so we can store that state in a bool:

```rust
let mut s_vert = false;

if input[i - 1][j] == "|" || input[i - 1][j] == "F" || input[i - 1][j] == "7" {
  direction = 0;
  if input[i + 1][j] == "|" || input[i + 1][j] == "J" || input[i + 1][j] == "L" {
    s_vert = true;
  }
} else if input[i][j + 1] == "-" || input[i][j + 1] == "J" || input[i][j + 1] == "7" {
  direction = 1;
  if input[i + 1][j] == "|" || input[i + 1][j] == "J" || input[i + 1][j] == "L" {
    s_vert = true;
  }
} else if input[i + 1][j] == "|" || input[i + 1][j] == "L" || input[i + 1][j] == "J" {
  direction = 2;
  if input[i - 1][j] == "|" || input[i - 1][j] == "J" || input[i - 1][j] == "L" {
    s_vert = true;
  }
} else if input[i][j - 1] == "-" || input[i][j - 1] == "F" || input[i][j - 1] == "L" {
  direction = 3;
  if input[i + 1][j] == "|" || input[i + 1][j] == "J" || input[i + 1][j] == "L" {
    s_vert = true;
  }
}
```

NOTE: this is extremely hacky! if "S" exists on the edge of `input.txt`, it will break very badly :(
luckily, AoC inputs are very neat and tidy, so this will work for our input.

the only change we need to make in our loop is appending the current position to our `relevant` vector:

```rust
loop {
  // ...

  moves += 1;
  relevant[i].push(j);

  // ...
}
```

after we break this loop, `relevant` will be a 2D array with the positions of all pipes in our network. now, we can loop over the original input (from top to bottom, left to right), check if we're inside or outside the pipe boundary, and add to our interior if we are, and the character position we're reading in isn't a part of the network.

we'll initialize a mutable variable to track the interior area:

```rust
let mut area = 0;
```

then, we loop over the input from top to bottom, left to right. for each row, we'll initialize a mutable variable to track whether we're inside the boundary or not:

```rust
for idx in 0..input.len() {
  let mut within = false;
  for jdx in 0..input[idx].len() {
    // ...
  }
}
```

if the position `(idx, jdx)` is in our `relevant` vector, then we can check the pipe type. we need to check for two conditions here:

1. the character is a "|", "F", or "7" pipe
2. the character is "S", and the `s_vert` bool is true

if either of these conditions are true, then we can set swap the `within` bool, since we've just crossed the boundary of the pipe.

```rust
if relevant[idx].contains(&jdx) {
  let pipe = input[idx][jdx];
  if pipe == "|" || pipe == "7" || pipe == "F" || (pipe == "S" && s_vert) {
    within = !within;
  }
} else {
  // ...
}
```

if the position _isn't_ within the `relevant` vector, then we can check if we're inside the boundary, and add to our interior area if we are:

```rust
if relevant[idx].contains(&jdx) {
  // ...
} else {
  if within {
      area += 1;
  }
}
```

here's the snipped for that whole loop. like the above code, it's pretty shaky; and will 100% break if the input isn't neatly formatted:

```rust
let mut area = 0;
for idx in 0..input.len() {
  let mut within = false;
  for jdx in 0..input[idx].len() {
    if relevant[idx].contains(&jdx) {
      let pipe = input[idx][jdx];
      if pipe == "|" || pipe == "7" || pipe == "F" || (pipe == "S" && s_vert) {
        within = !within;
      }
    } else {
      if within {
        area += 1;
      }
    }
  }
}
```

finally, we print the area:

```rust
println!("area is {}", area);
```

a much harder part 2 (compared to part 1), but still extremely fun! here's the full code:

```rust
use std::collections::HashMap;

fn main() {
  let binding = std::fs::read_to_string("input.txt").unwrap();
  let input = binding.split("\n").collect::<Vec<&str>>()[0..binding.split("\n").count() - 1]
    .iter()
    .map(|x| x.trim().split("").collect::<Vec<&str>>())
    .collect::<Vec<Vec<&str>>>()
    .iter()
    .map(|x| x[1..x.len() - 1].to_vec())
    .collect::<Vec<Vec<&str>>>();

  let mut relevant: Vec<Vec<usize>> = vec![];

  //find first "S" in binding
  let mut i = 0;
  let mut j = 0;

  for (index, row) in input.iter().enumerate() {
    relevant.push(vec![]);
    for (index2, col) in row.iter().enumerate() {
      if col == &"S" {
        i = index;
        j = index2;
        relevant[index].push(j);
      }
    }
  }

  // 0: N, 1: E, 2: S, 3: W
  let mut direction = 4;
  let mut s_vert = false;

  // WARNING: this is a hacky solution and does not account for cases where S
  // exists at the edge of the map
  if input[i - 1][j] == "|" || input[i - 1][j] == "F" || input[i - 1][j] == "7" {
    direction = 0;
    if input[i + 1][j] == "|" || input[i + 1][j] == "J" || input[i + 1][j] == "L" {
      s_vert = true;
    }
  } else if input[i][j + 1] == "-" || input[i][j + 1] == "J" || input[i][j + 1] == "7" {
    direction = 1;
    if input[i + 1][j] == "|" || input[i + 1][j] == "J" || input[i + 1][j] == "L" {
      s_vert = true;
    }
  } else if input[i + 1][j] == "|" || input[i + 1][j] == "L" || input[i + 1][j] == "J" {
    direction = 2;
    if input[i - 1][j] == "|" || input[i - 1][j] == "J" || input[i - 1][j] == "L" {
      s_vert = true;
    }
  } else if input[i][j - 1] == "-" || input[i][j - 1] == "F" || input[i][j - 1] == "L" {
    direction = 3;
    if input[i + 1][j] == "|" || input[i + 1][j] == "J" || input[i + 1][j] == "L" {
      s_vert = true;
    }
  }

  let mut mover = HashMap::new();
  mover.insert("F", [1, 4, 4, 2]);
  mover.insert("7", [3, 2, 4, 4]);
  mover.insert("L", [4, 4, 1, 0]);
  mover.insert("J", [4, 0, 3, 4]);

  let mut moves = 0;
  loop {
    match direction {
      0 => i -= 1,
      1 => j += 1,
      2 => i += 1,
      3 => j -= 1,
      _ => panic!("invalid direction"),
    }

    moves += 1;
    relevant[i].push((j));

    if input[i][j] == "S" {
      break;
    }

    if input[i][j] != "|" && input[i][j] != "-" {
      direction = mover.get(input[i][j]).unwrap()[direction];
    }
  }

  // WARNING: also shaky
  let mut area = 0;
  for idx in 0..input.len() {
    let mut within = false;
    for jdx in 0..input[idx].len() {
      if relevant[idx].contains(&jdx) {
        let pipe = input[idx][jdx];
        if pipe == "|" || pipe == "7" || pipe == "F" || (pipe == "S" && s_vert) {
          within = !within;
        }
      } else {
        if within {
          area += 1;
        }
      }
    }
  }

  println!("furthest point is {}; area is {}", moves / 2, area);
}
```
