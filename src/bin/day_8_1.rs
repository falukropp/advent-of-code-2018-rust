extern crate regex;

use std::fs::File;
use std::io::Read;
use std::vec::Vec;

type GenError = Box<std::error::Error>;

fn read_string_from_file(path: &str) -> Result<String, GenError> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn sum_of_metadata(nums: &[u32]) -> (u32, usize) {
    let children = nums[0];
    let meta_datas = nums[1] as usize;

    // println!(
    //     "children {} metadatas  {} nums {:?} ",
    //     children, meta_datas, nums
    // );

    let mut sum = 0;
    let mut pos = 2;

    for _ in 0..children {
        let (sum_of_child, size_of_child) = sum_of_metadata(&nums[pos..]);
        sum += sum_of_child;
        pos += size_of_child;
    }

    let end = pos + meta_datas;
    sum += nums[pos..end].iter().sum::<u32>();

    // println!("sum {} size  {} ", sum, end);
    (sum, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sum_of_metadata_from_file(path: &str) -> (u32, usize) {
        let string = read_string_from_file(path).unwrap(); // 25737

        let numbers: Vec<u32> = string
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        sum_of_metadata(&numbers)
    }

    #[test]
    fn test_day_8_1() {
        assert_eq!((138, 16), sum_of_metadata_from_file("src/res/day_8_ex.txt"));
        assert_eq!(
            (41760, 16175),
            sum_of_metadata_from_file("src/res/day_8.txt")
        );
    }
}

fn main() {
    // let string = read_string_from_file("src/res/day_8_ex.txt").unwrap(); // Sum = 138
    let string = read_string_from_file("src/res/day_8.txt").unwrap(); //

    let numbers: Vec<u32> = string
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Result {:?}", sum_of_metadata(&numbers).0);
}
