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

    let (sleepiest_guard, _) = minutes_by_guard
        .iter()
        .map(|(k, v)| (k, v.iter().sum::<u32>()))
        .max_by_key(|(_k, v)| *v)
        .unwrap();

    println!("Sleepiest guard {}", sleepiest_guard);

    let sleepiest_guard_minutes = minutes_by_guard.get(sleepiest_guard).unwrap();

    println!("{:?}", sleepiest_guard_minutes);

    let (max_minutes, _minutes) = minutes_by_guard
        .get(sleepiest_guard)
        .unwrap()
        .iter()
        .enumerate()
        .max_by_key(|&(_, item)| *item)
        .unwrap();

    return Ok(sleepiest_guard * (max_minutes as u32));
}

fn main() {
    let sum = process_file("src/res/day_4.txt").unwrap(); // 14346
    println!("Checksum {}", sum);
}
