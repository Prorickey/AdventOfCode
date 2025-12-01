// 2023 day 2

use std::time::{SystemTime};

use regex::Regex;

const INPUT: &str = include_str!("../challenge_data/2023/day2.txt");

pub fn day2() {
    let start = SystemTime::now();
    let p1 = part1();
    let p2 = part2();
    let duration: u128 = SystemTime::now().duration_since(start).unwrap().as_millis(); 

    println!("Part 1 ({p1}) & Part 2 ({p2}) in {}ms", duration);
}

fn part1() -> u32 {
    let re = Regex::new(r"\d+\s*(red|green|blue)").unwrap();

    let lines: Vec<&str> = INPUT.split("\n").collect();
    let mut total = 0;

    for line in lines {
        let split: Vec<&str> = line.split(":").collect();
        let id: u32 = split[0][5..].parse().expect("game id malformed");
        
        let played: Vec<&str> = split[1].trim().split(";").map(|s| s.trim()).collect();
        let mut possible = true;
        for game in played {
            for c in re.find_iter(game).map(|c| c.as_str()) {
                let split2: Vec<&str> = c.split(" ").collect();
                let n: u32 = split2[0].parse().expect("game content malformed");
                
                match split2[1] {
                    "blue" => {
                        if n > 14 {
                            possible = false
                        }
                    },
                    "red" => {
                        if n > 12 {
                            possible = false 
                        }
                    },
                    "green" => {
                        if n > 13 {
                            possible = false
                        }
                    },
                    _ => continue
                }
            }
        }

        if possible {
            total += id;
        }
    }

    total
}

fn part2() -> u32 {
    let re = Regex::new(r"\d+\s*(red|green|blue)").unwrap();

    let lines: Vec<&str> = INPUT.split("\n").collect();
    let mut total = 0;

    for line in lines {
        let split: Vec<&str> = line.split(":").collect();
        let played: Vec<&str> = split[1].trim().split(";").map(|s| s.trim()).collect();
        
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;
        for game in played {
            for c in re.find_iter(game).map(|c| c.as_str()) {
                let split2: Vec<&str> = c.split(" ").collect();
                let n: u32 = split2[0].parse().expect("game content malformed");
                
                match split2[1] {
                    "blue" => {
                        if blue < n {
                            blue = n;
                        }
                    },
                    "red" => {
                        if red < n {
                            red = n;
                        }
                    },
                    "green" => {
                        if green < n {
                            green = n;
                        }
                    },
                    _ => continue
                }
            }
        }

        total += green * blue * red;
    }

    total
}