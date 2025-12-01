// 2023 day 1

use std::{time::SystemTime};

use regex::Regex;
use itertools::Itertools;

const INPUT: &str = include_str!("../challenge_data/2023/day1.txt");

pub fn day1() {
    let start = SystemTime::now();
    let tot1 = part1();
    let tot2 = part2();
    let duration: u128 = SystemTime::now().duration_since(start).unwrap().as_millis(); 
    
    println!("Calibrations: Part 1 ({tot1}) & Part 2 ({tot2}) in {}ms", duration);

    let start = SystemTime::now();
    let p1 = adjusted_part1();
    let p2 = adjusted_part2();
    let duration: u128 = SystemTime::now().duration_since(start).unwrap().as_millis(); 

    println!("Calibrations: Part 1 ({p1}) & Part 2 ({p2}) in {}ms", duration);
}

fn part1() -> u32 {
    let lines: Vec<&str> = INPUT.split("\n").collect();

    let mut total = 0;
    for line in lines {
        let chars: Vec<&str> = line.split("").collect();
        let chars_rev: Vec<&str> = chars.iter().rev().copied().collect();

        for c in chars {
            let n: u32 = match c.parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            total += n*10;
            break;
        }

        for c in chars_rev {
            let n: u32 = match c.parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            total += n;
            break;
        }
    }

    total
}

fn adjusted_part1() -> i32 {
   INPUT
        .lines()
        .map(|line| {
            let mut it = line.chars().filter(|c| c.is_ascii_digit());
            let first = it.next().unwrap();
            let last = it.next_back().unwrap_or(first);
            format!("{first}{last}").parse::<i32>().unwrap()
        })
        .sum()
}

fn part2() -> u32 {
    let lines: Vec<&str> = INPUT.split("\n").collect();

    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();

    let mut total = 0;
    for line in lines {
        let matches: Vec<&str> = find_overlapping(re.clone(), line);

        total += get_num(matches[0]) * 10;
        total += get_num(matches[matches.len()-1]);
    }

    total
}

const NUMBERS: &[&str] = &[
    "0", "zero", "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7",
    "seven", "8", "eight", "9", "nine",
];

fn adjusted_part2() -> usize {
    INPUT
        .lines()
        .map(|line| {
            let (a, b) = NUMBERS 
                .iter()
                .enumerate()
                .flat_map(|(i, &n)| line.match_indices(n).map(move |(idx, _)| (idx, i / 2)))
                .minmax()
                .into_option()
                .unwrap();

            a.1 * 10 + b.1
        })
        .sum()
}

fn get_num(mat: &str) -> u32 {
    if mat.parse::<u32>().is_ok() {
        return mat.parse::<u32>()
            .expect("Weird and wild error happened");
    } else {
        return match mat {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 0
        };
    }
}

fn find_overlapping(re: Regex, text: &str) -> Vec<&str> {
    let mut matches = Vec::new();
    let mut start_index = 0;

    while start_index < text.len() {
        if let Some(m) = re.find_at(text, start_index) {
            matches.push(m.as_str());

            start_index = m.start() + 1;
        } else {
            start_index += 1;
        }
    }

    matches
}