fn main() {
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
}
