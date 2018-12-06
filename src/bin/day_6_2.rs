extern crate regex;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type GenError = Box<std::error::Error>;

fn process_list(coords: &Vec<Vec<u32>>) -> Result<i32, GenError> {
    let number_of_coords = coords.len();
    let mid_point_row: f64 =
        coords.iter().map(|v| v.get(1).unwrap()).sum().unwrap() / number_of_coords;
    let mid_point_col: f64 =
        coords.iter().map(|v| v.get(0).unwrap()).sum().unwrap() / number_of_coords;

    return Ok(0);
}

fn process_file(path: &str) -> Result<i32, GenError> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    let mut coords = Vec::new();

    r.lines().map(|l| l.unwrap()).for_each(|line| {
        let coord: Vec<u32> = line.split(",").map(|i| i.trim().parse().unwrap()).collect();
        coords.push(coord);
    });

    return process_list(&coords);
}

fn main() {
    let sum = process_file("src/res/day_6.txt").unwrap();
    println!("Checksum {}", sum);
}
