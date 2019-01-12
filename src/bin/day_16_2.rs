#![allow(unused_assignments)]
#![allow(dead_code)]

extern crate regex;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::mem::transmute;

use Opcode::*;

#[allow(unused_assignments)]
#[allow(dead_code)]

type GenError = Box<std::error::Error>;

// Original
// 14: {12, 11},
// 6: {4, 7, 12, 11, 8, 2, 9, 1, 5, 3, 10, 0, 6},
// 7: {12},
// 11: {2, 4, 6, 5, 15, 13, 9, 8}, 13: {13, 15},
// 5: {5, 4, 11, 13, 15, 10, 14, 12},
// 9: {9, 1},
// 10: {14, 13, 11, 12, 15, 10},
// 15: {14, 13, 12, 11},
// 2: {2, 8, 7, 11, 4, 15, 6, 1, 13, 9},
//  0: {14},
//  1: {15, 11, 4, 10, 13, 12, 14},
//  3: {2, 5, 9, 11, 4, 15, 14, 10},
//  8: {8, 10},
//  4: {15, 9, 12, 14, 10, 11, 13, 5, 4},
//  12: {0, 1, 9}

// After elimination
// 14: {11},
// 6: {3},
// 7: {12},
// 11: {6},
// 13: {15},
// 5: {5},
// 9: {1},
// 10: {10},
// 15: {13},
//  2: {7, 6},
//  0: {14},
//  1: {4},
//  3: {2},
//  8: {8},
//  4: {9},
//  12: {0}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Opcode {
    Eqri, // 0
    Banr, // 1
    Bori, // 2
    Mulr, // 3
    Seti, // 4
    Bani, // 5
    Muli, // 6
    Gtrr, // 7
    Setr, // 8
    Addi, // 9
    Gtir, // 10
    Borr, // 11
    Addr, // 12
    Eqrr, // 13
    Gtri, // 14
    Eqir, // 15
}

impl From<u32> for Opcode {
    fn from(x: u32) -> Self {
        unsafe { transmute(x as i8) }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Instruction {
    op_code: Opcode,
    a: u32,
    b: u32,
    c: u32,
}

impl Instruction {
    fn execute(&self, reg: &mut Vec<u32>) {
        reg[self.c as usize] = match self.op_code {
            Addr => reg[self.a as usize] + reg[self.b as usize],
            Addi => reg[self.a as usize] + self.b,
            Mulr => reg[self.a as usize] * reg[self.b as usize],
            Muli => reg[self.a as usize] * self.b,
            Banr => reg[self.a as usize] & reg[self.b as usize],
            Bani => reg[self.a as usize] & self.b,
            Borr => reg[self.a as usize] | reg[self.b as usize],
            Bori => reg[self.a as usize] | self.b,
            Setr => reg[self.a as usize],
            Seti => self.a,
            Gtir => {
                if self.a > reg[self.b as usize] {
                    1
                } else {
                    0
                }
            }

            Gtri => {
                if reg[self.a as usize] > self.b {
                    1
                } else {
                    0
                }
            }
            Gtrr => {
                if reg[self.a as usize] > reg[self.b as usize] {
                    1
                } else {
                    0
                }
            }
            Eqir => {
                if self.a == reg[self.b as usize] {
                    1
                } else {
                    0
                }
            }
            Eqri => {
                if reg[self.a as usize] == self.b {
                    1
                } else {
                    0
                }
            }
            Eqrr => {
                if reg[self.a as usize] == reg[self.b as usize] {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn get_vector(s: &str) -> Vec<u32> {
    let start = s.find('[').unwrap() + 1;
    let end = s.find(']').unwrap();
    s[start..end]
        .split(',')
        .map(|i| i.trim().parse().unwrap())
        .collect()
}

fn get_instruction(s: &str) -> Instruction {
    let raw_instruction_data: Vec<u32> = s.split(' ').map(|i| i.trim().parse().unwrap()).collect();
    Instruction {
        op_code: Opcode::from(raw_instruction_data[0]),
        a: raw_instruction_data[1],
        b: raw_instruction_data[2],
        c: raw_instruction_data[3],
    }
}

fn process_file(path: &str) -> Result<u32, GenError> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    let mut before: Vec<u32> = Vec::new();
    let mut instruction: Instruction = Instruction {
        op_code: Addi,
        a: 0,
        b: 0,
        c: 0,
    };
    let mut after: Vec<u32> = Vec::new();
    let mut empty_lines_in_row = 0;

    let mut part_1 = true;

    let mut registers_part2 = vec![0; 4];

    for line_result in r.lines() {
        let line = line_result?;

        // println!("Got line {} ", line);
        if line.is_empty() {
            empty_lines_in_row += 1;
            if empty_lines_in_row == 3 {
                part_1 = false;
            }
            continue;
        }
        empty_lines_in_row = 0;
        if line.starts_with("Before: ") {
            before = get_vector(&line);
        // println!("Got before {:?}", before);
        } else if line.starts_with("After: ") {
            after = get_vector(&line);
            // println!("Got after {:?}", after);
            let mut copy_of_before = before.clone();
            instruction.execute(&mut copy_of_before);
            if copy_of_before != after {
                println!("-------------------------------------------------");
                println!("Oh noes! No match for ");
                println!("Before {:?} ", before);
                println!("Instruction {:?}", instruction);
                println!("After {:?}", after);
            } else {
                // println!("Yay!");
            }
        } else if part_1 {
            instruction = get_instruction(&line);
        // println!("Got instruction {:?}", instruction);
        } else {
            get_instruction(&line).execute(&mut registers_part2);
        }
    }

    Ok(registers_part2[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_16_2() {
        assert_eq!(582, process_file("src/res/day_16.txt").unwrap());
    }
}

fn main() {
    println!("reg[0] {}", process_file("src/res/day_16.txt").unwrap()); // 582
}
