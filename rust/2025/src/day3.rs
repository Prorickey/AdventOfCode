// 2025 day 3

use std::{cmp, time::SystemTime};

const INPUT: &str = include_str!("../challenge_data/day3.txt");

pub fn day3() {
    let start = SystemTime::now();
    let tot1 = part1();
    let tot2 = part2();
    let duration: u128 = SystemTime::now().duration_since(start).unwrap().as_millis(); 
    
    println!("Numbers: Part 1 ({tot1}) & Part 2 ({tot2}) in {}ms", duration);
}

fn part1() -> u32 {
    INPUT
        .split("\n")
        .map(|s| {
            let digits = s.chars()
                .map(|d| d.to_digit(10).unwrap())
                .into_iter().collect::<Vec<_>>();

            let mut max: u32 = 0;
            for (i, d1) in digits.clone().iter().enumerate() {
                for d2 in &digits[(i+1)..digits.len()] {
                    max = cmp::max(d1 * 10 + d2, max);
                }
            }

            max
        })
        .sum()
}

fn part2() -> u64 {
    INPUT
        .split("\n")
        .map(|s| {
            let digits: Vec<u64> = s.chars()
                .map(|d| d.to_digit(10).unwrap().try_into().unwrap())
                .into_iter().collect::<Vec<_>>();

            let mut value = 0;
            let mut c_i = 0;
            let base: u64 = 10;
            for i in 0..12 {
                let new_digits = digits.clone()[c_i..digits.len()-11+i].to_vec();
                let largest = new_digits
                    .iter()
                    .enumerate()
                    .rev()
                    .max_by_key(|&(_, &val)| val)
                    .map(|(index, _)| index)
                    .unwrap();

                c_i += largest+1;
                value = value + new_digits[largest] * base.pow((12-i-1).try_into().unwrap())
            }

            value
        })
        .sum()
}