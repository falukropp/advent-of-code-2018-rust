use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::vec::Vec;

type GenError = Box<std::error::Error>;

fn process_file(path: &str, generations: u32) -> Result<i32, GenError> {
    let f = File::open(path)?;
    let mut r = BufReader::new(f);

    let mut first_line = String::new();
    r.read_line(&mut first_line)?;
    let mut current_state: String = first_line["initial state: ".len()..].to_string();
    current_state.pop(); // newline

    let mut positive_patterns: HashSet<String> = HashSet::new();

    r.lines().map(|l| l.unwrap()).for_each(|line| {
        let rule_parts: Vec<&str> = line.split(" => ").collect();
        if rule_parts.len() == 2 {
            if rule_parts[1] == "#" {
                positive_patterns.insert(rule_parts[0].to_string());
            }
        }
    });

    println!("0 0 {}", current_state);

    let mut first_position = 0i32;

    let mut result = 0;

    for gen in 1..=generations {
        // Pad if neccessary
        if let Some(pos) = current_state.find('#') {
            if pos < 4 {
                first_position -= 4 - pos as i32;
                current_state = ".".repeat(4 - pos) + &current_state;
            }
        }

        let mut current_len = current_state.len();
        if let Some(pos) = current_state.rfind('#') {
            if pos >= current_len - 4 {
                current_state = current_state + &".".repeat(pos + 5 - current_len);
            }
        }

        current_len = current_state.len();
        let mut next_state = String::with_capacity(current_len);
        next_state.push('.');
        next_state.push('.');

        for i in 0..current_len - 4 {
            if positive_patterns.contains(&current_state[i..i + 5]) {
                next_state.push('#');
            } else {
                next_state.push('.');
            }
        }
        next_state.push('.');
        next_state.push('.');

        current_state = next_state;

        result = 0;
        for (i, c) in current_state.char_indices() {
            if c == '#' {
                result += i as i32 + first_position;
            }
        }

        // println!("{} {} {}", gen, first_position, current_state);
        println!("{} {} ", gen, result);
    }

    Ok(result)
}

fn main() {
    // let sum = process_file("src/res/day_12_ex.txt",20).unwrap(); // 325
    let sum = process_file("src/res/day_12.txt", 20).unwrap(); // 3059
    let sum = process_file("src/res/day_12.txt", 200).unwrap(); // 16376

    // Seems to just be adding 73 to each number from now on?
    // So (50000000000-200)*73+16376 = 3650000001776
    println!("Result {}", sum);
}
