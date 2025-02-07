extern crate nom;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use nom::{
  IResult,
  character::complete::{digit1, space1},
  sequence::separated_pair,
  multi::separated_list1,
  bytes::complete::tag,
  error::Error,
};

fn read_input(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).expect("cannot open input file"));

    for line in reader.lines() {
        result.push(line.unwrap())
    }
    return result
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Card {
    id: u32,
    winning_numbers: Vec<u64>,
    own_numbers: Vec<u64>,
}

fn main() {
    let input = read_input("./input_1.txt");
    let mut result : u64 = 1;
    let mut cards: Vec<Card> = Vec::new();
    for line in input {
        cards.push(parse_card(&line).unwrap().1);
    }
    let mut intermediates = HashMap::with_capacity(cards.len());
    intermediates.insert(cards.len(), 1);
    for card_index in (0..cards.len()-1).rev() {
        intermediates.insert(card_index+1, get_value_for_card(&cards[card_index], &intermediates, &cards[card_index+1..]));
        result += intermediates.get(&(card_index+1))
            .expect(&format!("Value for card {} is known here", card_index + 1));
    }

    println!("{}", result);
}

fn get_value_for_card(card: &Card, intermediates: &HashMap<usize, u64>, next_cards: &[Card]) -> u64 {
    if next_cards.len() == 0 {
        return 0;
    }
    let mut result: u64 = 1;
    let winning_points = calculate_winning_points(card);
    for card_index in 0..winning_points {
        match next_cards.get(card_index) {
            Some(next_card) => {
                result += intermediates.get(&(next_card.id as usize))
                    .expect(&format!("Value for card {} is known here", next_card.id));
            }
            None => {
                return result;
            }
        }
    }
    result
}

fn calculate_winning_points(card: &Card) -> usize {
    let num_winning: u64 = card.own_numbers.iter()
        .filter(|own_number| card.winning_numbers.contains(own_number))
        .collect::<Vec<&u64>>()
        .len().try_into().unwrap();
    num_winning as usize
}

fn parse_card(card_string: &str) -> IResult<&str, Card> {
    let (remainder, card) = separated_pair(
            separated_pair(tag("Card"), space1, digit1),
            tag(": "),
            separated_pair(parse_numbers, tag(" | "), parse_numbers)
        )(card_string).unwrap();
    Ok((remainder, Card {id: card.0.1.parse::<u32>().unwrap(), winning_numbers: card.1.0, own_numbers: card.1.1}))
}

fn parse_numbers(numbers: &str) -> IResult<&str, Vec<u64>> {
    let (remainder, numbers) = separated_list1(space1::<&str, Error<_>>, digit1)(numbers.trim())
        .map(|(remainder, numbers)| {
            (remainder, numbers.iter()
                .map(|number| number.parse::<u64>().unwrap())
                .collect::<Vec<u64>>())
        }).unwrap();
    Ok((remainder, numbers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        let numbers_string = "41 48 83 86 17";
        let numbers = parse_numbers(numbers_string).unwrap().1;
        assert_eq!(numbers, vec![41, 48, 83, 86, 17]);

        let numbers_string = " 1 21 53 59 44";
        let numbers = parse_numbers(numbers_string).unwrap().1;
        assert_eq!(numbers, vec![1, 21, 53, 59, 44]);
    }

    #[test]
    fn test_parse_card() {
        let card_string = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = parse_card(card_string).unwrap().1;
        assert_eq!(card, Card {
            id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            own_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
        });

        let card_string = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let card = parse_card(card_string).unwrap().1;
        assert_eq!(card, Card {
            id: 3,
            winning_numbers: vec![1, 21, 53, 59, 44],
            own_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1]
        })
    }
}
