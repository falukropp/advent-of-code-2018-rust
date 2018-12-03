use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn find_checksum(path: &str) -> Result<i64, std::io::Error> {
    let f = File::open(path)?;
    let r = BufReader::new(f);
    let mut with2 = 0;
    let mut with3 = 0;
    for line_result in r.lines() {
        let line = line_result.unwrap();
        // Maybe use array?
        let mut frequency = HashMap::new();
        for c in line.chars() {
            *frequency.entry(c).or_insert(0) += 1;
        }
        if frequency.iter().any(|(_, value)| *value == 2) {
            with2 += 1;
        }
        if frequency.iter().any(|(_, value)| *value == 3) {
            with3 += 1;
        }
    }
    return Ok(with2 * with3);
}

fn main() {
    let sum = find_checksum("src/res/day_2.txt").unwrap();
    println!("The checksum is {}", sum);
}
