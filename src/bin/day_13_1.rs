extern crate regex;

use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type GenError = Box<std::error::Error>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next_turn(t: &Turn) -> Turn {
        match t {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        }
    }

    fn get_deltas(dir: &Direction) -> (i32, i32) {
        match dir {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, -0),
        }
    }
    fn turn_left(dir: &Direction) -> Direction {
        match dir {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
    fn turn_right(dir: &Direction) -> Direction {
        match dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Minecart {
    y: i32,
    x: i32,
    heading: Direction,
    next_turn: Turn,
}

fn process_file(path: &str) -> Result<(i32, i32), GenError> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    let mut row = 0;
    let mut tracks = Vec::new();
    let mut minecarts = Vec::new();
    r.lines().map(|l| l.unwrap()).for_each(|line| {
        line.match_indices(|c| c == 'v' || c == '<' || c == '>' || c == '^')
            .for_each(|(idx, s)| {
                let c = s.chars().next().unwrap();
                minecarts.push(Minecart {
                    y: row,
                    x: idx as i32,
                    heading: Direction::from_char(c).unwrap(),
                    next_turn: Turn::Left,
                })
            });
        row += 1;
        let mut cleaned_line = Vec::with_capacity(line.len());
        for c in line.chars() {
            cleaned_line.push(match c {
                '>' | '<' => '-',
                '^' | 'v' => '|',
                _ => c,
            });
        }
        tracks.push(cleaned_line);
    });
    // for row in tracks {
    //     println!("{}", row);
    // }
    // println!("{:?}", minecarts);

    let mut positions = HashSet::new();
    for minecart in &minecarts {
        positions.insert((minecart.x, minecart.y));
    }

    loop {
        minecarts.sort();

        for idx in 0..minecarts.len() {
            let ref mut minecart = minecarts[idx];
            let delta = Direction::get_deltas(&minecart.heading);

            positions.remove(&(minecart.x, minecart.y));

            minecart.x += delta.0;
            minecart.y += delta.1;

            if !positions.insert((minecart.x, minecart.y)) {
                // Collision
                return Ok((minecart.x, minecart.y));
            }

            minecart.heading = match tracks[minecart.y as usize][minecart.x as usize] {
                '\\' => match minecart.heading {
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                },
                '/' => match minecart.heading {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                },
                '+' => {
                    let current_turn = minecart.next_turn;
                    minecart.next_turn = Turn::next_turn(&current_turn);
                    match current_turn {
                        Turn::Left => Direction::turn_left(&minecart.heading),
                        Turn::Right => Direction::turn_right(&minecart.heading),
                        Turn::Straight => minecart.heading,
                    }
                }
                _ => minecart.heading,
            }
        }
    }
}

fn main() {
    // let collision_at = process_file("src/res/day_13_ex.txt").unwrap(); // (7,3)
    let collision_at = process_file("src/res/day_13.txt").unwrap(); // (43,91)
    println!("Collission at {:?}", collision_at);
}
