use regex::Regex;
use std::{fs, result};

fn read_input_file(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path).expect("Failed to read input file: {path}");
    input.lines().map(|line| line.to_string()).collect()
}

fn extract_game_id(line: &str) -> usize {
    let re = Regex::new(r"^Game (?P<game_id>\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    return caps
        .name("game_id")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
}

fn extract_red_cubes_number(line: &str) -> Vec<usize> {
    let re = Regex::new(r"(?P<red>\d+) red").unwrap();
    return re
        .captures_iter(line)
        .map(|cap| {
            return cap.name("red").unwrap().as_str().parse::<usize>().unwrap();
        })
        .collect::<Vec<usize>>();
}

fn extract_green_cubes_number(line: &str) -> Vec<usize> {
    let re = Regex::new(r"(?P<green>\d+) green").unwrap();
    return re
        .captures_iter(line)
        .map(|cap| {
            return cap
                .name("green")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
        })
        .collect::<Vec<usize>>();
}

fn extract_blue_cubes_number(line: &str) -> Vec<usize> {
    let re = Regex::new(r"(?P<blue>\d+) blue").unwrap();
    return re
        .captures_iter(line)
        .map(|cap| {
            return cap.name("blue").unwrap().as_str().parse::<usize>().unwrap();
        })
        .collect::<Vec<usize>>();
}

fn is_game_possible(max_red: usize, max_green: usize, max_blue: usize) -> bool {
    max_red <= 12 && max_green <= 13 && max_blue <= 14
}

fn part1(input: &Vec<String>) {
    let mut result = 0;

    for line in input {
        let game_id = extract_game_id(line);
        let reds = extract_red_cubes_number(line);
        let greens = extract_green_cubes_number(line);
        let blues = extract_blue_cubes_number(line);

        let max_red = reds.iter().max().unwrap();
        let max_green = greens.iter().max().unwrap();
        let max_blue = blues.iter().max().unwrap();

        if is_game_possible(*max_red, *max_green, *max_blue) {
            result += game_id;
        }
    }

    println!("Part 1 result : {result}");
}

fn part2(input: &Vec<String>) {
    let mut result = 0;

    for line in input {
        let reds = extract_red_cubes_number(line);
        let greens = extract_green_cubes_number(line);
        let blues = extract_blue_cubes_number(line);

        let max_red = reds.iter().max().unwrap();
        let max_green = greens.iter().max().unwrap();
        let max_blue = blues.iter().max().unwrap();

        let power = max_red * max_green * max_blue;
        result += power;
    }

    println!("Part 2 result : {result}");
}

fn main() {
    let content = read_input_file("src/day2/input.txt");

    part1(&content);
    part2(&content);
}
