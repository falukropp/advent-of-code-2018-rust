extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::vec::Vec;

type GenError = Box<std::error::Error>;

struct Star {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

fn process_file(path: &str) -> Result<u32, GenError> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    let re =
        Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();

    let mut stars: Vec<Star> = Vec::new();

    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX;
    let mut max_y = std::i32::MIN;

    for line_result in r.lines() {
        let line = line_result?;

        for cap in re.captures_iter(&line) {
            let x = *&cap[1].parse::<i32>().unwrap();
            let y = *&cap[2].parse::<i32>().unwrap();
            let dx = *&cap[3].parse::<i32>().unwrap();
            let dy = *&cap[4].parse::<i32>().unwrap();

            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);

            stars.push(Star { x, y, dx, dy });
        }
    }

    let mut area = std::i64::MAX;
    let mut last_area = std::i64::MAX;
    let mut seconds = 0;

    while area <= last_area {
        let mut min_x = std::i32::MAX;
        let mut max_x = std::i32::MIN;
        let mut min_y = std::i32::MAX;
        let mut max_y = std::i32::MIN;

        for star in &mut stars {
            star.x += star.dx;
            star.y += star.dy;

            min_x = min_x.min(star.x);
            max_x = max_x.max(star.x);
            min_y = min_y.min(star.y);
            max_y = max_y.max(star.y);
        }

        let width = max_x - min_x;
        let height = max_y - min_y;

        last_area = area;
        area = (width as i64) * (height as i64);

        if height <= 9 {
            let mut starfield = vec![vec![' '; (width + 1) as usize]; (height + 1) as usize];

            for star in &stars {
                starfield[(star.y - min_y) as usize][(star.x - min_x) as usize] = 'X';
            }
            for row in &starfield {
                println!("{:?}", row);
            }
        }
        seconds += 1;

        // println!(
        //     "{} {}-{} {}-{} w:{} h:{} A:{}",
        //     seconds, min_x, max_x, min_y, max_y, width, height, area
        // );
    }

    Ok(seconds - 1)
}

fn main() {
    // let sum = process_file("src/res/day_10_ex.txt").unwrap(); // "HI", 3 seconds

    let sum = process_file("src/res/day_10.txt").unwrap(); // "EJXNCCNX", 10612 seconds
    println!("Seconds {}", sum);
}
