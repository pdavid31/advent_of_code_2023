use std::str::FromStr;

advent_of_code::solution!(7);

mod p1 {
    use std::{cmp::Ordering, collections::HashMap, str::FromStr};

    #[repr(u8)]
    #[derive(Debug, Eq, Hash, PartialEq, PartialOrd)]
    pub enum Card {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Joker,
        Queen,
        King,
        Ace,
    }

    impl FromStr for Card {
        type Err = std::io::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let card = match s.to_uppercase().as_str() {
                "2" => Self::Two,
                "3" => Self::Three,
                "4" => Self::Four,
                "5" => Self::Five,
                "6" => Self::Six,
                "7" => Self::Seven,
                "8" => Self::Eight,
                "9" => Self::Nine,
                "T" => Self::Ten,
                "J" => Self::Joker,
                "Q" => Self::Queen,
                "K" => Self::King,
                "A" => Self::Ace,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Invalid card",
                    ));
                }
            };

            Ok(card)
        }
    }

    #[repr(u8)]
    #[derive(PartialEq, PartialOrd)]
    enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    #[derive(PartialEq)]
    pub struct Hand([Card; 5]);

    impl Hand {
        fn hand_type(&self) -> HandType {
            let mut hs: HashMap<&Card, u32> = HashMap::new();

            // count the number of occurences per card
            for card in self.0.iter() {
                if let Some(count) = hs.get_mut(&card) {
                    // if the key is already present in the map,
                    // count it up by one
                    *count += 1;
                } else {
                    // otherwise, insert the initial value 1
                    hs.insert(card, 1);
                }
            }

            // disregard the keys and only collect the values in a vec
            let values: Vec<u32> = hs.into_values().collect();

            if values.contains(&5) {
                HandType::FiveOfAKind
            } else if values.contains(&4) {
                HandType::FourOfAKind
            } else if values.contains(&3) && values.contains(&2) {
                HandType::FullHouse
            } else if values.contains(&3) {
                HandType::ThreeOfAKind
            } else if values.iter().filter(|x| *x == &2).count() == 2 {
                HandType::TwoPair
            } else if values.contains(&2) {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            // compare the two hands
            self.hand_type()
                .partial_cmp(&other.hand_type())
                // map the Some option
                .map(|ordering| {
                    if ordering == Ordering::Equal {
                        // if both are equal, we have to check the individual high cards
                        self.0
                            .iter()
                            // tie the left and right card iterators together
                            .zip(other.0.iter())
                            // compare the cards in order
                            .map(|(left, right)| left.partial_cmp(right))
                            // find the first appearance of a card that was
                            // Greater or Less than the other one
                            .find(|x| *x == Some(Ordering::Greater) || *x == Some(Ordering::Less))
                            // find returns an option (None if condition did not met on any index)
                            // so, here we flatten the Option<Option<Ordering>> into Option<Ordering>
                            .flatten()
                            // if it's None, we just assume that both cards are equal
                            .unwrap_or(Ordering::Equal)
                    } else {
                        // otherwise just return the original ordering
                        ordering
                    }
                })
        }
    }

    impl FromStr for Hand {
        type Err = std::io::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let cards: Vec<Card> = s
                .chars()
                .map(|char| Card::from_str(char.to_string().as_str()))
                .collect::<Result<Vec<Card>, std::io::Error>>()?;

            let arr = cards.try_into().unwrap();

            Ok(Self(arr))
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands_and_bids: Vec<(p1::Hand, u32)> = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();

            let card_str = split.next().unwrap();
            let bid_str = split.next().unwrap();

            let card = p1::Hand::from_str(card_str).unwrap();
            let bid: u32 = bid_str.parse().unwrap();

            (card, bid)
        })
        .collect();

    hands_and_bids.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let total_winnings = hands_and_bids
        .into_iter()
        .enumerate()
        .map(|(index, (_, bid))| (index as u32 + 1) * bid)
        .sum();

    Some(total_winnings)
}

mod p2 {
    use std::{cmp::Ordering, collections::HashMap, str::FromStr};

    #[repr(u8)]
    #[derive(Debug, Eq, Hash, PartialEq, PartialOrd)]
    pub enum Card {
        Joker,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Queen,
        King,
        Ace,
    }

    impl FromStr for Card {
        type Err = std::io::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let card = match s.to_uppercase().as_str() {
                "J" => Self::Joker,
                "2" => Self::Two,
                "3" => Self::Three,
                "4" => Self::Four,
                "5" => Self::Five,
                "6" => Self::Six,
                "7" => Self::Seven,
                "8" => Self::Eight,
                "9" => Self::Nine,
                "T" => Self::Ten,
                "Q" => Self::Queen,
                "K" => Self::King,
                "A" => Self::Ace,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Invalid card",
                    ));
                }
            };

            Ok(card)
        }
    }

    #[repr(u8)]
    #[derive(PartialEq, PartialOrd)]
    enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    #[derive(PartialEq)]
    pub struct Hand([Card; 5]);

    impl Hand {
        fn hand_type(&self) -> HandType {
            let mut hs: HashMap<&Card, u32> = HashMap::new();

            // count the number of occurences per card
            for card in self.0.iter() {
                if let Some(count) = hs.get_mut(&card) {
                    // if the key is already present in the map,
                    // count it up by one
                    *count += 1;
                } else {
                    // otherwise, insert the initial value 1
                    hs.insert(card, 1);
                }
            }

            // count the jokers
            let amount_of_jokers = hs.remove(&Card::Joker).unwrap_or(0);

            // disregard the keys and only collect the values in a vec
            let values: Vec<u32> = hs.into_values().collect();

            if values.contains(&5)
                || values.contains(&4) && amount_of_jokers == 1
                || values.contains(&3) && amount_of_jokers == 2
                || values.contains(&2) && amount_of_jokers == 3
                || values.contains(&1) && amount_of_jokers == 4
                || amount_of_jokers == 5
            {
                HandType::FiveOfAKind
            } else if values.contains(&4)
                || values.contains(&3) && amount_of_jokers == 1
                || values.contains(&2) && amount_of_jokers == 2
                || values.contains(&1) && amount_of_jokers == 3
                || amount_of_jokers == 4
            {
                HandType::FourOfAKind
            } else if values.contains(&3) && values.contains(&2)
                || values.contains(&3) && values.contains(&1) && amount_of_jokers == 1
                || values.contains(&2) && values.contains(&1) && amount_of_jokers == 2
                || values.contains(&1) && values.contains(&1) && amount_of_jokers == 3
                || values.iter().filter(|x| *x == &2).count() == 2 && amount_of_jokers == 1
            {
                HandType::FullHouse
            } else if values.contains(&3)
                || values.contains(&2) && amount_of_jokers == 1
                || values.contains(&1) && amount_of_jokers == 2
                || amount_of_jokers == 3
            {
                HandType::ThreeOfAKind
            } else if values.iter().filter(|x| *x == &2).count() == 2 {
                HandType::TwoPair
            } else if values.contains(&2)
                || values.contains(&1) && amount_of_jokers == 1
                || amount_of_jokers == 2
            {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            // compare the two hands
            self.hand_type()
                .partial_cmp(&other.hand_type())
                // map the Some option
                .map(|ordering| {
                    if ordering == Ordering::Equal {
                        // if both are equal, we have to check the individual high cards
                        self.0
                            .iter()
                            // tie the left and right card iterators together
                            .zip(other.0.iter())
                            // compare the cards in order
                            .map(|(left, right)| left.partial_cmp(right))
                            // find the first appearance of a card that was
                            // Greater or Less than the other one
                            .find(|x| *x == Some(Ordering::Greater) || *x == Some(Ordering::Less))
                            // find returns an option (None if condition did not met on any index)
                            // so, here we flatten the Option<Option<Ordering>> into Option<Ordering>
                            .flatten()
                            // if it's None, we just assume that both cards are equal
                            .unwrap_or(Ordering::Equal)
                    } else {
                        // otherwise just return the original ordering
                        ordering
                    }
                })
        }
    }

    impl FromStr for Hand {
        type Err = std::io::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let cards: Vec<Card> = s
                .chars()
                .map(|char| Card::from_str(char.to_string().as_str()))
                .collect::<Result<Vec<Card>, std::io::Error>>()?;

            let arr = cards.try_into().unwrap();

            Ok(Self(arr))
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands_and_bids: Vec<(p2::Hand, u32)> = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();

            let card_str = split.next().unwrap();
            let bid_str = split.next().unwrap();

            let card = p2::Hand::from_str(card_str).unwrap();
            let bid: u32 = bid_str.parse().unwrap();

            (card, bid)
        })
        .collect();

    hands_and_bids.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let total_winnings = hands_and_bids
        .into_iter()
        .enumerate()
        .map(|(index, (_, bid))| (index as u32 + 1) * bid)
        .sum();

    Some(total_winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
