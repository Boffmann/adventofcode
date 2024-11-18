extern crate nom;
use std::io::{BufRead, BufReader};
use std::fs::File;
use nom::multi::separated_list1;
use nom::bytes::complete::tag;
use nom::sequence::pair;
use nom::combinator::opt;
use nom::{
  IResult,
  character::complete::{digit1},
};

fn read_input(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).expect("cannot open input file"));

    for line in reader.lines() {
        result.push(line.unwrap())
    }
    return result
}

fn main() {
    let input = read_input("input_part1.txt");
    let mut result = 0;
    for line in &input {
        let extrapolated = process_history(parse_history(line).expect("A history").1, 0);
        result = result + extrapolated;
    }
    println!("Result {:?}", result);
}

fn process_history(history: Vec<i64>, depth: i64) -> i64 {
    let mut history_diff: Vec<i64> = Vec::new();
    for index in 0..history.len() - 1 {
        history_diff.push(history.get(index + 1).unwrap() - history.get(index).unwrap());
    }

    let finished: bool = history_diff.clone().into_iter()
        .filter(|entry| *entry != 0)
        .collect::<Vec<i64>>()
        .len() == 0;

    if finished {
        return *history.get(0).unwrap();
    }

    let new_value = process_history(history_diff.clone(), depth + 1);
    let result = history.get(history.len() - 1).unwrap() + new_value;
    result
}

fn parse_history(history_string: &str) -> IResult<&str, Vec<i64>> {
    let (_remainder, history) = separated_list1(tag(" "), pair(opt(tag("-")), digit1))(history_string)
        .map(|parsed| {
            ("", result_to_int(parsed.1))
        })?;
    Ok(("", history))
}

fn result_to_int(input: Vec<(Option<&str>, &str)>) -> Vec<i64> {
    let parsed_vector = input.iter()
        .map(|(sign, entry)| {
            let signed_entry = match sign {
                Some(s) => s.to_string() + entry,
                None => entry.to_string(),
            };
            signed_entry.parse::<i64>().expect("An i64")
        })
        .collect::<Vec<i64>>();
    parsed_vector
}
