// 2025 day 7

use std::{collections::HashSet, time::SystemTime};

const INPUT: &str = include_str!("../challenge_data/day7.txt");

pub fn day7() {
    let start = SystemTime::now();
    let tot1 = part1();
    let tot2 = part2();
    let duration: u128 = SystemTime::now().duration_since(start).unwrap().as_millis(); 
    
    println!("Numbers: Part 1 ({tot1}) & Part 2 ({tot2}) in {}ms", duration);
}

fn part1() -> usize {
    let grid: Vec<Vec<char>> = INPUT 
        .split("\n")
        .map(|l| l.chars().collect())
        .collect();

    let mut beam_indexes: HashSet<usize> = HashSet::new();

    for (i, c) in grid[0].clone().iter().enumerate() {
        if c.to_owned() == 'S' {
            beam_indexes.insert(i);
        }
    }

    let mut total = 0;

    for r in 1..grid.len() {
        let mut new_beam_indexes: HashSet<usize> = HashSet::new();
        for c in beam_indexes.clone().into_iter() {
            if grid[r][c].to_owned() == '.' {
                new_beam_indexes.insert(c);
            } else if grid[r][c].to_owned() == '^' {
                new_beam_indexes.insert(c-1);
                new_beam_indexes.insert(c+1);
                total += 1;
            }
        }

        beam_indexes = new_beam_indexes
    }

    total
}

fn part2() -> usize {
    let grid: Vec<Vec<char>> = INPUT 
        .split("\n")
        .map(|l| l.chars().collect())
        .enumerate()
        .filter(|(i, _)|  i % 2 == 0)
        .map(|(_, r)| r)
        .collect();

    let mut beam_possiblities: Vec<usize> = grid[0].clone().into_iter().map(|c| {
        if c.to_owned() == 'S' {
            return 1;
        } else {
            return 0;
        }
    }).collect();

    for r in 1..grid.len() {
        let mut new_beam_possibilities: Vec<usize> = vec![0; grid[0].clone().len()];
        for (i, c) in grid[r].clone().into_iter().enumerate() {
            if c.to_owned() == '.' {
                new_beam_possibilities[i] += beam_possiblities[i];
            } else if c.to_owned() == '^' {
                new_beam_possibilities[i-1] += beam_possiblities[i];
                new_beam_possibilities[i+1] += beam_possiblities[i];
            }
        }

        beam_possiblities = new_beam_possibilities;
    }

    beam_possiblities.into_iter().sum::<usize>()
}