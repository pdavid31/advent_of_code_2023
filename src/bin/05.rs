use rayon::prelude::*;

advent_of_code::solution!(5);

struct Map {
    source: u64,
    destination: u64,
    length: u64,
}

impl Map {
    fn contains_source(&self, value: u64) -> bool {
        self.source <= value && value < self.source + self.length
    }

    fn map(&self, value: u64) -> u64 {
        self.destination + value - self.source
    }
}

struct Mapping(Vec<Map>);

impl Mapping {
    fn new(maps: Vec<Map>) -> Self {
        Self(maps)
    }

    fn map(&self, value: u64) -> u64 {
        // check of any of the maps inside contains the value
        let map_opt = self.0.par_iter().find_any(|x| x.contains_source(value));

        // if it is included in one of the map,
        // use this map to compute the new destination value,
        // otherwise return the initial value
        if let Some(map) = map_opt {
            map.map(value)
        } else {
            value
        }
    }
}

// use the source ids and the mapping to create the vector of destination ids
fn map_ids(source_ids: &[u64], mapping: Mapping) -> Vec<u64> {
    source_ids.iter().map(|value| mapping.map(*value)).collect()
}

// parse a list of range descriptions from the given list of strings
fn parse_mapping_descriptor(descriptor: Vec<&str>) -> Mapping {
    let maps = descriptor
        .iter()
        .map(|x| {
            let mut split = x.split_whitespace();

            Map {
                destination: split.next().map(|x| x.parse().unwrap()).unwrap(),
                source: split.next().map(|x| x.parse().unwrap()).unwrap(),
                length: split.next().map(|x| x.parse().unwrap()).unwrap(),
            }
        })
        .collect();

    Mapping::new(maps)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_iter = input.lines().peekable();

    // read the first line to get the initial source id's
    let mut ids: Vec<u64> = input_iter
        // take the first row
        .nth(0)
        // if there is none, the input is not valid
        .unwrap()
        // remove the string 'seeds'
        .replace("seeds:", "")
        // split the numbers on whitespaces
        .split_whitespace()
        // parse each number
        .map(|n| n.trim().parse().unwrap())
        // collect them to vec
        .collect();

    // extract the mapping blocks into a vec
    let mut mapping_descriptors: Vec<Vec<&str>> = Vec::new();

    // loop while there are still items in the iterator
    while input_iter.peek().is_some() {
        let descriptor: Vec<&str> = (&mut input_iter)
            // take lines from the iterator until we find the next empty line
            .take_while(|line| !line.is_empty())
            // filter out lines that are empty or only house a description
            .filter(|line| !line.starts_with(|c: char| c.is_alphabetic()))
            .collect();

        if descriptor.is_empty() {
            continue;
        }

        mapping_descriptors.push(descriptor);
    }

    // for every mapping descriptor, parse it to a RangeDescription
    // create a mapping out of it and update the id's
    mapping_descriptors.into_iter().for_each(|d| {
        let mapping = parse_mapping_descriptor(d);

        ids = map_ids(&ids, mapping);
    });

    // after executing all mappings, we should have the final destination id's
    // so we finally just have to find the minimum value
    ids.iter().min().map(|x| *x as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input_iter = input.lines().peekable();

    // read the first line to get the initial source id's
    let identifier: Vec<u64> = input_iter
        // take the first row
        .nth(0)
        // if there is none, the input is not valid
        .unwrap()
        // remove the string 'seeds'
        .replace("seeds:", "")
        // split the numbers on whitespaces
        .split_whitespace()
        // parse each number
        .map(|n| n.trim().parse().unwrap())
        // collect them to vec
        .collect();

    let mut ids: Vec<u64> = identifier
        .par_iter()
        .chunks(2)
        .map(|chunk| {
            let start = chunk.first().unwrap().clone();
            let length = chunk.last().unwrap();
            let stop = *start + *length;

            (*start..stop).collect::<Vec<u64>>()
        })
        .flatten()
        .collect();

    // extract the mapping blocks into a vec
    let mut mapping_descriptors: Vec<Vec<&str>> = Vec::new();

    // loop while there are still items in the iterator
    while input_iter.peek().is_some() {
        let descriptor: Vec<&str> = (&mut input_iter)
            // take lines from the iterator until we find the next empty line
            .take_while(|line| !line.is_empty())
            // filter out lines that are empty or only house a description
            .filter(|line| !line.starts_with(|c: char| c.is_alphabetic()))
            .collect();

        if descriptor.is_empty() {
            continue;
        }

        mapping_descriptors.push(descriptor);
    }

    // for every mapping descriptor, parse it to a RangeDescription
    // create a mapping out of it and update the id's
    mapping_descriptors.into_iter().for_each(|d| {
        let mapping = parse_mapping_descriptor(d);

        ids = map_ids(&ids, mapping);
    });

    // after executing all mappings, we should have the final destination id's
    // so we finally just have to find the minimum value
    ids.iter().min().map(|x| *x as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
