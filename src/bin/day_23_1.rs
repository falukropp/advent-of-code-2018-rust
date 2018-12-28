extern crate regex;

use regex::Regex;
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
    fn within_range(&self, other: &Nanobot) -> bool {
        let manhattan_distance =
            (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs();
        // println!(
        //     "Bot {:?} is {} distance away, is in range {}",
        //     other,
        //     manhattan_distance,
        //     manhattan_distance <= self.r
        // );
        manhattan_distance <= self.r
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

fn find_largets_nanobots_neighbours(nanobots: &Vec<Nanobot>) -> u32 {
    let biggest_r = nanobots.iter().max_by(|&n1, &n2| n1.r.cmp(&n2.r)).unwrap();
    println!("biggest radius nanobot is {:?}", biggest_r);

    nanobots
        .iter()
        .filter(|&n| biggest_r.within_range(n))
        .count() as u32
}

fn main() {
    let nanobots_ex = read_nanobots_from_file("src/res/day_23_ex.txt").unwrap();
    println!("{}", find_largets_nanobots_neighbours(&nanobots_ex)); // 7

    let nanobots = read_nanobots_from_file("src/res/day_23.txt").unwrap(); // 602
    println!("{}", find_largets_nanobots_neighbours(&nanobots));
}
