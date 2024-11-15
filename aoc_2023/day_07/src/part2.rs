extern crate nom;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::fs::File;
use std::cmp::Ord;
use std::cmp::Ordering;
use nom::{
  IResult,
  character::complete::{not_line_ending, digit1, space1, alphanumeric1},
  sequence::separated_pair,
  multi::separated_list1,
  bytes::complete::tag,
};

fn read_input(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).expect("cannot open input file"));

    for line in reader.lines() {
        result.push(line.unwrap())
    }
    return result
}

const card_ordering: &'static [char] = &['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

#[derive(Default, Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u64,
    rank: i8,
    result: String,
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
                    let self_position = card_ordering.iter().position(|n| *n == self.cards[i]);
                    let other_position = card_ordering.iter().position(|n| *n == other.cards[i]);
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
        hand.result = "5 of a kind".to_string();
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
    let mut joker_amount: i8 = 0;
    if binned_hand.contains_key(&'J') {
        joker_amount = *binned_hand.get(&'J').unwrap();
    }
    for (card, amount) in binned_hand.into_iter() {
        if *card == 'J' {
            continue;
        }
        if *amount + joker_amount == expected_amount {
            return true;
        }
    }
    false
}

fn is_x_pairs(binned_hand: &HashMap<char, i8>, expected_pairs: i8) -> bool {
    let mut joker_amount: i8 = 0;
    if binned_hand.contains_key(&'J') {
        joker_amount = *binned_hand.get(&'J').unwrap();
    }
    if expected_pairs == 1 && joker_amount > 0 {
        return true;
    }
    let mut found_pairs = 0;
    for (card, amount) in binned_hand.into_iter() {
        if *card == 'J' {
            continue;
        }
        if *amount == 2 {
            found_pairs+=1;
        }
    }
    if joker_amount > 0 && found_pairs > 0 {
        // if there is already one pair and at least one joker, it is obvious that there will be a
        // second pair as well
        return true;
    }
    return found_pairs == expected_pairs;
}

fn is_full_house(binned_hand: &HashMap<char, i8>) -> bool {
    let mut joker_amount: i8 = 0;
    if binned_hand.contains_key(&'J') {
        joker_amount = *binned_hand.get(&'J').unwrap();
    }
    if joker_amount > 2 {
        // Three or more jokers is always full house
        return true;
    }
    let mut found_pairs = 0;
    let mut found_three_of_a_kind = false;
    for (card, amount) in binned_hand.into_iter() {
        if *card == 'J' {
            continue;
        }
        if *amount == 2 {
            found_pairs = found_pairs + 1;
        }
        if *amount == 3 {
            found_three_of_a_kind = true;
        }
    }
    if joker_amount == 2 && (found_three_of_a_kind || found_pairs > 0) {
        return true;
    }
    if joker_amount == 1 && (found_pairs == 2 || found_three_of_a_kind) {
        return true;
    }
    return found_pairs > 0 && found_three_of_a_kind;
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
    let (remainder, hand) = separated_list1(space1, alphanumeric1)(hand_string)
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
                result: "".to_string(),
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
        let mut hand = parse_hand("KKKKK 28").unwrap().1;
        let mut binned_hand = bin_cards(&hand);
        assert_eq!(is_x_of_a_kind(&binned_hand, 5), true);

        hand = parse_hand("KJKJK 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_x_of_a_kind(&binned_hand, 5), true);

        hand = parse_hand("KKKQK 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_x_of_a_kind(&binned_hand, 5), false);
        assert_eq!(is_x_of_a_kind(&binned_hand, 4), true);

        hand = parse_hand("KJKJQ 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_x_of_a_kind(&binned_hand, 5), false);
        assert_eq!(is_x_of_a_kind(&binned_hand, 4), true);

        hand = parse_hand("QKKQK 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_x_of_a_kind(&binned_hand, 5), false);
        assert_eq!(is_x_of_a_kind(&binned_hand, 4), false);
        assert_eq!(is_x_of_a_kind(&binned_hand, 3), true);

        hand = parse_hand("KJQJ1 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_x_of_a_kind(&binned_hand, 5), false);
        assert_eq!(is_x_of_a_kind(&binned_hand, 4), false);
        assert_eq!(is_x_of_a_kind(&binned_hand, 3), true);

    }

    #[test]
    fn test_is_x_pairs() {
        let mut hand = parse_hand("KKQ1Q 28").unwrap().1;
        let mut binned_hand = bin_cards(&hand);
        assert_eq!(is_x_pairs(&binned_hand, 2), true);

        hand = parse_hand("KJKJQ 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_x_pairs(&binned_hand, 2), true);

        hand = parse_hand("K1KJQ 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_x_pairs(&binned_hand, 2), true);

        hand = parse_hand("2KAQK 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_x_pairs(&binned_hand, 2), false);
        assert_eq!(is_x_pairs(&binned_hand, 1), true);

        hand = parse_hand("K21J3 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_x_pairs(&binned_hand, 2), false);
        assert_eq!(is_x_pairs(&binned_hand, 1), true);
    }

    #[test]
    fn test_is_full_house() {
        let mut hand = parse_hand("KKQ1Q 28").unwrap().1;
        let mut binned_hand = bin_cards(&hand);
        assert_eq!(is_full_house(&binned_hand), false);

        hand = parse_hand("KJKJQ 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_full_house(&binned_hand), true);

        hand = parse_hand("K1KJQ 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_full_house(&binned_hand), false);

        hand = parse_hand("JJJJJ 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_full_house(&binned_hand), true);

        hand = parse_hand("JJKJJ 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_full_house(&binned_hand), true);

        hand = parse_hand("JJKJQ 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_full_house(&binned_hand), true);

        hand = parse_hand("JJKJK 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_full_house(&binned_hand), true);

        hand = parse_hand("JJKQK 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_full_house(&binned_hand), true);

        hand = parse_hand("JJKQA 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_full_house(&binned_hand), false);

        hand = parse_hand("JJKKK 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_full_house(&binned_hand), true);

        hand = parse_hand("JKKQA 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_full_house(&binned_hand), false);

        hand = parse_hand("KJKKA 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_full_house(&binned_hand), true);

        hand = parse_hand("JQKKK 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_full_house(&binned_hand), true);

        hand = parse_hand("KJK12 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_full_house(&binned_hand), false);

        hand = parse_hand("KJK11 28").unwrap().1;
        binned_hand = bin_cards(&hand);
        assert_eq!(is_full_house(&binned_hand), true);
    }
}
