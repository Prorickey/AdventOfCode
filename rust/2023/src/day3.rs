use std::time::SystemTime;

const INPUT: &str = include_str!("../challenge_data/2023/day3.txt");

pub fn day3() {
    let start = SystemTime::now();
    let p1 = part1();
    let p2 = part2();
    let duration: u128 = SystemTime::now().duration_since(start).unwrap().as_millis(); 

    println!("Part 1 ({p1}) & Part 2 ({p2}) in {}ms", duration)
}

fn part1() -> i32 {
    let grid: Vec<Vec<char>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let mut total: i32 = 0;

    for r in 0..rows {
        let cols = grid[r].len();
        let mut c = 0usize;
        while c < cols {
            if grid[r][c].is_ascii_digit() {
                let start = c;
                let mut num: i32 = 0;
                while c < cols && grid[r][c].is_ascii_digit() {
                    num = num * 10 + (grid[r][c] as u8 - b'0') as i32;
                    c += 1;
                }
                let end = c - 1; 

                let mut valid = false;
                let r0 = r.saturating_sub(1);
                let r1 = (r + 1).min(rows - 1);
                for rr in r0..=r1 {
                    let row_len = grid[rr].len();
                    let c0 = start.saturating_sub(1);
                    let c1 = (end + 1).min(row_len.saturating_sub(1));
                    for cc in c0..=c1 {
                        let ch = grid[rr][cc];
                        if ch != '.' && !ch.is_ascii_digit() {
                            valid = true;
                            break;
                        }
                    }
                    if valid { break; }
                }

                if valid {
                    total += num;
                }
            } else {
                c += 1;
            }
        }
    }

    total
}

fn part2() -> i32 {
    let grid: Vec<Vec<char>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let mut total: i32 = 0;
    let mut gear_nums: Vec<((usize, usize), i32)> = Vec::new();

    for r in 0..rows {
        let cols = grid[r].len();
        let mut c = 0usize;
        while c < cols {
            if grid[r][c].is_ascii_digit() {
                let start = c;
                let mut num: i32 = 0;
                while c < cols && grid[r][c].is_ascii_digit() {
                    num = num * 10 + (grid[r][c] as u8 - b'0') as i32;
                    c += 1;
                }
                let end = c - 1; 

                let r0 = r.saturating_sub(1);
                let r1 = (r + 1).min(rows - 1);
                for rr in r0..=r1 {
                    let row_len = grid[rr].len();
                    let c0 = start.saturating_sub(1);
                    let c1 = (end + 1).min(row_len.saturating_sub(1));
                    for cc in c0..=c1 {
                        let ch = grid[rr][cc];
                        if ch == '*' {
                            gear_nums.push(((rr, cc), num));
                            break;
                        }
                    }
                }
            } else {
                c += 1;
            }
        }
    }

    let mut gear_nums2: Vec<((usize, usize), i32)> = Vec::new();

    for l1 in gear_nums {
        println!("Gear: (({}, {}), {})", l1.0.0, l1.0.1, l1.1);
        let mut found = false;
        for (i, l2) in gear_nums2.iter().enumerate() {
            if l1.0.0 == l2.0.0 && l1.0.1 == l2.0.1 {
                total += l1.1 * l2.1;
                gear_nums2.remove(i);
                found = true;
                break
            }
        }

        if !found {
            gear_nums2.push(l1);
        }
    };

    total
}