extern crate utils;

use std::env;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = (Vec<Movement>, Vec<Movement>);

enum Direction {
    North, East, South, West
}

struct Movement {
    direction: Direction,
    steps: i32
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn origo() -> Position {
        Position { x: 0, y: 0 }
    }
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn moved(direction: &Direction, p: &Position) -> Position {
    use Direction::*;
    match direction {
        North => Position { x: p.x, y: p.y + 1 },
        South => Position { x: p.x, y: p.y - 1 },
        East  => Position { x: p.x + 1, y: p.y },
        West  => Position { x: p.x - 1, y: p.y },
    }
}

#[derive(Clone, Copy, Debug)]
struct Step {
    pos: Position,
    steps: i32
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for Step {}

impl Hash for Step {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

fn solve(input: &Input) -> (i32, i32) {
    let (first, second) = input;

    let initial = (Position::origo(), 0, HashSet::new());
    let (_, _, visited) = first.iter()
        .fold(initial, |(position, tot_steps, mut visited), Movement { direction, steps }| {

            let mut next_position = position;
            for s in 1..=*steps {
                next_position = moved(direction, &next_position);
                let step = Step { pos: next_position, steps: tot_steps + s };
                if !visited.contains(&step) {
                    visited.insert(step);
                }
            }

            (next_position, tot_steps + steps, visited)
        });

    let initial = (Position::origo(), 0, (std::i32::MAX, std::i32::MAX));
    let (_, _, (p1, p2)) = second.iter()
        .fold(initial, |(position, tot_steps, (mut min_d, mut min_steps)), Movement { direction, steps }| {

            let mut next_position = position;
            for s in 1..=*steps {
                next_position = moved(direction, &next_position);
                let step = Step { pos: next_position, steps: tot_steps + s };

                if let Some(other) = visited.get(&step) {
                    let d = step.pos.distance();
                    if d < min_d {
                        min_d = d;
                    }

                    let steps = step.steps + other.steps;
                    if steps < min_steps {
                        min_steps = steps;
                    }
                }
            }

            (next_position, tot_steps + steps, (min_d, min_steps))
        });

    (p1, p2)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = solve(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

impl FromStr for Movement {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match &s[0..1] {
            "U" => Direction::North,
            "D" => Direction::South,
            "L" => Direction::West,
            "R" => Direction::East,
            _ => unreachable!()
        };
        let steps = s[1..].parse::<i32>()?;
        Ok(Movement { direction, steps })
    }
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    let mut lines = reader.lines();
    Ok((
        lines.next().unwrap()?.split(',').map(|i| i.parse::<Movement>().unwrap()).collect(),
        lines.next().unwrap()?.split(',').map(|i| i.parse::<Movement>().unwrap()).collect()
    ))
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &'static str =
       "R8,U5,L5,D3
        U7,R6,D4,L4";

    const INPUT2: &'static str =
       "R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83";

    const INPUT3: &'static str =
       "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
        U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input(INPUT1)).0, 6);
        assert_eq!(solve(&as_input(INPUT2)).0, 159);
        assert_eq!(solve(&as_input(INPUT3)).0, 135);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&as_input(INPUT1)).1, 30);
        assert_eq!(solve(&as_input(INPUT2)).1, 610);
        assert_eq!(solve(&as_input(INPUT3)).1, 410);
    }
}
