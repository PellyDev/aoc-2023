use std::collections::HashSet;

type Coords = (usize, usize);

fn search_nbrs(board: &Vec<Vec<char>>, row: usize, col: usize) -> Option<Vec<Coords>> {
    let mut out: Vec<(usize, usize)> = Vec::new();
    for i in row - 1..row + 2 {
        for j in col - 1..col + 2 {
            if board.get(i)?.get(j)?.is_ascii_digit() {
                out.push((i, j));
            }
        }
    }
    Some(out)
}
fn find_number(board: &Vec<Vec<char>>, idcs: Coords, set: &mut HashSet<Coords>) -> Option<u32> {
    let (row, col) = idcs;
    let mut offset: usize = 0;
    let mut start: usize = 0;
    let mut end: usize = 0;
    // search start
    while let Some(c) = board[row].get(col - offset) {
        if c.is_ascii_digit() {
            start = col - offset;

            if col as isize - offset as isize == 0 {
                break;
            }
            offset += 1;
        } else {
            break;
        }
    }
    // serach end
    offset = 0;
    while let Some(c) = board[row].get(col + offset) {
        if c.is_ascii_digit() {
            end = col + offset;
            offset += 1;
        } else {
            break;
        }
    }
    // check if num was already processed
    for n in start..=end {
        if !set.insert((row, n)) {
            return None;
        }
    }
    let ts: &[char] = &board[row][start..=end];
    Some(
        // build num from char slice
        ts.iter()
            .enumerate()
            .map(|(idx, char)| {
                let digit = char.to_digit(10).unwrap();
                let m = 10_u32.pow((ts.len() - idx - 1) as u32);
                digit * m
            })
            .sum(),
    )
}
fn main() {
    let mut res: u32 = 0;
    let mut visited: HashSet<Coords> = HashSet::new();
    let input = include_str!("../my_input.txt");
    let engine: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    for (r, line) in engine.iter().enumerate() {
        for (c, char) in line.iter().enumerate() {
            if !char.is_ascii_digit() && *char != '.' {
                if let Some(nbrs) = search_nbrs(&engine, r, c) {
                    nbrs.into_iter().for_each(|coords| {
                        if let Some(num) = find_number(&engine, coords, &mut visited) {
                            res += num;
                        };
                    })
                }
            }
        }
    }
    println!("{}", res);
}
