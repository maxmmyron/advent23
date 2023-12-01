# Day 1

## Part 1

### Problem

_a **calibration value** is a two digit number made from the first and last digit in an alphanumeric string. given a list of alphanumeric strings, find the sum of all calibration values._

### Train of thought

it's nice we're starting with some file i/o; that's always an important thing to learn in a new language since different languages can vary a lot in how they handle i/o.

the upshot of my initial-thought algo is:

1. read in a line
2. find the first digit `O(n)`
3. find the last digit, working backwards `O(n)`
4. concatenate the two digits `O(1)`
5. convert the string to an integer `O(n)`
6. add the integer to a running total `O(1)`
7. repeat until EOF

the worst case is that every line has a single digit in the exact middle, which gives a worst case complexity for any given line of `O(n)`.

### Implementation

we start by reading in the file, and splitting it into lines:

```rust
let input = std::fs::read_to_string("input.txt")?.split("\n");
```

this is fine for now, however, when we try to iterate over the lines, we get an error:

```rust
let input = std::fs::read_to_string("input.txt")?.split("\n");
//   ^ Error: temporary value dropped while borrowed
//            creates a temporary value which is freed while still in use

for line in input {
  // ...
}
```

ownership and borrowing are some of the first things you learn in rust. to summarize these concepts from the rust book:

> - Each value in Rust has an owner.
> - There can only be one owner at a time.
> - When the owner goes out of scope, the value will be dropped.

that last point is the key to what is going wrong here. we can take another line from the book:

> Doing [memory management] correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one `allocate` with exactly one `free`.
>
> Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.

so, after a value goes out of scope, it is dropped.

when we call `read_to_string("input.txt")`, we get back a value that owns the string: `Result<String>`. we use the `?` operator to unwrap the value and get the string, if it exists.

however, because we are further chaining methods, we're not storing that string in a variable, so it only exists ephemerally. the string is a "temporary value," and it's dropped at the _end of the statement_.

so, when we call `split("\n")`, we're trying to borrow the string, but it's been dropped since we didn't bind to it (i.e. `let input = ...`). this is why we get the error.

furthermore, since the string is dropped at the end of the statement, we can't use it in the `for` loop.

luckily, the fix is trivial:

```rust
let binding = std::fs::read_to_string("input.txt")?;
let input = binding.split("\n");

for line in input {
  // ...
}
```

now, the string is bound to binding, and it won't be dropped until the end of the scope (which is the end of the program since we defined it in the main function).

keeping in mind the concept of ownership and borrowing, we can now implement the rest of the algorithm.

for each line in the input, we do the following:

1. find the first and last byte indices of the numeric digits in the line using `<&str>.find(char::is_numeric)` and `<&str>.rfind(char::is_numeric)`. These methods return `Option<usize>`, which is an enum that can be either `Some(usize)` or `None`.

2. assuming the line is valid (which we will do because AoC puzzle inputs are usually very nicely formatted), we unwrap the index values using `unwrap()`. This will panic if the value is `None`, but we can assume that won't happen.

3. now that we have the indices of the first and last digits in the line, we retrieve the actual digit values using `<&str>.nth(<usize>).unwrap()`. Again, since AoC puzzle inputs are nicely formatted, we can assume that the indices are valid and inline the unwrapping.

4. we concatenate the digits into a string using `format!()`, and then use `.parse::<i32>()` to convert the string into an integer. Again, we can assume that the string is a valid integer, so we can inline the unwrapping.

5. now that we finally have the integer, we add it to a running total.

here's the full code for the first part of day 1:

```rust
let binding = std::fs::read_to_string("input.txt").unwrap();
let input = binding.split("\n").collect::<Vec<&str>>();

let mut sum = 0;

for line in input {
  if line.len() == 0 {
    break;
  }

  let first_char = line.find(char::is_numeric);
  let last_char = line.rfind(char::is_numeric);

  let first_char = first_char.unwrap();
  let last_char = last_char.unwrap();

  let first_char = line.chars().nth(first_char).unwrap();
  let last_char = line.chars().nth(last_char).unwrap();

  let val = format!("{}{}", first_char, last_char)
    .parse::<i32>()
    .unwrap();

  sum += val;
}

println!("{}", sum);
```

i did have to add a check for empty lines, since the last line in the input file is empty, and that would cause the program to panic (in all fairness i really should have explicitly checked the validity of the `unwrap()` calls).

and it works!

## Part 2

### Problem

_a **calibration value** is a two digit number made from the first and last digits parsed in an alphanumeric string. a given number may either be a digit, or a number spelled out (`one`, `two`, `three`, ...). given a list of alphanumeric strings, find the sum of all calibration values._

### Train of thought

the second part is, as it usually goes, a slight variation on the first part. i _think_ we can keep the `find()` and `rfind()` calls, but we might need to modify the predicate to check for either digits or spelled-out numbers.

on second thought, instead of creating a custom searcher, we can just use the `find()` and `rfind()` in a loop, and swap out the predicate for each iteration.

if we're working with `find()`, we check the index of a given match to the current best index. if it improves on the current best (i.e. the new index is lower), then we update the current best accordingly (and store the corresponding digit). it would work the same way for `rfind()`, except we would check if the new index is higher than the current best.

after we run that loop, we should have the first and last digits of the calibration value. we do what we did in part 1 to add it to the running total.

### Implementation

we're just going to define our written-out numbers in an array:

```rust
let patterns = [
   "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
```

we'll loop through this and use the `find()` and `rfind()` methods to find the first and last digits of the calibration value. but, we also need to check for digits too, so we'll add a check for that. in this case, i'm going to run the loop an extra time, and check for digits at the end.

initially, i wanted to store both the written representations and the `char::is_numeric` predicates in the same array; i thought this would work because they could both worked in the `.find` function as a `Pattern` trait. unfortunately, i can't define the array type as `[Pattern]`, because `Pattern` is dynamic: the compiler can't assume its size at compile time.

we first define a few variables:

```rust
let mut curr_find_idx = line.len();
let mut curr_rfind_idx = 0;

let mut tens: usize = 0;
let mut ones: usize = 0;
```

`curr_find_idx` and `curr_rfind_idx` both store the current best index for `find()` and `rfind()`, respectively. we initialize them to the opposite of what we did in part 1, since we're looking for the first and last digits now.

`tens` and `ones` store the values of the first and last digits, respectively. it's a little more convenient if we just convert the character to a number right away, so we don't have to use `format!()` and `parse::<i32>()` like we did in part 1.

i'm specifically using `usize` here because down the line, we're going to be adding to either of these using the current index of the `patterns` array, which is a `usize` value. on my machine `usize` is stored with 8bytes; it's a 64 bit value. if we were to convert it to an i32, we would risk losing data.

while that's... _fine_ in this trivial AoC puzzle, it's not good practice. it's better (and arguably easier) to just parse the `&str` digit to a `usize`.

with that, we can start the loop:

```rust
for i in 0..patterns.len() + 1 {
  if i == patterns.len() {
    // check against digits using char::is_numeric

    // break so we don't check against patterns[patterns.len()]
    break;
  }

  // check against patterns[i]
}
```

we'll handle the digit check first, since we're just rehashing the code we wrote in part 1.

we start by finding the index of the first and last digits in the line:

```rust
for i in 0..patterns.len() + 1 {
  if i == patterns.len() {
    let find_idx = line.find(char::is_numeric);
    let rfind_idx = line.rfind(char::is_numeric);

    // ...

    // break so we don't check against patterns[patterns.len()]
    break;
  }

  // check against patterns[i]
}
```

then, we check if the index we've just found is better than the current best. if it is, we update the current best and store the corresponding digit:

```rust
for i in 0..patterns.len() + 1 {
  if i == patterns.len() {
    let find_idx = line.find(char::is_numeric);
    let rfind_idx = line.rfind(char::is_numeric);

    if find_idx.is_some() {
      if find_idx.unwrap() <= curr_find_idx {
        let str_val = line.chars().nth(curr_find_idx).unwrap();
        let int_val = str_val.to_string().parse::<usize>().unwrap();
        tens = int_val * 10;
      }
    }

    if rfind_idx.is_some() {
      if rfind_idx.unwrap() >= curr_rfind_idx {
        curr_rfind_idx = rfind_idx.unwrap();
        let str_val = line.chars().nth(curr_rfind_idx).unwrap();
        let int_val = str_val.to_string().parse::<usize>().unwrap();
        ones = int_val;
      }
    }

    // break so we don't check against patterns[patterns.len()]
    break;
  }

  // check against patterns[i]
}
```

checking against the patterns is actually a bit easier, since we can just use the index of the current pattern, add one, and use that as the numeric representation of the pattern when storing the current best digit.

here's the snipped for that:

```rust
for i in 0..patterns.len() + 1 {
  if i == patterns.len() {
    // ...
    break;
  }

  let find_idx = line.find(patterns[i]);
  let rfind_idx = line.rfind(patterns[i]);

  if find_idx.is_some() {
    if find_idx.unwrap() <= curr_find_idx {
      curr_find_idx = find_idx.unwrap();
      tens = (i + 1) * 10;
    }
  }

  if rfind_idx.is_some() {
    if rfind_idx.unwrap() >= curr_rfind_idx {
      curr_rfind_idx = rfind_idx.unwrap();
      ones = i + 1;
    }
  }
}
```

finally, after we've run through this loop, we'll have the first and last digits stored in our `tens` and `ones` variables. all we do is add them together:

```rust
let val = tens + ones;
```

and, after this code runs for every line in the input, we'll have the sum of all calibration values.

here's the full code for part 2:

```rust
fn main() {
  let binding = std::fs::read_to_string("input.txt").unwrap();
  let input = binding.split("\n").collect::<Vec<&str>>();

  let mut sum = 0;

  let patterns = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
  ];

  for line in input {
    if line.len() == 0 {
      break;
    }

    let mut curr_find_idx = line.len();
    let mut curr_find = ' ';

    let mut curr_rfind_idx = 0;
    let mut curr_rfind = ' ';

    for i in 0..patterns.len() + 1 {
      if i == patterns.len() {
        let find_idx = line.find(char::is_numeric);
        let rfind_idx = line.rfind(char::is_numeric);

        if find_idx.is_some() {
          if find_idx.unwrap() <= curr_find_idx {
            curr_find_idx = find_idx.unwrap();
            let val = line.chars().nth(curr_find_idx).unwrap();
            curr_find = val;
          }
        }

        if rfind_idx.is_some() {
          if rfind_idx.unwrap() >= curr_rfind_idx {
            curr_rfind_idx = rfind_idx.unwrap();
            let val = line.chars().nth(curr_rfind_idx).unwrap();
            curr_rfind = val;
          }
        }

        break;
      }

      let find_idx = line.find(patterns[i]);
      let rfind_idx = line.rfind(patterns[i]);

      if find_idx.is_some() {
        if find_idx.unwrap() <= curr_find_idx {
          curr_find_idx = find_idx.unwrap();
          curr_find = (i + 1).to_string().chars().nth(0).unwrap();
        }
      }

      if rfind_idx.is_some() {
        if rfind_idx.unwrap() >= curr_rfind_idx {
          curr_rfind_idx = rfind_idx.unwrap();
          curr_rfind = (i + 1).to_string().chars().nth(0).unwrap();
        }
      }
    }

    let val = format!("{}{}", curr_find, curr_rfind)
      .parse::<i32>()
      .unwrap();

    sum += val;
  }

  println!("{}", sum);
}
```

it's really ugly, but it works.

there's a lot i need to learn in rust, but it seems nice so far. C++'s memory management is still a pain, so it's nice to have a language that handles it for you (without garbage collectors!)
