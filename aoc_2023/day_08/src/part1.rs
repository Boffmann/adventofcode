extern crate nom;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::fs::File;
use std::cmp::Ord;
use std::cmp::Ordering;
use nom::multi::separated_list1;
use nom::bytes::complete::tag;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::{
  IResult,
  character::complete::{space1, alpha1, not_line_ending, alphanumeric1},
};

fn read_input(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).expect("cannot open input file"));

    for line in reader.lines() {
        result.push(line.unwrap())
    }
    return result
}

#[derive(Debug)]
struct Crossway {
    key: String,
    left: String,
    right: String,
}

fn main() {
    let input = read_input("input_part1.txt");
    let mut crossways: HashMap<String, Crossway> = HashMap::new();
    let directions: &str = input.get(0).expect("Direction String");
    for line_index in 2..input.len() {
        let crossway = parse_crossway(&input[line_index]).expect("A Crossway").1;
        crossways.insert(crossway.key.clone(), crossway);
    }
    let mut current_crossway = crossways.get("AAA").unwrap();
    let mut direction_index = 0;
    while current_crossway.key != "ZZZ" {
        println!("{:?}", current_crossway);
        if directions.chars().nth(direction_index % directions.len()).unwrap() == 'R' {
            current_crossway = crossways.get(&current_crossway.right).unwrap();
        } else {
            current_crossway = crossways.get(&current_crossway.left).unwrap();
        }
        direction_index = direction_index + 1;
    }
    println!("{:?}", direction_index);
}

fn parse_crossway(crossway_string: &str) -> IResult<&str, Crossway> {
    let (remainder, crossway) = separated_pair(alpha1, tag(" = "),
        delimited(tag("("), separated_pair(alpha1, tag(", "), alpha1), tag(")")))(crossway_string)
        .map(|parsed| {
            let crossway = Crossway {
                key: parsed.1.0.to_string(),
                left: parsed.1.1.0.to_string(),
                right: parsed.1.1.1.to_string(),
            };
            ("", crossway)
        })?;
    Ok(("", crossway))
}
