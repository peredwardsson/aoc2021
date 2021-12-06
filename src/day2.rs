use std::{str::FromStr};

#[derive(Debug)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
pub struct Command {
    direction: Direction,
    distance: u32,
}

impl FromStr for Command {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s.to_lowercase().split_whitespace().next().unwrap() {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("Unknown direction: {}", s),
        };
        
        let distance = s[s.len() - 1..]
            .parse::<_>()
            .unwrap();

        Ok(Command { direction, distance })
    }
}

#[aoc_generator(day2)]
pub fn commands(input: &str) -> Vec<Command> {
    input
        .split('\n')
        .map(|s| s.trim())
        .map(|s| s.parse::<Command>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Command]) -> u32 {
    let mut output_tuple = (0, 0);
    for cmd in input {
        match cmd.direction {
            Direction::Down => {output_tuple.1 += cmd.distance},
            Direction::Up => {output_tuple.1 -= cmd.distance},
            Direction::Forward => {output_tuple.0 += cmd.distance},
        };
    }
        
    output_tuple.0 * output_tuple.1
}

#[aoc(day2, part2)]
pub fn part(input: &[Command]) -> u32 {
    let mut output_tuple = (0, 0, 0);

    for cmd in input {
        match cmd.direction {
            Direction::Down => {
                // output_tuple.1 += cmd.distance;
                output_tuple.2 += cmd.distance;
            },
            Direction::Up => {
                // output_tuple.1 -= cmd.distance;
                output_tuple.2 -= cmd.distance;
            },
            Direction::Forward => {
                output_tuple.0 += cmd.distance;
                output_tuple.1 += output_tuple.2 * cmd.distance;
            },
        };
    };
    
    output_tuple.0 * output_tuple.1
}