extern crate nom;
use std::io::{BufRead, BufReader};
use std::fs::File;
use nom::{
  IResult,
  character::complete::{digit1, alpha1, char},
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
struct Game {
    id: u32,
    bags: Vec<Bag>,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Bag {
    num_red: u32,
    num_green: u32,
    num_blue: u32,
}

impl Bag {
    fn from_color_list(color_list: Vec<(&str, &str)>) -> Self {
        let mut bag = Bag{num_red: 0, num_green: 0, num_blue: 0};
        for color in color_list {
            let number = color.0.parse::<u32>().unwrap();
            match color.1 {
                "red" => bag.num_red = number,
                "green" => bag.num_green = number,
                "blue" => bag.num_blue = number,
                _ => println!("Invalid color")
            }
        }
        bag
    }
}

fn main() {
    let input = read_input("./input.txt");
    let mut result : u32 = 0;
    for line in input {
        let game = parse_game(&line).unwrap().1;
        result += product_of_lease_required(&game);
    }
    println!("{}", result);
}

fn product_of_lease_required(game: &Game) -> u32 {
    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;
    for bag in &game.bags {
        max_red = std::cmp::max(max_red, bag.num_red);
        max_green = std::cmp::max(max_green, bag.num_green);
        max_blue = std::cmp::max(max_blue, bag.num_blue);
    }
    max_red * max_green * max_blue
}

fn parse_game(game_string: &str) -> IResult<&str, Game> {
    let (remainder, game) = separated_pair(
        separated_pair(tag("Game"), char::<&str, Error<_>>(' '), digit1),
        tag(": "),
        parse_bags)(game_string).unwrap();
    Ok((remainder, Game {id: game.0.1.parse::<u32>().unwrap(), bags: game.1}))
}

fn parse_bags(bags_string: &str) -> IResult<&str, Vec<Bag>> {
    separated_list1(tag("; "), parse_bag)(bags_string)
}

fn parse_bag(bag_string: &str) -> IResult<&str, Bag> {
    let (remainder, parsed_bag) = separated_list1(tag(", "), separated_pair(digit1, char::<&str, Error<_>>(' '), alpha1))(bag_string).unwrap();
    Ok((remainder, Bag::from_color_list(parsed_bag)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let game_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        // parse_game(game_string);
        let game = parse_game(game_string).unwrap().1;
        assert_eq!(game, Game {id: 1, bags: vec![
            Bag {num_red: 4, num_green: 0, num_blue: 3},
            Bag {num_red: 1, num_green: 2, num_blue: 6},
            Bag {num_red: 0, num_green: 2, num_blue: 0}]});
    }

    #[test]
    fn test_parse_bags() {
        let bags_string = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let bags = parse_bags(bags_string).unwrap().1;
        assert_eq!(bags, vec![
            Bag {num_red: 4, num_green: 0, num_blue: 3},
            Bag {num_red: 1, num_green: 2, num_blue: 6},
            Bag {num_red: 0, num_green: 2, num_blue: 0}]);
    }

    #[test]
    fn test_parse_bag() {
        let bag_string = "3 blue, 4 red";
        let bag = parse_bag(bag_string).unwrap().1;
        assert_eq!(bag, Bag {num_red: 4, num_green: 0, num_blue: 3});
    }

}
