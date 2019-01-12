extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::mem;

type GenError = Box<std::error::Error>;

fn merge_next_state_for_cell(current: i32, next: i32) -> i32 {
    if current == next {
        return current;
    }
    if current == -1 {
        return next;
    }
    if next == -1 {
        return current;
    }
    -2
}

fn process_list(coords: &[Vec<u32>]) -> Result<i32, GenError> {
    let max_row = *coords.iter().map(|v| &v[1]).max().unwrap() as usize;
    let max_col = *coords.iter().map(|v| &v[0]).max().unwrap() as usize;

    let mut prev_state = vec![vec![-1; max_col as usize + 1]; max_row + 1];

    for (idx, point) in coords.iter().enumerate() {
        prev_state[point[1] as usize][point[0] as usize] = idx as i32;
    }

    let mut next_state = vec![vec![-1; max_col as usize + 1]; max_row + 1];
    let mut done = false;

    while !done {
        // println!("---------------------------------------------------------------------------");
        // for row in &prev_state {
        //     println!("{:?}", row);
        // }

        done = true;
        for y in 0..=max_row {
            for x in 0..=max_col {
                if prev_state[y][x] == -1 {
                    done = false;
                    let mut next_state_for_cell = if y > 0 { prev_state[y - 1][x] } else { -1 };
                    if y < max_row {
                        next_state_for_cell =
                            merge_next_state_for_cell(prev_state[y + 1][x], next_state_for_cell);
                    }
                    if x > 0 {
                        next_state_for_cell =
                            merge_next_state_for_cell(prev_state[y][x - 1], next_state_for_cell);
                    }
                    if x < max_col {
                        next_state_for_cell =
                            merge_next_state_for_cell(prev_state[y][x + 1], next_state_for_cell);
                    }
                    next_state[y][x] = next_state_for_cell
                } else {
                    next_state[y][x] = prev_state[y][x];
                }
            }
        }

        mem::swap(&mut prev_state, &mut next_state);
    }

    // println!("---------------------------------------------------------------------------");
    // for row in &prev_state {
    //     println!("{:?}", row);
    // }

    let mut frequency = HashMap::new();
    for y in 0..=max_row {
        for x in 0..=max_col {
            *frequency.entry(prev_state[y][x]).or_insert(0) += 1;
        }
    }

    // Remove all zone on the edges.
    for y in 0..=max_row {
        frequency.remove(&prev_state[y][0]);
        frequency.remove(&prev_state[y][max_col]);
    }
    for x in 0..=max_col {
        frequency.remove(&prev_state[0][x]);
        frequency.remove(&prev_state[max_row][x]);
    }

    println!("{:?}", frequency);

    return Ok(*frequency.values().max().unwrap());
}

fn process_file(path: &str) -> Result<i32, GenError> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    let mut coords = Vec::new();

    r.lines().map(|l| l.unwrap()).for_each(|line| {
        let coord: Vec<u32> = line.split(',').map(|i| i.trim().parse().unwrap()).collect();
        coords.push(coord);
    });

    process_list(&coords)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_6_1() {
        assert_eq!(3569, process_file("src/res/day_6.txt").unwrap());
    }
}

fn main() {
    let sum = process_file("src/res/day_6.txt").unwrap();
    println!("Checksum {}", sum);
}
