advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    // split the input into individual cards
    let sum = input
        .lines()
        .map(|card| {
            let mut char_iter = card.chars();

            // skip the string "Card "
            char_iter.nth(4);

            let _id: u32 = (&mut char_iter)
                // continue reading until we get to the colon character
                .take_while(|c| *c != ':')
                // collect all characters before that into a string
                .collect::<String>()
                // remove whitespaces
                .trim()
                // try to parse it to u32
                .parse()
                .unwrap();

            let winning_numbers: Vec<u32> = (&mut char_iter)
                // take all characters until the pipe operator
                .take_while(|c| *c != '|')
                // construct a string with the taken characters
                .collect::<String>()
                // split the string on whitespaces
                .split_whitespace()
                // parse all values in the split
                .map(|p| p.parse().unwrap())
                // collect into vec
                .collect();

            char_iter
                // construct a string with the taken characters
                .collect::<String>()
                // split the string on whitespaces
                .split_whitespace()
                // parse all values in the split
                .map(|p| p.parse::<u32>().unwrap())
                // compute the cards worth
                .fold(0, |acc, cur| {
                    // if the current number is not a winning one,
                    // return the accumulator unmodified
                    if !winning_numbers.contains(&cur) {
                        return acc;
                    }

                    if acc == 0 {
                        1
                    } else {
                        acc * 2
                    }
                })
        })
        .sum();

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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
