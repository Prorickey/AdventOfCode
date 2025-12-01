// 2025 day 1

use std::{time::SystemTime};

const INPUT: &str = include_str!("../challenge_data/day1.txt");

pub fn day1() {
    let start = SystemTime::now();
    let tot1 = part1();
    let tot2 = part2();
    let duration: u128 = SystemTime::now().duration_since(start).unwrap().as_millis(); 
    
    println!("Numbers: Part 1 ({tot1}) & Part 2 ({tot2}) in {}ms", duration);
}

fn part1() -> u32 {
    let mut total: u32 = 0;
    
    let mut dial: i32 = 50;
    INPUT.split("\n").for_each(|instruction| { 
        let num: i32 = instruction
                .to_owned()
                .chars()
                .next()
                .map(|c| &instruction[c.len_utf8()..])
                .unwrap()
                .trim()
                .parse()
                .expect("Weird number");

        if instruction.starts_with("L") {
            dial -= num;
            let mut subzero = dial < 0;
            while subzero {
                dial = 100 + dial;
                if dial >= 0 {
                    subzero = false;
                }
            }
        } else {
            dial += num;
            let mut outofrange = dial > 99;
            while outofrange {
                dial = dial - 100;
                if dial <= 99 {
                    outofrange = false;
                }
            }
        }

        // println!("Instruction: {} Result: {}", instruction, dial);

        if dial == 0 {
            total += 1;
        }
    });

    total
}

fn part2() -> u32 {
    let mut total: u32 = 0;
    
    let mut dial: i32 = 50;
    INPUT.split("\n").for_each(|instruction| { 
        let num: i32 = instruction
                .to_owned()
                .chars()
                .next()
                .map(|c| &instruction[c.len_utf8()..])
                .unwrap()
                .trim()
                .parse()
                .expect("Weird number");

        if instruction.starts_with("L") {
            if dial == 0 {
                total -= 1;
            }
            dial -= num;
            let mut subzero = dial < 0;
            while subzero {
                dial = 100 + dial;
                total += 1;
                if dial >= 0 {
                    subzero = false;
                } 
            }
        } else {
            dial += num;
            let mut outofrange = dial > 99;
            while outofrange {
                dial = dial - 100;
                total += 1;
                if dial == 0 {
                    outofrange = false;
                    total -= 1;
                } else if dial <= 99 {
                    outofrange = false;
                }
            }
        }

        if dial == 0 {
            total += 1;
        }

        // println!("Instruction: {} Result: {} Total: {}", instruction, dial, total);
    });

    total
}