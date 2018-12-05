extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type GenError = Box<std::error::Error>;

fn process_file(path: &str) -> Result<u32, GenError> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    let mut lines: Vec<String> = Vec::new();

    for line_result in r.lines() {
        let line = line_result?;
        lines.push(line);
    }
    lines.sort();

    // println!("{:?}", lines);

    let mut minutes_by_guard: HashMap<u32, Vec<u32>> = HashMap::new();

    let guard_re = Regex::new(r"Guard #(\d*)")?;
    let asleep_re = Regex::new(r"(\d{2})\] falls asleep")?;
    let wake_re = Regex::new(r"(\d{2})\] wakes up")?;

    let mut current_guard = 0;
    let mut start_minute: usize = 0;

    lines.iter().for_each(|l| {
        guard_re.captures(l).map(|cap| {
            current_guard = cap[1].parse::<u32>().unwrap();
        });
        asleep_re.captures(l).map(|cap| {
            start_minute = cap[1].parse::<usize>().unwrap();
        });
        wake_re.captures(l).map(|cap| {
            let end_minute = cap[1].parse::<usize>().unwrap();

            let minutes = minutes_by_guard.entry(current_guard).or_insert(vec![0; 60]);
            for min in start_minute..end_minute {
                minutes[min] += 1;
            }
        });
    });

    let mut max_guard = 0;
    let mut max_minutes_for_any_guard = 0;
    let mut max_guard_minute_idx = 0;

    for (guard, minutes) in minutes_by_guard.iter() {
        let (max_minute_idx, max_minutes_for_this_guard) = minutes
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item.clone())
            .unwrap();

        if max_minutes_for_this_guard > &max_minutes_for_any_guard {
            max_guard = *guard;
            max_guard_minute_idx = max_minute_idx;
            max_minutes_for_any_guard = *max_minutes_for_this_guard;
        }
    }

    return Ok(max_guard * (max_guard_minute_idx as u32));
}

fn main() {
    let sum = process_file("src/res/day_4.txt").unwrap();
    println!("Checksum {}", sum);
}
