// 2025 day 6

use std::{time::SystemTime};

const INPUT: &str = include_str!("../challenge_data/day6.txt");

pub fn day6() {
    let start = SystemTime::now();
    let tot1 = part1();
    let tot2 = part2();
    let duration: u128 = SystemTime::now().duration_since(start).unwrap().as_millis(); 
    
    println!("Numbers: Part 1 ({tot1}) & Part 2 ({tot2}) in {}ms", duration);
}

fn part1() -> usize {
    let input: Vec<Vec<char>> = INPUT
        .split("\n")
        .map(|r| r.chars().collect())
        .collect();

    let mut columns_indexes: Vec<usize> = Vec::new();
    columns_indexes.push(0);

    'outer: for i in 0..input[0].len() {
        for row in input.clone() {
            if row[i] != ' ' {
                continue 'outer;
            }
        }

        columns_indexes.push(i)
    }

    columns_indexes.push(input[0].len());

    let mut total: usize = 0;

    for i in 1..columns_indexes.len() {
        let mut nums: Vec<usize> = Vec::new();
        let mut multiply = false;
        for row in input.clone() {
            let s: String = row[columns_indexes[i-1]..columns_indexes[i]].into_iter().collect();
            if s.trim() == "" {
                break
            }

            if s.trim() == "*" {
                multiply = true;
                break
            } else if s.trim() == "+" {
                multiply = false;
                break
            }

            nums.push(s.trim().parse().unwrap())
        }

        if multiply {
            total += nums.iter().product::<usize>()
        } else {
            total += nums.iter().sum::<usize>()
        }
    }

    total
}

fn part2() -> usize {
    let input: Vec<Vec<char>> = INPUT
        .split("\n")
        .map(|r| r.chars().collect())
        .collect();

    let mut columns_indexes: Vec<usize> = Vec::new();
    columns_indexes.push(0);

    'outer: for i in 0..input[0].len() {
        for row in input.clone() {
            if row[i] != ' ' {
                continue 'outer;
            }
        }

        columns_indexes.push(i)
    }

    columns_indexes.push(input[0].len());

    let mut total: usize = 0;

    for i in 1..columns_indexes.len() {
        let mut nums: Vec<usize> = Vec::new();
        let mut multiply = false;

        for col in columns_indexes[i-1]..columns_indexes[i] {
            let s: String = input[0..input.len()-1].iter()
                .filter_map(|line| Some(line[col]))
                .collect();

            if s.trim() == "" {
                continue
            }

            nums.push(s.trim().parse().unwrap())
        }

        let s: String = input[input.len()-1][columns_indexes[i-1]..columns_indexes[i]].into_iter().collect();

        if s.trim() == "*" {
            multiply = true;
        } else if s.trim() == "+" {
            multiply = false;
        }

        if multiply {
            total += nums.iter().product::<usize>()
        } else {
            total += nums.iter().sum::<usize>()
        }
    }

    total
}