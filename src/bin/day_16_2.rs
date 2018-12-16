extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::mem::transmute;

use Opcode::*;

#[allow(unused_assignments)]
#[allow(dead_code)]

type GenError = Box<std::error::Error>;

// 14: {12, 11},
// 6: {4, 7, 12, 11, 8, 2, 9, 1, 5, 3, 10, 0, 6},
// 7: {14, 12},
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
    Gtrr, // 7
    Eqir,
    Eqri, // 0
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

    // Maps from raw_op_code_in_instruction to Opcode.ordinals
    let mut possible_results: HashMap<u32, HashSet<u32>> = HashMap::new();
    for raw_op_code in 0..=15 {
        possible_results.insert(raw_op_code, (0..=15).collect());
    }

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
            let mut matches = HashSet::new();
            for raw_op_code in 0..=15 {
                instruction.op_code = Opcode::from(raw_op_code);

                let mut copy_of_before = before.clone();
                instruction.execute(&mut copy_of_before);
                if copy_of_before == after {
                    // println!("Match for {:?}", instruction.op_code);
                    matches.insert(raw_op_code);
                }
            }

            let prev = possible_results
                .get(&raw_op_code_in_instruction)
                .unwrap()
                .clone();
            possible_results.insert(
                raw_op_code_in_instruction,
                prev.intersection(&matches).cloned().collect(),
            );
        } else if part_1 {
            // println!("Instruction line {}", line);
            raw_op_code_in_instruction = line.split(" ").next().unwrap().trim().parse().unwrap();
            instruction = get_unknown_instruction(&line);
        // println!("Got instruction {:?}", instruction);
        } else {

        }
    }

    println!("possible_results {:?}", possible_results);

    let mut changes = true;
    let mut already_solved = HashSet::new();
    while changes {
        changes = false;
        let mut to_be_removed = HashSet::new();
        for (raw_op_code, set) in &possible_results {
            if set.len() == 1 && !already_solved.contains(&raw_op_code) {
                println!(
                    "{} {:?} ",
                    raw_op_code,
                    Opcode::from(*set.iter().next().unwrap())
                );
                already_solved.insert(raw_op_code);
                to_be_removed.insert(raw_op_code);
                changes = true;
            }
        }

        for set in possible_results.values_mut() {
            set.retain(|s| !to_be_removed.contains(s));
        }
    }

    Ok(0)
}

fn main() {
    println!("reg[0] {}", process_file("src/res/day_16.txt").unwrap()); // (547,
}
