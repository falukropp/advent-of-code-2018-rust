extern crate regex;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type GenError = Box<std::error::Error>;

type Playfield = Vec<Vec<char>>;

struct DrawInstruction {
    x: (usize, usize),
    y: (usize, usize),
}

fn process_file(path: &str) -> Result<(u32, u32), GenError> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    let mut draw_instructions = Vec::new();

    let mut min_x = std::usize::MAX;
    let mut max_x = std::usize::MIN;
    let mut min_y = std::usize::MAX;
    let mut max_y = std::usize::MIN;

    for line_result in r.lines() {
        let line = line_result?;

        let mut x = (0, 0);
        let mut y = (0, 0);

        let parts: Vec<&str> = line.split(", ").collect();
        for part in parts {
            let assignment: Vec<&str> = part.split("=").collect();
            let range: Vec<usize> = assignment[1]
                .split("..")
                .map(|i| i.trim().parse().unwrap())
                .collect();
            let pair = (range[0], if range.len() == 2 { range[1] } else { range[0] });
            if assignment[0] == "x" {
                x = pair;
                min_x = min_x.min(pair.0);
                max_x = max_x.max(pair.1);
            } else {
                y = pair;
                min_y = min_y.min(pair.0);
                max_y = max_y.max(pair.1);
            }
        }

        draw_instructions.push(DrawInstruction { x, y });
    }

    let mut world = vec![vec!['.'; max_x + 2]; max_y + 1];

    for ins in draw_instructions {
        for y in ins.y.0..=ins.y.1 {
            for x in ins.x.0..=ins.x.1 {
                world[y][x] = '#';
            }
        }
    }

    // dump_playfield(&world, min_x, max_x);

    fill_water(&mut world, 500, 0, min_x, max_x);

    println!("--------------------------------------");

    dump_playfield(&world, min_x, max_x);

    println!("({} {}) ({} {})", min_x, max_x, min_y, max_y);

    let mut water_tiles = 0;
    let mut still_tiles = 0;

    for y in min_y..=max_y {
        for x in min_x - 1..=max_x + 1 {
            if world[y][x] == '|' {
                water_tiles += 1;
            } else if world[y][x] == '~' {
                water_tiles += 1;
                still_tiles += 1;
            }
        }
    }

    Ok((water_tiles, still_tiles))
}

// Returns tue if overlflowing, false if draining
fn fill_water(
    world: &mut Playfield,
    start_x: usize,
    start_y: usize,
    min_x: usize,
    max_x: usize,
) -> bool {
    // Drop down while no floor.
    let mut x = start_x;
    let mut y = start_y;
    let world_depth = world.len();

    while y < world_depth && world[y][x] == '.' {
        world[y][x] = '|';
        y += 1;
    }
    if y >= world_depth {
        return false;
    }
    if world[y][x] == '|' {
        // println!("Found running water at {} {} ", x, y);
        return false;
    }
    // if world[y][x] != '#' {
    //     println!("Found unexpected tile '{}' at {} {}", world[y][x], x, y);
    //     dump_playfield(world, min_x, max_x);
    //     panic!("This should never happen!!!!");
    // }

    // println!("Start expanding row at {} {}", x, y);

    //
    let mut both_blocked = true;

    while y > start_y && both_blocked {
        let mut blocked_left = None;
        let mut blocked_right = None;

        // Fill to the left.
        x = start_x;

        loop {
            while world[y - 1][x - 1] == '.' && world[y][x] != '.' {
                x -= 1;
                world[y - 1][x] = '|';
            }

            if world[y][x] == '.' {
                if !fill_water(world, x, y, min_x, max_x) {
                    break;
                }
                world[y - 1][x] = '|';
            }

            // No drain found yet, and wall encountered.
            if world[y - 1][x - 1] == '#' {
                blocked_left = Some(x);
                break;
            }
        }

        // dump_playfield(world, min_x, max_x);

        x = start_x;

        // Fill to the right.
        loop {
            while world[y - 1][x + 1] == '.' && world[y][x] != '.' {
                // println!("1. {} {} {} ", x, y, world[y][x]);
                x += 1;
                world[y - 1][x] = '|';
            }

            if world[y][x] == '.' {
                // println!("2. {} {} {} ", x, y, world[y][x]);
                if !fill_water(world, x, y, min_x, max_x) {
                    break;
                }
                world[y - 1][x] = '|';
            }

            // No drain found yet, and wall encountered.

            // println!("3. {} {} {} ", x, y, world[y - 1][x + 1]);

            if world[y - 1][x + 1] == '#' {
                blocked_right = Some(x);
                break;
            }
            // println!("4. {} {} {} ", x, y, world[y - 1][x + 1]);
        }
        // println!("5. {} {} {} ", x, y, world[y - 1][x + 1]);

        // Both blocked? Fill with stillwater and backup one step.
        both_blocked = false;
        if let Some(min_x) = blocked_left {
            if let Some(max_x) = blocked_right {
                both_blocked = true;
                // println!(" {} {} ", min_x, max_x);
                for col in min_x..=max_x {
                    world[y - 1][col] = '~';
                }
                y -= 1;
            }
        }
        if !both_blocked {
            return false;
        }
        // dump_playfield(world, min_x, max_x);
        // println!(
        //     "both_blocked {} {:?} {:?} ",
        //     both_blocked, blocked_left, blocked_right
        // );
    }

    // println!("6. {} {} {} ", x, y, world[y - 1][x + 1]);
    // Overflowing
    true
}

fn dump_playfield(world: &Playfield, min_x: usize, max_x: usize) {
    println!("");
    for row in world {
        for col in min_x - 1..=max_x + 1 {
            print!("{}", row[col]);
        }
        println!("");
    }
    println!("");
}

fn main() {
    // println!(
    //     "water_tiles {:?}",
    //     process_file("src/res/day_17_ex.txt").unwrap() // (57,29)
    // );

    println!(
        "water_tiles {:?}",
        process_file("src/res/day_17.txt").unwrap() //  (40879, 34693)
    );
}
