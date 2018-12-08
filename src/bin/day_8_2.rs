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

    println!("children {} metadatas  {}", children, meta_datas);

    let mut sum = 0;
    let mut pos = 2;

    let mut child_value = Vec::new();

    for _ in 0..children {
        let (sum_of_child, size_of_child) = sum_of_metadata(&nums[pos..]);
        child_value.push(sum_of_child);
        pos += size_of_child;
    }

    let end = pos + meta_datas;
    if children == 0 {
        sum = nums[pos..end].iter().sum::<u32>();
    } else {
        for idx in pos..end {
            let child_index = nums[idx] as usize;
            if child_index <= child_value.len() {
                sum += child_value[child_index - 1];
            }
        }
    }
    // println!(
    //     "child_value {:?} meta_datas  {:?} sum {} ",
    //     child_value,
    //     &nums[pos..end],
    //     sum
    // );

    // println!("sum {} size  {} ", sum, end);
    (sum, end)
}

fn main() {
    // let string = read_string_from_file("src/res/day_8_ex.txt").unwrap(); // Sum = 66

    let string = read_string_from_file("src/res/day_8.txt").unwrap(); // 25737

    let numbers: Vec<u32> = string
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Result {:?}", sum_of_metadata(&numbers).0);
}
