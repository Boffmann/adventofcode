extern crate nom;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::fs::File;
use std::cmp::Ord;
use std::cmp::Ordering;
use nom::multi::separated_list1;
use nom::{
  IResult,
  character::complete::{space1, alphanumeric1},
};

fn read_input(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).expect("cannot open input file"));

    for line in reader.lines() {
        result.push(line.unwrap())
    }
    return result
}

const CARD_ORDERING: &'static [char] = &['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

#[derive(Default, Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u64,
    rank: i8,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rank > other.rank {
            return Ordering::Greater;
        }
        if self.rank < other.rank {
            return Ordering::Less;
        }
        if self.rank == other.rank {
            for i in 0..self.cards.len() {
                if self.cards[i] != other.cards[i] {
                    let self_position = CARD_ORDERING.iter().position(|n| *n == self.cards[i]);
                    let other_position = CARD_ORDERING.iter().position(|n| *n == other.cards[i]);
                    if self_position > other_position {
                        return Ordering::Greater;
                    } else {
                        return Ordering::Less;
                    }
                }
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {}

fn rank_hand(hand: &mut Hand) {
    let binned_hand = bin_cards(hand);

    if is_x_of_a_kind(&binned_hand, 5) {
        hand.rank = 7;
    } else if is_x_of_a_kind(&binned_hand, 4) {
        hand.rank = 6;
    } else if is_full_house(&binned_hand) {
        hand.rank = 5;
    } else if is_x_of_a_kind(&binned_hand, 3) {
        hand.rank = 4;
    } else if is_x_pairs(&binned_hand, 2) {
        hand.rank = 3;
    } else if is_x_pairs(&binned_hand, 1) {
        hand.rank = 2;
    } else {
        hand.rank = 1;
    }
}

fn bin_cards(hand: &Hand) -> HashMap<char, i8> {
    let mut bins: HashMap<char, i8> = HashMap::new();
    for card in &hand.cards {
        if bins.contains_key(card) {
            bins.insert(*card, bins[card] + 1);
        } else {
            bins.insert(*card, 1);
        }
    }

    bins
}

fn is_x_of_a_kind(binned_hand: &HashMap<char, i8>, expected_amount: i8) -> bool {
    for (_, amount) in binned_hand.into_iter() {
        if *amount == expected_amount {
            return true;
        }
    }
    false
}

fn is_x_pairs(binned_hand: &HashMap<char, i8>, expected_pairs: i8) -> bool {
    let mut found_pairs = 0;
    for (_, amount) in binned_hand.into_iter() {
        if *amount == 2 {
            found_pairs+=1;
        }
    }
    return found_pairs == expected_pairs;
}

fn is_full_house(binned_hand: &HashMap<char, i8>) -> bool {
    let mut found_pair = false;
    let mut found_three_of_a_kind = false;
    for (_, amount) in binned_hand.into_iter() {
        if *amount == 2 {
            found_pair = true;
        }
        if *amount == 3 {
            found_three_of_a_kind = true;
        }
    }
    return found_pair && found_three_of_a_kind;
}

fn main() {
    let input = read_input("input_part1.txt");
    let mut hands: Vec<Hand> = Vec::new();

    for line_index in 0..input.len() {
        let line = &input[line_index];
        let mut hand = parse_hand(line).expect("A Hand of Cards").1;
        rank_hand(&mut hand);
        hands.push(hand);
    }

    hands.sort();

    let mut result = 0;
    for hand_index in 0..hands.len() {
        result = result + hands[hand_index].bid * (hand_index + 1) as u64;
    }
    println!("{:?}", result);
}

fn parse_hand(hand_string: &str) -> IResult<&str, Hand> {
    let (_, hand) = separated_list1(space1, alphanumeric1)(hand_string)
        .map(|parsed| {
            let cards_string = parsed.1[0];
            let bid = parsed.1[1];
            let mut cards: Vec<char> = Vec::new();
            for c in cards_string.chars() {
                cards.push(c);
            }
            let hand = Hand {
                cards: cards,
                bid: bid.parse::<u64>().expect("A number"),
                rank: 0,
            };
            ("", hand)
        })?;
    Ok(("", hand))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_bin_hand() {
        let hand = parse_hand("KK677 28").unwrap().1;

        let binned_hand = bin_cards(&hand);
        assert_eq!(*binned_hand.get(&'K').unwrap(), 2 as i8);
        assert_eq!(*binned_hand.get(&'7').unwrap(), 2);
        assert_eq!(*binned_hand.get(&'6').unwrap(), 1);

        assert_eq!(is_x_pairs(&binned_hand, 2), true);
    }

    #[test]
    fn test_is_x_of_a_kind() {
        let mut binned_hand: HashMap<char, i8> = HashMap::new();
        binned_hand.insert('A', 4);
        binned_hand.insert('B', 5);
        binned_hand.insert('C', 3);

        assert_eq!(is_x_of_a_kind(&binned_hand, 5), true);

        let mut binned_hand_2: HashMap<char, i8> = HashMap::new();
        binned_hand_2.insert('A', 4);
        binned_hand_2.insert('B', 2);
        binned_hand_2.insert('C', 3);

        assert_eq!(is_x_of_a_kind(&binned_hand_2, 5), false);
        assert_eq!(is_x_of_a_kind(&binned_hand_2, 4), true);
    }

    #[test]
    fn test_is_x_pairs() {
        let mut binned_hand: HashMap<char, i8> = HashMap::new();
        binned_hand.insert('A', 2);
        binned_hand.insert('B', 5);
        binned_hand.insert('C', 2);

        assert_eq!(is_x_pairs(&binned_hand, 1), false);
        assert_eq!(is_x_pairs(&binned_hand, 2), true);
        assert_eq!(is_x_pairs(&binned_hand, 3), false);

        let mut binned_hand_2: HashMap<char, i8> = HashMap::new();
        binned_hand_2.insert('A', 4);
        binned_hand_2.insert('B', 2);
        binned_hand_2.insert('C', 3);
        binned_hand_2.insert('D', 3);

        assert_eq!(is_x_pairs(&binned_hand_2, 1), true);
        assert_eq!(is_x_pairs(&binned_hand_2, 2), false);
        assert_eq!(is_x_pairs(&binned_hand_2, 3), false);
    }

    #[test]
    fn test_is_full_house() {
        let mut binned_hand: HashMap<char, i8> = HashMap::new();
        binned_hand.insert('A', 2);
        binned_hand.insert('B', 1);
        binned_hand.insert('C', 2);

        assert_eq!(is_full_house(&binned_hand), false);

        let mut binned_hand_2: HashMap<char, i8> = HashMap::new();
        binned_hand_2.insert('B', 2);
        binned_hand_2.insert('C', 3);

        assert_eq!(is_full_house(&binned_hand_2), true);
    }
}
