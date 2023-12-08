advent_of_code::solution!(2);

use regex::Regex;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(?<amount>\d+)\s*(?<color>red|green|blue)").unwrap();

    let sum = input
        .lines()
        .map(|line| {
            let mut char_iter = line.chars();

            // skip the string "Game "
            char_iter.nth(4);

            let id: u32 = (&mut char_iter)
                // continue reading until we get to the colon character
                .take_while(|c| *c != ':')
                // collect all characters before that into a string
                .collect::<String>()
                // try to parse it to u32
                .parse()
                // if we are not able to parse the id, that's a bug
                .unwrap();

            // collect the remaining iterator into string
            let rest: String = char_iter.collect();

            (id, rest)
        })
        .filter_map(|(id, rest)| {
            // check if any of the games in the rest is invalid
            // split the remaining string on ';' to get each game
            let has_invalid_games = rest
                .split(';')
                // for every game in the split, check if it is invalid
                .any(|game| {
                    // split each game by ',' to get each draw and
                    // check if the game has any invalid draws
                    game.split(',').any(|draw| {
                        // match the regex against the draw
                        let Some(captures) = re.captures(draw) else {
                            println!("no captures found in Game {}", id);
                            // if no captures were found,
                            // we assume the game was invalid
                            return true;
                        };

                        // try to parse the amount,
                        // if it fails we assume the game was invalid
                        let Ok(amount) = &captures["amount"].parse::<u32>() else {
                            println!("no color amount found in Game {}", id);
                            return true;
                        };

                        // match the color and compare against the actual limit
                        match &captures["color"] {
                            "red" => amount > &MAX_RED,
                            "green" => amount > &MAX_GREEN,
                            "blue" => amount > &MAX_BLUE,
                            // if we matched something else,
                            // that should not be right
                            _ => panic!("matched an unknown color"),
                        }
                    })
                });

            if has_invalid_games {
                None
            } else {
                Some(id)
            }
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(?<amount>\d+)\s*(?<color>red|green|blue)").unwrap();

    let sum = input
        .lines()
        .map(|line| {
            let mut char_iter = line.chars();

            // skip the string "Game "
            char_iter.nth(4);

            let id: u32 = (&mut char_iter)
                // continue reading until we get to the colon character
                .take_while(|c| *c != ':')
                // collect all characters before that into a string
                .collect::<String>()
                // try to parse it to u32
                .parse()
                // if we are not able to parse the id, that's a bug
                .unwrap();

            let rest: String = char_iter.collect();

            (id, rest)
        })
        .map(|(id, rest)| {
            let color_values = rest
                .split(';')
                // for every game in the split, get the required minimum of cubes
                .map(|game| {
                    // initialize the colors with their respective minimum
                    let mut red = u32::MIN;
                    let mut green = u32::MIN;
                    let mut blue = u32::MIN;

                    // split each game by ',' to get each draw and
                    // set the respective color values
                    game.split(',').for_each(|draw| {
                        // match the regex against the draw
                        let Some(captures) = re.captures(draw) else {
                            panic!("no captures found in Game {}", id);
                        };

                        let Ok(amount) = &captures["amount"].parse::<u32>() else {
                            panic!("no color amount found in Game {}", id);
                        };

                        // set the color value
                        match &captures["color"] {
                            "red" => {
                                red = *amount;
                            }
                            "green" => {
                                green = *amount;
                            }
                            "blue" => {
                                blue = *amount;
                            }
                            // if we matched something else,
                            // that should not be right
                            _ => panic!("matched an unknown color"),
                        }
                    });

                    (red, green, blue)
                })
                .reduce(|acc, cur| {
                    (
                        u32::max(acc.0, cur.0),
                        u32::max(acc.1, cur.1),
                        u32::max(acc.2, cur.2),
                    )
                })
                .unwrap();

            color_values.0 * color_values.1 * color_values.2
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
