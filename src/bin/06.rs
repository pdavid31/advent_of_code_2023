use std::u64;

advent_of_code::solution!(6);

// acceleration in meters / s * s
const ACCELERATION: u64 = 1;

fn get_number_of_winning_combinations(time: u64, record_distance: u64) -> u64 {
    (0..=time)
        .map(|button_press_time| {
            // compute achieved distance
            let travel_speed = ACCELERATION * button_press_time;
            let travel_time = time - button_press_time;

            travel_speed * travel_time
        })
        // filter out the races that did not beat the record distance
        .filter(|achieved_distance| achieved_distance > &record_distance)
        // count the winning races
        .count()
        // convert the count (usize) to u32
        .try_into()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let time_line = lines.next().unwrap();
    let distances_line = lines.next().unwrap();

    let times: Vec<u64> = time_line
        .replace("Time:", "")
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let distances: Vec<u64> = distances_line
        .replace("Distance:", "")
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let races: Vec<(u64, u64)> = times.into_iter().zip(distances.into_iter()).collect();

    let product: u64 = races
        .into_iter()
        .map(|(time, distance)| get_number_of_winning_combinations(time, distance))
        .product();

    Some(product as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    let time_line = lines.next().unwrap();
    let distances_line = lines.next().unwrap();

    let time: u64 = time_line
        .replace("Time:", "")
        .replace(" ", "")
        .parse()
        .unwrap();

    let distance: u64 = distances_line
        .replace("Distance:", "")
        .replace(" ", "")
        .parse()
        .unwrap();

    let combinations = get_number_of_winning_combinations(time, distance);

    Some(combinations as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
