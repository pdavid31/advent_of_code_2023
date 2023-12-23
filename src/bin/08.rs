use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    str::FromStr,
};

advent_of_code::solution!(8);

enum Instruction {
    Left,
    Right,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(Error::new(ErrorKind::Other, "unknown instruction")),
        }
    }
}

struct Map(HashMap<String, (String, String)>);

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hs = HashMap::new();

        for line in s.lines() {
            let mut split = line.split('=');

            let source_str = split.next().unwrap().trim();
            let remainder = split.next().unwrap().replace('(', "").replace(')', "");

            let mut destination_split = remainder.split(',');

            let left_destination = destination_split.next().unwrap().trim();
            let right_destination = destination_split.next().unwrap().trim();

            let source = String::from(source_str);
            let destination = (
                String::from(left_destination),
                String::from(right_destination),
            );

            hs.insert(source, destination);
        }

        Ok(Self(hs))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_lines = input.lines();

    let instructions: Vec<Instruction> = input_lines
        .next()
        .unwrap()
        .chars()
        .map(|c| Instruction::from_str(c.to_string().as_str()).unwrap())
        .collect();

    let remainder: Vec<&str> = input_lines.skip(1).collect();
    let map_input = remainder.join("\n");

    let map = Map::from_str(&map_input).unwrap();

    let mut current_node = "AAA";
    let mut steps = 0;

    for instruction in instructions.iter().cycle() {
        if current_node == "ZZZ" {
            break;
        }

        let (left, right) = map.0.get(current_node).unwrap();

        let destination_node = match instruction {
            Instruction::Left => left,
            Instruction::Right => right,
        };

        current_node = destination_node;
        steps += 1;
    }

    Some(steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
