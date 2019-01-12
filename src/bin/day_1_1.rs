use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn sum_file(path: &str) -> Result<i64, std::io::Error> {
    let f = File::open(path)?;
    let r = BufReader::new(f);
    let mut sum = 0i64;
    for line_result in r.lines() {
        let line = line_result?;
        sum += line.parse::<i64>().unwrap();
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_1() {
        assert_eq!(433, sum_file("src/res/day_1_1.txt").unwrap());
    }
}

fn main() {
    let sum = sum_file("src/res/day_1_1.txt").unwrap();
    println!("The sum is {}", sum);
}
