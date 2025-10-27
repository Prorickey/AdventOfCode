const INPUT: &str = include_str!("../../../challenge_data/2023/day3.txt");

pub fn day3() {
    let p1 = part1();
    println!("Part 1 ({p1})")
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

fn part2() {

}