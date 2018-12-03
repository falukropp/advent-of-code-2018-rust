extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// #[derive(Copy, Clone, Debug, PartialEq)]
struct Slab {
    id: u32,
    top: usize,
    left: usize,
    bottom: usize,
    right: usize,
}

fn sum_file(path: &str) -> Result<u32, std::io::Error> {
    let f = File::open(path)?;
    let r = BufReader::new(f);
    // #1 @ 1,3: 4x4
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    let mut slabs: Vec<Slab> = Vec::new();
    let mut max_height: usize = 0;
    let mut max_width: usize = 0;

    for line_result in r.lines() {
        let line = line_result?;

        for cap in re.captures_iter(&line) {
            let id = *&cap[1].parse::<u32>().unwrap();
            let top = *&cap[2].parse::<usize>().unwrap();
            let left = *&cap[3].parse::<usize>().unwrap();
            let bottom = top + &cap[4].parse::<usize>().unwrap();
            let right = left + &cap[5].parse::<usize>().unwrap();

            max_height = max_height.max(bottom);
            max_width = max_width.max(right);

            slabs.push(Slab {
                id,
                top,
                left,
                bottom,
                right,
            });
        }
    }

    println!("{} {}", max_width, max_height);

    let mut state = vec![vec![0; max_width]; max_height];
    let multiple_marker = slabs.len() as u32;
    let mut uncovered_slabs: HashSet<u32> = HashSet::new();

    for slab in &slabs {
        let mut collision = false;
        for row in slab.top..slab.bottom {
            for col in slab.left..slab.right {
                state[row][col] = match state[row][col] {
                    0 => slab.id,
                    m if m == multiple_marker => {
                        collision = true;
                        multiple_marker
                    }
                    other_slab => {
                        uncovered_slabs.remove(&other_slab);
                        collision = true;
                        multiple_marker
                    }
                }
            }
        }
        if !collision {
            uncovered_slabs.insert(slab.id);
        }
    }

    assert_eq!(1, uncovered_slabs.len());

    // for row in state {
    //     for col in row {
    //         print!("{}", col);
    //     }
    //     println!("");
    // }

    return Ok(*uncovered_slabs.iter().next().unwrap());
}

fn main() {
    let sum = sum_file("src/res/day_3.txt").unwrap();
    println!("Id of uncovered slab {}", sum);
}
