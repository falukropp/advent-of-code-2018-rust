extern crate regex;

use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type GenError = Box<std::error::Error>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Nanobot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

impl Nanobot {
    // Just a stupid rip of https://gist.github.com/pdewacht/fa7aa7e60952c6d67956599d6d4af360
    // What a disappointment.
    fn within_cube(&self, x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> bool {
        let mut dist = 0;
        if self.x < x1 {
            dist += x1 - self.x;
        } else if self.x > x2 {
            dist += self.x - x2;
        }
        if self.y < y1 {
            dist += y1 - self.y;
        } else if self.y > y2 {
            dist += self.y - y2;
        }
        if self.z < z1 {
            dist += z1 - self.z;
        } else if self.z > z2 {
            dist += self.z - z2;
        }

        dist <= self.r
    }
}

fn read_nanobots_from_file(path: &str) -> Result<Vec<Nanobot>, GenError> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();

    let mut nanobots: Vec<Nanobot> = Vec::new();

    for line_result in r.lines() {
        let line = line_result?;

        for cap in re.captures_iter(&line) {
            let x = *&cap[1].parse().unwrap();
            let y = *&cap[2].parse().unwrap();
            let z = *&cap[3].parse().unwrap();
            let r = *&cap[4].parse().unwrap();

            let nanobot = Nanobot { x, y, z, r };

            //            println!("{:?}", nanobot);

            nanobots.push(nanobot);
        }
    }

    Ok(nanobots)
}

// Basically just a rip https://gist.github.com/pdewacht/fa7aa7e60952c6d67956599d6d4af360. Can't take any credit for this.
// AoC 2018 is such a disappointment compared to 2017... :(

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Subdivision {
    matches: u32,
    dist: i64,
    s: i64,
    x1: i64,
    y1: i64,
    z1: i64,
}

impl Ord for Subdivision {
    fn cmp(&self, other: &Subdivision) -> Ordering {
        self.matches
            .cmp(&other.matches)
            .then_with(|| other.dist.cmp(&self.dist)) // Reverse search on dist, smaller is better
            .then_with(|| other.s.cmp(&self.s)) // Smaller is better
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Subdivision {
    fn partial_cmp(&self, other: &Subdivision) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_closest_with_most_overlap(nanobots: &Vec<Nanobot>) -> i64 {
    // Find world_corners
    let x1 = nanobots.iter().min_by_key(|n| n.x).unwrap().x;
    let x2 = nanobots.iter().max_by_key(|n| n.x).unwrap().x;
    let y1 = nanobots.iter().min_by_key(|n| n.y).unwrap().y;
    let y2 = nanobots.iter().max_by_key(|n| n.y).unwrap().y;
    let z1 = nanobots.iter().min_by_key(|n| n.z).unwrap().z;
    let z2 = nanobots.iter().max_by_key(|n| n.z).unwrap().z;

    println!("{} {} {} {} {} {}", x1, x2, y1, y2, z1, z2);

    let max_diff = (x2 - x1).max(y2 - y1).max(z2 - z1);

    let mut s = 1;
    while s < max_diff {
        s *= 2;
    }

    println!("cube_size {} division_size {}", max_diff, s);

    let mut subdivisions = BinaryHeap::new();
    let matches = nanobots.len() as u32;
    let dist = x1.abs().min(x2.abs()) + y1.abs().min(y2.abs()) + z1.abs().min(z2.abs());
    subdivisions.push(Subdivision {
        matches,
        dist,
        s,
        x1,
        y1,
        z1,
    });

    while let Some(sdiv) = subdivisions.pop() {
        if sdiv.s == 1 {
            return sdiv.dist;
        }

        let s = sdiv.s / 2;

        for start_point in [
            (sdiv.x1, sdiv.y1, sdiv.z1),
            (sdiv.x1, sdiv.y1, sdiv.z1 + s),
            (sdiv.x1, sdiv.y1 + s, sdiv.z1),
            (sdiv.x1, sdiv.y1 + s, sdiv.z1 + s),
            (sdiv.x1 + s, sdiv.y1, sdiv.z1),
            (sdiv.x1 + s, sdiv.y1, sdiv.z1 + s),
            (sdiv.x1 + s, sdiv.y1 + s, sdiv.z1),
            (sdiv.x1 + s, sdiv.y1 + s, sdiv.z1 + s),
        ]
            .iter()
        {
            let x1 = start_point.0;
            let y1 = start_point.1;
            let z1 = start_point.2;

            let x2 = x1 + s - 1;
            let y2 = y1 + s - 1;
            let z2 = z1 + s - 1;

            let mut matches = nanobots
                .iter()
                .filter(|&n| n.within_cube(x1, x2, y1, y2, z1, z2))
                .count() as u32;

            if matches > 0 {
                let dist = x1.abs().min(x2.abs()) + y1.abs().min(y2.abs()) + z1.abs().min(z2.abs());
                subdivisions.push(Subdivision {
                    matches,
                    dist,
                    s,
                    x1,
                    y1,
                    z1,
                })
            }
        }
    }

    panic!("No solution found!");
    // 0
}

fn main() {
    let nanobots_ex = read_nanobots_from_file("src/res/day_23_ex_3.txt").unwrap();
    println!("{}", find_closest_with_most_overlap(&nanobots_ex)); // 36

    let nanobots = read_nanobots_from_file("src/res/day_23.txt").unwrap();
    println!("{}", find_closest_with_most_overlap(&nanobots)); // 110620102
}
