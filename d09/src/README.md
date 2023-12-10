# Day 9

## Part 1: Problem

_a line of text is a space-delimited sequence of numbers. each line can be defined as a sequence, and the next value can be predicted:_

```
0   3   6   9   12   15
```

_we can take the difference between each number to get the sequence of differences:_

```
0   3   6   9   12   15
  3   3   3   3    3
```

_we can then take the difference between each number in the sequence of differences to get the sequence of differences of differences:_

```
0   3   6   9   12   15
  3   3   3   3    3
    0   0   0    0
```

_the line here is entirely composed of 0s, so we can stop. If we add up the final values in each sequence, we get:_

```
0 + 3 + 15 = 18
```

_which is the next sequence in the sequence. given a series of lines, find the sum of the final values in each sequence._

## Part 1: Train of thought

let's work through the example, piece by piece. we'll call the initial sequence $f(x)$:

```
0   3   6   9   12   15
```

we can define the sequence of differences as $f'(x)$. Any number in $f'(x)$ can be defined as:

$$
f'(x) = f(x) - f(x-1)
$$

this is analogous to the derivative of $f(x)$, which is why we use the same notation. because we want to find the next number in the original sequence, we'll start building out the derivative sequence from the end:

$$
f'(x) = f(x) - f(x-1) = 15 - 12 = 3
$$

```
0   3   6   9   12   15
                   3
```

we continue:

f`(x) = f(x) - f(x-1) = 12 - 9 = 3

```
0   3   6   9   12   15
               3   3
```

and so on...

```
0   3   6   9   12   15
  3   3   3   3    3
```

until the entire sequence $f'(x)$ is defined from $0$ to $15$.

we can build out f``(x) in the same way:

```
0   3   6   9   12   15
  3   3   3   3    3
    0   0   0    0
```

we don't need to build out $f^{(3)}(x)$ because $f''(x)$ is already entirely composed of zeros. we can stop here, and calculate the sum of the final values in each sequence:

$$
f(0) + f'(0) + f''(0) = 0 + 3 + 15 = 18
$$

in order to make lookup easier, we can use a dynamic programming approach to build out the sequence. for the above example, we can build out this table:

| $n$ | $f^n(1)$ | $f^n(2)$ | $f^n(3)$ | $f^n(4)$ | $f^n(5)$ | $f^n(6)$ |
| --- | -------- | -------- | -------- | -------- | -------- | -------- |
| 0   | 0        | 3        | 6        | 9        | 12       | 15       |
| 1   | n/a      | 3        | 3        | 3        | 3        | 3        |
| 2   | n/a      | n/a      | 0        | 0        | 0        | 0        |

we can then calculate the sum of the final values in each sequence by summing the last column.

give a sequence of length $L$, we'll have a worst-time array size of $L * L$. to account for this, we'll assume our array is that size. luckily, we can break out early if two subsequent values in the sequence are the same. a sequence of the same constant value will have a derivative sequence of all zeros.

## Part 1: Implementation

for each line in the input, we want to create a new array of length $L * L$, where $L$ is the length of the line. we'll then fill in the first row of the array with the values from the line. after that, we'll begin filling in the rest of the array, starting from the end of the line.

```rust
let mut sum = 0;
for line in input {
  if line.len() == 0 {
      continue;
  }

  // split the line into a vector of strings
  let split_line = line.trim().split(" ").collect::<Vec<&str>>();

  // build the dp array and fill in the first with None values
  let mut dp: Vec<Vec<Option<i32>>> = vec![vec![None; split_line.len()]; split_line.len()];

  // fill in the first row of the dp array with the values from the line (as i32s)
  for i in 0..split_line.len() {
      dp[0][i] = Some(split_line[i].parse::<i32>().unwrap());
  }

  // ...
}
```

we can then start building our the full dp array. the first index in `dp[m][n]`, `m`, will represent the current derivative of the orignial sequence $f(x)$. `n` will represent the index of the sequence. for example, `dp[0][0]` will represent the first value in the original sequence, $f(0)$, and `dp[1][0]` will represent the first value in the derivative sequence, $f'(0)$.

we'll increment through the derivatives of $f(x)$ first: from 1 to the length of the line:

```rust
for deriv in 1..split_line.len() {
  // ...
}
```

if we look at the above table between $x$ and $f^{(n)}(x)$, we'll see that, for any given $n$, the first $n$ values in the sequence cannot be computed (because there is no previous term to subtract from). we can move from this value (`deriv`) to the end of the line in reverse order, and fill in the values in the dp array:

```rust
for deriv in 1..split_line.len() {
  for i in (deriv..split_line.len()).rev() {
      // ...
  }
}
```

We're working with `Option<i32>` values, however we can assume that every value in the dp array will be None when we first reach it. Likewise, any values in the previous `deriv` row will exist as `Some(i32)` values.

we'll calculate the derivative at the point `dp[deriv][val]` by subtracting the value at `dp[deriv - 1][val - 1]` from the value at `dp[deriv - 1][val]`:

```rust
let diff = Some(dp[deriv - 1][val].unwrap() - dp[deriv - 1][val - 1].unwrap());
dp[deriv][val] = diff;
```

here's the entire dp loop. it's pretty simple:

```rust
for deriv in 1..split_line.len() {
  for val in (deriv..split_line.len()).rev() {
    let diff = Some(dp[deriv - 1][val].unwrap() - dp[deriv - 1][val - 1].unwrap());
    dp[deriv][val] = diff;
  }
}
```

after we've built out the relevant parts of the array, we can find the next value. for this, we just sum the last column of the array:

```rust
let mut next = 0;

for i in 0..split_line.len() {
    if dp[i][dp[i].len() - 1].is_none() {
        break;
    }

    next += dp[i][dp[i].len() - 1].unwrap();
}
```

finally, we can add the next value to the sum:

```rust
sum += next;
```

after we loop over every line in the input, we can print the sum:

```rust
println!("sum: {}", sum);
```

and that's all there is to it! i find this solution to be pretty nice, and it runs in ~4.5ms on my laptop. sweet! here's the full solution:

```rust
fn main() {
  let binding = std::fs::read_to_string("input.txt").unwrap();
  let input = binding.split("\n").collect::<Vec<&str>>();

  let mut sum = 0;
  for line in input {
    if line.len() == 0 {
      continue;
    }

    // splitting line and creating vector takes ~6-12us
    let split_line = line.trim().split(" ").collect::<Vec<&str>>();
    let mut dp: Vec<Vec<Option<i32>>> = vec![vec![None; split_line.len()]; split_line.len()];

    for i in 0..split_line.len() {
      dp[0][i] = Some(split_line[i].parse::<i32>().unwrap());
    }

    // indexing via arr[n]: nth derivative of function
    // indexing via arr[n][m]: value of f^n(m)

    for deriv in 1..split_line.len() {
      for val in (deriv..split_line.len()).rev() {
        let diff = Some(dp[deriv - 1][val].unwrap() - dp[deriv - 1][val - 1].unwrap());
        dp[deriv][val] = diff;
      }
    }

    let mut next = 0;

    for i in 0..split_line.len() {
      if dp[i][dp[i].len() - 1].is_none() {
        break;
      }

      next += dp[i][dp[i].len() - 1].unwrap();
    }

    sum += next;
  }

  println!("Sum: {}", sum);
}
```

## Part 2: Problem

_instead of finding the next number in a given sequence, find the previous number in the sequence. return the sum of all numbers that precede the given sequences._

## Part 2: Train of thought

lol this is pretty similar to part 1. in fact, we can reuse 90% of our code. we'll just swap around a few things:

1. instead of building out the dp array from the end of the line, we'll build it out from the beginning of the line
2. to find the derivative at a point, we'll subtract the current value in the sequence from the _next_ value in the sequence
3. instead of summing the last column of the dp array, we'll sum the first column of the dp array

## Part 2: Implementation

we can just copy over our code from part 1 into a new file. we'll change the inner `for` loop to build out the dp array from the beginning of the line. in part 1, we started from `deriv` and moved to the end of the line. in part 2, we'll start at 0, and move to the end of the line minus `deriv`:

```rust
for deriv in 1..split_line.len() {
  for val in 0..(split_line.len() - deriv) {
    // ...
  }
}
```

to calculate the difference, we'll also work backwards; we'll subtract the next value in the sequence from the current value in the sequence:

```rust
let diff = Some(dp[deriv - 1][val].unwrap() - dp[deriv - 1][val + 1].unwrap());
dp[deriv][val] = diff;
```

so the full loop looks like this:

```rust
for deriv in 1..split_line.len() {
  for val in 0..(split_line.len() - deriv) {
    let diff = Some(dp[deriv - 1][val].unwrap() - dp[deriv - 1][val + 1].unwrap());
    dp[deriv][val] = diff;
  }
}
```

finally, we'll sum the first column of the dp array, and add it to the sum:

```rust
let mut next = 0;

for i in 0..split_line.len() {
    if dp[i][0].is_none() {
        break;
    }

    next += dp[i][0].unwrap();
}

sum += next;
```

and that's it! because we changed nearly nothing (only some array indices), we get the same ~4.5ms runtime. here's the full solution:

```rust
fn main() {
  let binding = std::fs::read_to_string("input.txt").unwrap();
  let input = binding.split("\n").collect::<Vec<&str>>();

  let mut sum = 0;
  for line in input {
    if line.len() == 0 {
      continue;
    }

    // splitting line and creating vector takes ~6-12us
    let split_line = line.trim().split(" ").collect::<Vec<&str>>();
    let mut dp: Vec<Vec<Option<i32>>> = vec![vec![None; split_line.len()]; split_line.len()];

    for i in 0..split_line.len() {
      dp[0][i] = Some(split_line[i].parse::<i32>().unwrap());
    }

    // indexing via arr[n]: nth derivative of function
    // indexing via arr[n][m]: value of f^n(m)

    for deriv in 1..split_line.len() {
      for val in 0..(split_line.len() - deriv) {
        let diff = Some(dp[deriv - 1][val].unwrap() - dp[deriv - 1][val + 1].unwrap());
        dp[deriv][val] = diff;
      }
    }

    let mut next = 0;

    for i in 0..split_line.len() {
      if dp[i][0].is_none() {
        break;
      }

      next += dp[i][0].unwrap();
    }

    sum += next;
  }

  println!("Sum: {}", sum);
}
```

overall, a pretty easy problem, but a fun one nonetheless!
