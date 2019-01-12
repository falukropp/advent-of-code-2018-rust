extern crate regex;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::mem::swap;

type GenError = Box<std::error::Error>;

type Playfield = Vec<Vec<char>>;

fn process_file(path: &str, max_minutes: u32) -> Result<u32, GenError> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    let mut playfield: Playfield = Vec::new();
    let mut cols = 0;

    for line_result in r.lines() {
        let line = line_result?;

        if cols == 0 {
            // Add empty row at start
            cols = line.len() + 2;
            playfield.push(" ".repeat(cols).chars().collect());
        }

        let mut row = Vec::new();

        row.push(' ');
        for c in line.chars() {
            row.push(c);
        }
        row.push(' ');

        playfield.push(row);
    }
    // Add empty row at end
    playfield.push(" ".repeat(cols).chars().collect());

    let rows = playfield.len();

    let mut work_buffer: Playfield = vec![vec![' '; cols]; rows];

    let current_playfield = &mut (&mut playfield);
    let next_playfield = &mut (&mut work_buffer);

    dump_playfield(&current_playfield);

    println!("cols {} rows {}", cols, rows);

    let mut minute = 1;
    while minute <= max_minutes {
        let mut any_changes = false;

        for y in 1..=rows - 2 {
            for x in 1..=cols - 2 {
                let (_open, trees, lumberyards) = count_neigbours(current_playfield, x, y);
                next_playfield[y][x] = match current_playfield[y][x] {
                    '.' => {
                        if trees >= 3 {
                            any_changes = true;
                            '|'
                        } else {
                            '.'
                        }
                    }
                    '|' => {
                        if lumberyards >= 3 {
                            any_changes = true;
                            '#'
                        } else {
                            '|'
                        }
                    }
                    '#' => {
                        if trees == 0 || lumberyards == 0 {
                            any_changes = true;
                            '.'
                        } else {
                            '#'
                        }
                    }
                    _ => current_playfield[y][x],
                };
                // println!("open {}, trees {}, lumberyards {}", open, trees, lumberyards);
                // println!("{} {} current_playfield {} next_playfield {}", x, y, current_playfield[y][x], next_playfield[y][x]);
            }
        }

        swap(current_playfield, next_playfield);

        println!(
            "After {} minutes value is {} ",
            minute,
            find_value(&current_playfield, rows, cols)
        );

        // dump_playfield(&current_playfield);

        if !any_changes {
            break;
        }

        minute += 1
    }

    println!("Done after {} minutes ", minute - 1);
    dump_playfield(&current_playfield);

    Ok(find_value(&current_playfield, rows, cols))
}

fn find_value(playfield: &Playfield, rows: usize, cols: usize) -> u32 {
    let mut wooded_tiles = 0;
    let mut lumberyard_tiles = 0;

    for y in 1..=rows - 2 {
        for x in 1..=cols - 2 {
            match playfield[y][x] {
                '|' => wooded_tiles += 1,
                '#' => lumberyard_tiles += 1,
                _ => {}
            }
        }
    }
    // println!("Wood {} lumberyards {}", wooded_tiles, lumberyard_tiles);

    wooded_tiles * lumberyard_tiles
}

fn count_neigbours(playfield: &Playfield, center_x: usize, center_y: usize) -> (u32, u32, u32) {
    let mut open = 0;
    let mut trees = 0;
    let mut lumberyards = 0;

    for y in center_y - 1..=center_y + 1 {
        for x in center_x - 1..=center_x + 1 {
            if y == center_y && x == center_x {
                continue;
            }
            match playfield[y][x] {
                '.' => open += 1,
                '|' => trees += 1,
                '#' => lumberyards += 1,
                _ => {}
            }
        }
    }

    (open, trees, lumberyards)
}

fn dump_playfield(world: &Playfield) {
    println!();
    for row in world {
        for tile in row {
            print!("{}", tile);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_18() {
        assert_eq!(1147, process_file("src/res/day_18_ex.txt", 10).unwrap());
        assert_eq!(355918, process_file("src/res/day_18.txt", 10).unwrap());
    }
}

fn main() {
    println!(
        "Value {}",
        process_file("src/res/day_18_ex.txt", 10).unwrap() // 1147
    );

    println!(
        "Value {}",
        process_file("src/res/day_18.txt", 10).unwrap() // 355918
    );

    // This will not complete, but loops
    // println!(
    //     "Value {}",
    //     process_file("src/res/day_18.txt",1_000_000_000).unwrap()
    // );

    // Loops

    // After 506 minutes value is 209788
    // After 507 minutes value is 207640 <-- Loop starts,  should be the same as for minute 999999983 too
    // After 508 minutes value is 201123
    // After 509 minutes value is 196768
    // After 510 minutes value is 192552
    // After 511 minutes value is 191416
    // After 512 minutes value is 188244
    // After 513 minutes value is 188670
    // After 514 minutes value is 187530
    // After 515 minutes value is 189663
    // After 516 minutes value is 189904
    // After 517 minutes value is 192625
    // After 518 minutes value is 193101
    // After 519 minutes value is 193536
    // After 520 minutes value is 194688
    // After 521 minutes value is 195797
    // After 522 minutes value is 198068
    // After 523 minutes value is 200349
    // After 524 minutes value is 202806 <<- This should be the same as 1_000_000_000
    // After 525 minutes value is 204600
    // After 526 minutes value is 208292
    // After 527 minutes value is 211050
    // After 528 minutes value is 214524
    // After 529 minutes value is 214977
    // After 530 minutes value is 214524
    // After 531 minutes value is 215130
    // After 532 minutes value is 214557
    // After 533 minutes value is 214560
    // After 534 minutes value is 210960
    // After 535 minutes value is 207640 <-- First loop with length 28
    // After 536 minutes value is 201123
    // After 537 minutes value is 196768

    // (1_000_000_000 - 535) / 28 = 35714266 loops (after 535)
    // 535 + 28 * 35714266 = 999999983
}
