use regex::Regex;

extern crate regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        // first, we split our string into lines
        .lines()
        .filter_map(|line| {
            // we take each line and filter all numerical values from it
            let numbers: Vec<_> = line.chars().filter(|c| c.is_ascii_digit()).collect();

            // we then take the first value
            let left_opt = numbers.first();

            // if a digit is found, use it as the left part
            if let Some(left) = left_opt {
                // try to find the last digit, but fall back to the
                // already found left value if none is found
                let right = numbers.last().unwrap_or(left);

                // construct the number string from the left and right part
                Some(format!("{}{}", left, right))
            } else {
                // if no digit was found in the string, filter out the line
                None
            }
        })
        // parse the digit strings to u32
        .map(|digit| digit.parse::<u32>().unwrap())
        // sum it all up
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[1-9])").unwrap();

    let sum = input
        .to_lowercase()
        // first, we split our string into lines
        .lines()
        .map(|line| {
            // we take each line and filter all numerical values from it
            let numbers: Vec<u32> = re
                // find all regex matches in thre string
                .find_iter(line)
                // map every match by replacing it with the numerical value
                .map(|m| match m.as_str() {
                    "one" | "1" => 1,
                    "two" | "2" => 2,
                    "three" | "3" => 3,
                    "four" | "4" => 4,
                    "five" | "5" => 5,
                    "six" | "6" => 6,
                    "seven" | "7" => 7,
                    "eight" | "8" => 8,
                    "nine" | "9" => 9,
                    _ => 0,
                })
                .collect();

            // we then take the first value
            // if a digit is found, use it as the left part
            if let Some(left) = numbers.first() {
                // try to find the last digit, but fall back to the
                // already found left value if none is found
                let right = numbers.last().unwrap_or(left);

                // construct the number from the left and right part
                left * 10 + right
            } else {
                // if no digit was found in the string, filter out the line
                panic!("no digits found in {}", line);
            }
        })
        // sum it all up
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55090));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54500));
    }

    // taken straight from the example
    #[test_case("xtwone3four" => Some(24))]
    #[test_case("zoneight234" => Some(14))]
    #[test_case("7pqrstsixteen" => Some(76))]
    #[test_case("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen" => Some(281))]
    fn test_part_two_single(input: &str) -> Option<u32> {
        part_two(input)
    }
}
