// 2025 day 5

use std::{collections::HashSet, time::SystemTime};

const INPUT: &str = include_str!("../challenge_data/day5.txt");

pub fn day5() {
    let start = SystemTime::now();
    let tot1 = part1();
    let tot2 = part2();
    let duration: u128 = SystemTime::now().duration_since(start).unwrap().as_millis(); 
    
    println!("Numbers: Part 1 ({tot1}) & Part 2 ({tot2}) in {}ms", duration);
}

fn part1() -> usize {
    let spl: Vec<&str> = INPUT.split("\n\n").collect();

    let rules: Vec<Vec<usize>> = spl[0]
        .split("\n")
        .map(|r| r.split("-").map(|a| a.parse::<usize>().unwrap()).collect())
        .collect();

    spl[1]
        .split("\n")
        .map(|n| n.parse::<usize>().unwrap())
        .map(|n| {
            let mut valid = false;
            for rule in rules.clone() {
                if n >= rule[0] && n <= rule[1] {
                    valid = true;
                    break
                }
            }

            return valid as usize
        })
        .sum()
}

fn part2() -> usize {
    let spl: Vec<&str> = INPUT.split("\n\n").collect();

    let mut rules: Vec<Vec<usize>> = spl[0]
        .split("\n")
        .map(|r| r.split("-").map(|a| a.parse::<usize>().unwrap()).collect())
        .collect();

    rules.sort_by(|v1, v2| v1[0].cmp(&v2[0]));

    let mut i = 0;
    loop {
        if i >= rules.len()-1 {
            break
        }

        if rules[i][1] >= rules[i+1][0] {
            rules[i][1] = rules[i][1].max(rules[i+1][1]);
            rules.remove(i+1);
        } else {
            i += 1;
        }
    }

    for rule in rules.clone() {
        println!("Rule: {}-{}", rule[0], rule[1])
    }

    let mut valid_ingredients: usize = 0;

    for rule in rules {
        valid_ingredients += rule[1]-rule[0]+1
    }

    valid_ingredients
}