extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<i32>;

fn fuel_required(mass: &i32) -> i32 {
    (mass / 3) - 2
}

fn part1(input: &Input) -> i32 {
    input.iter()
        .map(fuel_required)
        .sum::<i32>()
}

fn part2(input: &Input) -> i32 {
    input.iter()
        .map(|mass| {
            let mut fuel_req = fuel_required(&mass);
            let mut fuel_req_sum = 0;

            while fuel_req > 0 {
                fuel_req_sum += fuel_req;
                fuel_req = fuel_required(&fuel_req);
            }

            fuel_req_sum
        })
        .sum::<i32>()
    }

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap()).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "12
        14
        1969
        100756";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 2 + 2 + 654 + 33583);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 2 + 2 + 966 + 50346);
    }
}
