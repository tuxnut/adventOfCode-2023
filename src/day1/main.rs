use std::fs;

fn read_input_file(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path).expect("Failed to read input file: {path}");
    input.lines().map(|line| line.to_string()).collect()
}

fn part1(input: &Vec<String>) {
    let mut result = 0;

    for line in input {
        let mut numbers = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                numbers.push(c.to_digit(10).unwrap());
            }
        }

        result += numbers.first().unwrap() * 10 + numbers.last().unwrap();
    }

    println!("Part 1 result : {result}");
}

fn part2(input: &Vec<String>) {
    let mut result = 0;

    for line in input {
        let mut numbers = Vec::new();
        for (i, c) in line.char_indices() {
            if c.is_numeric() {
                numbers.push(c.to_digit(10).unwrap());
                continue;
            }

            let line_slice = &line[i..];

            if line_slice.starts_with("one") {
                numbers.push(1);
            } else if line_slice.starts_with("two") {
                numbers.push(2);
            } else if line_slice.starts_with("three") {
                numbers.push(3);
            } else if line_slice.starts_with("four") {
                numbers.push(4);
            } else if line_slice.starts_with("five") {
                numbers.push(5);
            } else if line_slice.starts_with("six") {
                numbers.push(6);
            } else if line_slice.starts_with("seven") {
                numbers.push(7);
            } else if line_slice.starts_with("eight") {
                numbers.push(8);
            } else if line_slice.starts_with("nine") {
                numbers.push(9);
            }
        }

        result += numbers.first().unwrap() * 10 + numbers.last().unwrap();
    }
    println!("Part 2 result : {result}");
}

fn main() {
    let content = read_input_file("src/day1/input.txt");

    part1(&content);
    part2(&content);
}
