use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type GenError = Box<std::error::Error>;

fn next_matching(run_slice: &str, match_c: char) -> Option<usize> {
    let mut level = 0;

    println!("Looking for {} in {}", match_c, run_slice);

    for (i, c) in run_slice.char_indices() {
        match c {
            m if m == match_c && level == 0 => {
                return Some(i);
            }
            '(' => level += 1,
            ')' => level -= 1,
            _ => {}
        }
    }
    None
}

fn parse_run(run_slice: &str) {
    let mut skip_to_idx = 0;

    println!("slice {}", run_slice);

    for (i, c) in run_slice.char_indices() {
        println!("{} {}", i, c);

        if i < skip_to_idx {
            println!("skipping");
            continue;
        }

        match c {
            'E' => print!("E"),
            'N' => print!("N"),
            'S' => print!("S"),
            'W' => print!("W"),
            '(' => {
                let mut idx = i + 1;
                parse_run(&run_slice[idx..]);
                while let Some(next_idx) = next_matching(&run_slice[idx..], '|') {
                    idx = next_idx + i + 2;
                    println!(" next run starts at {}", idx);
                    parse_run(&run_slice[idx..]);
                }
                println!(" Done! Returning!");
                return;
            }
            '|' => {
                skip_to_idx = next_matching(&run_slice[i..], ')').unwrap() + i + 1; // Find ) and continue there.
                println!(" skip to idx {}", skip_to_idx);
            }
            '$' => {
                println!("");
            }
            '^' => {}
            _ => panic!("What the hell!? {}", c),
        }
    }
}

fn process_file(path: &str) -> Result<u32, GenError> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    for line_result in r.lines() {
        let line = line_result?;
        parse_run(&line);
    }
    Ok(0)
}

fn main() {
    println!(
        "longest_distance {}",
        process_file("src/res/day_20_ex_1.txt").unwrap()
    );

    // println!("{:?}", next_matching("a(b)c)dd", ')'));
    // println!("longest_distance", process_file("src/res/day_20_ex_1.txt", 0).unwrap());
    // assert_eq!(23, process_file("src/res/day_20_ex_3.txt", 0).unwrap()));
    // assert_eq!(31, process_file("src/res/day_20_ex_4.txt", 0).unwrap()));
}
