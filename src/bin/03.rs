advent_of_code::solution!(3);

use regex::Regex;

fn subslice_has_symbol(input: &str, start: usize, end: usize) -> bool {
    input[start..end]
        // since we don't want to match on '.' characters,
        // replace them with numbers
        .replace('.', "0")
        // check if the string contains any symbols
        .contains(|c: char| c.is_ascii_punctuation())
}

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"\d+").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut numbers: Vec<u32> = Vec::new();

    for i in 0..lines.len() {
        let line = lines.get(i).unwrap();

        let matched_indices: Vec<(usize, usize)> = re
            // create an iterator over all matches of the regex
            .find_iter(lines[i])
            // for every match, store the start and end index
            .map(|m| (m.start(), m.end()))
            // collect them to a vector
            .collect();

        let mut numbers_with_adjacent_symbols: Vec<(usize, usize)> = Vec::new();

        // loop over all matches
        for (start, end) in matched_indices {
            // check if there are symbols to the left of our string
            if start > 0 && subslice_has_symbol(line, start - 1, start) {
                numbers_with_adjacent_symbols.push((start, end));
                continue;
            }

            // check if there are symbols to the right of our string
            if end < line.len() && subslice_has_symbol(line, end, end + 1) {
                numbers_with_adjacent_symbols.push((start, end));
                continue;
            }

            // the previous checks take into account that the match might
            // be at the beginning or the end of the string.
            // in the prior checks this is done explicitly to have the `subslice_has_symbol`
            // check being executed conditionally.
            // as we want to run at least one of the checks below either way,
            // we compute the safe indices here, that take start and end of the line into account
            let safe_start = if start > 0 { start - 1 } else { start };
            let safe_end = if end < line.len() - 1 { end + 1 } else { end };

            // if we are not in the first line, check the previous line for symbols
            if i > 0 && subslice_has_symbol(lines[i - 1], safe_start, safe_end) {
                numbers_with_adjacent_symbols.push((start, end));
                continue;
            }

            // if we are not in the last line, check the next line for symbols
            if i < lines.len() - 1 && subslice_has_symbol(lines[i + 1], safe_start, safe_end) {
                numbers_with_adjacent_symbols.push((start, end));
                continue;
            }
        }

        let parsed_numbers: Vec<u32> = numbers_with_adjacent_symbols
            .into_iter()
            .map(|(start, end)| line[start..end].parse().unwrap())
            .collect();

        numbers.extend(parsed_numbers);
    }

    let sum = numbers.iter().sum();

    Some(sum)
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}