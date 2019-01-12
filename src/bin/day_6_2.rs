extern crate regex;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type GenError = Box<std::error::Error>;

fn process_list(coords: &[Vec<i32>], max_distance: i32) -> Result<i32, GenError> {
    let max_row = *coords.iter().map(|v| &v[1]).max().unwrap() as usize;
    let max_col = *coords.iter().map(|v| &v[0]).max().unwrap() as usize;

    // BRUTE FORCE !!!!!!
    let mut region_size = 0;
    for y in 0..=max_row {
        for x in 0..=max_col {
            let distance: i32 = coords
                .iter()
                .map(|c| (c[0] - x as i32).abs() + (c[1] - y as i32).abs())
                .sum();
            if distance < max_distance {
                region_size += 1;
            }
        }
    }

    Ok(region_size)
}

fn process_file(path: &str, max_distance: i32) -> Result<i32, GenError> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    let mut coords = Vec::new();

    r.lines().map(|l| l.unwrap()).for_each(|line| {
        let coord: Vec<i32> = line.split(',').map(|i| i.trim().parse().unwrap()).collect();
        coords.push(coord);
    });

    process_list(&coords, max_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_6_2() {
        assert_eq!(48978, process_file("src/res/day_6.txt", 10000).unwrap());
    }
}

fn main() {
    let sum = process_file("src/res/day_6.txt", 10000).unwrap();
    println!("Checksum {}", sum);
}
