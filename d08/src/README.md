# Day 8

## Part 1: Problem

_a network is comprised of **nodes**, where each node has a label, a right direction that points to another node, and a left direction that points to another node._

_a map contains—alongside a list of nodes—a line of **instructions**: a series of "R" and "L" moves that indicate which direction to move in the network on each step:_

```
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
```

_starting at the AAA node, count the number of steps it takes to reach ZZZ by following the instructions._

## Part 1: Train of thought

so, let's start by running through the example real quick. We start at `AAA`, and the first instruction is to go left. The left leaf on the `AAA` node is `BBB`, so we travel there:

| step | node | instruction | next node |
| ---- | ---- | ----------- | --------- |
| 0    | AAA  | L           | BBB       |

now, we're at `BBB`, and we increment the step counter. The next instruction is to go left. the left leaf on the `BBB` node tells us to go to `AAA`:

| step | node | instruction | next node |
| ---- | ---- | ----------- | --------- |
| 0    | AAA  | L           | BBB       |
| 1    | BBB  | L           | AAA       |

now we're back to `AAA`. this time, we go right, which points to `BBB`.

| step | node | instruction | next node |
| ---- | ---- | ----------- | --------- |
| 0    | AAA  | L           | BBB       |
| 1    | BBB  | L           | AAA       |
| 2    | AAA  | R           | BBB       |

at this point, we've exhausted the instructions and we're not at ZZZ. luckily, we can just repeat the sequence. From `BBB`, we go left to reach `AAA`. Then, from `AAA`, we go left to reach `BBB`. finally, we go right from `BBB` to reach `ZZZ`.

| step | node | instruction | next node |
| ---- | ---- | ----------- | --------- |
| 0    | AAA  | L           | BBB       |
| 1    | BBB  | L           | AAA       |
| 2    | AAA  | R           | BBB       |
| 3    | BBB  | L           | AAA       |
| 4    | AAA  | L           | BBB       |
| 5    | BBB  | R           | ZZZ       |
| 6    | ZZZ  |             |           |

once we reach `ZZZ`, we're done. we've taken 6 steps to reach `ZZZ`.

this problem smells very much like a graph traversal problem, so that's exactly what we'll do. we'll build a graph, then find the `AAA` node (ideally, we'll just store it as we're constructing the graph), then we'll traverse the graph by following the instructions until we reach `ZZZ`.

## Part 1: Implementation

an interesting thing to node is that [recursive structs are not allowed in rust](https://stackoverflow.com/a/25296420/9473692). this means we can't directly create a `Node` struct like:

```rust
struct Node {
    label: &str,
    left: Option<Node>,
    right: Option<Node>,
}
```

luckily, we don't even really need to do this, since we have the entire graph built out in `input.txt`! we'll keep track of the node we're currently on with an index.

let's take a look at the example file again:

```
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
```

we'll extract the instruction from the first line, and split that along an empty string to get a vector of strings. we'll also define a HashMap, so we can get an easy lookup between the node label and the index of the node's definition in the input file.

```rust
let instructions = input[0].trim().split("").collect::<Vec<&str>>();
// remove first and last instruction (since they're empty strings)
let instructions = instructions[1..(instructions.len() - 1)].to_vec();

let mut nodes = HashMap::new();
```

we need a way to parse a given line of the input file into a tuple of `(label, left, right)`. we'll start with a regex:

```rust
let node_re = Regex::new(r"(?<label>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();
```

we'll define a function that takes a line and the regex, and returns a tuple of `(label, left, right)`. we're passing the regex in as an argument so we can avoid recompiling it every time we call the function (we only need to `clone()` it):

```rust
fn parse_line(line: &str, regex: regex::Regex) -> (&str, &str, &str) {
    let Some(caps) = regex.captures(line) else {
        panic!("Invalid node: {}", line);
    };

    let label = caps.name("label").unwrap().as_str();
    let left = caps.name("left").unwrap().as_str();
    let right = caps.name("right").unwrap().as_str();

    return (label, left, right);
}
```

we'll iterate over the input file that defines the struct ($[2, \text{len(input)} - 1]$) and map the node label to the index of the node.

```rust
for idx in 2..(input.len() - 1) {
  let line = input[idx].trim();

  let (label, left, right) = parse_line(line, node_re.clone());

  nodes.insert(label, idx);
}
```

after we have built our map, we'll define three variables to track some details:

- `count`: the current number of steps we've taken
- `current`: the label of the node we're currently on
- `current_index`: the index of the node we're currently on

```rust
let mut count = 0;
let mut current = "AAA";
let mut current_index = nodes.get(current).unwrap();
```

then, we'll begin a loop and get both the current instruction and line. for the instruction, we need to take the modulo of the current count and the length of the instructions, since we'll be repeating the instructions over and over again if we haven't reached `ZZZ` yet.

```rust
loop {
  let instruction = instructions[count % instructions.len()];
  let line = input[*current_index].trim();
}
```

we'll then parse the line into a tuple of `(label, left, right)`, and check if the current label is `ZZZ`. if it is, we'll break out of the loop. otherwise, we'll increment the count since we're about to take a step:

```rust
let (label, left, right) = parse_line(line, node_re.clone());

if label == "ZZZ" {
  break;
}

count += 1;
```

if the instruction is "L", we set the `current` label to the `left` value return from `parse_line`, and set the `current_index` to the index of the `left` value in the `nodes` map. otherwise, we do the same thing, but for the `right` value.

```rust
if instruction == "L" {
  current = left;
  current_index = nodes.get(current).unwrap();
} else {
  current = right;
  current_index = nodes.get(current).unwrap();
}
```

we'll eventually break out of the loop, and print the count:

```rust
println!("Count: {}", count);
```

and this works! one problem: it's slow. really slow. when I ran this, It took 68888ms to run. we can do better.

let's start by removing the regex parsing. I have a feeling that using regex is quite slow, and I don't think using `clone()` to create a new regex every time we call `parse_line` is helping either. we'll rewrite parse_line to use string indexing instead:

```rust
fn parse_line(line: &str) -> (&str, &str, &str) {
  // XXX = (YYY, ZZZ)
  let label = &line[0..3];
  let left = &line[7..10];
  let right = &line[12..15];

  return (label, left, right);
}
```

we can also remove the regex from the main function.

this works as well, and gets us a runtime of 27ms. much better!

### Full code: with regex

here's the full code with the regex:

```rust
use regex::Regex;
use std::collections::HashMap;

fn parse_line(line: &str, regex: regex::Regex) -> (&str, &str, &str) {
  let Some(caps) = regex.captures(line) else {
      panic!("Invalid node: {}", line);
  };

  let label = caps.name("label").unwrap().as_str();
  let left = caps.name("left").unwrap().as_str();
  let right = caps.name("right").unwrap().as_str();

  return (label, left, right);
}

fn main() {
  let time = std::time::Instant::now();
  let node_re = Regex::new(r"(?<label>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();

  let binding = std::fs::read_to_string("input.txt").unwrap();
  let input = binding.split("\n").collect::<Vec<&str>>();

  let instructions = input[0].trim().split("").collect::<Vec<&str>>();
  // remove first and last instruction
  let instructions = instructions[1..(instructions.len() - 1)].to_vec();

  let mut nodes: HashMap<&str, usize> = HashMap::new();

  for idx in 2..(input.len() - 1) {
    let line = input[idx].trim();

    let (label, left, right) = parse_line(line, node_re.clone());

    nodes.insert(label, idx);
  }

  let mut count = 0;
  let mut current = "AAA";
  let mut current_index = nodes.get(current).unwrap();

  loop {
    let instruction = instructions[count % instructions.len()];
    let line = input[*current_index].trim();

    let (label, left, right) = parse_line(line, node_re.clone());

    if label == "ZZZ" {
      break;
    }

    count += 1;
    if instruction == "L" {
      current = left;
      current_index = nodes.get(current).unwrap();
    } else if instruction == "R" {
      current = right;
      current_index = nodes.get(current).unwrap();
    }
  }

  println!("Count: {}", count);
}
```

### Full code: no regex

and here's the full code without the regex:

```rust
use std::collections::HashMap;

fn parse_line(line: &str) -> (&str, &str, &str) {
  // XXX = (YYY, ZZZ)

  let label = &line[0..3];
  let left = &line[7..10];
  let right = &line[12..15];

  return (label, left, right);
}

fn main() {
  let binding = std::fs::read_to_string("input.txt").unwrap();
  let input = binding.split("\n").collect::<Vec<&str>>();

  let instructions = input[0].trim().split("").collect::<Vec<&str>>();
  // remove first and last instruction
  let instructions = instructions[1..(instructions.len() - 1)].to_vec();

  let mut nodes: HashMap<&str, usize> = HashMap::new();

  for idx in 2..(input.len() - 1) {
    let line = input[idx].trim();

    let (label, left, right) = parse_line(line);

    nodes.insert(label, idx);
  }

  let mut count = 0;
  let mut current = "AAA";
  let mut current_index = nodes.get(current).unwrap();

  loop {
    let instruction = instructions[count % instructions.len()];
    let line = input[*current_index].trim();

    let (label, left, right) = parse_line(line);

    if label == "ZZZ" {
        break;
    }

    count += 1;
    if instruction == "L" {
      current = left;
      current_index = nodes.get(current).unwrap();
    } else if instruction == "R" {
      current = right;
      current_index = nodes.get(current).unwrap();
    }
  }

  println!("Count: {}", count);
}
```

### Runtime results

| regex? | exec time (ms) |
| ------ | -------------- |
| yes    | 68888          |
| no     | 27             |

## Part 2: Problem

_the number of node labels ending in `A` is equal to the number of nodes ending in `Z`. using this, we can start multiple searches._

_how many steps does it take before every search starting at all nodes ending in `A` simultaneously end with `Z`?_

## Part 2: Train of thought

From the site, we have this example:

```
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
```

there are two nodes here that end in `A`: `11A` and `22A`. there are also two nodes that end in `Z`: `11Z` and `22Z`.

we can start two searches: one at `11A`, and one at `22A`. we'll first move left on each node, to move from `11A` and `22A` to `11B` and `22B`, respectively:

```

steps:  0      1
        11A -> 11B
        22A -> 22B
```

then, we'll move right on each node, to move from `11B` and `22B` to `11Z` and `22C`, respectively:

```
steps:  0      1      2
        11A -> 11B -> 11Z
        22A -> 22B -> 22C
```

we'll continue this on until both searches end at nodes that end in `Z`:

```
steps:  0      1      2      3      4      5      6
        11A -> 11B -> 11Z -> 11B -> 11Z -> 11B -> 11Z
        22A -> 22B -> 22C -> 22Z -> 22B -> 22C -> 22Z
```

we can see that both searches end at `11Z` and `22Z` at the same time, after 6 steps.

if we ran this with a brute force approach, we'd very quickly encounter something akin to [Day 5](https://mmyron.com/posts/advent23_5/) -- our solve time would be in the tens of minutes range (at least).

there's only two nodes that we're working with, but we can notice an interesting pattern. let's remove the first node in each search (and reformat it as a table)

| 1 step | 2 steps | 3 steps | 4 steps | 5 steps | 6 steps |
| ------ | ------- | ------- | ------- | ------- | ------- |
| 11B    | 11Z     | 11B     | 11Z     | 11B     | 11Z     |
| 22B    | 22C     | 22Z     | 22B     | 22C     | 22Z     |

we can see that each search falls into a cycle! the first search cycles from `11B` to `11Z`, and back to `11B`. the second search cycles from `22B` to `22C` to `22Z`, and back to `22B`. this is more apparent if we highlight the end of each cycle (i.e. each time we reach a node ending in `Z`):

| 1 step | 2 steps | 3 steps | 4 steps | 5 steps | 6 steps |
| ------ | ------- | ------- | ------- | ------- | ------- |
| 11B    | **11Z** | 11B     | **11Z** | 11B     | **11Z** |
| 22B    | 22C     | **22Z** | 22B     | 22C     | **22Z** |

so, the first search has an overall cycle of $2$. the second search has an overall cycle of $3$. once a search enters a cycle, it will continue to cycle. this cyclic behavior means we can find the least common multiple of the two cycles to find the number of steps it takes for both search to end at nodes ending in `Z` at the same time. this sounds like a stretch, but we can think of each search hitting a node that ends in `Z` as a multiple of the cycle length.

so, the LCM of $2$ and $3$ is $6$, which does match up with our example!

so, we can first find the cycle length of each search (i.e. the number of steps it takes for a search to reach a node ending in `Z`). then, we can find the LCM of the cycle lengths, which should give us the smallest number of steps necessary to reach a node ending in `Z` for all searches.

## Part 2: Implementation

we just need to slightly modify some of our code to handle a series of searches, instead of a single one. before we build the `nodes` map, we'll define two vectors: one for all of the search step counts, and one for the node labels:

```rust
let instructions = input[0].trim().split("").collect::<Vec<&str>>();
// remove first and last instruction
let instructions = instructions[1..(instructions.len() - 1)].to_vec();

let mut counts = Vec::new();
let mut currents = Vec::new();

let mut nodes: HashMap<&str, usize> = HashMap::new();

for idx in 2..(input.len() - 1) {
  // ...
}
```

when we build the nodes map, we'll also want to check if the current label ends in `A`. if it does, we'll push that label to the `currents` vector, and push `0` to the `counts` vector:

```rust
for idx in 2..(input.len() - 1) {
  let line = input[idx].trim();

  let (label, left, right) = parse_line(line);

  if label.ends_with('A') {
    counts.push(0);
    currents.push(label);
  }

  nodes.insert(label, idx);
}
```

then, we'll iterate over one the `currents` vector, and index it to get the current label:

```rust
for i in 0..currents.len() {
  let current = currents[i];

  // ...
}
```

then, we'll run the same algorithm as before. we'll get the index of the current node's label, and run through the loop.

there are a few things we need to change here:

1. we need to index the instructions vector with `counts[i]` instead of `count`
2. we need to update the end-label check: if the current label ends in `Z` (instead of being equal to `ZZZ`), we'll break out of the loop
3. we need to increment the count via `counts[i] += 1` instead of `count += 1`

```rust
for i in 0..currents.len() {
  let current = currents[i];

  let mut current_index = nodes.get(current).unwrap();

  loop {
    let instruction = instructions[counts[i] % instructions.len()];
    let line = input[*current_index].trim();

    let (label, left, right) = parse_line(line);

    if label.ends_with('Z') {
      break;
    }

    counts[i] += 1;
    if instruction == "L" {
      current = left;
      current_index = nodes.get(current).unwrap();
    } else if instruction == "R" {
      current = right;
      current_index = nodes.get(current).unwrap();
    }
  }
}
```

after we run that snippet for each search, our `counts` vector will have the number of steps it takes for each search to reach a node ending in `Z`. in other words, we have the cycle length of each search. with this, we can find the LCM of the cycle lengths. we'll just use the `num` crate to find the LCM:

```rust
let mut lcm = counts[0];
for i in 1..counts.len() {
  lcm = num::integer::lcm(lcm, counts[i]);
}

println!("LCM: {}", lcm);
```

and this works! in fact, it runs relatively quickly: around ~70ms on my laptop. there are plenty of optimizations to make, but I'm happy with this for now.

### Full code

here's the full code:

```rust
use num;
use std::collections::HashMap;

fn parse_line(line: &str) -> (&str, &str, &str) {
  // XXX = (YYY, ZZZ)
  let label = &line[0..3];
  let left = &line[7..10];
  let right = &line[12..15];

  return (label, left, right);
}

fn main() {
  let time = std::time::Instant::now();

  let binding = std::fs::read_to_string("input.txt").unwrap();
  let input = binding.split("\n").collect::<Vec<&str>>();

  let instructions = input[0].trim().split("").collect::<Vec<&str>>();
  // remove first and last instruction
  let instructions = instructions[1..(instructions.len() - 1)].to_vec();

  let mut counts = Vec::new();
  let mut currents = Vec::new();
  let mut nodes: HashMap<&str, usize> = HashMap::new();

  for idx in 2..(input.len() - 1) {
    let line = input[idx].trim();

    let (label, left, right) = parse_line(line);

    if label.ends_with('A') {
      counts.push(0);
      currents.push(label);
    }

    nodes.insert(label, idx);
  }

  for i in 0..currents.len() {
    let mut current = currents[i];

    let mut current_index = nodes.get(current).unwrap();

    loop {
      let instruction = instructions[counts[i] % instructions.len()];
      let line = input[*current_index].trim();

      let (label, left, right) = parse_line(line);

      if label.ends_with('Z') {
        break;
      }

      counts[i] += 1;
      if instruction == "L" {
        current = left;
        current_index = nodes.get(current).unwrap();
      } else if instruction == "R" {
        current = right;
        current_index = nodes.get(current).unwrap();
      }
    }
  }

  let mut lcm = counts[0];
  for i in 1..counts.len() {
    lcm = num::integer::lcm(lcm, counts[i]);
  }

  println!("{}", lcm);
}
```
