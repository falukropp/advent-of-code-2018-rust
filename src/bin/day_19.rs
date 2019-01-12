extern crate regex;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use Opcode::*;

#[allow(unused_assignments)]
#[allow(dead_code)]

type GenError = Box<std::error::Error>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Opcode {
    Eqri = 0,
    Banr = 1,
    Bori = 2,
    Mulr = 3,
    Seti = 4,
    Bani = 5,
    Muli = 6,
    Gtrr = 7,
    Setr = 8,
    Addi = 9,
    Gtir = 10,
    Borr = 11,
    Addr = 12,
    Eqrr = 13,
    Gtri = 14,
    Eqir = 15,
}

#[derive(Debug)]
enum Error {
    Whatever,
}

impl FromStr for Opcode {
    type Err = Error;

    fn from_str(opcode: &str) -> Result<Self, Self::Err> {
        match opcode {
            "addi" => Ok(Addi),
            "addr" => Ok(Addr),
            "mulr" => Ok(Mulr),
            "muli" => Ok(Muli),
            "banr" => Ok(Banr),
            "bani" => Ok(Bani),
            "borr" => Ok(Borr),
            "bori" => Ok(Bori),
            "setr" => Ok(Setr),
            "seti" => Ok(Seti),
            "gtir" => Ok(Gtir),
            "gtri" => Ok(Gtri),
            "gtrr" => Ok(Gtrr),
            "eqir" => Ok(Eqir),
            "eqri" => Ok(Eqri),
            "eqrr" => Ok(Eqrr),

            _ => Err(Error::Whatever),
        }
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

fn get_instruction(s: &str) -> Instruction {
    let raw_instruction_data: Vec<&str> = s.split(' ').collect();
    Instruction {
        op_code: Opcode::from_str(raw_instruction_data[0]).unwrap(),
        a: raw_instruction_data[1].trim().parse().unwrap(),
        b: raw_instruction_data[2].trim().parse().unwrap(),
        c: raw_instruction_data[3].trim().parse().unwrap(),
    }
}

fn process_file(path: &str, reg0_initialvalue: u32) -> Result<u32, GenError> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    let mut program: Vec<Instruction> = Vec::new();

    let mut registers: Vec<u32> = vec![0; 6];
    registers[0] = reg0_initialvalue;

    let mut ip_reg = 0;

    for line_result in r.lines() {
        let line = line_result?;

        // println!("{}", line);
        if line.starts_with("#ip ") {
            // Assume that this instruction can only occur once, and that it's globally applicable.
            ip_reg = line["#ip ".len()..].parse()?;
        } else {
            program.push(get_instruction(&line));
        }
    }

    // Run program
    let len = program.len();

    while (registers[ip_reg] as usize) < len {
        program[registers[ip_reg] as usize].execute(&mut registers);
        registers[ip_reg] += 1;

        // println!("registers {:?}", registers)
    }

    Ok(registers[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_19() {
        assert_eq!(1922, process_file("src/res/day_19.txt", 0).unwrap());
    }
}

fn main() {
    println!("reg[0] {}", process_file("src/res/day_19.txt", 0).unwrap()); // 1922
                                                                           // Sum of factors of 976 == 1922

    // println!("reg[0] {}", process_file("src/res/day_19.txt", 1).unwrap()); // Trololololo. See day_19_reverse_engineered for what happens.
    // Sum of factors of 10551376 == 22302144
}
