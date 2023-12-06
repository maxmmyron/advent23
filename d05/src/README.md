# Day 5

## Part 1: Problem

_an almanac is composed of two data types: an array of seeds to plant, and a series of source/destination maps:_

```
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
```

_each map is composed of a number of mappings, where a mapping has the following form:_

```
<destination start index> <source start index> <range>
```

_the seed-to-soil map has a mapping of `50 98 2`, so 98 and 99 in the source (seed number) correspond to 50 and 51 in the destination (soil number). if a map doesn't contain an explicit destination, the destination is the original source. by running through each mapping, we can generate a **location** for each seed number._

_what is the lowest location number for the list of provided seeds?_

## Park 1: Train of Thought

we're probably in for a pretty annoying parsing problem, much like day 3. the strategy there was to parse the entire file into a series of array and maps, and then deal with the data later; that seems like a good move here.

i was initially thinking of some crazy backwards-parsing single-map solution, but the answer here is honestly far more trivial if we just work with several maps. down the line, we'd be able to chain them like:

```
...[soil_fertilizer_map[seed_soil_map[seed]]]
```

so, here's the idea:

1. parse the input seeds into a vector.
2. for each x-y map, parse the source and destination ranges into a hashmap.
3. for each seed, work through the maps to find the final location.
4. store the lowest location number.

### Update

so, to make a long story short i tried to work out the solution using maps. it worked great for the sample input that the AoC provides, however it took _forever_ to run with the actual input.

if we take a look at one of the mapping lines:

```
seed-to-soil map:
748585809 2125564114 88980459
```

these are big numbers! in fact, i ran into an overflow error the first time i ran my original program with the actual input. i lazily changed it to i64, but execution time took _forever_.

constructing a hashmap with millions of mappings is a pretty ridiculous way to solve this problem, even though it sounded like a good idea at first.

i had an early class today, and missed out on brewing a cup of coffee. however, after finally getting a cup of coffee and sitting down with a clearer mind, a more obvious (and elegant) solution is pretty obvious.

let's say we're mapping from a seed number to a soil number:

```
seed-to-soil map:
<destination start index 1> <source start index 1> <range 1>
<destination start index 2> <source start index 2> <range 2>
...
```

and we have some seed number `s`. for a given line, let's call the destination start index `dest_start`, the source start index `source_start`, and the range `range`.

`s` is within a certain mapping if it is no less than `source_start`, and no more than `source_start + range - 1`. range includes `source_start` itself, which is why we subtract 1.

we know `s` is within the map range with the inequality:

```
source_start <= s <= source_start + range - 1
```

because sources and destinations are mapped 1-to-1, we can solve for the offset of `s` in the destination (assuming it's in the range):

```
offset = s - source_start
```

and then we can find the destination by adding that offset to the destination start index:

```
dest = dest_start + offset
```

let's work this out quickly from the example:

```
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48
```

the first seed has a value of `79`. for the first mapping in the seed-to-soil map, we can check the inequality:

```
98 <= 79 <= 98 + 2 - 1
98 <= 79 <= 99
```

this is false, so we'll move onto the next mapping:

```
50 <= 79 <= 50 + 48 - 1
50 <= 79 <= 97
```

this one holds true, so we can calculate the offset from the `source_start`:

```
offset = s - source_start
offset = 79 - 50
offset = 29
```

and then we can calculate the destination:

```
dest = dest_start + offset
dest = 52 + 29
dest = 81
```

so, the first seed `49` maps to soil `81`.

so, here's the new algorithm:

1. parse the input seeds into a vector.
2. for each x-y map, parse the source and destination ranges into a vector of tuples.
3. for each seed, work through the mappings, and calculate the destinations based on our formulas.
4. store the lowest location number.

## Part 1: Implementation

we need to parse our input file into a vector of seeds and a vector of mapping sections.

### Parsing seeds

parsing the seeds is easy enough. the first line has the following format:

```
seeds: <num> <num> <num> <num> ...
```

i've been using function chaining over the past few days to handle parsing inputs, but let's try out the [`regex`](https://crates.io/crates/regex) crate. we'll install it first with `crates add regex`, and then import it into our program:

```rust
use regex::Regex;
```

we only want the numbers from the first line, so we can use the following regex

```
(\d+)
```

to capture all of the numbers. we'll instantiate a `Regex` type with this pattern, and then use the `find_iters` method to capture all of the numbers (and return an iterator):

```rust
let seed_re = Regex::new(r"(\d+)").unwrap();

let seeds = seed_re.captures_iter(input[0]);
```

after we run the `captures_iter()` method, we get an iterator of `Captures` types. we can use the `map` method to convert each `Captures` type into `i64`s, and then use `collect` to convert the iterator into a vector:

```rust
let seeds = seed_re
      // capture the non-overlapping matches
      .captures_iter(input[0])
      // parse the matches into integers
      .map(|x| x[1].parse::<i64>().unwrap())
      // collect the iterator into a vector
      .collect::<Vec<i64>>();
```

### Parsing mappings

parsing the mappings is a bit more complicated, but there are a few things to notice. in general, each mapping section has the following format:

```
<map name>:
<dest start> <source start> <range>
<dest start> <source start> <range>
<dest start> <source start> <range>
...
```

and the entire file is organized like so:

```
seeds: <num> <num> <num> <num> ...

<A>-to-<B>:
<dest start> <source start> <range>
<dest start> <source start> <range>
...

<B>-to-<C>:
<dest start> <source start> <range>
<dest start> <source start> <range>
...

<C>-to-<D>:
<dest start> <source start> <range>
<dest start> <source start> <range>
...

...
```

each section maps into the successive section's source: notice how `<A>-to-<B>` precedes `<B>-to-<C>`, and so on. likewise, each section is separated by exactly one whitespace.

we're going to store all of our mappings in a double vector, where each inner vector is a tuple:

```rust
let mut source_dest_maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();
```

we'll also use regex to our advantage again, so for any one section we'll just store all the lines in a single String:

```rust
let mut section = String::new();
```

and finally, we'll use a similar Regex pattern to what we used for the seeds:

```rust
let map_re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
```

here, we're capturing three numbers at a time, each of which is separated by a single whitespace.

we'll start parsing the file at line index 2, since we've already parsed the seeds (and can ignore the blank line between the seeds and the first mapping section):

```rust
for line in input[2..input.len()].iter() {
// ...
}
```

if the line is empty (i.e. the last line in the file), or if the line has a length of 1 (i.e. the blank line between sections, which only has a carriage return), we'll know that we've reached the end of the section.

so, if the line isn't empty or a spacer, we'll just add it to our `section` string:

```rust
if line.len() > 1 {
  section.push_str(line);
}
```

otherwise, we'll run our regex. we're working with the entire section compressed into a string, and we need a place to store the mappings before we add them to our `source_dest_maps` vector. we'll use a vector of tuples for this:

```rust
if line.len() > 1 {
  section.push_str(line);
} else {
  let mut source_dest_map: Vec<(i64, i64, i64)> = Vec::new();
}
```

because we're capturing three numbers at a time, we'll use the `captures_iter` method again. here, however, we're going to use the `extract()` method to extract the captures, and then we'll iterate over the captures in groups of three:

```rust
if line.len() > 1 {
  section.push_str(line);
}
else {
  let mut source_dest_map: Vec<(i64, i64, i64)> = Vec::new();

  for (_, [source, dest, range]) in
    map_re.captures_iter(section.as_str()).map(|x| x.extract())
  {
    // ...
  }
}
```

this give us source, dest, and range strings. we'll parse them into integers, and then add them to our `source_dest_map` vector:

```rust
if line.len() > 1 {
  section.push_str(line);
}
else {
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
}
```

finally, after we've iterated through all of the mappings, we'll add the `source_dest_map` to our `source_dest_maps` vector, and reset the `section` string:

```rust
if line.len() > 1 {
  section.push_str(line);
}
else {
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
```

when we run this for the example provided by AoC and print the resulting `source_dest_maps` var, we get the following:

```
> cargo run
  Finished dev [unoptimized + debuginfo] target(s) in 1.21s
     Running `...\d05.exe`
[(50, 98, 2), (52, 50, 48)]
[(0, 15, 37), (37, 52, 2), (39, 0, 15)]
[(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)]
[(88, 18, 7), (18, 25, 70)]
[(45, 77, 23), (81, 45, 19), (68, 64, 13)]
[(0, 69, 1), (1, 0, 69)]
[(60, 56, 37), (56, 93, 4)]
```

### Calculating the lowest location

that was actually the hardest part! now, we just apply the formulas we worked out earlier. we'll iterate through each seed, and then through each mapping, and then calculate the destination.

we first need to initialize a variable to store the lowest location:

```rust
let mut min = i64::MAX;
```

we'll iterate through each seed, and store the seed number in a variable called `curr_dest`. we'll update this over time as we iterate through each mapping vector in `source_dest_maps`:

```rust
for seed in seeds {
  let mut curr_dest = seed;

  // ...
}
```

we'll iterate through each mapping vector in `source_dest_maps`, and then through each mapping in the vector. we'll extract out the `source_start`, `source_end`, and `dest_start` values for convenience. as a refresher, each mapping has the form `(dest_start, source_start, range)`:

```rust
for source_dest_map in &source_dest_maps {
  for map in source_dest_map {
    let source_start = map.1;
    let source_end = map.1 + map.2 - 1;
    let dest_start = map.0;

    // ...
  }
}
```

if the seed is within the mapping range (i.e. `source_start <= seed <= source_end`), we'll set the `curr_dest` variable to the destination start, plus the offset of the seed within the mapping. we'll also break out of the loop, since we've found the mapping we're looking for:

```rust
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

  // ...
}
```

after we've run through every `source_dest_map`, we'll update the minimum location if `curr_dest` is less than the current minimum:

```rust
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

  if curr_dest < min {
    min = curr_dest;
  }
}
```

finally, we'll print the minimum location:

```rust
println!("min: {}", min);
```

and it runs _much_ faster than the hashmap solution i thought would work. here's the full code:

### Full code

```rust
use regex::Regex;

fn main() {
  let binding = std::fs::read_to_string("input.txt").unwrap();
  let input = binding.split("\n").collect::<Vec<&str>>();

  let seed_re = Regex::new(r"(\d+)").unwrap();

  // parse seed
  let seeds = seed_re
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

  println!("\n{}", min);
}
```

### runtime

i found using regex to be pretty satisfying, even if rust is still pretty confusing as a whole for me. i really need to take time to learn more about how rust differs from C/C++ in terms of memory management.

i was curious to see how must faster raw parsing would have been, so i threw together an implementation of problem 1 using that method. you can see the code for that on [my github repo for AoC 2023](https://github.com/maxmmyron/advent23).

here are the run times for each method:

- with HashMap: ??? (i stopped it after a while)
- with regex parsing: ~17ms
- with raw parsing: ~450us

raw parsing was a _lot_ faster! i was surprised to see how much of a different regex parsing makes, but it does also make sense.

17 ms vs 450us isn't that big of a deal for AoC, but it's still interesting nonetheless to see how these tiny differences impact performance.

## Part 2: Problem

_the seed map now corresponds to a series of seed ranges. a given range is composed of a starting number, and a range value. for example:_

```
seeds: 79 14 55 13
```

_there are two ranges here. one starts at seed 79, and goes to 92 (79, 80, 81, ..., 92). the other starts at 55 and goes to 67 (55, 56, 57, ..., 67). find the minimum location given the seed ranges_

## Part 2: Train of Thought

let's start by blindly replacing our existing `for seed in seeds` loop with a custom one that calculates the range, and iterates through that. we'll use the `step_by()` iterator method to iterate through the range in increments of 2, since each seed range is composed of two numbers:

```rust
for idx in (0..seeds.len()).step_by(2) {
  let start = seeds[idx];
  let end = seeds[idx] + seeds[idx + 1] - 1;

  for seed in start..end {
    let mut curr_dest = seed;

    // ...
  }
}
```

so, this works right off the bat with the sample input. however, it takes _forever_ to run with the actual input. we'll need to find a better way to do this.

let's work with a seriously simplified example:

```
seeds: 2 4

seed-to-location map:
2 3 2
```

we can physically map this out:

```
mapping seeds: 2, 3, 4, 5

0 -> 0
1 -> 1
2 -> 2
3 -> 2
4 -> 3
5 -> 5
6 -> 6
```

notice how we can split this mapping into three regions, where each region is a linear mapping. the first region starts at 0, goes for 3 numbers, and has a change of 0. the second region starts at 3, goes for 2 numbers, and has a change of -1. the third region starts at 5, goes for 2 numbers, and has a change of 0.

```
0 -> 0   3 -> 2   5 -> 5
1 -> 1   4 -> 3   6 -> 6
2 -> 2
```

our seed mapping is `[2, 3, 4, 5],` let's split into vectors based on the region the original number is in:

```
[2, 3, 4, 5] -> [[2], [3, 4], [5]]
```

we can then map each region to its final location:

```
[2] -> [2]
[3, 4] -> [2, 3]
[5] -> [5]
```

for each "seed region," no matter the mapping, the first number will have the lowest final mapping for that region. so, we can just take the first number from each region, and then find the lowest number from that list:

```
[2] -> [2]
[3, 4] -> [3] (ignore 4) -> [2]
[5] -> [5]
```

then, we can work with that final vector (in this case, `[2,2,5] -> [2,5]`), and move to the next mapping.

i'm actually going to leave it here for now--it's pretty late. final's week is coming up and i unfortunately cannot solve AoC problems all day, every day.

(i did get the answer by brute-forcing it... it look nearly 52 minutes.)

## Part 2: Implementation

TBD!
