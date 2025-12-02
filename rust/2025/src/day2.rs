// 2025 day 2

use std::{time::SystemTime};

const INPUT: &str = include_str!("../challenge_data/day2.txt");

pub fn day2() {
    let start = SystemTime::now();
    let tot1 = part1();
    let tot2 = part2();
    let duration: u128 = SystemTime::now().duration_since(start).unwrap().as_millis(); 
    
    println!("Numbers: Part 1 ({tot1}) & Part 2 ({tot2}) in {}ms", duration);
}

fn part1() -> u64 {
    let mut total = 0;
    let ranges = INPUT.trim().split(",").map(|s| {
        let nums: Vec<u64> = s.split("-").map(|n| n.parse::<u64>().unwrap()).collect();

        return vec![nums[0], nums[1]]
    });

    for range in ranges {
        let n1 = range[0];
        let n2 = range[1];

        for i in 0..=(n2-n1) {
            let num = (n1+i).to_string();
            let mid_byte_index = num.len() / 2;

            let mut split_index = mid_byte_index;
            while !num.is_char_boundary(split_index) {
                split_index -= 1;
            }

            let (num1, num2) = num.split_at(split_index);
            if num1 == num2 {
                total += num.parse::<u64>().unwrap()
            }
        }
    }

    total
}

fn part2() -> u64 {
    let mut total = 0;
    let ranges = INPUT.trim().split(",").map(|s| {
        let nums: Vec<u64> = s.split("-").map(|n| n.parse::<u64>().unwrap()).collect();

        return vec![nums[0], nums[1]]
    });

    for range in ranges {
        let n1 = range[0];
        let n2 = range[1];

        'outer: for i in 0..=(n2-n1) {
            let num = (n1+i).to_string();

            'inner: for i in 1..=num.len() {
                let temp_num = &num[0..i];
                if num.len() % i != 0 || i == num.len() {
                    continue 'inner;
                }

                let mut valid = true;

                for j in 0..(num.len() / i) {
                    if temp_num != &num[j*i..(j+1)*i] {
                        valid = false;
                    }
                }

                if valid {
                    total += num.parse::<u64>().unwrap();
                    continue 'outer
                }
            }
        }
    }

    total
}