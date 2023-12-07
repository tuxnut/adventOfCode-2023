use std::{collections::HashMap, fs};

fn read_input_file(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path).expect("Failed to read input file: {path}");
    input.lines().map(|line| line.to_string()).collect()
}

fn part1(input: &Vec<String>) {
    let mut result = 0;

    let mut copy = input.clone();

    copy.sort_by(|a, b| {
        let first_hand: Vec<&str> = a.split(" ").collect();
        let f = first_hand.get(0).unwrap();

        let f_type = get_hand_type(f);

        let second_hand: Vec<&str> = b.split(" ").collect();
        let s = second_hand.get(0).unwrap();

        let s_type = get_hand_type(s);

        if f_type > s_type {
            return std::cmp::Ordering::Greater;
        } else if f_type < s_type {
            return std::cmp::Ordering::Less;
        } else {
            return compare_by_highest_card(f, s);
        }
    });

    // iterate over sorted hands with index
    for (i, hand) in copy.iter().enumerate() {
        let hand_and_bid: Vec<&str> = hand.split(" ").collect();
        let bid: i32 = hand_and_bid.get(1).unwrap().parse().unwrap();

        result += bid * (i as i32 + 1);
    }

    println!("Part 1 result : {result}");
}

fn get_hand_type(f: &str) -> i32 {
    // disctinct char in string
    let mut distinct_chars: Vec<char> = f.chars().collect();
    distinct_chars.sort();
    distinct_chars.dedup();

    if distinct_chars.len() == 1 {
        return 7; // five of a kind
    } else if distinct_chars.len() == 2 {
        if get_nb_occurences(distinct_chars[0], f) == 4
            || get_nb_occurences(distinct_chars[1], f) == 4
        {
            return 6; // four of a kind
        } else {
            return 5; // full house
        }
    } else if distinct_chars.len() == 3 {
        if get_nb_occurences(distinct_chars[0], f) == 3
            || get_nb_occurences(distinct_chars[1], f) == 3
            || get_nb_occurences(distinct_chars[2], f) == 3
        {
            return 4; // three of a kind
        } else {
            return 3; // two pairs
        }
    } else if distinct_chars.len() == 4 {
        return 2; // one pair
    } else {
        return 1; // high card
    }
}

fn get_nb_occurences(arg: char, f: &str) -> i32 {
    let mut occurence = 0;
    for c in f.chars() {
        if c == arg {
            occurence += 1;
        }
    }
    occurence
}

fn compare_by_highest_card(f: &str, s: &str) -> std::cmp::Ordering {
    let map_card_to_strongness: HashMap<char, i32> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);

    for i in 0..f.len() {
        let f_char = f.chars().nth(i).unwrap();
        let s_char = s.chars().nth(i).unwrap();

        if map_card_to_strongness.get(&f_char) > map_card_to_strongness.get(&s_char) {
            return std::cmp::Ordering::Greater;
        } else if map_card_to_strongness.get(&f_char) < map_card_to_strongness.get(&s_char) {
            return std::cmp::Ordering::Less;
        }
    }
    return std::cmp::Ordering::Less;
}

fn part2(input: &Vec<String>) {
    let result = 0;
    println!("Part 2 result : {result}");
}

fn main() {
    let content = read_input_file("src/day7/input.txt");

    part1(&content);
    part2(&content);
}
