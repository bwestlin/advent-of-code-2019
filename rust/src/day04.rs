extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = (usize, usize);
type Password = [u8; 6];

fn valid_password(password: &Password) -> [bool; 2] {
    let mut cnts = [0; 10];
    let mut prev = password[0];
    for &c in password {
        if c < prev {
            return [false; 2];
        }
        cnts[c as usize] += 1;
        prev = c;
    }
    [cnts.iter().any(|&c| c >= 2), cnts.iter().any(|&c| c == 2)]
}

fn as_password(s: &str) -> Password {
    let s: Vec<_> = s.as_bytes().iter().map(|c| c - '0' as u8).collect();
    let mut password = [0; 6];
    password.copy_from_slice(&s[..6]);
    password
}

fn inc_password(password: &mut Password) {
    let len = password.len();
    for i in 0..len {
        let idx = len - i - 1;
        if password[idx] < 9 {
            password[idx] += 1;
            break;
        }
        password[idx] = 0;
    }
}

fn solve(input: &Input) -> [i32; 2] {
    let (start, end) = input;
    let n_iter = end - start;
    let start = as_password(&(format!("{}", start)[..]));

    let (_, counts) = (0..=n_iter)
        .fold((start, [0, 0]), |(mut password, mut counts), _| {
            let valid = valid_password(&password);
            for i in 0..2 {
                counts[i] += if valid[i] { 1 } else { 0 }
            }
            inc_password(&mut password);
            (password, counts)
        });

    counts
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let [part1, part2] = solve(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    fn parse(s: &String) -> (usize, usize) {
        let mut split = s.split('-');
        (split.next().unwrap().parse::<usize>().unwrap(), split.next().unwrap().parse::<usize>().unwrap())
    }
    Ok(reader.lines().next().map(|l| parse(&l.unwrap())).unwrap())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password1() {
        assert_eq!(valid_password(&as_password("111111"))[0], true);
        assert_eq!(valid_password(&as_password("223450"))[0], false);
        assert_eq!(valid_password(&as_password("123789"))[0], false);
    }

    #[test]
    fn test_valid_password2() {
        assert_eq!(valid_password(&as_password("112233"))[1], true);
        assert_eq!(valid_password(&as_password("123444"))[1], false);
        assert_eq!(valid_password(&as_password("111122"))[1], true);
    }
}
