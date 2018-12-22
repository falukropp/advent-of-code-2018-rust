#![allow(unused_assignments)]
#![allow(dead_code)]

extern crate regex;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::mem::transmute;

use Opcode::*;

type GenError = Box<std::error::Error>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

// let y: MyEnum = unsafe { transmute(x as i8) };

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
            Gtir => if self.a > reg[self.b as usize] {
                1
            } else {
                0
            },

            Gtri => if reg[self.a as usize] > self.b {
                1
            } else {
                0
            },
            Gtrr => if reg[self.a as usize] > reg[self.b as usize] {
                1
            } else {
                0
            },
            Eqir => if self.a == reg[self.b as usize] {
                1
            } else {
                0
            },
            Eqri => if reg[self.a as usize] == self.b {
                1
            } else {
                0
            },
            Eqrr => if reg[self.a as usize] == reg[self.b as usize] {
                1
            } else {
                0
            },
        }
    }
}

fn get_vector(s: &str) -> Vec<u32> {
    let start = s.find('[').unwrap() + 1;
    let end = s.find(']').unwrap();
    s[start..end]
        .split(",")
        .map(|i| i.trim().parse().unwrap())
        .collect()
}

fn get_unknown_instruction(s: &str) -> Instruction {
    let raw_instruction_data: Vec<u32> = s.split(" ").map(|i| i.trim().parse().unwrap()).collect();
    Instruction {
        op_code: Addr,
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
    let mut raw_op_code_in_instruction = 0;

    let mut part_1 = true;

    let mut triple_matches = 0;

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
            let mut matches = 0;
            let mut last_match = None;
            for raw_op_code in 0..=15 {
                instruction.op_code = Opcode::from(raw_op_code);

                let mut copy_of_before = before.clone();
                instruction.execute(&mut copy_of_before);
                if copy_of_before == after {
                    // println!("Match for {:?}", instruction.op_code);
                    matches += 1;
                    last_match = Some(instruction.op_code);
                }
            }
            if matches == 1 {
                // println!("Single match {:?}", last_match.unwrap());
            } else if matches >= 3 {
                triple_matches += 1;
            }
        } else if part_1 {
            // println!("Instruction line {}", line);
            instruction = get_unknown_instruction(&line);
            // println!("Got instruction {:?}", instruction);
        }
    }

    Ok(triple_matches)
}

fn main() {
    println!(
        "triples_matches {:?}",
        process_file("src/res/day_16.txt").unwrap()
    ); // (547,
}
