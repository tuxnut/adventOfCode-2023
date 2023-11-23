use std::fs;

fn read_input_file(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path).expect("Failed to read input file: {path}");
    input.lines().map(|line| line.to_string()).collect()
}

fn part1(input: &Vec<String>) {
    let result = 0;
    println!("Part 1 result : {result}");
}

fn part2(input: &Vec<String>) {
    let result = 0;
    println!("Part 2 result : {result}");
}

fn main() {
    let content = read_input_file("src/day1/input.txt");

    part1(&content);
    part2(&content);
}
