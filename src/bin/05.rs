use std::collections::HashMap;

advent_of_code::solution!(5);

struct RangeDescription(u32, u32, u32);
type Mapping = HashMap<u32, u32>;

// use the source ids and the mapping to create the vector of destination ids
fn map_ids(source_ids: &[u32], mapping: Mapping) -> Vec<u32> {
    source_ids
        .iter()
        // since the mapping only contains the given ranges,
        // we have to default to the given id if the key is not present in the map
        .map(|id| mapping.get(id).unwrap_or(id))
        .cloned()
        .collect()
}

// create a mapping table from the given range descriptions
fn create_mapping(range_descriptions: Vec<RangeDescription>) -> Mapping {
    // initialize our mapping
    let mut mapping = HashMap::new();

    // for every range_description, create the two ranges
    // and update the mapping
    range_descriptions.iter().for_each(|description| {
        // construct the source range
        let source_range = description.1..description.1 + description.2;
        // construct the destination range
        let destination_range = description.0..description.0 + description.2;

        // zip both ranges into a single iterator,
        // so we can just use these as key value pairs
        for (k, v) in source_range.into_iter().zip(destination_range.into_iter()) {
            // insert the current key value pair
            mapping.insert(k, v);
        }
    });

    // return the mapping
    mapping
}

// parse a list of range descriptions from the given list of strings
fn parse_mapping_descriptor(descriptor: Vec<&str>) -> Vec<RangeDescription> {
    descriptor
        .iter()
        .map(|x| {
            let mut split = x.split_whitespace();

            RangeDescription(
                // get the next string,
                // map Some options to u32 by parsing and unwrapping
                // and unwrap the option afterwards
                split.next().map(|x| x.parse().unwrap()).unwrap(),
                split.next().map(|x| x.parse().unwrap()).unwrap(),
                split.next().map(|x| x.parse().unwrap()).unwrap(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_iter = input.lines().peekable();

    // read the first line to get the initial source id's
    let mut ids: Vec<u32> = input_iter
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
        let range_descriptions = parse_mapping_descriptor(d);
        let mapping = create_mapping(range_descriptions);

        ids = map_ids(&ids, mapping);
    });

    // after executing all mappings, we should have the final destination id's
    // so we finally just have to find the minimum value
    ids.iter().min().cloned()
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
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
