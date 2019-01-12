use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn find_duplicate_sum(path: &str) -> Result<i64, std::io::Error> {
    let f = File::open(path)?;
    let r = BufReader::new(f);
    let numbers: Vec<i64> = r
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect();

    let mut sum = 0i64;

    let mut encountered: HashSet<i64> = HashSet::new();
    encountered.insert(0);
    loop {
        for number in &numbers {
            sum += number;
            if !encountered.insert(sum) {
                return Ok(sum);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_2() {
        assert_eq!(256, find_duplicate_sum("src/res/day_1_1.txt").unwrap());
    }
}

fn main() {
    let sum = find_duplicate_sum("src/res/day_1_1.txt").unwrap();
    println!("The first duplicated sum is {}", sum);
}
