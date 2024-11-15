extern crate nom;
use std::io::{BufRead, BufReader};
use std::fs::File;
use nom::{
  IResult,
  character::complete::{space1},
  sequence::separated_pair,
  bytes::complete::tag,
};

#[derive(Default, Debug, PartialEq, Eq)]
struct Race {
    time: u64,
    distance: u64,
}

fn read_input(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).expect("cannot open input file"));

    for line in reader.lines() {
        result.push(line.unwrap())
    }
    return result
}

fn main() {
    let input = read_input("./input_part1.txt");
    let time = parse_time(&input[0]).expect("Valid Times").1;
    let distance = parse_distance(&input[1]).expect("Valid Distance").1;
    let race = Race {
        time: time,
        distance: distance,
    };
    let mut times_to_win = 0;
    for charge_time in 0..race.time {
        if does_beat(&race, charge_time) {
            times_to_win += 1;
        }
    }
    println!("{:?}", times_to_win);
}

fn does_beat(race: &Race, charge_time: u64) -> bool {
    let travel_distance = charge_time * (race.time - charge_time);
    if travel_distance > race.distance {
        return true;
    }
    false
}

fn parse_time(time_string: &str) -> IResult<&str, u64> {
    assert!(time_string.contains("Time:"));
    let (remainder, time) = separated_pair(tag("Time:"), space1,
        nom::character::complete::not_line_ending)(time_string)
        .map(|parsed| {
            line_to_int(parsed.1)
        })?;
    Ok((remainder, time))
}

fn parse_distance(time_string: &str) -> IResult<&str, u64> {
    assert!(time_string.contains("Distance:"));
    let (remainder, distance) = separated_pair(tag("Distance:"), space1,
        nom::character::complete::not_line_ending)(time_string)
        .map(|parsed| {
            line_to_int(parsed.1)
        })?;
    Ok((remainder, distance))
}

fn line_to_int<'a>(input: (&'a str, &str)) -> (&'a str, u64) {
    let line_no_whitespace: String = input.1.chars().filter(|c| !c.is_whitespace()).collect();
    let parsed_line = line_no_whitespace.parse::<u64>().expect("An u64");
    (input.0, parsed_line)
}

