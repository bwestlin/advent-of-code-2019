extern crate utils;

use std::env;
use std::ops::{Index,IndexMut};
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<i32>;

struct Computer {
    memory: Vec<i32>
}

impl Computer {
    fn new(program: &Vec<i32>) -> Computer {
        Computer { memory: program.clone() }
    }

    fn run(&mut self) {
        let mut pc = 0;

        while self.memory[pc] != 99 {
            let opcode = self.memory[pc];
            let pos1   = self.memory[pc + 1] as usize;
            let pos2   = self.memory[pc + 2] as usize;
            let target = self.memory[pc + 3] as usize;

            match opcode {
                1 => {
                    self.memory[target] = self.memory[pos1] + self.memory[pos2];
                },
                2 => {
                    self.memory[target] = self.memory[pos1] * self.memory[pos2];
                },
                _ => {
                    unreachable!()
                }
            }
            pc += 4;
        }
    }
}

impl Index<usize> for Computer {
    type Output = i32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.memory[index]
    }
}

impl IndexMut<usize> for Computer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.memory[index]
    }
}

fn run_with_values(input: &Input, noun: i32, verb: i32) -> i32 {
    let mut computer = Computer::new(input);
    computer[1] = noun;
    computer[2] = verb;
    computer.run();
    computer[0]
}

fn part1(input: &Input) -> i32 {
    run_with_values(input, 12, 2)
}

fn part2(input: &Input) -> i32 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let result = run_with_values(input, noun, verb);
            if result == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().next().unwrap()?.split(',').map(|i| i.parse::<i32>().unwrap()).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_computer() {
        let mut computer = Computer::new(&as_input("1,9,10,3,2,3,11,0,99,30,40,50"));
        computer.run();
        assert_eq!(computer[0], 3500);
        assert_eq!(computer[3], 70);

        let mut computer = Computer::new(&as_input("1,0,0,0,99"));
        computer.run();
        assert_eq!(computer[0], 2);

        let mut computer = Computer::new(&as_input("2,3,0,3,99"));
        computer.run();
        assert_eq!(computer[3], 6);

        let mut computer = Computer::new(&as_input("2,4,4,5,99,0"));
        computer.run();
        assert_eq!(computer[5], 9801);

        let mut computer = Computer::new(&as_input("1,1,1,4,99,5,6,0,99"));
        computer.run();
        assert_eq!(computer[0], 30);
    }
}
