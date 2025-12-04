// 2025 day 4

use std::{collections::HashSet, time::SystemTime};

const INPUT: &str = include_str!("../challenge_data/day4.txt");

pub fn day4() {
    let start = SystemTime::now();
    let tot1 = part1();
    let tot2 = part2();
    let duration: u128 = SystemTime::now().duration_since(start).unwrap().as_millis(); 
    
    println!("Numbers: Part 1 ({tot1}) & Part 2 ({tot2}) in {}ms", duration);
}

fn part1() -> usize {
    let grid: Vec<Vec<usize>> = INPUT
        .split("\n")
        .map(|arr| {
            arr.chars().map(|c| {
                if c == '.' {
                    return 0
                } else {
                    return 1
                }
            })
            .collect()
        })
        .collect();

    let mut existing_rolls: HashSet<(usize, usize)> = HashSet::new();

    run_thingy(grid, &mut existing_rolls);

    existing_rolls.len()
}

fn part2() -> usize {
    let mut grid: Vec<Vec<usize>> = INPUT
        .split("\n")
        .map(|arr| {
            arr.chars().map(|c| {
                if c == '.' {
                    return 0
                } else {
                    return 1
                }
            })
            .collect()
        })
        .collect();

    let mut existing_rolls: HashSet<(usize, usize)> = HashSet::new();

    loop {
        let (new_grid, total) = run_thingy(grid, &mut existing_rolls);

        grid = new_grid;

        if total == 0 {
            break
        }
    }

    existing_rolls.len()
}

fn run_thingy(grid: Vec<Vec<usize>>, existing_rolls: &mut HashSet<(usize, usize)>) -> (Vec<Vec<usize>>, usize) {
    let mut new_grid = grid.clone();
    let mut total = 0;
    for (r, row) in grid.clone().into_iter().enumerate() {
        for (c, spot) in row.clone().into_iter().enumerate() {
            if spot == 0 {
                continue
            }

            let mut count = 0;

            if r > 0 {
                if c > 0 && grid[r-1][c-1] == 1 {
                    count += 1;
                }

                if c < row.len()-1 && grid[r-1][c+1] == 1 {
                    count += 1;
                }

                if grid[r-1][c] == 1 {
                    count += 1;
                }
            }

            if r < (grid.len()-1) {
                if c > 0 && grid[r+1][c-1] == 1 {
                    count += 1;
                }

                if c < (row.len()-1) && grid[r+1][c+1] == 1 {
                    count += 1;
                }

                if grid[r+1][c] == 1 {
                    count += 1;
                }
            }

            if c > 0 && grid[r][c-1] == 1 {
                count += 1;
            }

            if c < (grid.len()-1) && grid[r][c+1] == 1 {
                count += 1;
            }

            if count < 4 {
                total += 1;
                new_grid[r][c] = 0;
                existing_rolls.insert((r, c));
            }
        }
    } 

    (new_grid, total)
}