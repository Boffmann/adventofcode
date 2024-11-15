extern crate nom;
use std::io::{BufRead, BufReader};
use std::fs::File;
use nom::{
  IResult,
  character::complete::{digit1, space1},
  sequence::separated_pair,
  multi::separated_list1,
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
    let times = parse_times(&input[0]).expect("Valid Times").1;
    let distances = parse_distances(&input[1]).expect("Valid Distances").1;
    assert!(times.len() == distances.len());
    let mut races: Vec<Race> = Vec::new();
    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        });
    }
    let mut result = 1;
    for race in races {
        let mut times_to_win = 0;
        for charge_time in 0..race.time {
            if does_beat(&race, charge_time) {
                times_to_win += 1;
            }
        }
        result *= times_to_win;
    }

    println!("{:?}", result);
}

fn does_beat(race: &Race, charge_time: u64) -> bool {
    let travel_distance = charge_time * (race.time - charge_time);
    if travel_distance > race.distance {
        return true;
    }
    false
}

fn parse_times(time_string: &str) -> IResult<&str, Vec<u64>> {
    assert!(time_string.contains("Time:"));
    let (remainder, times) = separated_pair(tag("Time:"), space1,
        separated_list1(space1, digit1))(time_string)
        .map(|parsed| {
            result_to_int(parsed.1)
        })?;
    Ok((remainder, times))
}

fn parse_distances(time_string: &str) -> IResult<&str, Vec<u64>> {
    assert!(time_string.contains("Distance:"));
    let (remainder, distances) = separated_pair(tag("Distance:"), space1,
        separated_list1(space1, digit1))(time_string)
        .map(|parsed| {
            result_to_int(parsed.1)
        })?;
    Ok((remainder, distances))
}

fn result_to_int<'a>(input: (&'a str, Vec<&str>)) -> (&'a str, Vec<u64>) {
    let parsed_vector = input.1.iter()
        .map(|entry| entry.parse::<u64>().expect("An u64"))
        .collect::<Vec<u64>>();
    (input.0, parsed_vector)
}
