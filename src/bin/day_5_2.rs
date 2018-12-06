extern crate regex;

use std::fs::File;
use std::io::Read;

type GenError = Box<std::error::Error>;

fn improve_polymer(str : &str)  -> usize {
    (b'A' ..= b'z')
    .map(|c| c as char)
    .map(|c| {
        let mut s = str.to_string();
        s = s.replace(c, "");
        s = s.replace(c.to_ascii_uppercase(), "");
        react_polymer(&s).unwrap()
    }).min().unwrap()
}

fn react_polymer(str : &str) -> Result<usize, GenError> {
    let mut s = str.to_string();
    let mut previous_result_length = 0;
    let mut result_length = s.len();

    // Insanely inefficient, though.
    while previous_result_length != result_length {
        let mut result = String::new();

        let mut last_char : Option<char> = None;
        for cc in s.chars() {

            if last_char.is_none() {
                last_char = Some(cc);
                continue;
            }
            let lc = last_char.unwrap();
            if !(cc.is_uppercase() && lc == cc.to_ascii_lowercase()) && !(cc.is_lowercase() && lc == cc.to_ascii_uppercase()) {
                result.push(lc);
                last_char = Some(cc);
            } else {
                last_char = None;
            }
        }
        last_char.map(|c| if c != '\n' {result.push(c)});

        previous_result_length =  result_length;
        result_length = result.len();

        // println!("{} ", &result);

        s = result;

        // println!("previous_result_length {}, new length {} ", previous_result_length, result_length);
    }

    // println!("`{}` ", &s);
    return Ok(result_length);
}

fn process_file(path: &str) -> Result<usize, GenError> {
    let mut f = File::open(path)?;
    let mut s = String::new();
	f.read_to_string(&mut s)?;


    return Ok(improve_polymer(&s));
}

fn main() {
    println!("Checksum {}", improve_polymer("dabAcCaCBAcCcaDA"));
    let sum = process_file("src/res/day_5.txt").unwrap();
    println!("Checksum {}", sum);
}
